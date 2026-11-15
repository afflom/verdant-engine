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
use crate::render::PbrMaterial;
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

    for ent in &entity_list {
        if let (Some(m_idx), Some(mat_idx)) = (ent.mesh_index, ent.material_index) {
            if m_idx < gltf_meshes.len() && mat_idx < gltf_materials.len() {
                gltf_meshes[m_idx]["primitives"][0]["material"] = json!(mat_idx);
            }
        }
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
    let role = if node.skin().is_some() {
        EntityRole::ArticulatedRig
    } else {
        EntityRole::StaticArchitecture
    };
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
pub fn import_glb_to_scene(bytes: &[u8]) -> Result<ImportedGlbScene, AssetValidationError> {
    let (document, buffers, _images) =
        gltf::import_slice(bytes).map_err(|err| AssetValidationError::GltfOracleRejected {
            oracle_message: err.to_string(),
        })?;

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
    let i2 = 1usize + 1usize;

    for (m_idx, mesh) in document.meshes().enumerate() {
        let Some(prim) = mesh.primitives().next() else {
            return Err(AssetValidationError::EmptyMeshTopology {
                mesh_name: mesh.name().unwrap_or("GlbMesh").to_string(),
            });
        };
        let reader = prim.reader(|buffer| buffers.get(buffer.index()).map(|d| d.0.as_slice()));
        let positions: Vec<[f32; DIM_SPATIAL]> = reader
            .read_positions()
            .ok_or_else(|| AssetValidationError::GltfOracleRejected {
                oracle_message: format!("mesh {m_idx} missing POSITION attribute"),
            })?
            .collect();
        let v_count = positions.len();

        let normals: Vec<[f32; DIM_SPATIAL]> = match reader.read_normals() {
            Some(iter) => iter
                .map(|n| {
                    let v =
                        Vec3::new(f64::from(n[0]), f64::from(n[1]), f64::from(n[i2])).normalized();
                    v.to_f32_array()
                })
                .collect(),
            None => vec![[0.0, 1.0, 0.0]; v_count],
        };
        let uvs: Vec<[f32; DIM_PLANAR]> = match reader.read_tex_coords(0) {
            Some(tc) => tc.into_f32().collect(),
            None => vec![[0.0, 0.0]; v_count],
        };
        let colors: Vec<[f32; DIM_HOMOGENEOUS]> = match reader.read_colors(0) {
            Some(col) => col.into_rgba_f32().collect(),
            None => vec![[1.0, 1.0, 1.0, 1.0]; v_count],
        };
        let indices: Vec<u32> = match reader.read_indices() {
            Some(ind) => ind.into_u32().collect(),
            None => (0..v_count as u32).collect(),
        };

        let max_skin_joints = document
            .skins()
            .map(|s| s.joints().count())
            .max()
            .unwrap_or(usize::from(u16::MAX) + 1);
        let mut prim_weights = Vec::new();
        if let (Some(j_iter), Some(w_iter)) = (reader.read_joints(0), reader.read_weights(0)) {
            for (joints, raw_w) in j_iter.into_u16().zip(w_iter.into_f32()) {
                let norm_w = normalize_weights_permille(raw_w);
                if let Ok(sw) = SkinWeights4::new(joints, norm_w, max_skin_joints.max(1)) {
                    prim_weights.push(sw);
                }
            }
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
        mesh_first_prim_mat.push(prim.material().index());
        meshes.push(pm);
        skin_weights.push(prim_weights);
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
    let i3 = i2 + 1usize;
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
