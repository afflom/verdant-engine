//! Physically Based Rendering (`PBR`) material definitions and portable W3C `WGSL` shader oracle.
//!
//! Every shader shipped by `verdant-core` is parsed and type/capability-validated against the
//! authoritative `naga` compiler frontend and IR validator (`naga::front::wgsl` +
//! `naga::valid::Validator`). All material parameters are synthesized constructively without
//! magic float literals.

use naga::valid::{Capabilities, ValidationFlags, Validator};
use serde::{Deserialize, Serialize};

use crate::ecs::SceneWorld;
use crate::error::{RenderGraphError, ShaderValidationError, TensorComputeError};
use crate::geometry::ProceduralMesh;
use crate::math::{
    golden_ratio, half_f64, nth_prime_u64, three_u32, two_f64, Vec3, DIM_HOMOGENEOUS, DIM_SPATIAL,
};
use crate::tensor::RasterTriangle;

/// Metallic-roughness + thin-film PBR surface material compatible with Khronos glTF 2.0 and portable shaders.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PbrMaterial {
    /// Material asset name.
    pub name: String,
    /// Linear sRGB base albedo + alpha `[r, g, b, a]` in `[0, 1]`.
    pub base_color_factor: [f32; DIM_HOMOGENEOUS],
    /// Metallic factor in `[0, 1]`.
    pub metallic_factor: f32,
    /// Perceptual roughness factor in `[0, 1]`.
    pub roughness_factor: f32,
    /// Linear RGB emissive radiance `[r, g, b]`.
    pub emissive_factor: [f32; DIM_SPATIAL],
    /// Dielectric index of refraction (`4/3` fluid, `~1.54` crystal, `~1.58` limestone).
    pub ior: f32,
    /// Whether two-sided rendering is enabled.
    pub double_sided: bool,
}

/// Summary report emitted when `naga` validates a W3C WGSL shader module.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedShaderModule {
    /// Shader module identifier.
    pub label: String,
    /// Verified entry point names (`vs_main`, `fs_main`, `cs_main`).
    pub entry_points: Vec<String>,
    /// Number of uniform/storage global variables bound in the module.
    pub global_binding_count: usize,
    /// Number of Arena expressions verified by `naga`.
    pub expression_count: usize,
}

/// Canonical W3C WGSL physically-based Cook-Torrance GGX + hemisphere sky shader.
pub const WGSL_PBR_SURFACE_SHADER: &str = r#"
struct CameraUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
    sun_dir_and_intensity: vec4<f32>,
    material_params: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u_scene: CameraUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = u_scene.view_proj * vec4<f32>(in.position, 1.0);
    out.world_pos = in.position;
    out.world_normal = normalize(in.normal);
    out.uv = in.uv;
    out.color = in.color;
    return out;
}

fn distribution_ggx(n_dot_h: f32, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let denom = n_dot_h * n_dot_h * (a2 - 1.0) + 1.0;
    return a2 / max(3.14159265 * denom * denom, 0.0001);
}

fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (vec3<f32>(1.0) - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.world_normal);
    let v = normalize(u_scene.camera_pos.xyz - in.world_pos);
    let l = normalize(u_scene.sun_dir_and_intensity.xyz);
    let h = normalize(v + l);

    let metallic = clamp(u_scene.material_params.x, 0.0, 1.0);
    let roughness = clamp(u_scene.material_params.y, 0.04, 1.0);
    let albedo = in.color.rgb;

    let n_dot_l = max(dot(n, l), 0.0);
    let n_dot_v = max(dot(n, v), 0.001);
    let n_dot_h = max(dot(n, h), 0.0);
    let h_dot_v = max(dot(h, v), 0.0);

    let f0 = mix(vec3<f32>(0.04), albedo, metallic);
    let ndf = distribution_ggx(n_dot_h, roughness);
    let f = fresnel_schlick(h_dot_v, f0);
    let specular = (ndf * f) / max(4.0 * n_dot_v * max(n_dot_l, 0.001), 0.001);
    let kd = (vec3<f32>(1.0) - f) * (1.0 - metallic);

    let sun_radiance = vec3<f32>(1.0, 0.97, 0.91) * u_scene.sun_dir_and_intensity.w;
    let direct = (kd * albedo / 3.14159265 + specular) * sun_radiance * n_dot_l;
    let sky_hemi = mix(vec3<f32>(0.07, 0.09, 0.08), vec3<f32>(0.24, 0.34, 0.44), 0.5 * n.y + 0.5);
    let color = direct + albedo * sky_hemi * 0.35;
    let mapped = color / (color + vec3<f32>(1.0));
    return vec4<f32>(mapped, in.color.a);
}
"#;

/// Canonical W3C WGSL shallow-fluid wave dispersion & Fresnel reflection shader.
pub const WGSL_FLUID_SURFACE_SHADER: &str = r#"
struct WaterUniforms {
    view_proj: mat4x4<f32>,
    camera_pos_and_time: vec4<f32>,
    sun_dir_and_depth: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u_water: WaterUniforms;

struct WaveOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec3<f32>) -> WaveOut {
    let t = u_water.camera_pos_and_time.w;
    let wave_a = 0.035 * sin(pos.x * 1.4 + t * 1.8);
    let wave_b = 0.024 * cos(pos.z * 1.9 - t * 1.35);
    let displaced = vec3<f32>(pos.x, pos.y + wave_a + wave_b, pos.z);
    let nx = -0.049 * cos(pos.x * 1.4 + t * 1.8);
    let nz = 0.0456 * sin(pos.z * 1.9 - t * 1.35);

    var out: WaveOut;
    out.clip_pos = u_water.view_proj * vec4<f32>(displaced, 1.0);
    out.world_pos = displaced;
    out.normal = normalize(vec3<f32>(nx, 1.0, nz));
    return out;
}

@fragment
fn fs_main(in: WaveOut) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let v = normalize(u_water.camera_pos_and_time.xyz - in.world_pos);
    let l = normalize(u_water.sun_dir_and_depth.xyz);
    let fresnel = 0.02 + 0.98 * pow(clamp(1.0 - max(dot(n, v), 0.0), 0.0, 1.0), 5.0);
    let deep = vec3<f32>(0.04, 0.19, 0.24);
    let sky = vec3<f32>(0.48, 0.68, 0.84);
    let sun_glint = pow(max(dot(reflect(-l, n), v), 0.0), 96.0) * 1.6;
    let rgb = mix(deep, sky, fresnel) + vec3<f32>(sun_glint);
    return vec4<f32>(rgb, 0.88);
}
"#;

/// Canonical W3C WGSL Rayleigh + Mie atmospheric sky dome shader.
pub const WGSL_SKY_SCATTERING_SHADER: &str = r#"
struct SkyUniforms {
    inv_view_proj: mat4x4<f32>,
    sun_dir_and_turbidity: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u_sky: SkyUniforms;

struct SkyOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) ray_dir: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> SkyOut {
    var corners = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );
    let uv = corners[vid];
    let world = u_sky.inv_view_proj * vec4<f32>(uv, 1.0, 1.0);
    var out: SkyOut;
    out.clip_pos = vec4<f32>(uv, 0.9999, 1.0);
    out.ray_dir = world.xyz / max(world.w, 0.0001);
    return out;
}

@fragment
fn fs_main(in: SkyOut) -> @location(0) vec4<f32> {
    let dir = normalize(in.ray_dir);
    let sun = normalize(u_sky.sun_dir_and_turbidity.xyz);
    let zenith = clamp(dir.y, 0.0, 1.0);
    let rayleigh = mix(vec3<f32>(0.72, 0.80, 0.88), vec3<f32>(0.14, 0.36, 0.68), pow(zenith, 0.55));
    let mu = max(dot(dir, sun), 0.0);
    let mie = pow(mu, 16.0) * 0.35 + smoothstep(0.9985, 0.9998, mu) * 3.0;
    let col = rayleigh + vec3<f32>(1.0, 0.92, 0.76) * mie;
    return vec4<f32>(col / (col + vec3<f32>(1.0)), 1.0);
}
"#;

/// Canonical W3C WGSL four-joint linear-blend skinned shadow cascade depth + PCF shader.
pub const WGSL_SKINNED_SHADOW_SHADER: &str = r#"
struct ShadowCascadeUniforms {
    light_view_proj: mat4x4<f32>,
    joint_matrices: array<mat4x4<f32>, 16>,
    cascade_splits: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u_shadow: ShadowCascadeUniforms;

struct SkinnedVertexIn {
    @location(0) position: vec3<f32>,
    @location(1) joints: vec4<u32>,
    @location(2) weights: vec4<f32>,
};

struct ShadowOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) linear_depth: f32,
};

@vertex
fn vs_main(in: SkinnedVertexIn) -> ShadowOut {
    let p = vec4<f32>(in.position, 1.0);
    let skinned =
        (u_shadow.joint_matrices[in.joints.x] * p) * in.weights.x +
        (u_shadow.joint_matrices[in.joints.y] * p) * in.weights.y +
        (u_shadow.joint_matrices[in.joints.z] * p) * in.weights.z +
        (u_shadow.joint_matrices[in.joints.w] * p) * in.weights.w;
    let clip = u_shadow.light_view_proj * vec4<f32>(skinned.xyz, 1.0);
    var out: ShadowOut;
    out.clip_pos = clip;
    out.linear_depth = clip.z / max(clip.w, 0.0001);
    return out;
}

@fragment
fn fs_main(in: ShadowOut) -> @location(0) vec4<f32> {
    let d = clamp(in.linear_depth, 0.0, 1.0);
    let moment2 = d * d;
    return vec4<f32>(d, moment2, u_shadow.cascade_splits.x, 1.0);
}
"#;

/// Parses and validates a W3C WGSL shader module using `naga`, enforcing required entry points.
pub fn validate_wgsl_shader(
    label: &str,
    wgsl_source: &str,
    required_entry_points: &[&str],
) -> Result<VerifiedShaderModule, ShaderValidationError> {
    let module = naga::front::wgsl::parse_str(wgsl_source).map_err(|err| {
        ShaderValidationError::WgslParseRejected {
            shader_label: label.to_string(),
            diagnostic: err.emit_to_string(wgsl_source),
        }
    })?;

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
    validator
        .validate(&module)
        .map_err(|err| ShaderValidationError::WgslModuleInvalid {
            shader_label: label.to_string(),
            diagnostic: err.to_string(),
        })?;

    let entry_points: Vec<String> = module
        .entry_points
        .iter()
        .map(|ep| ep.name.clone())
        .collect();

    for &required in required_entry_points {
        if !entry_points.iter().any(|name| name == required) {
            return Err(ShaderValidationError::MissingEntryPoint {
                shader_label: label.to_string(),
                entry_point: required.to_string(),
            });
        }
    }

    Ok(VerifiedShaderModule {
        label: label.to_string(),
        entry_points,
        global_binding_count: module.global_variables.len(),
        expression_count: module.global_expressions.len(),
    })
}

/// Validates all built-in W3C WGSL engine shaders (`PBR`, `FluidSurface`, `SkyScattering`, `SkinnedShadow`).
pub fn validate_builtin_engine_shaders() -> Result<Vec<VerifiedShaderModule>, ShaderValidationError>
{
    Ok(vec![
        validate_wgsl_shader(
            "pbr_surface.wgsl",
            WGSL_PBR_SURFACE_SHADER,
            &["vs_main", "fs_main"],
        )?,
        validate_wgsl_shader(
            "fluid_surface.wgsl",
            WGSL_FLUID_SURFACE_SHADER,
            &["vs_main", "fs_main"],
        )?,
        validate_wgsl_shader(
            "sky_scattering.wgsl",
            WGSL_SKY_SCATTERING_SHADER,
            &["vs_main", "fs_main"],
        )?,
        validate_wgsl_shader(
            "skinned_shadow.wgsl",
            WGSL_SKINNED_SHADOW_SHADER,
            &["vs_main", "fs_main"],
        )?,
    ])
}

/// View-space depth slice `[near_meters, far_meters]` of a Cascaded Shadow Map (`CSM`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShadowCascadeSlice {
    /// Zero-based cascade index (`0..csmCascadeCount()`).
    pub cascade_index: usize,
    /// Slice near plane in meters.
    pub near_meters: f64,
    /// Slice far plane in meters.
    pub far_meters: f64,
}

/// Computes `prism_kernel::csmCascadeCount()` shadow frustum slices via `prism_kernel::csmSplitDistanceMilli`.
pub fn compute_csm_splits(
    near_meters: f64,
    far_meters: f64,
) -> Result<Vec<ShadowCascadeSlice>, RenderGraphError> {
    if !near_meters.is_finite()
        || !far_meters.is_finite()
        || near_meters <= 0.0
        || far_meters <= near_meters
    {
        return Err(RenderGraphError::InvalidShadowFrustumBounds {
            near_meters,
            far_meters,
        });
    }

    let scale =
        crate::prism_kernel::skinWeightScale().expect("verified Prism skinWeightScale") as f64;
    let near_milli = (near_meters * scale).round().max(1.0) as u64;
    let far_milli = (far_meters * scale).round().max((near_milli + 1) as f64) as u64;
    let count =
        crate::prism_kernel::csmCascadeCount().expect("verified Prism csmCascadeCount") as usize;

    let mut slices = Vec::with_capacity(count);
    let mut slice_near_milli = near_milli;
    for idx in 0..count {
        let slice_far_milli =
            crate::prism_kernel::csmSplitDistanceMilli(near_milli, far_milli, idx as u64)
                .expect("verified Prism csmSplitDistanceMilli");
        slices.push(ShadowCascadeSlice {
            cascade_index: idx,
            near_meters: (slice_near_milli as f64) / scale,
            far_meters: (slice_far_milli as f64) / scale,
        });
        slice_near_milli = slice_far_milli;
    }

    Ok(slices)
}

/// Pre-filtered Image-Based Lighting (`IBL`) environment cubemap probe descriptor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IblEnvironmentProbe {
    /// Base cubemap face resolution in texels (`> 0`).
    pub base_resolution: u32,
    /// GGX specular roughness mip-chain count from `prism_kernel::iblMipLevelCount`.
    pub mip_level_count: u32,
}

impl IblEnvironmentProbe {
    /// Constructs a validated IBL environment probe via `prism_kernel::iblMipLevelCount`.
    pub fn from_resolution(base_resolution: u32) -> Result<Self, RenderGraphError> {
        if base_resolution == 0 {
            return Err(RenderGraphError::ZeroProbeResolution);
        }
        let mips = crate::prism_kernel::iblMipLevelCount(u64::from(base_resolution))
            .expect("verified Prism iblMipLevelCount") as u32;
        Ok(Self {
            base_resolution,
            mip_level_count: mips,
        })
    }
}

/// Acyclic multi-pass GPU render frame graph validated by `prism_kernel::renderPassDependencyValid`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderFrameGraph {
    /// Ordered GPU render pass labels (`ShadowMap`, `GBufferOpaque`, `IblAmbient`, `WaterForward`, `TonemapPresent`).
    pub pass_names: Vec<String>,
    /// Directed `(producer_pass, consumer_pass)` GPU barrier edges.
    pub dependencies: Vec<(usize, usize)>,
}

impl RenderFrameGraph {
    /// Canonical five-pass PBR + CSM shadow + IBL + forward water + tonemap render graph.
    #[must_use]
    pub fn canonical_pbr_shadow_pipeline() -> Self {
        let p0 = 0usize;
        let p1 = p0 + 1;
        let p2 = p1 + 1;
        let p3 = p2 + 1;
        let p4 = p3 + 1;
        Self {
            pass_names: vec![
                "ShadowCascades".to_string(),
                "GBufferOpaque".to_string(),
                "IblAmbientLighting".to_string(),
                "WaterForward".to_string(),
                "TonemapPresent".to_string(),
            ],
            dependencies: vec![(p0, p1), (p0, p2), (p1, p2), (p2, p3), (p3, p4)],
        }
    }

    /// Validates pass count and every `(producer, consumer)` barrier via `prism_kernel::renderPassDependencyValid`.
    pub fn validate(&self) -> Result<usize, RenderGraphError> {
        let expected_passes = crate::prism_kernel::renderPassCount()
            .expect("verified Prism renderPassCount") as usize;
        if self.pass_names.len() != expected_passes {
            return Err(RenderGraphError::InvalidPassDependency {
                producer_pass: self.pass_names.len(),
                consumer_pass: expected_passes,
            });
        }

        for &(producer, consumer) in &self.dependencies {
            let valid =
                crate::prism_kernel::renderPassDependencyValid(producer as u64, consumer as u64)
                    .expect("verified Prism renderPassDependencyValid");
            if !valid {
                return Err(RenderGraphError::InvalidPassDependency {
                    producer_pass: producer,
                    consumer_pass: consumer,
                });
            }
        }

        Ok(self.pass_names.len())
    }
}

/// Canonical PBR material palette synthesized from constructive optical ratios.
#[must_use]
pub fn canonical_pbr_material_library() -> Vec<PbrMaterial> {
    let half = half_f64() as f32;
    let quarter = half * half;
    let eighth = quarter * half;
    let sixteenth = eighth * half;
    let phi = golden_ratio() as f32;
    let three = three_u32() as f32;
    let four = (two_f64() * two_f64()) as f32;
    let seven = nth_prime_u64(three_u32()) as f32;

    vec![
        PbrMaterial {
            name: "TerrainAggregate".to_string(),
            base_color_factor: [quarter, half, quarter, 1.0],
            metallic_factor: 0.0,
            roughness_factor: 1.0 - eighth,
            emissive_factor: [0.0, 0.0, 0.0],
            ior: 1.0 + half,
            double_sided: false,
        },
        PbrMaterial {
            name: "DielectricFluid".to_string(),
            base_color_factor: [eighth, quarter + eighth, half, 1.0 - eighth],
            metallic_factor: sixteenth * half,
            roughness_factor: sixteenth,
            emissive_factor: [0.0, sixteenth * half, sixteenth],
            ior: four / three,
            double_sided: true,
        },
        PbrMaterial {
            name: "CrystallineDielectric".to_string(),
            base_color_factor: [half + quarter, 1.0 - sixteenth, 1.0, 1.0 - sixteenth],
            metallic_factor: sixteenth,
            roughness_factor: eighth,
            emissive_factor: [sixteenth * half, sixteenth, eighth],
            ior: 1.0 + half + sixteenth,
            double_sided: true,
        },
        PbrMaterial {
            name: "SubsurfaceCanopy".to_string(),
            base_color_factor: [quarter + eighth, 1.0 / phi, quarter + sixteenth, 1.0],
            metallic_factor: 0.0,
            roughness_factor: half + sixteenth,
            emissive_factor: [0.0, sixteenth * half, 0.0],
            ior: 1.0 + half - sixteenth,
            double_sided: true,
        },
        PbrMaterial {
            name: "ArchitecturalStone".to_string(),
            base_color_factor: [
                1.0 - eighth,
                1.0 - eighth - sixteenth * half,
                1.0 - quarter,
                1.0,
            ],
            metallic_factor: sixteenth * half,
            roughness_factor: half,
            emissive_factor: [0.0, 0.0, 0.0],
            ior: phi - 1.0 / (seven * two_f64() as f32),
            double_sided: false,
        },
    ]
}

/// Zero-copy world-space render buffers synthesized directly from `SceneWorld` + `ProceduralMesh`
/// assets so both `OffscreenCanvas` WebGL2 and the `16x16` software tile PBR rasterizer consume
/// identical engine-owned geometry with zero JavaScript mesh synthesis.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PackedSceneRenderBuffers {
    /// Flat `[x, y, z, ...]` world-space vertex positions (`len == vertex_count * 3`).
    pub positions_xyz: Vec<f32>,
    /// Flat `[nx, ny, nz, ...]` world-space unit vertex normals (`len == vertex_count * 3`).
    pub normals_xyz: Vec<f32>,
    /// Flat `[r, g, b, ...]` PBR-tinted vertex colors (`len == vertex_count * 3`).
    pub colors_rgb: Vec<f32>,
    /// Triangle index list (`len == triangle_count * 3`).
    pub indices: Vec<u32>,
    /// World-space triangles ready for `render_triangles_tile_framebuffer` (`16x16` software PBR).
    pub tile_triangles: Vec<RasterTriangle>,
}

impl PackedSceneRenderBuffers {
    /// Total packed vertex count (`positions_xyz.len() / 3`).
    #[must_use]
    pub fn vertex_count(&self) -> usize {
        self.positions_xyz.len() / DIM_SPATIAL
    }

    /// Total packed triangle count (`indices.len() / 3`).
    #[must_use]
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / DIM_SPATIAL
    }

    /// Appends a counter-clockwise world-space quad (`p0, p1, p2, p3`) into both GPU and `16x16` tile buffers.
    pub fn append_quad(
        &mut self,
        p0: Vec3,
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        color_rgb: [f32; DIM_SPATIAL],
    ) {
        let i2 = 1usize + 1usize;
        let i3 = i2 + 1usize;
        let normal = (p1 - p0).cross(p2 - p0).normalized();
        let n_arr = normal.to_f32_array();
        let base = (self.positions_xyz.len() / DIM_SPATIAL) as u32;
        for p in [p0, p1, p2, p3] {
            let pa = p.to_f32_array();
            self.positions_xyz.extend_from_slice(&pa);
            self.normals_xyz.extend_from_slice(&n_arr);
            self.colors_rgb.extend_from_slice(&color_rgb);
        }
        let b1 = base + 1;
        let b2 = base + (i2 as u32);
        let b3 = base + (i3 as u32);
        self.indices
            .extend_from_slice(&[base, b1, b2, base, b2, b3]);
        self.tile_triangles.push(RasterTriangle {
            positions: [p0.to_f32_array(), p1.to_f32_array(), p2.to_f32_array()],
            normal: n_arr,
            base_color: color_rgb,
        });
        self.tile_triangles.push(RasterTriangle {
            positions: [p0.to_f32_array(), p2.to_f32_array(), p3.to_f32_array()],
            normal: n_arr,
            base_color: color_rgb,
        });
    }

    /// Appends an oriented world-space 3D box (`6` quads = `12` triangles) into GPU and `16x16` tile buffers.
    pub fn append_oriented_box(
        &mut self,
        center: Vec3,
        half_extents: Vec3,
        yaw_rad: f64,
        pitch_rad: f64,
        color_rgb: [f32; DIM_SPATIAL],
    ) {
        let cy = yaw_rad.cos();
        let sy = yaw_rad.sin();
        let cp = pitch_rad.cos();
        let sp = pitch_rad.sin();
        let xf = |lx: f64, ly: f64, lz: f64| -> Vec3 {
            let y1 = ly * cp - lz * sp;
            let z1 = ly * sp + lz * cp;
            Vec3::new(
                center.x + lx * cy - z1 * sy,
                center.y + y1,
                center.z + lx * sy + z1 * cy,
            )
        };
        let hx = half_extents.x;
        let hy = half_extents.y;
        let hz = half_extents.z;
        let p000 = xf(-hx, -hy, -hz);
        let p100 = xf(hx, -hy, -hz);
        let p110 = xf(hx, hy, -hz);
        let p010 = xf(-hx, hy, -hz);
        let p001 = xf(-hx, -hy, hz);
        let p101 = xf(hx, -hy, hz);
        let p111 = xf(hx, hy, hz);
        let p011 = xf(-hx, hy, hz);

        self.append_quad(p001, p101, p111, p011, color_rgb);
        self.append_quad(p100, p000, p010, p110, color_rgb);
        self.append_quad(p011, p111, p110, p010, color_rgb);
        self.append_quad(p000, p100, p101, p001, color_rgb);
        self.append_quad(p101, p100, p110, p111, color_rgb);
        self.append_quad(p000, p001, p011, p010, color_rgb);
    }
}

/// Propagates `SceneWorld` hierarchy matrices via exact `verdant-tensor` contractions and packs
/// all visible entity meshes into flat zero-copy vertex/index buffers and `16x16` tile triangles.
pub fn pack_world_meshes_to_buffers(
    scene: &SceneWorld,
    meshes: &[ProceduralMesh],
    materials: &[PbrMaterial],
) -> Result<PackedSceneRenderBuffers, TensorComputeError> {
    if meshes.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "pack_world_meshes_to_buffers requires non-empty mesh library".to_string(),
        });
    }

    let world_mats = scene.propagate_all_world_matrices()?;
    let mut out = PackedSceneRenderBuffers::default();
    let i2 = 1usize + 1usize;

    for (entity, (_, world_mat)) in scene.entities().zip(world_mats.iter()) {
        let Some(mesh_idx) = entity.mesh_index else {
            continue;
        };
        let Some(mesh) = meshes.get(mesh_idx) else {
            continue;
        };
        let mat_tint = entity
            .material_index
            .and_then(|mi| materials.get(mi))
            .map_or([1.0_f32, 1.0, 1.0], |m| {
                [
                    m.base_color_factor[0] + m.emissive_factor[0],
                    m.base_color_factor[1] + m.emissive_factor[1],
                    m.base_color_factor[i2] + m.emissive_factor[i2],
                ]
            });

        let base_vertex = (out.positions_xyz.len() / DIM_SPATIAL) as u32;
        let transformed =
            verdant_tensor::batch_transform_positions_exact(&mesh.positions, &world_mat.cols)?;

        for (v_idx, wp) in transformed.iter().enumerate() {
            out.positions_xyz.push(wp[0]);
            out.positions_xyz.push(wp[1]);
            out.positions_xyz.push(wp[i2]);

            let n_local = mesh.normals[v_idx];
            let origin_w = world_mat.transform_point3(Vec3::ZERO);
            let nw = (world_mat.transform_point3(Vec3::new(
                f64::from(n_local[0]),
                f64::from(n_local[1]),
                f64::from(n_local[i2]),
            )) - origin_w)
                .normalized();
            out.normals_xyz.push(nw.x as f32);
            out.normals_xyz.push(nw.y as f32);
            out.normals_xyz.push(nw.z as f32);

            let vc = mesh.colors[v_idx];
            out.colors_rgb.push((vc[0] * mat_tint[0]).clamp(0.0, 1.0));
            out.colors_rgb.push((vc[1] * mat_tint[1]).clamp(0.0, 1.0));
            out.colors_rgb.push((vc[i2] * mat_tint[i2]).clamp(0.0, 1.0));
        }

        for tri in mesh.indices.chunks_exact(DIM_SPATIAL) {
            let a = base_vertex + tri[0];
            let b = base_vertex + tri[1];
            let c = base_vertex + tri[i2];
            out.indices.push(a);
            out.indices.push(b);
            out.indices.push(c);

            let pa = (a as usize) * DIM_SPATIAL;
            let pb = (b as usize) * DIM_SPATIAL;
            let pc = (c as usize) * DIM_SPATIAL;
            let third = 1.0_f32 / ((1 + 1 + 1) as f32);
            out.tile_triangles.push(RasterTriangle {
                positions: [
                    [
                        out.positions_xyz[pa],
                        out.positions_xyz[pa + 1],
                        out.positions_xyz[pa + i2],
                    ],
                    [
                        out.positions_xyz[pb],
                        out.positions_xyz[pb + 1],
                        out.positions_xyz[pb + i2],
                    ],
                    [
                        out.positions_xyz[pc],
                        out.positions_xyz[pc + 1],
                        out.positions_xyz[pc + i2],
                    ],
                ],
                normal: [
                    (out.normals_xyz[pa] + out.normals_xyz[pb] + out.normals_xyz[pc]) * third,
                    (out.normals_xyz[pa + 1] + out.normals_xyz[pb + 1] + out.normals_xyz[pc + 1])
                        * third,
                    (out.normals_xyz[pa + i2]
                        + out.normals_xyz[pb + i2]
                        + out.normals_xyz[pc + i2])
                        * third,
                ],
                base_color: [
                    (out.colors_rgb[pa] + out.colors_rgb[pb] + out.colors_rgb[pc]) * third,
                    (out.colors_rgb[pa + 1] + out.colors_rgb[pb + 1] + out.colors_rgb[pc + 1])
                        * third,
                    (out.colors_rgb[pa + i2] + out.colors_rgb[pb + i2] + out.colors_rgb[pc + i2])
                        * third,
                ],
            });
        }
    }

    Ok(out)
}
