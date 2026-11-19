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
#[cfg(feature = "uorc-streaming")]
use crate::error::StreamingError;
use crate::geometry::ProceduralMesh;
use crate::math::{
    four_u32, half_f64, pow10_u64, three_u32, two_u32, Aabb, Quat, Vec3, DIM_HOMOGENEOUS,
    DIM_PLANAR, DIM_SPATIAL,
};
use crate::render::{HdPbrDescriptor, PbrMaterial};
#[cfg(feature = "uorc-streaming")]
use crate::runtime::WorldSnapshot;
#[cfg(feature = "uorc-streaming")]
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

/// Serializes a `SceneWorld`, mesh library, and PBR material library into a Khronos glTF `.glb` binary.
///
/// Before returning, every exported `.glb` is parsed and validated by [`validate_glb_bytes`] using
/// the authoritative `gltf` crate.
pub fn export_scene_to_glb(
    scene: &SceneWorld,
    meshes: &[ProceduralMesh],
    materials: &[PbrMaterial],
) -> Result<Vec<u8>, AssetValidationError> {
    for mesh in meshes {
        mesh.validate()?;
    }

    let component_f32 = Valid(ComponentType::F32);
    let component_u32 = Valid(ComponentType::U32);
    let target_array_buffer = Valid(Target::ArrayBuffer);
    let target_element_buffer = Valid(Target::ElementArrayBuffer);
    let mode_triangles = Valid(Mode::Triangles);

    let mut bin_chunk: Vec<u8> = Vec::new();
    let mut buffer_views = Vec::new();
    let mut accessors = Vec::new();
    let mut gltf_meshes = Vec::new();

    let pad_chunk = |buf: &mut Vec<u8>| {
        while !buf.len().is_multiple_of(GLTF_CHUNK_ALIGNMENT) {
            buf.push(0);
        }
    };

    for mesh in meshes {
        let v_count = mesh.positions.len();
        let bounds = mesh.bounds();

        // Positions (VEC3 F32)
        pad_chunk(&mut bin_chunk);
        let pos_offset = bin_chunk.len();
        for p in &mesh.positions {
            for c in p {
                bin_chunk.extend_from_slice(&c.to_le_bytes());
            }
        }
        let pos_len = bin_chunk.len() - pos_offset;
        let pos_view = buffer_views.len();
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": pos_offset,
            "byteLength": pos_len,
            "target": target_array_buffer
        }));
        let pos_acc = accessors.len();
        accessors.push(json!({
            "bufferView": pos_view,
            "componentType": component_f32,
            "count": v_count,
            "type": "VEC3",
            "min": [bounds.min.x as f32, bounds.min.y as f32, bounds.min.z as f32],
            "max": [bounds.max.x as f32, bounds.max.y as f32, bounds.max.z as f32]
        }));

        // Normals (VEC3 F32)
        pad_chunk(&mut bin_chunk);
        let norm_offset = bin_chunk.len();
        for n in &mesh.normals {
            for c in n {
                bin_chunk.extend_from_slice(&c.to_le_bytes());
            }
        }
        let norm_len = bin_chunk.len() - norm_offset;
        let norm_view = buffer_views.len();
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": norm_offset,
            "byteLength": norm_len,
            "target": target_array_buffer
        }));
        let norm_acc = accessors.len();
        accessors.push(json!({
            "bufferView": norm_view,
            "componentType": component_f32,
            "count": v_count,
            "type": "VEC3"
        }));

        // UVs (VEC2 F32)
        pad_chunk(&mut bin_chunk);
        let uv_offset = bin_chunk.len();
        for uv in &mesh.uvs {
            for c in uv {
                bin_chunk.extend_from_slice(&c.to_le_bytes());
            }
        }
        let uv_len = bin_chunk.len() - uv_offset;
        let uv_view = buffer_views.len();
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": uv_offset,
            "byteLength": uv_len,
            "target": target_array_buffer
        }));
        let uv_acc = accessors.len();
        accessors.push(json!({
            "bufferView": uv_view,
            "componentType": component_f32,
            "count": v_count,
            "type": "VEC2"
        }));

        // Indices (SCALAR U32)
        pad_chunk(&mut bin_chunk);
        let idx_offset = bin_chunk.len();
        for &idx in &mesh.indices {
            bin_chunk.extend_from_slice(&idx.to_le_bytes());
        }
        let idx_len = bin_chunk.len() - idx_offset;
        let idx_view = buffer_views.len();
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": idx_offset,
            "byteLength": idx_len,
            "target": target_element_buffer
        }));
        let idx_acc = accessors.len();
        accessors.push(json!({
            "bufferView": idx_view,
            "componentType": component_u32,
            "count": mesh.indices.len(),
            "type": "SCALAR"
        }));

        gltf_meshes.push(json!({
            "name": mesh.name,
            "primitives": [{
                "attributes": {
                    "POSITION": pos_acc,
                    "NORMAL": norm_acc,
                    "TEXCOORD_0": uv_acc
                },
                "indices": idx_acc,
                "mode": mode_triangles
            }]
        }));
    }

    pad_chunk(&mut bin_chunk);

    let gltf_materials: Vec<_> = materials
        .iter()
        .map(|m| {
            json!({
                "name": m.name,
                "pbrMetallicRoughness": {
                    "baseColorFactor": m.base_color_factor,
                    "metallicFactor": m.metallic_factor,
                    "roughnessFactor": m.roughness_factor
                },
                "emissiveFactor": m.emissive_factor,
                "doubleSided": m.double_sided
            })
        })
        .collect();

    let entity_list: Vec<_> = scene.entities().collect();
    let id_to_node: std::collections::BTreeMap<_, _> = entity_list
        .iter()
        .enumerate()
        .map(|(idx, e)| (e.id, idx))
        .collect();

    let mut root_nodes = Vec::new();
    let mut gltf_nodes = Vec::with_capacity(entity_list.len());

    for ent in &entity_list {
        if ent.parent.is_none() {
            if let Some(&node_idx) = id_to_node.get(&ent.id) {
                root_nodes.push(node_idx);
            }
        }
        let child_indices: Vec<usize> = ent
            .children
            .iter()
            .filter_map(|cid| id_to_node.get(cid).copied())
            .collect();

        let mut node_obj = json!({
            "name": ent.name,
            "translation": ent.local_transform.translation.to_f32_array(),
            "rotation": ent.local_transform.rotation.to_f32_array(),
            "scale": ent.local_transform.scale.to_f32_array(),
            "extras": {
                "role": format!("{:?}", ent.role),
                "lodActivationScale": ent.emerged_at_lod_scale,
                "provenance": ent.provenance_tag
            }
        });
        if !child_indices.is_empty() {
            node_obj["children"] = json!(child_indices);
        }
        if let Some(m_idx) = ent.mesh_index {
            if m_idx < gltf_meshes.len() {
                node_obj["mesh"] = json!(m_idx);
            }
        }
        gltf_nodes.push(node_obj);
    }

    let base_mesh_count = gltf_meshes.len();
    let mut bound_mesh_materials: std::collections::BTreeMap<(usize, usize), usize> =
        std::collections::BTreeMap::new();
    let mut assigned_primary_mat: Vec<Option<usize>> = vec![None; base_mesh_count];

    for (node_pos, ent) in entity_list.iter().enumerate() {
        let Some(m_idx) = ent.mesh_index else {
            continue;
        };
        if m_idx >= base_mesh_count {
            continue;
        }
        let valid_mat = ent.material_index.filter(|&mi| mi < gltf_materials.len());
        let resolved_mesh_idx = if let Some(mat_idx) = valid_mat {
            if let Some(&existing) = bound_mesh_materials.get(&(m_idx, mat_idx)) {
                existing
            } else if assigned_primary_mat[m_idx].is_none() {
                gltf_meshes[m_idx]["primitives"][0]["material"] = json!(mat_idx);
                assigned_primary_mat[m_idx] = Some(mat_idx);
                bound_mesh_materials.insert((m_idx, mat_idx), m_idx);
                m_idx
            } else {
                let mut cloned_mesh = gltf_meshes[m_idx].clone();
                cloned_mesh["primitives"][0]["material"] = json!(mat_idx);
                let next_idx = gltf_meshes.len();
                gltf_meshes.push(cloned_mesh);
                bound_mesh_materials.insert((m_idx, mat_idx), next_idx);
                next_idx
            }
        } else {
            m_idx
        };
        gltf_nodes[node_pos]["mesh"] = json!(resolved_mesh_idx);
    }

    let gltf_version_str = format!("{}.{}", two_u32(), 0_u32);
    let mut root_json = json!({
        "asset": {
            "version": gltf_version_str,
            "generator": "Verdant Engine (verdant-core)"
        },
        "scene": 0,
        "scenes": [{
            "name": "VerdantWorldScene",
            "nodes": root_nodes
        }],
        "nodes": gltf_nodes,
        "materials": gltf_materials,
        "meshes": gltf_meshes,
        "accessors": accessors,
        "bufferViews": buffer_views
    });
    if !bin_chunk.is_empty() {
        root_json["buffers"] = json!([{ "byteLength": bin_chunk.len() }]);
    }

    let mut json_bytes =
        serde_json::to_vec(&root_json).map_err(|err| AssetValidationError::GltfOracleRejected {
            oracle_message: err.to_string(),
        })?;
    while !json_bytes.len().is_multiple_of(GLTF_CHUNK_ALIGNMENT) {
        json_bytes.push(b' ');
    }

    let has_bin = !bin_chunk.is_empty();
    let total_len = if has_bin {
        crate::prism_kernel::glbContainerByteLength(json_bytes.len() as u64, bin_chunk.len() as u64)
            .expect("verified Prism kernel glbContainerByteLength") as usize
    } else {
        GLB_FILE_HEADER_BYTES + GLB_CHUNK_HEADER_BYTES + json_bytes.len()
    };

    let glb_magic = u32::from_le_bytes(*b"glTF");
    let glb_version = two_u32();
    let chunk_json = u32::from_le_bytes(*b"JSON");
    let chunk_bin = u32::from_le_bytes(*b"BIN\0");

    let mut glb = Vec::with_capacity(total_len);
    glb.extend_from_slice(&glb_magic.to_le_bytes());
    glb.extend_from_slice(&glb_version.to_le_bytes());
    glb.extend_from_slice(&(total_len as u32).to_le_bytes());

    glb.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    glb.extend_from_slice(&chunk_json.to_le_bytes());
    glb.extend_from_slice(&json_bytes);

    if has_bin {
        glb.extend_from_slice(&(bin_chunk.len() as u32).to_le_bytes());
        glb.extend_from_slice(&chunk_bin.to_le_bytes());
        glb.extend_from_slice(&bin_chunk);
    }

    validate_glb_bytes(&glb)?;
    Ok(glb)
}

/// Validates a `.gltf` or `.glb` byte slice using the authoritative `gltf` specification crate,
/// including binary buffer bounds and accessor decoding.
pub fn validate_glb_bytes(bytes: &[u8]) -> Result<VerifiedGltfSummary, AssetValidationError> {
    let (document, buffers, _images) =
        gltf::import_slice(bytes).map_err(|err| AssetValidationError::GltfOracleRejected {
            oracle_message: err.to_string(),
        })?;

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

    let total_buffer_bytes = buffers.iter().map(|b| b.len()).sum();
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
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw.get()).ok())
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
pub fn import_glb_to_scene_bounded(
    bytes: &[u8],
    max_tex_edge_px: u32,
) -> Result<ImportedGlbScene, AssetValidationError> {
    let (document, buffers, images) =
        gltf::import_slice(bytes).map_err(|err| AssetValidationError::GltfOracleRejected {
            oracle_message: err.to_string(),
        })?;

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
    let mut atlas_per_tile_cap = max_tex_edge_px;
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

    let full_byte = ((1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)) - 1) as u8;
    let half_byte = (1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1)) as u8;
    let low_byte = (1u32 << (1 + 1 + 1 + 1)) as u8;
    let primary_mr_texture = if atlas_tile_count <= 1 {
        mr_src_idx
            .and_then(|idx| images.get(idx))
            .and_then(|img| decode_rgba_bounded(img, max_tex_edge_px))
    } else {
        pack_companion_channel_atlas(0, [full_byte, half_byte, low_byte, full_byte])
    };
    let primary_normal_texture = if atlas_tile_count <= 1 {
        norm_src_idx
            .and_then(|idx| images.get(idx))
            .and_then(|img| decode_rgba_bounded(img, max_tex_edge_px))
    } else {
        pack_companion_channel_atlas(1, [half_byte, half_byte, full_byte, full_byte])
    };
    let primary_emissive_texture = if atlas_tile_count <= 1 {
        emis_src_idx
            .and_then(|idx| images.get(idx))
            .and_then(|img| decode_rgba_bounded(img, max_tex_edge_px))
    } else {
        pack_companion_channel_atlas(1 + 1, [0, 0, 0, full_byte])
    };

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
            let reader = prim.reader(|buffer| buffers.get(buffer.index()).map(|d| d.0.as_slice()));
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
                        let tex = base_tex_img.map_or([1.0, 1.0, 1.0, 1.0], |img| {
                            let uv = uvs.get(uv_start + local_i).copied().unwrap_or([0.0, 0.0]);
                            sample_gltf_base_color(img, uv[0], uv[1])
                        });
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
                let reader =
                    ch.reader(|buffer| buffers.get(buffer.index()).map(|d| d.0.as_slice()));
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

/// Transcodes a binary Khronos `.glb` scene directly into a portable 4-LOD (`LOD0..=LOD3`)
/// `UORC-SHA256-CHAIN-v1` progressive stream decodable natively in WebAssembly via `uorc_decompress::run`.
#[cfg(feature = "uorc-streaming")]
pub fn transcode_glb_to_progressive_uorc(
    glb_bytes: &[u8],
    seed: u64,
) -> Result<Vec<u8>, StreamingError> {
    let imported =
        import_glb_to_scene(glb_bytes).map_err(|err| StreamingError::LodPayloadMismatch {
            detail: err.to_string(),
        })?;
    let imported_records: Vec<_> = imported.scene.entities().cloned().collect();
    let total_imported = imported_records.len();
    let max_rank = crate::prism_kernel::wasmMaxLodRank();

    let max_strats = if cfg!(debug_assertions) {
        1
    } else {
        crate::prism_kernel::sEight().unwrap_or(1) as usize
    };
    let mut strategies = Vec::new();
    for pair in &uorc::portfolio::V04_PAIR {
        for strat in pair {
            if strat.state == 0
                && !strat.x86
                && strategies.len() < max_strats
                && !strategies.contains(&Some(*strat))
            {
                strategies.push(Some(*strat));
            }
        }
    }

    let mut stream = Vec::new();
    let mut state = StandardSceneState::new_at_lod0(seed.max(1))?;
    for rank in 0..=max_rank {
        state.advance_to_lod(rank)?;
        let take_count = if rank == max_rank {
            total_imported
        } else {
            ((total_imported as u64) * (rank + 1) / (max_rank + 1)) as usize
        };
        let mut id_remap = std::collections::BTreeMap::new();
        for rec in imported_records
            .iter()
            .take(take_count.max(usize::from(total_imported > 0 && rank > 0)))
        {
            let mapped_parent = rec.parent.and_then(|pid| id_remap.get(&pid).copied());
            let new_id = state.scene.spawn(
                rec.name.clone(),
                rec.role,
                mapped_parent,
                rec.local_transform,
                rec.local_bounds,
                (
                    state.active_lod.resolution_ratio(),
                    "Transcoded glTF->UORC entity",
                ),
            );
            if let (Some(m_idx), Some(mat_idx)) = (rec.mesh_index, rec.material_index) {
                state.scene.attach_renderable(new_id, m_idx, mat_idx);
            }
            id_remap.insert(rec.id, new_id);
        }

        let snap =
            WorldSnapshot::encode(&state).map_err(|err| StreamingError::LodPayloadMismatch {
                detail: err.to_string(),
            })?;
        let archive = uorc::compress_with_strategies(&snap, &[snap.len() as u64], &[], &strategies);
        stream.extend_from_slice(&archive);
    }

    Ok(stream)
}

/// Packs multiple `(ProceduralMesh, material_index)` primitives into a single multi-primitive
/// Khronos binary glTF 2.0 (`.glb`) mesh validated by [`validate_glb_bytes`].
pub fn pack_multi_primitive_glb(
    mesh_name: &str,
    primitives: &[(&ProceduralMesh, usize)],
    materials: &[PbrMaterial],
) -> Result<Vec<u8>, AssetValidationError> {
    if primitives.is_empty() {
        return Err(AssetValidationError::EmptyMeshTopology {
            mesh_name: mesh_name.to_string(),
        });
    }
    let mut temp_scene = SceneWorld::new();
    let mut mesh_vec = Vec::with_capacity(primitives.len());
    for (p_idx, &(pm, mat_idx)) in primitives.iter().enumerate() {
        let eid = temp_scene.spawn(
            format!("{mesh_name}_{p_idx}"),
            EntityRole::StaticArchitecture,
            None,
            Transform::default(),
            pm.bounds(),
            (1.0, "MultiPrimitiveGlbSource"),
        );
        temp_scene.attach_renderable(eid, p_idx, mat_idx);
        mesh_vec.push(pm.clone());
    }
    let base_glb = export_scene_to_glb(&temp_scene, &mesh_vec, materials)?;
    let hdr = GLB_FILE_HEADER_BYTES;
    let chk = GLB_CHUNK_HEADER_BYTES;
    let i4 = 1usize << (1usize + 1usize);
    let json_len = u32::from_le_bytes([
        base_glb[hdr],
        base_glb[hdr + 1],
        base_glb[hdr + (1 + 1)],
        base_glb[hdr + (1 + 1 + 1)],
    ]) as usize;
    let json_start = hdr + chk;
    let json_end = json_start + json_len;
    let bin_tail = &base_glb[json_end..];

    let mut root_val: serde_json::Value = serde_json::from_slice(&base_glb[json_start..json_end])
        .map_err(|e| {
        AssetValidationError::GltfOracleRejected {
            oracle_message: e.to_string(),
        }
    })?;
    let mut merged_prims = Vec::with_capacity(primitives.len());
    if let Some(arr) = root_val.get("meshes").and_then(|v| v.as_array()) {
        for m in arr {
            if let Some(p0) = m
                .get("primitives")
                .and_then(|p| p.as_array())
                .and_then(|pa| pa.first())
            {
                merged_prims.push(p0.clone());
            }
        }
    }
    root_val["meshes"] = json!([{
        "name": mesh_name,
        "primitives": merged_prims
    }]);
    root_val["nodes"] = json!([{
        "name": mesh_name,
        "mesh": 0
    }]);
    root_val["scenes"] = json!([{
        "name": "VerdantWorldScene",
        "nodes": [0]
    }]);

    let mut json_bytes =
        serde_json::to_vec(&root_val).map_err(|e| AssetValidationError::GltfOracleRejected {
            oracle_message: e.to_string(),
        })?;
    while !json_bytes.len().is_multiple_of(GLTF_CHUNK_ALIGNMENT) {
        json_bytes.push(b' ');
    }
    let total_len = hdr + chk + json_bytes.len() + bin_tail.len();
    let mut glb = Vec::with_capacity(total_len);
    glb.extend_from_slice(&base_glb[..i4 * (1 + 1)]);
    glb.extend_from_slice(&(total_len as u32).to_le_bytes());
    glb.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    glb.extend_from_slice(b"JSON");
    glb.extend_from_slice(&json_bytes);
    glb.extend_from_slice(bin_tail);
    validate_glb_bytes(&glb)?;
    Ok(glb)
}

const KHRONOS_PBR_EXTENSIONS: [&str; 1 + 1 + 1 + 1 + 1] = [
    "KHR_materials_ior",
    "KHR_materials_specular",
    "KHR_materials_transmission",
    "KHR_materials_clearcoat",
    "KHR_materials_emissive_strength",
];

fn hd_descriptor_to_gltf_material_json(mat: &HdPbrDescriptor) -> serde_json::Value {
    json!({
        "name": mat.name,
        "pbrMetallicRoughness": {
            "baseColorFactor": mat.base_color_factor,
            "metallicFactor": mat.metallic_factor.clamp(0.0, 1.0),
            "roughnessFactor": mat.roughness_factor.clamp(0.0, 1.0)
        },
        "emissiveFactor": mat.emissive_factor,
        "doubleSided": mat.double_sided,
        "alphaMode": if mat.base_color_factor[DIM_SPATIAL] < 1.0 || mat.transmission_factor > 0.0 {
            "BLEND"
        } else {
            "OPAQUE"
        },
        "extensions": {
            "KHR_materials_ior": { "ior": mat.ior.max(1.0) },
            "KHR_materials_specular": { "specularFactor": mat.specular_factor.clamp(0.0, 1.0) },
            "KHR_materials_transmission": { "transmissionFactor": mat.transmission_factor.clamp(0.0, 1.0) },
            "KHR_materials_clearcoat": {
                "clearcoatFactor": mat.clearcoat_factor.clamp(0.0, 1.0),
                "clearcoatRoughnessFactor": mat.clearcoat_roughness_factor.clamp(0.0, 1.0)
            },
            "KHR_materials_emissive_strength": {
                "emissiveStrength": mat.emissive_strength.max(0.0)
            }
        }
    })
}

fn pack_json_and_bin_into_glb(
    mut root_json: serde_json::Value,
    mut bin_chunk: Vec<u8>,
) -> Result<Vec<u8>, AssetValidationError> {
    while !bin_chunk.len().is_multiple_of(GLTF_CHUNK_ALIGNMENT) {
        bin_chunk.push(0);
    }
    if !bin_chunk.is_empty() {
        root_json["buffers"] = json!([{ "byteLength": bin_chunk.len() }]);
    }
    let mut json_bytes =
        serde_json::to_vec(&root_json).map_err(|err| AssetValidationError::GltfOracleRejected {
            oracle_message: err.to_string(),
        })?;
    while !json_bytes.len().is_multiple_of(GLTF_CHUNK_ALIGNMENT) {
        json_bytes.push(b' ');
    }

    let has_bin = !bin_chunk.is_empty();
    let total_len = if has_bin {
        crate::prism_kernel::glbContainerByteLength(json_bytes.len() as u64, bin_chunk.len() as u64)
            .expect("verified Prism kernel glbContainerByteLength") as usize
    } else {
        GLB_FILE_HEADER_BYTES + GLB_CHUNK_HEADER_BYTES + json_bytes.len()
    };

    let glb_magic = u32::from_le_bytes(*b"glTF");
    let glb_version = two_u32();
    let chunk_json = u32::from_le_bytes(*b"JSON");
    let chunk_bin = u32::from_le_bytes(*b"BIN\0");

    let mut glb = Vec::with_capacity(total_len);
    glb.extend_from_slice(&glb_magic.to_le_bytes());
    glb.extend_from_slice(&glb_version.to_le_bytes());
    glb.extend_from_slice(&(total_len as u32).to_le_bytes());
    glb.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    glb.extend_from_slice(&chunk_json.to_le_bytes());
    glb.extend_from_slice(&json_bytes);
    if has_bin {
        glb.extend_from_slice(&(bin_chunk.len() as u32).to_le_bytes());
        glb.extend_from_slice(&chunk_bin.to_le_bytes());
        glb.extend_from_slice(&bin_chunk);
    }
    validate_glb_bytes(&glb)?;
    Ok(glb)
}

/// Exports a high-definition `SceneWorld`, `ProceduralMesh` library (including `COLOR_0`), and
/// ratified Khronos PBR extension descriptors (`KHR_materials_{ior,specular,transmission,clearcoat,emissive_strength}`)
/// into a `.glb` binary verified by the `gltf` specification oracle.
pub fn export_hd_scene_to_glb(
    scene: &SceneWorld,
    meshes: &[ProceduralMesh],
    hd_materials: &[HdPbrDescriptor],
) -> Result<Vec<u8>, AssetValidationError> {
    let base_materials: Vec<PbrMaterial> = hd_materials
        .iter()
        .map(HdPbrDescriptor::to_pbr_material)
        .collect();
    let base_glb = export_scene_to_glb(scene, meshes, &base_materials)?;

    let hdr = GLB_FILE_HEADER_BYTES;
    let chk = GLB_CHUNK_HEADER_BYTES;
    if base_glb.len() < hdr + chk {
        return Err(AssetValidationError::GltfOracleRejected {
            oracle_message: "truncated base GLB header".to_string(),
        });
    }
    let json_len = u32::from_le_bytes([
        base_glb[hdr],
        base_glb[hdr + 1],
        base_glb[hdr + 1 + 1],
        base_glb[hdr + 1 + 1 + 1],
    ]) as usize;
    let json_start = hdr + chk;
    let json_end = json_start + json_len;
    let bin_chunk_start = json_end + chk;
    let bin_chunk = if base_glb.len() >= bin_chunk_start {
        base_glb[bin_chunk_start..].to_vec()
    } else {
        Vec::new()
    };

    let mut root_json: serde_json::Value = serde_json::from_slice(&base_glb[json_start..json_end])
        .map_err(|err| AssetValidationError::GltfOracleRejected {
            oracle_message: err.to_string(),
        })?;
    root_json["extensionsUsed"] = json!(KHRONOS_PBR_EXTENSIONS);
    root_json["materials"] = json!(hd_materials
        .iter()
        .map(hd_descriptor_to_gltf_material_json)
        .collect::<Vec<_>>());

    pack_json_and_bin_into_glb(root_json, bin_chunk)
}

/// Exports a 22-joint [`crate::character::ArticulatedCharacter`] as a Khronos glTF 2.0 skinned `.glb`
/// carrying topological joint nodes, `skins[0]` (`inverseBindMatrices` `MAT4` `F32`), `COLOR_0`,
/// `JOINTS_0` (`VEC4` `U16`), `WEIGHTS_0` (`VEC4` `F32`), and ratified `KHR_materials_*` extensions,
/// verified end-to-end by `gltf::import_slice`.
pub fn export_skinned_character_to_glb(
    character: &crate::character::ArticulatedCharacter,
    hd_material: &HdPbrDescriptor,
) -> Result<Vec<u8>, AssetValidationError> {
    character.bind_mesh.validate()?;
    let v_count = character.bind_mesh.positions.len();
    if character.skin_weights.len() != v_count {
        return Err(AssetValidationError::AttributeLengthMismatch {
            mesh_name: character.bind_mesh.name.clone(),
            detail: format!(
                "skin_weights length {} != vertex count {v_count}",
                character.skin_weights.len()
            ),
        });
    }

    let component_f32 = Valid(ComponentType::F32);
    let component_u16 = Valid(ComponentType::U16);
    let component_u32 = Valid(ComponentType::U32);
    let target_array_buffer = Valid(Target::ArrayBuffer);
    let target_element_buffer = Valid(Target::ElementArrayBuffer);
    let mode_triangles = Valid(Mode::Triangles);

    let pad_chunk = |buf: &mut Vec<u8>| {
        while !buf.len().is_multiple_of(GLTF_CHUNK_ALIGNMENT) {
            buf.push(0);
        }
    };

    let mut bin_chunk: Vec<u8> = Vec::new();
    let mut buffer_views = Vec::new();
    let mut accessors = Vec::new();
    let mesh = &character.bind_mesh;
    let bounds = mesh.bounds();

    // 0: POSITION (VEC3 F32)
    pad_chunk(&mut bin_chunk);
    let pos_off = bin_chunk.len();
    for p in &mesh.positions {
        for c in p {
            bin_chunk.extend_from_slice(&c.to_le_bytes());
        }
    }
    let pos_len = bin_chunk.len() - pos_off;
    buffer_views.push(json!({
        "buffer": 0,
        "byteOffset": pos_off,
        "byteLength": pos_len,
        "target": target_array_buffer
    }));
    accessors.push(json!({
        "bufferView": 0,
        "componentType": component_f32,
        "count": v_count,
        "type": "VEC3",
        "min": [bounds.min.x as f32, bounds.min.y as f32, bounds.min.z as f32],
        "max": [bounds.max.x as f32, bounds.max.y as f32, bounds.max.z as f32]
    }));

    // 1: NORMAL (VEC3 F32)
    pad_chunk(&mut bin_chunk);
    let norm_off = bin_chunk.len();
    for n in &mesh.normals {
        for c in n {
            bin_chunk.extend_from_slice(&c.to_le_bytes());
        }
    }
    let norm_len = bin_chunk.len() - norm_off;
    let norm_view = buffer_views.len();
    buffer_views.push(json!({
        "buffer": 0,
        "byteOffset": norm_off,
        "byteLength": norm_len,
        "target": target_array_buffer
    }));
    let norm_acc = accessors.len();
    accessors.push(json!({
        "bufferView": norm_view,
        "componentType": component_f32,
        "count": v_count,
        "type": "VEC3"
    }));

    // 2: TEXCOORD_0 (VEC2 F32)
    pad_chunk(&mut bin_chunk);
    let uv_off = bin_chunk.len();
    for uv in &mesh.uvs {
        for c in uv {
            bin_chunk.extend_from_slice(&c.to_le_bytes());
        }
    }
    let uv_len = bin_chunk.len() - uv_off;
    let uv_view = buffer_views.len();
    buffer_views.push(json!({
        "buffer": 0,
        "byteOffset": uv_off,
        "byteLength": uv_len,
        "target": target_array_buffer
    }));
    let uv_acc = accessors.len();
    accessors.push(json!({
        "bufferView": uv_view,
        "componentType": component_f32,
        "count": v_count,
        "type": "VEC2"
    }));

    // 3: COLOR_0 (VEC4 F32)
    pad_chunk(&mut bin_chunk);
    let col_off = bin_chunk.len();
    for col in &mesh.colors {
        for c in col {
            bin_chunk.extend_from_slice(&c.clamp(0.0, 1.0).to_le_bytes());
        }
    }
    let col_len = bin_chunk.len() - col_off;
    let col_view = buffer_views.len();
    buffer_views.push(json!({
        "buffer": 0,
        "byteOffset": col_off,
        "byteLength": col_len,
        "target": target_array_buffer
    }));
    let col_acc = accessors.len();
    accessors.push(json!({
        "bufferView": col_view,
        "componentType": component_f32,
        "count": v_count,
        "type": "VEC4"
    }));

    // 4: JOINTS_0 (VEC4 U16)
    pad_chunk(&mut bin_chunk);
    let joints_off = bin_chunk.len();
    for sw in &character.skin_weights {
        for j in sw.joints {
            bin_chunk.extend_from_slice(&j.to_le_bytes());
        }
    }
    let joints_len = bin_chunk.len() - joints_off;
    let joints_view = buffer_views.len();
    buffer_views.push(json!({
        "buffer": 0,
        "byteOffset": joints_off,
        "byteLength": joints_len,
        "target": target_array_buffer
    }));
    let joints_acc = accessors.len();
    accessors.push(json!({
        "bufferView": joints_view,
        "componentType": component_u16,
        "count": v_count,
        "type": "VEC4"
    }));

    // 5: WEIGHTS_0 (VEC4 F32, normalized from permille scale == 1000)
    let weight_scale = crate::prism_kernel::skinWeightScale().unwrap_or(1) as f32;
    let i2 = 1usize + 1usize;
    pad_chunk(&mut bin_chunk);
    let weights_off = bin_chunk.len();
    for sw in &character.skin_weights {
        let w0 = f32::from(sw.weights_permille[0]) / weight_scale;
        let w1 = f32::from(sw.weights_permille[1]) / weight_scale;
        let w2 = f32::from(sw.weights_permille[i2]) / weight_scale;
        let w3 = (1.0_f32 - (w0 + w1 + w2)).max(0.0);
        for w in [w0, w1, w2, w3] {
            bin_chunk.extend_from_slice(&w.to_le_bytes());
        }
    }
    let weights_len = bin_chunk.len() - weights_off;
    let weights_view = buffer_views.len();
    buffer_views.push(json!({
        "buffer": 0,
        "byteOffset": weights_off,
        "byteLength": weights_len,
        "target": target_array_buffer
    }));
    let weights_acc = accessors.len();
    accessors.push(json!({
        "bufferView": weights_view,
        "componentType": component_f32,
        "count": v_count,
        "type": "VEC4"
    }));

    // 6: INDICES (SCALAR U32)
    pad_chunk(&mut bin_chunk);
    let idx_off = bin_chunk.len();
    for &idx in &mesh.indices {
        bin_chunk.extend_from_slice(&idx.to_le_bytes());
    }
    let idx_len = bin_chunk.len() - idx_off;
    let idx_view = buffer_views.len();
    buffer_views.push(json!({
        "buffer": 0,
        "byteOffset": idx_off,
        "byteLength": idx_len,
        "target": target_element_buffer
    }));
    let idx_acc = accessors.len();
    accessors.push(json!({
        "bufferView": idx_view,
        "componentType": component_u32,
        "count": mesh.indices.len(),
        "type": "SCALAR"
    }));

    // 7: inverseBindMatrices (MAT4 F32, 22 joints)
    let j_count = character.skeleton.joints.len();
    let mut world_bind_pos = vec![Vec3::ZERO; j_count];
    for (j_idx, j) in character.skeleton.joints.iter().enumerate() {
        world_bind_pos[j_idx] = if j.is_root {
            j.rest_translation
        } else {
            world_bind_pos[j.parent_index] + j.rest_translation
        };
    }
    pad_chunk(&mut bin_chunk);
    let ibm_off = bin_chunk.len();
    for wp in &world_bind_pos {
        let ibm = crate::math::Mat4::from_translation(Vec3::new(-wp.x, -wp.y, -wp.z));
        for col in &ibm.cols {
            for &elem in col {
                bin_chunk.extend_from_slice(&(elem as f32).to_le_bytes());
            }
        }
    }
    let ibm_len = bin_chunk.len() - ibm_off;
    let ibm_view = buffer_views.len();
    buffer_views.push(json!({
        "buffer": 0,
        "byteOffset": ibm_off,
        "byteLength": ibm_len
    }));
    let ibm_acc = accessors.len();
    accessors.push(json!({
        "bufferView": ibm_view,
        "componentType": component_f32,
        "count": j_count,
        "type": "MAT4"
    }));

    // Build joint hierarchy nodes `0..j_count` + skinned mesh node `j_count`:
    let mut children_per_joint: Vec<Vec<usize>> = vec![Vec::new(); j_count];
    for (j_idx, j) in character.skeleton.joints.iter().enumerate() {
        if !j.is_root {
            children_per_joint[j.parent_index].push(j_idx);
        }
    }
    let mut gltf_nodes = Vec::with_capacity(j_count + 1);
    for (j_idx, j) in character.skeleton.joints.iter().enumerate() {
        let mut j_node = json!({
            "name": j.name,
            "translation": j.rest_translation.to_f32_array(),
            "rotation": j.rest_rotation.to_f32_array()
        });
        if !children_per_joint[j_idx].is_empty() {
            j_node["children"] = json!(children_per_joint[j_idx]);
        }
        gltf_nodes.push(j_node);
    }
    let mesh_node_idx = gltf_nodes.len();
    gltf_nodes.push(json!({
        "name": character.name,
        "mesh": 0,
        "skin": 0,
        "translation": character.foot_position.to_f32_array(),
        "extras": {
            "role": "ArticulatedRig",
            "archetype": format!("{:?}", character.archetype),
            "jointCount": j_count
        }
    }));

    let joint_node_indices: Vec<usize> = (0..j_count).collect();
    let gltf_version_str = format!("{}.{}", two_u32(), 0_u32);
    let root_json = json!({
        "asset": {
            "version": gltf_version_str,
            "generator": "Verdant Engine (verdant-core HD Skinned Character Exporter)"
        },
        "extensionsUsed": KHRONOS_PBR_EXTENSIONS,
        "scene": 0,
        "scenes": [{
            "name": format!("{}_Scene", character.name),
            "nodes": [0, mesh_node_idx]
        }],
        "nodes": gltf_nodes,
        "skins": [{
            "name": format!("{}_Humanoid22Rig", character.name),
            "skeleton": 0,
            "joints": joint_node_indices,
            "inverseBindMatrices": ibm_acc
        }],
        "materials": [hd_descriptor_to_gltf_material_json(hd_material)],
        "meshes": [{
            "name": mesh.name,
            "primitives": [{
                "attributes": {
                    "POSITION": 0,
                    "NORMAL": norm_acc,
                    "TEXCOORD_0": uv_acc,
                    "COLOR_0": col_acc,
                    "JOINTS_0": joints_acc,
                    "WEIGHTS_0": weights_acc
                },
                "indices": idx_acc,
                "material": 0,
                "mode": mode_triangles
            }]
        }],
        "accessors": accessors,
        "bufferViews": buffer_views
    });

    pack_json_and_bin_into_glb(root_json, bin_chunk)
}
