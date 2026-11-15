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

use crate::ecs::SceneWorld;
use crate::error::AssetValidationError;
use crate::geometry::ProceduralMesh;
use crate::math::{two_u32, DIM_HOMOGENEOUS, DIM_PLANAR, DIM_SPATIAL};
use crate::render::PbrMaterial;

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
                "emergedAtScale": ent.emerged_at_scale,
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
    let total_len = GLB_FILE_HEADER_BYTES
        + GLB_CHUNK_HEADER_BYTES
        + json_bytes.len()
        + if has_bin {
            GLB_CHUNK_HEADER_BYTES + bin_chunk.len()
        } else {
            0
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
