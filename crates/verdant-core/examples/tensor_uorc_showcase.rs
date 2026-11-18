//! Showcase of Prism-hosted `uor-matmul` tensor kernels, progressive `UORC` streaming,
//! `naga` WGSL->WebGL2 `GLSL ES 3.00` transpilation, and HD glTF 2.0 PBR export over the
//! cataloged `sunspire-regatta` maritime domain pack.
//! Zero unexplained magic constants (`FI-08`): all scalars derive from `{0, 1}` or `prism_kernel`.

use verdant_core::gltf_io::{export_hd_scene_to_glb, validate_glb_bytes};
use verdant_core::math::{Mat4, Vec3};
use verdant_core::prism_kernel;
use verdant_core::render::{
    transpile_builtin_engine_shaders_webgl2, validate_builtin_engine_shaders,
};
use verdant_core::standards::CatalogBundleRegistry;
use verdant_core::streaming::{
    decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream, verify_progressive_uorc_stream,
};
use verdant_core::tensor::{
    batch_transform_positions_exact, render_triangles_tile_framebuffer,
    verified_engine_tensor_kernels, RasterTriangle,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kernels = verified_engine_tensor_kernels()?;
    let registry = CatalogBundleRegistry::embedded()?;
    let domain = registry.instantiate_domain_pack("sunspire-regatta", None)?;

    let max_lod = prism_kernel::wasmMaxLodRank();
    let stream_bytes = pack_progressive_uorc_stream(domain.spec.seed)?;
    let summary = verify_progressive_uorc_stream(&stream_bytes)?;
    let lod_world = decode_uorc_stream_up_to_lod(&stream_bytes, max_lod)?;

    let two = prism_kernel::sTwo() as f32;
    let two_f64 = f64::from(two);
    let eight_f64 = prism_kernel::sEight().unwrap_or(1) as f64;
    let nine_f64 = prism_kernel::sNine().unwrap_or(1) as f64;
    let ten_f64 = prism_kernel::sTen().unwrap_or(1) as f64;
    let fourteen_f64 = prism_kernel::sFourteen().unwrap_or(1) as f64;
    let sixteen_f64 = prism_kernel::sSixteen().unwrap_or(1) as f64;

    let positions = [[1.0_f32, 0.0, -two], [0.0, two + 1.0, 1.0]];
    let xform = Mat4::from_translation(Vec3::new(two_f64, 1.0, -(two_f64 + 1.0)));
    let transformed = batch_transform_positions_exact(&positions, &xform)?;

    let vp = Mat4::perspective(
        1.0,
        sixteen_f64 / nine_f64,
        1.0 / ten_f64,
        fourteen_f64 * ten_f64,
    ) * Mat4::look_at(Vec3::new(0.0, two_f64, eight_f64), Vec3::ZERO, Vec3::UP);
    let half = 1.0_f32 / two;
    let tri = RasterTriangle {
        positions: [
            [-1.0 - half, -1.0, 0.0],
            [1.0 + half, -1.0, 0.0],
            [0.0, two, 0.0],
        ],
        normal: [0.0, 0.0, 1.0],
        base_color: [half, 1.0 - half * half, 1.0],
    };
    let tile_edge = prism_kernel::tileEdgePixels().unwrap_or(1) as u32;
    let fb_w = tile_edge * (prism_kernel::sEight().unwrap_or(1) as u32);
    let fb_h = tile_edge * (prism_kernel::sFive().unwrap_or(1) as u32);
    let fb = render_triangles_tile_framebuffer(
        fb_w,
        fb_h,
        &vp,
        Vec3::new(f64::from(half), 1.0, f64::from(half)),
        max_lod,
        &[tri],
    )?;

    let shaders = validate_builtin_engine_shaders()?;
    let webgl2 = transpile_builtin_engine_shaders_webgl2()?;
    let glb = export_hd_scene_to_glb(
        &domain.sector.scene,
        &domain.sector.meshes,
        &domain.hd_materials,
    )?;
    let glb_summary = validate_glb_bytes(&glb)?;

    println!(
        "Sunspire Tensor & HD Showcase OK: kernels={} uorc={}B->{}B lod_entities={} v0={:?} tile_fb={}x{}({} tiles) wgsl={} webgl2={} hd_glb={}B({} meshes)",
        kernels.len(),
        summary.decoded_total_bytes,
        summary.compressed_bytes,
        lod_world.scene.entity_count(),
        transformed[0],
        fb.width,
        fb.height,
        fb.tiles_shaded,
        shaders.len(),
        webgl2.len(),
        glb.len(),
        glb_summary.mesh_count
    );
    Ok(())
}
