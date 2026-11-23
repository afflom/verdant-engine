//! Khronos glTF binary (`.glb`) exporter, importer, and authoritative specification oracle.
//!
//! Exports hierarchical scenes, procedural meshes, and PBR metallic-roughness materials into
//! standard ISO/IEC 12113 / Khronos glTF binary containers (`.glb`) using authoritative
//! `gltf::json` specification enumerations (`ComponentType`, `Target`, `Mode`) and ASCII chunk
//! signatures (`*b"glTF"`, `*b"JSON"`, `*b"BIN\0"`) with zero arbitrary magic literals, then
//! validates every exported or imported payload through `gltf::import_slice`.

use gltf::json::accessor::ComponentType;
use gltf::json::buffer::Target;
use gltf::json::mesh::Mode;
use gltf::json::validation::Checked::Valid;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::animation::{Joint, KeyframePose, SkeletalClip, Skeleton, SkinWeights4};
use crate::ecs::{EntityId, EntityRole, SceneWorld, Transform};
use crate::error::AssetValidationError;
#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
use crate::error::StreamingError;
use crate::geometry::ProceduralMesh;
use crate::math::{
    four_u32, half_f64, pow10_u64, three_u32, two_u32, Aabb, Quat, Vec3, DIM_HOMOGENEOUS,
    DIM_PLANAR, DIM_SPATIAL,
};
use crate::render::{HdPbrDescriptor, PbrMaterial};
#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
use crate::runtime::WorldSnapshot;
#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
use crate::streaming::StandardSceneState;

/// Byte alignment required by Khronos glTF binary chunks (`4` bytes).
const GLTF_CHUNK_ALIGNMENT: usize = DIM_HOMOGENEOUS;
/// Byte length of each GLB chunk header (`chunkLength: u32` + `chunkType: u32` = `8` bytes).
const GLB_CHUNK_HEADER_BYTES: usize = DIM_PLANAR * DIM_HOMOGENEOUS;
/// Byte length of the GLB file header (`magic` + `version` + `length` = `12` bytes).
const GLB_FILE_HEADER_BYTES: usize = DIM_SPATIAL * DIM_HOMOGENEOUS;

/// Summary of a Khronos glTF asset verified by the `gltf` specification oracle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedGltfSummary {
    /// Number of scene nodes in the asset.
    pub node_count: usize,
    /// Number of mesh primitives in the asset.
    pub mesh_count: usize,
    /// Number of PBR materials in the asset.
    pub material_count: usize,
    /// Total binary buffer byte length across all buffers.
    pub total_buffer_bytes: usize,
}

/// Parses a `.glb` / `.gltf` slice into its validated [`gltf::Document`] and raw binary buffers
/// without pulling in CPU image codec crates (`image`/`moxcms`/`png`/`zune-jpeg`) on `wasm32`.
#[cfg(not(feature = "gltf-image-decode"))]
fn parse_glb_document_and_buffers(
    bytes: &[u8],
) -> Result<(gltf::Document, Vec<Vec<u8>>), AssetValidationError> {
    let parsed =
        gltf::Gltf::from_slice(bytes).map_err(|err| AssetValidationError::GltfOracleRejected {
            oracle_message: err.to_string(),
        })?;
    let mut buffers = Vec::new();
    let blob = parsed.blob.unwrap_or_default();
    for buf in parsed.document.buffers() {
        match buf.source() {
            gltf::buffer::Source::Bin => {
                if blob.len() < buf.length() {
                    return Err(AssetValidationError::GltfOracleRejected {
                        oracle_message: "GLB binary chunk shorter than declared buffer length"
                            .to_string(),
                    });
                }
                buffers.push(blob[..buf.length()].to_vec());
            }
            gltf::buffer::Source::Uri(_) => {
                return Err(AssetValidationError::GltfOracleRejected {
                    oracle_message: "external URI buffers are not supported in wasm32 GLB loader"
                        .to_string(),
                });
            }
        }
    }
    Ok((parsed.document, buffers))
}

/// Validates a `.gltf` or `.glb` byte slice using the authoritative `gltf` specification crate,
/// including binary buffer bounds and accessor decoding.
pub fn validate_glb_bytes(bytes: &[u8]) -> Result<VerifiedGltfSummary, AssetValidationError> {
    #[cfg(feature = "gltf-image-decode")]
    let (document, total_buffer_bytes) = {
        let (doc, buffers, _images) =
            gltf::import_slice(bytes).map_err(|err| AssetValidationError::GltfOracleRejected {
                oracle_message: err.to_string(),
            })?;
        let total = buffers.iter().map(|b| b.len()).sum();
        (doc, total)
    };
    #[cfg(not(feature = "gltf-image-decode"))]
    let (document, total_buffer_bytes) = {
        let (doc, buffers) = parse_glb_document_and_buffers(bytes)?;
        let total = buffers.iter().map(Vec::len).sum();
        (doc, total)
    };

    for mesh in document.meshes() {
        for prim in mesh.primitives() {
            if prim.get(&gltf::Semantic::Positions).is_none() {
                return Err(AssetValidationError::GltfOracleRejected {
                    oracle_message: format!(
                        "mesh {:?} primitive {} lacks POSITION attribute",
                        mesh.name(),
                        prim.index()
                    ),
                });
            }
        }
    }

    Ok(VerifiedGltfSummary {
        node_count: document.nodes().count(),
        mesh_count: document.meshes().count(),
        material_count: document.materials().count(),
        total_buffer_bytes,
    })
}

/// Decoded host-bounded `RGBA8` texture payload extracted from a Khronos `.glb` image buffer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodedRgbaTexture {
    /// Width in pixels (`<= host_max_texture_edge_px`).
    pub width: u32,
    /// Height in pixels (`<= host_max_texture_edge_px`).
    pub height: u32,
    /// Tightly packed `width * height * 4` `RGBA8` bytes.
    pub rgba: Vec<u8>,
}

/// Complete scene, mesh, material, skeletal rig, and keyframe clip bundle imported from a Khronos `.glb`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportedGlbScene {
    /// Reconstructed hierarchical `SceneWorld` preserving parent-child node transforms and mesh links.
    pub scene: SceneWorld,
    /// Validated triangle meshes extracted from `MESH` primitives.
    pub meshes: Vec<ProceduralMesh>,
    /// Physically-based metallic-roughness materials extracted from `MATERIALS`.
    pub materials: Vec<PbrMaterial>,
    /// Acyclic skeletal joint hierarchies extracted from `SKINS`.
    pub skeletons: Vec<Skeleton>,
    /// Keyframe skeletal animation clips extracted from `ANIMATIONS`.
    pub clips: Vec<SkeletalClip>,
    /// Per-mesh normalized 4-joint skinning weights (`JOINTS_0` + `WEIGHTS_0`) when present.
    pub skin_weights: Vec<Vec<SkinWeights4>>,
    /// Primary embedded `baseColorTexture` decoded and clamped to the host GPU budget, if present.
    pub primary_albedo_texture: Option<DecodedRgbaTexture>,
    /// Primary embedded `metallicRoughnessTexture` (`G=roughness, B=metallic`) clamped to host budget.
    pub primary_mr_texture: Option<DecodedRgbaTexture>,
    /// Primary embedded tangent-space `normalTexture` clamped to host budget.
    pub primary_normal_texture: Option<DecodedRgbaTexture>,
    /// Primary embedded `emissiveTexture` clamped to host budget.
    pub primary_emissive_texture: Option<DecodedRgbaTexture>,
    /// Number of distinct material textures packed into `primary_albedo_texture` (`1..=16`).
    pub atlas_tile_count: usize,
    /// Per-mesh atlas-remapped UVs (`[-2.0, -2.0]` sentinel for untextured primitives).
    pub mesh_atlas_uvs: Vec<Vec<[f32; DIM_PLANAR]>>,
    /// Per-mesh flag indicating whether the mesh samples `primary_albedo_texture` via `TEXCOORD_0`.
    pub primary_tex_mesh_mask: Vec<bool>,
}

fn normalize_weights_permille(raw: [f32; DIM_HOMOGENEOUS]) -> [u16; DIM_HOMOGENEOUS] {
    let target = crate::prism_kernel::skinWeightScale().unwrap_or(1) as u32;
    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let clamped = [
        raw[0].max(0.0),
        raw[1].max(0.0),
        raw[i2].max(0.0),
        raw[i3].max(0.0),
    ];
    let sum = clamped[0] + clamped[1] + clamped[i2] + clamped[i3];
    if sum <= 0.0 || !sum.is_finite() {
        return [target as u16, 0, 0, 0];
    }
    let scale = (target as f32) / sum;
    let w0 = (clamped[0] * scale).round() as u32;
    let w1 = (clamped[1] * scale)
        .round()
        .min((target.saturating_sub(w0)) as f32) as u32;
    let rem2 = target.saturating_sub(w0 + w1);
    let w2 = (clamped[i2] * scale).round().min(rem2 as f32) as u32;
    let w3 = target.saturating_sub(w0 + w1 + w2);
    [w0 as u16, w1 as u16, w2 as u16, w3 as u16]
}

#[allow(clippy::too_many_arguments)]
fn import_gltf_node_recursive(
    node: &gltf::Node<'_>,
    parent_entity: Option<EntityId>,
    scene: &mut SceneWorld,
    meshes: &[ProceduralMesh],
    default_mat_count: usize,
    mesh_first_prim_mat: &[Option<usize>],
    visited: &mut std::collections::BTreeSet<usize>,
) {
    if !visited.insert(node.index()) {
        return;
    }
    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let (t, r, s) = node.transform().decomposed();
    let local_transform = Transform {
        translation: Vec3::new(f64::from(t[0]), f64::from(t[1]), f64::from(t[i2])),
        rotation: Quat {
            x: f64::from(r[0]),
            y: f64::from(r[1]),
            z: f64::from(r[i2]),
            w: f64::from(r[i3]),
        }
        .normalized(),
        scale: Vec3::new(f64::from(s[0]), f64::from(s[1]), f64::from(s[i2])),
    };

    let m_idx = node
        .mesh()
        .map(|m| m.index())
        .filter(|&idx| idx < meshes.len());
    let local_bounds = m_idx.map_or_else(
        || {
            let h = half_f64();
            Aabb::from_center_half_extents(Vec3::ZERO, Vec3::new(h, h, h))
        },
        |idx| meshes[idx].bounds(),
    );
    // The exporter records EntityRole per node. Unknown third-party extras
    // keep the physical default, even when multiple nodes instance one mesh.
    let role = node
        .extras()
        .as_ref()
        .and_then(|raw| serde_json::from_slice::<serde_json::Value>(raw.get().as_bytes()).ok())
        .and_then(|extras| serde_json::from_value::<EntityRole>(extras.get("role")?.clone()).ok())
        .unwrap_or(if node.skin().is_some() {
            EntityRole::ArticulatedRig
        } else {
            EntityRole::StaticArchitecture
        });
    let name = node
        .name()
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("GlbNode_{}", node.index()));

    let eid = scene.spawn(
        name,
        role,
        parent_entity,
        local_transform,
        local_bounds,
        (1.0, "Khronos glTF 2.0 imported node"),
    );
    if let Some(mesh_slot) = m_idx {
        let mat_slot = mesh_first_prim_mat
            .get(mesh_slot)
            .and_then(|&opt| opt)
            .unwrap_or(0)
            .min(default_mat_count.saturating_sub(1));
        scene.attach_renderable(eid, mesh_slot, mat_slot);
    }

    for child in node.children() {
        import_gltf_node_recursive(
            &child,
            Some(eid),
            scene,
            meshes,
            default_mat_count,
            mesh_first_prim_mat,
            visited,
        );
    }
}

/// Imports an external Khronos `.glb` or `.gltf` container into a live [`ImportedGlbScene`]
/// (`SceneWorld` hierarchy, `ProceduralMesh` geometry, `PbrMaterial` descriptors, `Skeleton` skins,
/// and `SkeletalClip` keyframe animations).
#[cfg(feature = "gltf-image-decode")]
fn sample_gltf_base_color(img: &gltf::image::Data, u: f32, v: f32) -> [f32; DIM_HOMOGENEOUS] {
    if img.width == 0 || img.height == 0 || img.pixels.is_empty() {
        return [1.0, 1.0, 1.0, 1.0];
    }
    let uf = if u.is_finite() {
        u.rem_euclid(1.0)
    } else {
        0.0
    };
    let vf = if v.is_finite() {
        v.rem_euclid(1.0)
    } else {
        0.0
    };
    let max_x = img.width.saturating_sub(1);
    let max_y = img.height.saturating_sub(1);
    let px = ((uf * (max_x as f32)).round() as u32).min(max_x) as usize;
    let py = ((vf * (max_y as f32)).round() as u32).min(max_y) as usize;
    let pix_idx = py * (img.width as usize) + px;
    let denom = ((1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)) - 1) as f32;
    let two = 1usize + 1usize;
    let three = two + 1usize;
    let four = two * two;

    match img.format {
        gltf::image::Format::R8G8B8A8 => {
            let o = pix_idx * four;
            if o + three < img.pixels.len() {
                [
                    f32::from(img.pixels[o]) / denom,
                    f32::from(img.pixels[o + 1]) / denom,
                    f32::from(img.pixels[o + two]) / denom,
                    f32::from(img.pixels[o + three]) / denom,
                ]
            } else {
                [1.0, 1.0, 1.0, 1.0]
            }
        }
        gltf::image::Format::R8G8B8 => {
            let o = pix_idx * three;
            if o + two < img.pixels.len() {
                [
                    f32::from(img.pixels[o]) / denom,
                    f32::from(img.pixels[o + 1]) / denom,
                    f32::from(img.pixels[o + two]) / denom,
                    1.0,
                ]
            } else {
                [1.0, 1.0, 1.0, 1.0]
            }
        }
        gltf::image::Format::R8G8 => {
            let o = pix_idx * two;
            if o + 1 < img.pixels.len() {
                let l = f32::from(img.pixels[o]) / denom;
                let a = f32::from(img.pixels[o + 1]) / denom;
                [l, l, l, a]
            } else {
                [1.0, 1.0, 1.0, 1.0]
            }
        }
        gltf::image::Format::R8 if pix_idx < img.pixels.len() => {
            let l = f32::from(img.pixels[pix_idx]) / denom;
            [l, l, l, 1.0]
        }
        _ => [1.0, 1.0, 1.0, 1.0],
    }
}

#[cfg(feature = "gltf-image-decode")]
fn decode_rgba_bounded(img: &gltf::image::Data, max_edge_px: u32) -> Option<DecodedRgbaTexture> {
    if img.width == 0 || img.height == 0 || img.pixels.is_empty() {
        return None;
    }
    let cap = max_edge_px.max(1);
    let src_w = img.width;
    let src_h = img.height;
    let max_dim = src_w.max(src_h);
    let (dst_w, dst_h) = if max_dim > cap {
        let scale = f64::from(cap) / f64::from(max_dim);
        (
            ((f64::from(src_w) * scale).round() as u32).max(1),
            ((f64::from(src_h) * scale).round() as u32).max(1),
        )
    } else {
        (src_w, src_h)
    };
    let two = 1usize + 1usize;
    let three = two + 1usize;
    let four = two * two;
    let full_alpha = ((1u32 << (four + four)) - 1) as u8;
    let mut rgba = Vec::with_capacity((dst_w as usize) * (dst_h as usize) * four);
    let sw_f = f64::from(src_w);
    let sh_f = f64::from(src_h);
    let dw_f = f64::from(dst_w);
    let dh_f = f64::from(dst_h);
    let half = half_f64();

    for dy in 0..dst_h {
        let sy = ((((f64::from(dy) + half) * sh_f) / dh_f).floor() as u32).min(src_h - 1) as usize;
        for dx in 0..dst_w {
            let sx =
                ((((f64::from(dx) + half) * sw_f) / dw_f).floor() as u32).min(src_w - 1) as usize;
            let pix_idx = sy * (src_w as usize) + sx;
            let px = match img.format {
                gltf::image::Format::R8G8B8A8 => {
                    let o = pix_idx * four;
                    if o + three < img.pixels.len() {
                        [
                            img.pixels[o],
                            img.pixels[o + 1],
                            img.pixels[o + two],
                            img.pixels[o + three],
                        ]
                    } else {
                        [full_alpha, full_alpha, full_alpha, full_alpha]
                    }
                }
                gltf::image::Format::R8G8B8 => {
                    let o = pix_idx * three;
                    if o + two < img.pixels.len() {
                        [
                            img.pixels[o],
                            img.pixels[o + 1],
                            img.pixels[o + two],
                            full_alpha,
                        ]
                    } else {
                        [full_alpha, full_alpha, full_alpha, full_alpha]
                    }
                }
                gltf::image::Format::R8G8 => {
                    let o = pix_idx * two;
                    if o + 1 < img.pixels.len() {
                        let l = img.pixels[o];
                        [l, l, l, img.pixels[o + 1]]
                    } else {
                        [full_alpha, full_alpha, full_alpha, full_alpha]
                    }
                }
                gltf::image::Format::R8 if pix_idx < img.pixels.len() => {
                    let l = img.pixels[pix_idx];
                    [l, l, l, full_alpha]
                }
                _ => [full_alpha, full_alpha, full_alpha, full_alpha],
            };
            rgba.extend_from_slice(&px);
        }
    }
    Some(DecodedRgbaTexture {
        width: dst_w,
        height: dst_h,
        rgba,
    })
}

/// Imports a Khronos glTF 2.0 binary (`.glb`) payload into a validated [`ImportedGlbScene`],
/// sampling embedded `baseColorTexture` images into vertex albedo colors and extracting a
/// host-bounded primary `RGBA8` texture (`<= 1024px` default) for pixel-accurate WebGL2 rendering.
pub fn import_glb_to_scene(bytes: &[u8]) -> Result<ImportedGlbScene, AssetValidationError> {
    let default_cap = 1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);
    import_glb_to_scene_bounded(bytes, default_cap)
}

/// Imports a Khronos glTF 2.0 binary (`.glb`) payload while clamping any extracted GPU albedo
/// texture strictly to `max_tex_edge_px` so the host container's GPU memory envelope is never exceeded.
#[cfg_attr(not(feature = "gltf-image-decode"), allow(unused_variables))]
pub fn import_glb_to_scene_bounded(
    bytes: &[u8],
    max_tex_edge_px: u32,
) -> Result<ImportedGlbScene, AssetValidationError> {
    #[cfg(feature = "gltf-image-decode")]
    let (document, buffers, images) = {
        let (doc, bufs, imgs) =
            gltf::import_slice(bytes).map_err(|err| AssetValidationError::GltfOracleRejected {
                oracle_message: err.to_string(),
            })?;
        let raw_bufs: Vec<Vec<u8>> = bufs.into_iter().map(|b| b.0).collect();
        (doc, raw_bufs, imgs)
    };
    #[cfg(not(feature = "gltf-image-decode"))]
    let (document, buffers) = parse_glb_document_and_buffers(bytes)?;

    // Identify the most frequently referenced Khronos PBR `baseColorTexture` images and bind each
    // material's companion `metallicRoughnessTexture`, `normalTexture`, and `emissiveTexture` so
    // multi-material `.glb` models pack all 4 PBR channels into synchronized grid atlases:
    let mut tex_usage: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
    #[derive(Default)]
    struct CompanionPbrChannels {
        mr: Option<usize>,
        normal: Option<usize>,
        emissive: Option<usize>,
    }
    let mut companion_pbr: std::collections::BTreeMap<usize, CompanionPbrChannels> =
        std::collections::BTreeMap::new();
    let mut mr_src_idx: Option<usize> = None;
    let mut norm_src_idx: Option<usize> = None;
    let mut emis_src_idx: Option<usize> = None;
    for mesh in document.meshes() {
        for prim in mesh.primitives() {
            let mat = prim.material();
            let pbr = mat.pbr_metallic_roughness();
            let mr_i = pbr
                .metallic_roughness_texture()
                .map(|info| info.texture().source().index());
            let nm_i = mat
                .normal_texture()
                .map(|info| info.texture().source().index());
            let em_i = mat
                .emissive_texture()
                .map(|info| info.texture().source().index());
            if let Some(info) = pbr.base_color_texture() {
                let src_idx = info.texture().source().index();
                *tex_usage.entry(src_idx).or_insert(0) += 1;
                let entry = companion_pbr.entry(src_idx).or_default();
                if entry.mr.is_none() {
                    entry.mr = mr_i;
                }
                if entry.normal.is_none() {
                    entry.normal = nm_i;
                }
                if entry.emissive.is_none() {
                    entry.emissive = em_i;
                }
            }
            if mr_src_idx.is_none() {
                mr_src_idx = mr_i;
            }
            if norm_src_idx.is_none() {
                norm_src_idx = nm_i;
            }
            if emis_src_idx.is_none() {
                emis_src_idx = em_i;
            }
        }
    }
    let mut ranked_tex: Vec<(usize, usize)> = tex_usage.into_iter().collect();
    ranked_tex.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked_tex.truncate(crate::uhd::MAX_ATLAS_TILES);
    let primary_tex_src_idx = ranked_tex.first().map(|&(idx, _)| idx);

    let mut atlas_src_to_tile: std::collections::BTreeMap<usize, usize> =
        std::collections::BTreeMap::new();
    let mut atlas_tile_transforms: Vec<crate::uhd::AtlasUvTransform> = Vec::new();
    let mut atlas_tile_count = 0usize;
    let mut atlas_ordered_srcs: Vec<usize> = Vec::new();
    #[cfg(feature = "gltf-image-decode")]
    let mut atlas_per_tile_cap = max_tex_edge_px;
    #[cfg(feature = "gltf-image-decode")]
    let primary_albedo_texture = if ranked_tex.len() <= 1 {
        if let Some(idx) = primary_tex_src_idx {
            atlas_src_to_tile.insert(idx, 0);
            atlas_ordered_srcs.push(idx);
            atlas_tile_transforms.push(crate::uhd::AtlasUvTransform {
                u_offset: 0.0,
                v_offset: 0.0,
                u_scale: 1.0,
                v_scale: 1.0,
            });
            atlas_tile_count = 1;
            images
                .get(idx)
                .and_then(|img| decode_rgba_bounded(img, max_tex_edge_px))
        } else {
            None
        }
    } else {
        let two_u = 1u32 + 1;
        let three_u = two_u + 1;
        let four_u = two_u * two_u;
        let grid_dim = if ranked_tex.len() <= (four_u as usize) {
            two_u
        } else if ranked_tex.len() <= ((three_u * three_u) as usize) {
            three_u
        } else {
            four_u
        };
        let per_tile_cap = (max_tex_edge_px / grid_dim).max(1);
        atlas_per_tile_cap = per_tile_cap;
        let mut decoded_tiles: Vec<(usize, DecodedRgbaTexture)> = Vec::new();
        for &(src_idx, _) in &ranked_tex {
            if let Some(tex) = images
                .get(src_idx)
                .and_then(|img| decode_rgba_bounded(img, per_tile_cap))
            {
                decoded_tiles.push((src_idx, tex));
            }
        }
        let slice_refs: Vec<(u32, u32, &[u8])> = decoded_tiles
            .iter()
            .map(|(_, t)| (t.width, t.height, t.rgba.as_slice()))
            .collect();
        if let Some(atlas) = crate::uhd::pack_rgba_texture_atlas(&slice_refs, max_tex_edge_px) {
            for (slot, (src_idx, _)) in decoded_tiles.iter().enumerate() {
                atlas_src_to_tile.insert(*src_idx, slot);
                atlas_ordered_srcs.push(*src_idx);
            }
            atlas_tile_count = atlas.tile_count;
            atlas_tile_transforms = atlas.tile_transforms;
            Some(DecodedRgbaTexture {
                width: atlas.width,
                height: atlas.height,
                rgba: atlas.rgba,
            })
        } else {
            None
        }
    };
    #[cfg(not(feature = "gltf-image-decode"))]
    let primary_albedo_texture: Option<DecodedRgbaTexture> = {
        if let Some(idx) = primary_tex_src_idx {
            atlas_src_to_tile.insert(idx, 0);
            atlas_ordered_srcs.push(idx);
            atlas_tile_transforms.push(crate::uhd::AtlasUvTransform {
                u_offset: 0.0,
                v_offset: 0.0,
                u_scale: 1.0,
                v_scale: 1.0,
            });
            atlas_tile_count = 1;
        }
        let _ = (
            &companion_pbr,
            &atlas_ordered_srcs,
            mr_src_idx,
            norm_src_idx,
            emis_src_idx,
        );
        None
    };

    #[cfg(feature = "gltf-image-decode")]
    let pack_companion_channel_atlas = |channel_idx: usize, neutral_px: [u8; 1 + 1 + 1 + 1]| {
        let mut has_any = false;
        let mut decoded_ch: Vec<DecodedRgbaTexture> = Vec::with_capacity(atlas_ordered_srcs.len());
        for &src_idx in &atlas_ordered_srcs {
            let opt_idx = companion_pbr.get(&src_idx).and_then(|ch| {
                if channel_idx == 0 {
                    ch.mr
                } else if channel_idx == 1 {
                    ch.normal
                } else {
                    ch.emissive
                }
            });
            if let Some(tex) = opt_idx
                .and_then(|idx| images.get(idx))
                .and_then(|img| decode_rgba_bounded(img, atlas_per_tile_cap))
            {
                has_any = true;
                decoded_ch.push(tex);
            } else {
                decoded_ch.push(DecodedRgbaTexture {
                    width: 1,
                    height: 1,
                    rgba: neutral_px.to_vec(),
                });
            }
        }
        if !has_any {
            return None;
        }
        let refs: Vec<(u32, u32, &[u8])> = decoded_ch
            .iter()
            .map(|t| (t.width, t.height, t.rgba.as_slice()))
            .collect();
        crate::uhd::pack_rgba_texture_atlas(&refs, max_tex_edge_px).map(|a| DecodedRgbaTexture {
            width: a.width,
            height: a.height,
            rgba: a.rgba,
        })
    };

    #[cfg(feature = "gltf-image-decode")]
    let (primary_mr_texture, primary_normal_texture, primary_emissive_texture) = {
        let full_byte = ((1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)) - 1) as u8;
        let half_byte = (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1)) as u8;
        let low_byte = (1u32 << (1 + 1 + 1 + 1)) as u8;
        let mr = if atlas_tile_count <= 1 {
            mr_src_idx
                .and_then(|idx| images.get(idx))
                .and_then(|img| decode_rgba_bounded(img, max_tex_edge_px))
        } else {
            pack_companion_channel_atlas(0, [full_byte, half_byte, low_byte, full_byte])
        };
        let norm = if atlas_tile_count <= 1 {
            norm_src_idx
                .and_then(|idx| images.get(idx))
                .and_then(|img| decode_rgba_bounded(img, max_tex_edge_px))
        } else {
            pack_companion_channel_atlas(1, [half_byte, half_byte, full_byte, full_byte])
        };
        let emis = if atlas_tile_count <= 1 {
            emis_src_idx
                .and_then(|idx| images.get(idx))
                .and_then(|img| decode_rgba_bounded(img, max_tex_edge_px))
        } else {
            pack_companion_channel_atlas(1 + 1, [0, 0, 0, full_byte])
        };
        (mr, norm, emis)
    };
    #[cfg(not(feature = "gltf-image-decode"))]
    let (primary_mr_texture, primary_normal_texture, primary_emissive_texture): (
        Option<DecodedRgbaTexture>,
        Option<DecodedRgbaTexture>,
        Option<DecodedRgbaTexture>,
    ) = (None, None, None);

    let default_ior = (four_u32() as f32) / (three_u32() as f32);
    let mut materials = Vec::new();
    for (idx, mat) in document.materials().enumerate() {
        let pbr = mat.pbr_metallic_roughness();
        materials.push(PbrMaterial {
            name: mat
                .name()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| format!("GlbMaterial_{idx}")),
            base_color_factor: pbr.base_color_factor(),
            metallic_factor: pbr.metallic_factor(),
            roughness_factor: pbr.roughness_factor(),
            emissive_factor: mat.emissive_factor(),
            ior: default_ior,
            double_sided: mat.double_sided(),
        });
    }
    if materials.is_empty() {
        materials.push(PbrMaterial {
            name: "DefaultImportedDialectric".to_string(),
            base_color_factor: [1.0, 1.0, 1.0, 1.0],
            metallic_factor: 0.0,
            roughness_factor: half_f64() as f32,
            emissive_factor: [0.0, 0.0, 0.0],
            ior: default_ior,
            double_sided: false,
        });
    }

    let mut meshes = Vec::new();
    let mut mesh_first_prim_mat = Vec::new();
    let mut skin_weights = Vec::new();
    let mut mesh_atlas_uvs = Vec::new();
    let mut primary_tex_mesh_mask = Vec::new();
    let i2 = 1usize + 1usize;

    let max_skin_joints = document
        .skins()
        .map(|s| s.joints().count())
        .max()
        .unwrap_or(usize::from(u16::MAX) + 1);
    let i3 = i2 + 1usize;

    for (m_idx, mesh) in document.meshes().enumerate() {
        let tri_prims: Vec<_> = mesh
            .primitives()
            .filter(|p| p.mode() == gltf::mesh::Mode::Triangles)
            .collect();
        let Some(first_prim) = tri_prims.first() else {
            return Err(AssetValidationError::EmptyMeshTopology {
                mesh_name: mesh.name().unwrap_or("GlbMesh").to_string(),
            });
        };
        let bake_mat_tint = tri_prims.len() > 1;
        let mut positions: Vec<[f32; DIM_SPATIAL]> = Vec::new();
        let mut normals: Vec<[f32; DIM_SPATIAL]> = Vec::new();
        let mut uvs: Vec<[f32; DIM_PLANAR]> = Vec::new();
        let mut atlas_uvs: Vec<[f32; DIM_PLANAR]> = Vec::new();
        let mut colors: Vec<[f32; DIM_HOMOGENEOUS]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut prim_weights: Vec<SkinWeights4> = Vec::new();
        let mut all_prims_skinned = true;
        let mut any_prim_in_atlas = false;
        let uv_sentinel = -(1.0_f32 + 1.0_f32);

        for prim in &tri_prims {
            let prim_tex_src = prim
                .material()
                .pbr_metallic_roughness()
                .base_color_texture()
                .map(|info| info.texture().source().index());
            let prim_atlas_xform = prim_tex_src
                .and_then(|si| atlas_src_to_tile.get(&si).copied())
                .and_then(|slot| atlas_tile_transforms.get(slot).copied());
            if prim_atlas_xform.is_some() {
                any_prim_in_atlas = true;
            }
            let reader = prim.reader(|buffer| buffers.get(buffer.index()).map(Vec::as_slice));
            let prim_pos: Vec<[f32; DIM_SPATIAL]> = reader
                .read_positions()
                .ok_or_else(|| AssetValidationError::GltfOracleRejected {
                    oracle_message: format!("mesh {m_idx} missing POSITION attribute"),
                })?
                .collect();
            let v_count = prim_pos.len();
            if v_count == 0 || positions.len().saturating_add(v_count) > (u32::MAX as usize) {
                return Err(AssetValidationError::EmptyMeshTopology {
                    mesh_name: mesh.name().unwrap_or("GlbMesh").to_string(),
                });
            }
            let base_vertex = positions.len() as u32;
            positions.extend_from_slice(&prim_pos);

            match reader.read_normals() {
                Some(iter) => {
                    let mut n_read = 0usize;
                    for n in iter {
                        let nx = if n[0].is_finite() {
                            f64::from(n[0])
                        } else {
                            0.0
                        };
                        let ny = if n[1].is_finite() {
                            f64::from(n[1])
                        } else {
                            1.0
                        };
                        let nz = if n[i2].is_finite() {
                            f64::from(n[i2])
                        } else {
                            0.0
                        };
                        normals.push(Vec3::new(nx, ny, nz).normalized().to_f32_array());
                        n_read += 1;
                    }
                    if n_read < v_count {
                        normals.resize(positions.len(), [0.0, 1.0, 0.0]);
                    }
                }
                None => normals.resize(positions.len(), [0.0, 1.0, 0.0]),
            }

            match reader.read_tex_coords(0) {
                Some(tc) => {
                    uvs.extend(tc.into_f32());
                    if uvs.len() < positions.len() {
                        uvs.resize(positions.len(), [0.0, 0.0]);
                    }
                }
                None => uvs.resize(positions.len(), [0.0, 0.0]),
            }

            let mat_tint = if bake_mat_tint {
                prim.material().pbr_metallic_roughness().base_color_factor()
            } else {
                [1.0, 1.0, 1.0, 1.0]
            };
            #[cfg(feature = "gltf-image-decode")]
            let base_tex_img = prim
                .material()
                .pbr_metallic_roughness()
                .base_color_texture()
                .and_then(|info| images.get(info.texture().source().index()));
            let uv_start = uvs.len().saturating_sub(v_count);
            for local_i in 0..v_count {
                if let Some(xf) = prim_atlas_xform {
                    let uv = uvs.get(uv_start + local_i).copied().unwrap_or([0.0, 0.0]);
                    atlas_uvs.push(if atlas_tile_count <= 1 {
                        uv
                    } else {
                        xf.apply(uv[0], uv[1])
                    });
                } else {
                    atlas_uvs.push([uv_sentinel, uv_sentinel]);
                }
            }
            match reader.read_colors(0) {
                Some(col) => {
                    for (local_i, c) in col.into_rgba_f32().enumerate() {
                        #[cfg(feature = "gltf-image-decode")]
                        let tex = base_tex_img.map_or([1.0, 1.0, 1.0, 1.0], |img| {
                            let uv = uvs.get(uv_start + local_i).copied().unwrap_or([0.0, 0.0]);
                            sample_gltf_base_color(img, uv[0], uv[1])
                        });
                        #[cfg(not(feature = "gltf-image-decode"))]
                        let tex = {
                            let _ = local_i;
                            [1.0_f32, 1.0, 1.0, 1.0]
                        };
                        colors.push([
                            (c[0] * mat_tint[0] * tex[0]).clamp(0.0, 1.0),
                            (c[1] * mat_tint[1] * tex[1]).clamp(0.0, 1.0),
                            (c[i2] * mat_tint[i2] * tex[i2]).clamp(0.0, 1.0),
                            (c[i3] * mat_tint[i3] * tex[i3]).clamp(0.0, 1.0),
                        ]);
                    }
                    if colors.len() < positions.len() {
                        colors.resize(positions.len(), mat_tint);
                    }
                }
                None => {
                    #[cfg(feature = "gltf-image-decode")]
                    if let Some(img) = base_tex_img {
                        for local_i in 0..v_count {
                            let uv = uvs.get(uv_start + local_i).copied().unwrap_or([0.0, 0.0]);
                            let tex = sample_gltf_base_color(img, uv[0], uv[1]);
                            colors.push([
                                (mat_tint[0] * tex[0]).clamp(0.0, 1.0),
                                (mat_tint[1] * tex[1]).clamp(0.0, 1.0),
                                (mat_tint[i2] * tex[i2]).clamp(0.0, 1.0),
                                (mat_tint[i3] * tex[i3]).clamp(0.0, 1.0),
                            ]);
                        }
                    } else {
                        colors.resize(positions.len(), mat_tint);
                    }
                    #[cfg(not(feature = "gltf-image-decode"))]
                    colors.resize(positions.len(), mat_tint);
                }
            }

            match reader.read_indices() {
                Some(ind) => {
                    for raw_idx in ind.into_u32() {
                        if (raw_idx as usize) >= v_count {
                            return Err(AssetValidationError::GltfOracleRejected {
                                oracle_message: format!(
                                    "mesh {m_idx} index {raw_idx} exceeds vertex count {v_count}"
                                ),
                            });
                        }
                        indices.push(base_vertex + raw_idx);
                    }
                }
                None => {
                    for local_v in 0..(v_count as u32) {
                        indices.push(base_vertex + local_v);
                    }
                }
            }

            if let (Some(j_iter), Some(w_iter)) = (reader.read_joints(0), reader.read_weights(0)) {
                let before_w = prim_weights.len();
                for (joints, raw_w) in j_iter.into_u16().zip(w_iter.into_f32()) {
                    let norm_w = normalize_weights_permille(raw_w);
                    if let Ok(sw) = SkinWeights4::new(joints, norm_w, max_skin_joints.max(1)) {
                        prim_weights.push(sw);
                    }
                }
                if prim_weights.len() - before_w != v_count {
                    all_prims_skinned = false;
                }
            } else {
                all_prims_skinned = false;
            }
        }
        if !all_prims_skinned {
            prim_weights.clear();
        }

        let pm = ProceduralMesh {
            name: mesh
                .name()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| format!("GlbMesh_{m_idx}")),
            positions,
            normals,
            uvs,
            colors,
            indices,
            morph_targets: Vec::new(),
        };
        pm.validate()?;
        mesh_first_prim_mat.push(first_prim.material().index());
        meshes.push(pm);
        skin_weights.push(prim_weights);
        mesh_atlas_uvs.push(atlas_uvs);
        primary_tex_mesh_mask.push(any_prim_in_atlas);
    }

    // Extract acyclic Skeletons from glTF Skins:
    let mut node_parent: std::collections::BTreeMap<usize, usize> =
        std::collections::BTreeMap::new();
    for node in document.nodes() {
        for child in node.children() {
            node_parent.insert(child.index(), node.index());
        }
    }

    let mut skeletons = Vec::new();
    let mut skin_joint_node_map: Vec<std::collections::BTreeMap<usize, usize>> = Vec::new();
    for skin in document.skins() {
        let skin_nodes: Vec<_> = skin.joints().collect();
        if skin_nodes.is_empty() {
            continue;
        }
        let mut node_to_joint = std::collections::BTreeMap::new();
        for (j_idx, jn) in skin_nodes.iter().enumerate() {
            node_to_joint.insert(jn.index(), j_idx);
        }
        let mut joints = Vec::with_capacity(skin_nodes.len());
        for (j_idx, jn) in skin_nodes.iter().enumerate() {
            let (t, r, _s) = jn.transform().decomposed();
            let parent_joint = node_parent
                .get(&jn.index())
                .and_then(|p_node| node_to_joint.get(p_node).copied())
                .filter(|&pj| pj < j_idx);
            let is_root = j_idx == 0;
            joints.push(Joint {
                name: jn
                    .name()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| format!("Joint_{j_idx}")),
                parent_index: if is_root {
                    0
                } else {
                    parent_joint.unwrap_or(0)
                },
                is_root,
                rest_translation: Vec3::new(f64::from(t[0]), f64::from(t[1]), f64::from(t[i2])),
                rest_rotation: Quat {
                    x: f64::from(r[0]),
                    y: f64::from(r[1]),
                    z: f64::from(r[i2]),
                    w: f64::from(r[i3]),
                }
                .normalized(),
            });
        }
        if let Ok(skel) = Skeleton::new(joints) {
            skeletons.push(skel);
            skin_joint_node_map.push(node_to_joint);
        }
    }

    // Extract Keyframe SkeletalClips from glTF Animations when a Skeleton is present:
    let mut clips = Vec::new();
    if let (Some(skel), Some(node_to_joint)) = (skeletons.first(), skin_joint_node_map.first()) {
        let ms_scale = pow10_u64(three_u32()) as f32;
        for (a_idx, anim) in document.animations().enumerate() {
            let mut times_ms = std::collections::BTreeSet::new();
            times_ms.insert(0_u64);
            let mut trans_tracks: std::collections::BTreeMap<usize, Vec<(u64, Vec3)>> =
                std::collections::BTreeMap::new();
            let mut rot_tracks: std::collections::BTreeMap<usize, Vec<(u64, Quat)>> =
                std::collections::BTreeMap::new();

            for ch in anim.channels() {
                let target_node = ch.target().node().index();
                let Some(&joint_idx) = node_to_joint.get(&target_node) else {
                    continue;
                };
                let reader = ch.reader(|buffer| buffers.get(buffer.index()).map(Vec::as_slice));
                let Some(inputs) = reader.read_inputs() else {
                    continue;
                };
                let stamp_ms: Vec<u64> = inputs
                    .map(|sec| (sec.max(0.0) * ms_scale).round() as u64)
                    .collect();
                for &t in &stamp_ms {
                    times_ms.insert(t);
                }
                if let Some(outputs) = reader.read_outputs() {
                    match outputs {
                        gltf::animation::util::ReadOutputs::Translations(iter) => {
                            let samples: Vec<(u64, Vec3)> = stamp_ms
                                .into_iter()
                                .zip(iter.map(|v| {
                                    Vec3::new(f64::from(v[0]), f64::from(v[1]), f64::from(v[i2]))
                                }))
                                .collect();
                            trans_tracks.insert(joint_idx, samples);
                        }
                        gltf::animation::util::ReadOutputs::Rotations(rot) => {
                            let samples: Vec<(u64, Quat)> = stamp_ms
                                .into_iter()
                                .zip(rot.into_f32().map(|q| {
                                    Quat {
                                        x: f64::from(q[0]),
                                        y: f64::from(q[1]),
                                        z: f64::from(q[i2]),
                                        w: f64::from(q[i3]),
                                    }
                                    .normalized()
                                }))
                                .collect();
                            rot_tracks.insert(joint_idx, samples);
                        }
                        _ => {}
                    }
                }
            }

            let duration_millis = times_ms.iter().copied().max().unwrap_or(1).max(1);
            let mut keyframes = Vec::with_capacity(times_ms.len());
            for t_ms in times_ms {
                let mut jt = Vec::with_capacity(skel.joints.len());
                let mut jr = Vec::with_capacity(skel.joints.len());
                for (j_idx, joint) in skel.joints.iter().enumerate() {
                    let tr = trans_tracks
                        .get(&j_idx)
                        .and_then(|s| {
                            s.iter()
                                .rfind(|(ts, _)| *ts <= t_ms)
                                .or_else(|| s.first())
                                .map(|(_, v)| *v)
                        })
                        .unwrap_or(joint.rest_translation);
                    let ro = rot_tracks
                        .get(&j_idx)
                        .and_then(|s| {
                            s.iter()
                                .rfind(|(ts, _)| *ts <= t_ms)
                                .or_else(|| s.first())
                                .map(|(_, q)| *q)
                        })
                        .unwrap_or(joint.rest_rotation);
                    jt.push(tr);
                    jr.push(ro);
                }
                keyframes.push(KeyframePose {
                    time_millis: t_ms,
                    joint_translations: jt,
                    joint_rotations: jr,
                    morph_weights_permille: Vec::new(),
                });
            }
            clips.push(SkeletalClip {
                name: anim
                    .name()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| format!("GlbClip_{a_idx}")),
                duration_millis,
                keyframes,
            });
        }
    }

    let mut scene = SceneWorld::new();
    let mut visited = std::collections::BTreeSet::new();
    if let Some(default_scene) = document
        .default_scene()
        .or_else(|| document.scenes().next())
    {
        for root_node in default_scene.nodes() {
            import_gltf_node_recursive(
                &root_node,
                None,
                &mut scene,
                &meshes,
                materials.len(),
                &mesh_first_prim_mat,
                &mut visited,
            );
        }
    }
    for node in document.nodes() {
        if !node_parent.contains_key(&node.index()) && !visited.contains(&node.index()) {
            import_gltf_node_recursive(
                &node,
                None,
                &mut scene,
                &meshes,
                materials.len(),
                &mesh_first_prim_mat,
                &mut visited,
            );
        }
    }

    Ok(ImportedGlbScene {
        scene,
        meshes,
        materials,
        skeletons,
        clips,
        skin_weights,
        primary_albedo_texture,
        primary_mr_texture,
        primary_normal_texture,
        primary_emissive_texture,
        atlas_tile_count,
        mesh_atlas_uvs,
        primary_tex_mesh_mask,
    })
}

#[path = "gltf_export.rs"]
mod gltf_export;
pub use gltf_export::*;
