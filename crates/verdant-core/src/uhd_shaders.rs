//! W3C WGSL UHD compute meshlet culling, PBR & CAS shaders (`uhd_shaders.rs`).

#[cfg(feature = "wgsl-oracle")]
use super::*;

/// W3C WGSL `@compute @workgroup_size(64)` GPU meshlet frustum + backface normal-cone + SSE culler.
pub const WGSL_UHD_MESHLET_CULL_SHADER: &str = r#"
struct MeshletBounds {
    center_radius: vec4<f32>,
    cone_axis_cutoff: vec4<f32>,
    index_offset: u32,
    index_count: u32,
    material_slot: u32,
    _pad: u32,
};

struct CullUniforms {
    eye_fov: vec4<f32>,
    viewport_sse: vec4<f32>,
    frustum_planes: array<vec4<f32>, 6>,
    meshlet_count: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

struct DrawArgs {
    visible_count: atomic<u32>,
};

@group(0) @binding(0) var<uniform> u_cull: CullUniforms;
@group(0) @binding(1) var<storage, read> b_meshlets: array<MeshletBounds>;
@group(0) @binding(2) var<storage, read_write> b_visible_ids: array<u32>;
@group(0) @binding(3) var<storage, read_write> b_draw: DrawArgs;

@compute @workgroup_size(64)
fn cs_cull_meshlets(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= u_cull.meshlet_count) {
        return;
    }
    let m = b_meshlets[idx];
    let center = m.center_radius.xyz;
    let radius = max(m.center_radius.w, 0.001);

    // 1. Six-plane camera frustum sphere test:
    for (var p: u32 = 0u; p < 6u; p = p + 1u) {
        let pl = u_cull.frustum_planes[p];
        if (dot(pl.xyz, center) + pl.w < -radius) {
            return;
        }
    }

    // 2. Normal-cone backface cluster rejection:
    let to_center = center - u_cull.eye_fov.xyz;
    let dist = max(length(to_center), 0.001);
    let view_dir = to_center / dist;
    let cutoff = m.cone_axis_cutoff.w;
    if (cutoff < 1.0 && dot(view_dir, m.cone_axis_cutoff.xyz) >= cutoff) {
        return;
    }

    // 3. UHD Screen-Space Error (SSE) sub-pixel cluster rejection:
    let proj_px = (radius * u_cull.viewport_sse.y) / max(dist * u_cull.eye_fov.w, 0.001);
    if (proj_px < u_cull.viewport_sse.z) {
        return;
    }

    let slot = atomicAdd(&b_draw.visible_count, 1u);
    b_visible_ids[slot] = idx;
}
"#;

/// W3C WGSL UHD 4-channel Khronos glTF 2.0 PBR (`albedo`, `metallicRoughness`, screen-space
/// cotangent `normal`, HDR `emissive`) + Equirectangular HDRI IBL + ACES Filmic shader.
pub const WGSL_UHD_PBR_CAS_SHADER: &str = r#"
struct UhdSceneUniforms {
    mvp: mat4x4<f32>,
    eye_time: vec4<f32>,
    sun_cas: vec4<f32>,
    sky_flags: vec4<f32>,
    pbr_flags: vec4<f32>,
};

struct VsIn {
    @location(0) pos: vec3<f32>,
    @location(1) norm: vec3<f32>,
    @location(2) col: vec3<f32>,
    @location(3) uv: vec2<f32>,
};

struct VsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) world_norm: vec3<f32>,
    @location(2) vert_col: vec3<f32>,
    @location(3) uv: vec2<f32>,
};

@group(0) @binding(0) var<uniform> u_scene: UhdSceneUniforms;
@group(0) @binding(1) var u_sampler: sampler;
@group(0) @binding(2) var u_albedo_tex: texture_2d<f32>;
@group(0) @binding(3) var u_mr_tex: texture_2d<f32>;
@group(0) @binding(4) var u_normal_tex: texture_2d<f32>;
@group(0) @binding(5) var u_emissive_tex: texture_2d<f32>;
@group(0) @binding(6) var u_env_tex: texture_2d<f32>;

@vertex
fn vs_uhd_pbr(in: VsIn) -> VsOut {
    var out: VsOut;
    out.clip_pos = u_scene.mvp * vec4<f32>(in.pos, 1.0);
    out.world_pos = in.pos;
    out.world_norm = normalize(in.norm);
    out.vert_col = in.col;
    out.uv = in.uv;
    return out;
}

fn aces_filmic(x: vec3<f32>) -> vec3<f32> {
    return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0));
}

fn sample_equirect_env(dir: vec3<f32>) -> vec3<f32> {
    let d = normalize(dir);
    let u = atan2(d.z, d.x) * 0.15915494 + 0.5;
    let v = acos(clamp(d.y, -1.0, 1.0)) * 0.31830989;
    return pow(max(textureSample(u_env_tex, u_sampler, vec2<f32>(u, v)).rgb, vec3<f32>(0.001)), vec3<f32>(2.2));
}

@fragment
fn fs_uhd_pbr(in: VsOut) -> @location(0) vec4<f32> {
    let n_base = normalize(in.world_norm);
    let v = normalize(u_scene.eye_time.xyz - in.world_pos);
    let l = normalize(u_scene.sun_cas.xyz);
    let uv = max(in.uv, vec2<f32>(0.0));
    let use_tex = u_scene.sky_flags.w > 0.5 && in.uv.x > -0.5;
    let has_mr = u_scene.pbr_flags.x > 0.5 && in.uv.x > -0.5;
    let has_norm = u_scene.pbr_flags.y > 0.5 && in.uv.x > -0.5;
    let has_emis = u_scene.pbr_flags.z > 0.5 && in.uv.x > -0.5;
    let has_env = u_scene.pbr_flags.w > 0.5;

    let tex_albedo = textureSample(u_albedo_tex, u_sampler, uv).rgb;
    let tex_mr = textureSample(u_mr_tex, u_sampler, uv).rgb;
    let tex_nm = textureSample(u_normal_tex, u_sampler, uv).rgb * 2.0 - vec3<f32>(1.0);
    let tex_em = textureSample(u_emissive_tex, u_sampler, uv).rgb;

    // Screen-space cotangent frame for normal mapping (computed in uniform control flow):
    let dp1 = dpdx(in.world_pos);
    let dp2 = dpdy(in.world_pos);
    let duv1 = dpdx(uv);
    let duv2 = dpdy(uv);
    let dp2perp = cross(dp2, n_base);
    let dp1perp = cross(n_base, dp1);
    let t_raw = dp2perp * duv1.x + dp1perp * duv2.x;
    let b_raw = dp2perp * duv1.y + dp1perp * duv2.y;
    let invmax = inverseSqrt(max(dot(t_raw, t_raw), dot(b_raw, b_raw)) + 0.00001);
    let n_mapped = normalize(t_raw * (tex_nm.x * invmax) + b_raw * (tex_nm.y * invmax) + n_base * max(tex_nm.z, 0.1));
    let n = select(n_base, n_mapped, has_norm);

    let h = normalize(l + v);
    let base_col = select(in.vert_col, tex_albedo, use_tex);
    let albedo = pow(max(base_col, vec3<f32>(0.02)), vec3<f32>(2.2));
    let rough = select(0.55, clamp(tex_mr.g, 0.08, 0.98), has_mr);
    let metal = select(0.08, clamp(tex_mr.b, 0.0, 1.0), has_mr);
    let ndl = max(dot(n, l), 0.0);
    let ndv = max(dot(n, v), 0.001);
    let ndh = max(dot(n, h), 0.0);
    let a = rough * rough;
    let a2 = a * a;
    let denom = ndh * ndh * (a2 - 1.0) + 1.0;
    let d_ggx = a2 / max(3.14159265 * denom * denom, 0.0001);
    let f0 = mix(vec3<f32>(0.04), albedo, metal);
    let fresnel = f0 + (vec3<f32>(1.0) - f0) * pow(1.0 - ndv, 5.0);
    let spec = (d_ggx * fresnel) / max(4.0 * ndv * ndl + 0.001, 0.001);
    let kd = (vec3<f32>(1.0) - fresnel) * (1.0 - metal);
    let direct = (kd * albedo / 3.14159265 + spec) * vec3<f32>(2.6, 2.4, 2.05) * ndl;
    let r_dir = reflect(-v, n);
    let env_refl = sample_equirect_env(r_dir);
    let env_diff = sample_equirect_env(n);
    let hemi = mix(vec3<f32>(0.08, 0.08, 0.09), u_scene.sky_flags.xyz + vec3<f32>(0.12), 0.5 + 0.5 * n.y);
    let amb_diff = select(hemi, env_diff * 0.85, has_env);
    let amb_spec = select(hemi, env_refl * 1.15, has_env);
    let emissive = select(vec3<f32>(0.0), pow(max(tex_em, vec3<f32>(0.0)), vec3<f32>(2.2)) * 1.6, has_emis);
    let hdr = direct + kd * albedo * amb_diff + fresnel * amb_spec * (1.0 - rough * 0.65) + emissive;
    let ldr = pow(aces_filmic(hdr * 1.06), vec3<f32>(1.0 / 2.2));
    return vec4<f32>(ldr, 1.0);
}
"#;

/// W3C WGSL 5-tap AMD FidelityFX Contrast-Adaptive Sharpening (`CAS`) resolve pass for WebGPU
/// UHD Dynamic Resolution Scaling (`65%..100%` internal -> native 4K/5K/8K output).
pub const WGSL_UHD_CAS_RESOLVE_SHADER: &str = r#"
struct CasUniforms {
    inv_res_sharp: vec4<f32>,
};

struct CasVsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0) @binding(0) var<uniform> u_cas: CasUniforms;
@group(0) @binding(1) var u_cas_sampler: sampler;
@group(0) @binding(2) var u_color_tex: texture_2d<f32>;

@vertex
fn vs_cas_resolve(@builtin(vertex_index) vid: u32) -> CasVsOut {
    var out: CasVsOut;
    let x = f32((vid & 1u) << 2u) - 1.0;
    let y = f32((vid & 2u) << 1u) - 1.0;
    out.pos = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>(x * 0.5 + 0.5, 0.5 - y * 0.5);
    return out;
}

@fragment
fn fs_cas_resolve(in: CasVsOut) -> @location(0) vec4<f32> {
    let ir = u_cas.inv_res_sharp.xy;
    let sharp = clamp(u_cas.inv_res_sharp.z, 0.0, 1.0);
    let c = textureSample(u_color_tex, u_cas_sampler, in.uv).rgb;
    let n = textureSample(u_color_tex, u_cas_sampler, in.uv + vec2<f32>(0.0, -ir.y)).rgb;
    let s = textureSample(u_color_tex, u_cas_sampler, in.uv + vec2<f32>(0.0, ir.y)).rgb;
    let e = textureSample(u_color_tex, u_cas_sampler, in.uv + vec2<f32>(ir.x, 0.0)).rgb;
    let w = textureSample(u_color_tex, u_cas_sampler, in.uv + vec2<f32>(-ir.x, 0.0)).rgb;
    let mn = min(c, min(min(n, s), min(e, w)));
    let mx = max(c, max(max(n, s), max(e, w)));
    let amp = clamp(min(mn, vec3<f32>(1.0) - mx) / max(mx, vec3<f32>(0.001)), vec3<f32>(0.0), vec3<f32>(1.0));
    let weight = -sqrt(amp) * (0.125 + 0.075 * sharp);
    let sharpened = (c + (n + s + e + w) * weight) / max(vec3<f32>(1.0) + 4.0 * weight, vec3<f32>(0.25));
    return vec4<f32>(clamp(sharpened, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
"#;

/// Validates Verdant's UHD WebGPU WGSL shaders (`cs_cull_meshlets`, `vs_uhd_pbr`, `fs_uhd_pbr`,
/// `vs_cas_resolve`, `fs_cas_resolve`) through the authoritative `naga` W3C WGSL oracle.
#[cfg(feature = "wgsl-oracle")]
pub fn validate_uhd_shaders() -> Result<Vec<VerifiedShaderModule>, ShaderValidationError> {
    Ok(vec![
        validate_wgsl_shader(
            "uhd_meshlet_cull.wgsl",
            WGSL_UHD_MESHLET_CULL_SHADER,
            &["cs_cull_meshlets"],
        )?,
        validate_wgsl_shader(
            "uhd_pbr_cas.wgsl",
            WGSL_UHD_PBR_CAS_SHADER,
            &["vs_uhd_pbr", "fs_uhd_pbr"],
        )?,
        validate_wgsl_shader(
            "uhd_cas_resolve.wgsl",
            WGSL_UHD_CAS_RESOLVE_SHADER,
            &["vs_cas_resolve", "fs_cas_resolve"],
        )?,
    ])
}
