//! High-definition Khronos PBR descriptors and W3C WGSL-to-WebGL2 GLSL ES 3.00 transpilation (`render_shaders.rs`).

use super::*;

/// High-definition Khronos glTF 2.0 PBR descriptor carrying optical
/// `KHR_materials_{ior,specular,transmission,clearcoat,emissive_strength}` extensions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HdPbrDescriptor {
    /// Catalog material identifier (`MAT-*`).
    pub id: String,
    /// Material asset name.
    pub name: String,
    /// Linear sRGB base albedo + alpha `[r, g, b, a]` in `[0, 1]`.
    pub base_color_factor: [f32; DIM_HOMOGENEOUS],
    /// Metallic factor in `[0, 1]`.
    pub metallic_factor: f32,
    /// Perceptual roughness factor in `[0, 1]`.
    pub roughness_factor: f32,
    /// Linear RGB emissive color `[r, g, b]` in `[0, 1]`.
    pub emissive_factor: [f32; DIM_SPATIAL],
    /// `KHR_materials_emissive_strength` multiplier (`>= 0`).
    pub emissive_strength: f32,
    /// `KHR_materials_ior` index of refraction (`>= 1`).
    pub ior: f32,
    /// `KHR_materials_specular` dielectric F0 strength in `[0, 1]`.
    pub specular_factor: f32,
    /// `KHR_materials_transmission` optical transmission factor in `[0, 1]`.
    pub transmission_factor: f32,
    /// `KHR_materials_clearcoat` top-coat intensity in `[0, 1]`.
    pub clearcoat_factor: f32,
    /// `KHR_materials_clearcoat` top-coat perceptual roughness in `[0, 1]`.
    pub clearcoat_roughness_factor: f32,
    /// Whether two-sided rendering is enabled.
    pub double_sided: bool,
}

impl HdPbrDescriptor {
    /// Projects this high-definition descriptor onto the core engine [`PbrMaterial`].
    #[must_use]
    pub fn to_pbr_material(&self) -> PbrMaterial {
        let i2 = 1usize + 1usize;
        let strength = self.emissive_strength.max(0.0);
        PbrMaterial {
            name: self.name.clone(),
            base_color_factor: self.base_color_factor,
            metallic_factor: self.metallic_factor.clamp(0.0, 1.0),
            roughness_factor: self.roughness_factor.clamp(0.0, 1.0),
            emissive_factor: [
                (self.emissive_factor[0] * strength).clamp(0.0, 1.0),
                (self.emissive_factor[1] * strength).clamp(0.0, 1.0),
                (self.emissive_factor[i2] * strength).clamp(0.0, 1.0),
            ],
            ior: self.ior.max(1.0),
            double_sided: self.double_sided,
        }
    }

    /// Maps Khronos HD optical extensions deterministically to [`GpuSurfaceClass`].
    #[must_use]
    pub fn surface_class(&self) -> GpuSurfaceClass {
        let half = half_f64() as f32;
        let sixteenth = half * half * half * half;
        let peak_emissive = self.emissive_factor.iter().copied().fold(0.0_f32, f32::max)
            * self.emissive_strength.max(0.0);
        if self.metallic_factor >= half {
            GpuSurfaceClass::Metal
        } else if self.transmission_factor >= half && peak_emissive <= sixteenth {
            GpuSurfaceClass::Fluid
        } else if peak_emissive > sixteenth || self.clearcoat_factor >= half {
            GpuSurfaceClass::EmissiveGlass
        } else {
            GpuSurfaceClass::OpaqueDielectric
        }
    }
}

/// Pipeline stage selector for W3C WGSL-to-WebGL2 GLSL ES `300 es` transpilation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShaderStageKind {
    /// `@vertex` stage.
    Vertex,
    /// `@fragment` stage.
    Fragment,
}

/// Verified WebGL2 GLSL ES `300 es` output emitted by the `naga` shader compiler oracle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranspiledWebGl2Shader {
    /// Source shader label.
    pub shader_label: String,
    /// Verified entry point name.
    pub entry_point: String,
    /// Pipeline stage (`Vertex` or `Fragment`).
    pub stage: ShaderStageKind,
    /// Generated `#version 300 es` GLSL source text.
    pub glsl_source: String,
}

/// Parses and validates a W3C `WGSL` module via `naga`, then transpiles `entry_point` into
/// WebGL2-compliant `GLSL ES 3.00` (`#version 300 es`) via `naga::back::glsl::Writer`.
#[cfg(feature = "wgsl-oracle")]
pub fn transpile_wgsl_to_webgl2_glsl(
    shader_label: &str,
    wgsl_source: &str,
    entry_point: &str,
    stage: ShaderStageKind,
) -> Result<TranspiledWebGl2Shader, OracleValidationError> {
    let module = naga::front::wgsl::parse_str(wgsl_source).map_err(|err| {
        OracleValidationError::GlslTranspilationFailed {
            shader_label: shader_label.to_string(),
            diagnostic: err.emit_to_string(wgsl_source),
        }
    })?;

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
    let info = validator.validate(&module).map_err(|err| {
        OracleValidationError::GlslTranspilationFailed {
            shader_label: shader_label.to_string(),
            diagnostic: err.to_string(),
        }
    })?;

    let naga_stage = match stage {
        ShaderStageKind::Vertex => naga::ShaderStage::Vertex,
        ShaderStageKind::Fragment => naga::ShaderStage::Fragment,
    };
    let has_entry = module
        .entry_points
        .iter()
        .any(|ep| ep.name == entry_point && ep.stage == naga_stage);
    if !has_entry {
        return Err(OracleValidationError::GlslTranspilationFailed {
            shader_label: shader_label.to_string(),
            diagnostic: format!(
                "entry point `{entry_point}` for stage {stage:?} not found in `{shader_label}`"
            ),
        });
    }

    let es_version = (three_u32() as u16) * (pow10_u64(two_u32()) as u16);
    let options = naga::back::glsl::Options {
        version: naga::back::glsl::Version::Embedded {
            version: es_version,
            is_webgl: true,
        },
        writer_flags: naga::back::glsl::WriterFlags::empty(),
        binding_map: naga::back::glsl::BindingMap::default(),
        zero_initialize_workgroup_memory: true,
    };
    let pipeline_options = naga::back::glsl::PipelineOptions {
        shader_stage: naga_stage,
        entry_point: entry_point.to_string(),
        multiview: None,
    };

    let mut glsl_source = String::new();
    let mut writer = naga::back::glsl::Writer::new(
        &mut glsl_source,
        &module,
        &info,
        &options,
        &pipeline_options,
        naga::proc::BoundsCheckPolicies::default(),
    )
    .map_err(|err| OracleValidationError::GlslTranspilationFailed {
        shader_label: shader_label.to_string(),
        diagnostic: err.to_string(),
    })?;

    writer
        .write()
        .map_err(|err| OracleValidationError::GlslTranspilationFailed {
            shader_label: shader_label.to_string(),
            diagnostic: err.to_string(),
        })?;

    Ok(TranspiledWebGl2Shader {
        shader_label: shader_label.to_string(),
        entry_point: entry_point.to_string(),
        stage,
        glsl_source,
    })
}

/// Transpiles all built-in W3C WGSL engine shaders (`vs_main` and `fs_main`) to WebGL2 `GLSL ES 3.00`.
#[cfg(feature = "wgsl-oracle")]
pub fn transpile_builtin_engine_shaders_webgl2(
) -> Result<Vec<TranspiledWebGl2Shader>, OracleValidationError> {
    let builtins = [
        ("pbr_surface.wgsl", WGSL_PBR_SURFACE_SHADER),
        ("fluid_surface.wgsl", WGSL_FLUID_SURFACE_SHADER),
        ("sky_scattering.wgsl", WGSL_SKY_SCATTERING_SHADER),
        ("skinned_shadow.wgsl", WGSL_SKINNED_SHADOW_SHADER),
    ];
    let mut out = Vec::with_capacity(builtins.len() * (1 + 1));
    for (label, src) in builtins {
        out.push(transpile_wgsl_to_webgl2_glsl(
            label,
            src,
            "vs_main",
            ShaderStageKind::Vertex,
        )?);
        out.push(transpile_wgsl_to_webgl2_glsl(
            label,
            src,
            "fs_main",
            ShaderStageKind::Fragment,
        )?);
    }
    Ok(out)
}
