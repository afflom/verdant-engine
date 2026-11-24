//! Khronos glTF 2.0 HD/skinned export & progressive UORC transcoding (`gltf_export.rs`).

use super::*;

/// Transcodes a binary Khronos `.glb` scene directly into a portable 4-LOD (`LOD0..=LOD3`)
/// `UORC-SHA256-CHAIN-v1` progressive stream decodable natively in WebAssembly via `uorc_decompress::run`.
#[cfg(any(feature = "uorc-streaming", feature = "uorc-wasm-decode"))]
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
        let archive = crate::streaming::compress_uorc_portable(&snap);
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
            .expect("prism") as usize
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
            .expect("prism") as usize
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
pub(super) fn parse_glb_document_and_buffers(
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
