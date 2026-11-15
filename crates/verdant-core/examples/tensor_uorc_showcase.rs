//! End-to-end showcase of Prism-hosted `uor-matmul` tensor kernels and progressive `UORC`
//! (`UORC-SHA256-CHAIN-v1`) multi-LOD scene streaming up to lossless source resolution.

use verdant_core::geometry::canonical_scene_mesh_library;
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::{Mat4, Vec3};
use verdant_core::render::{canonical_pbr_material_library, validate_builtin_engine_shaders};
use verdant_core::streaming::{
    decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream, verify_progressive_uorc_stream,
};
use verdant_core::tensor::{
    batch_transform_positions_exact, render_triangles_tile_framebuffer,
    verified_engine_tensor_kernels, RasterTriangle,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kernels = verified_engine_tensor_kernels()?;
    println!("Certified Prism tensor kernels: {}", kernels.len());

    let stream_bytes = pack_progressive_uorc_stream(42)?;
    let summary = verify_progressive_uorc_stream(&stream_bytes)?;
    println!(
        "Progressive UORC stream verified: {} raw bytes -> {} compressed stream bytes across {} LOD objects",
        summary.decoded_total_bytes,
        summary.compressed_bytes,
        summary.lod_object_count
    );

    let lod3_world = decode_uorc_stream_up_to_lod(&stream_bytes, 3)?;
    println!(
        "Decoded LOD3 lossless scene: {} ECS entities, {} universal node presets",
        lod3_world.scene.entity_count(),
        lod3_world.scene_nodes.len()
    );

    let positions = [[1.0_f32, 0.0, -2.0], [0.0, 3.0, 1.0]];
    let xform = Mat4::from_translation(Vec3::new(2.0, 1.0, -3.0));
    let transformed = batch_transform_positions_exact(&positions, &xform)?;
    println!(
        "Exact uor-matmul transformed vertex 0: {:?}",
        transformed[0]
    );

    let vp = Mat4::perspective(1.0, 16.0 / 9.0, 0.1, 150.0)
        * Mat4::look_at(Vec3::new(0.0, 2.0, 8.0), Vec3::ZERO, Vec3::UP);
    let tri = RasterTriangle {
        positions: [[-1.5, -1.0, 0.0], [1.5, -1.0, 0.0], [0.0, 1.8, 0.0]],
        normal: [0.0, 0.0, 1.0],
        base_color: [0.28, 0.72, 0.90],
    };
    let fb =
        render_triangles_tile_framebuffer(128, 80, &vp, Vec3::new(0.35, 0.8, 0.45), 3, &[tri])?;
    println!(
        "Portable Prism 16x16 tile framebuffer rendered: {}x{} ({} tiles, {} triangles)",
        fb.width, fb.height, fb.tiles_shaded, fb.rasterized_triangles
    );

    let shaders = validate_builtin_engine_shaders()?;
    let meshes = canonical_scene_mesh_library(lod3_world.seed);
    let materials = canonical_pbr_material_library();
    let glb = export_scene_to_glb(&lod3_world.scene, &meshes, &materials)?;
    let glb_summary = validate_glb_bytes(&glb)?;
    println!(
        "Validated {} WGSL shaders and exported {} bytes of glTF 2.0 (.glb) ({} meshes)",
        shaders.len(),
        glb.len(),
        glb_summary.mesh_count
    );
    Ok(())
}
