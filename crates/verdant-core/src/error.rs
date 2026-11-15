//! Typed failure classifications for `verdant-core` engine subsystems.
//!
//! Every public fallible operation returns one of these domain-specific error
//! types, each carrying parametric observed-versus-admissible bounds and
//! registered in `model/ids.toml` with a corresponding negative conformance test.

use std::fmt;

/// Refusal returned when engine or viewport configuration parameters violate bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineConfigError {
    /// Viewport width or height is zero.
    ZeroViewportDimension {
        /// Requested viewport width in pixels.
        width: u32,
        /// Requested viewport height in pixels.
        height: u32,
    },
    /// Simulation tick rate in hertz lies outside `[min_hz, max_hz]`.
    InvalidTickRateHz {
        /// Requested simulation tick frequency.
        tick_rate_hz: u32,
        /// Maximum admissible simulation tick frequency.
        max_hz: u32,
    },
    /// Target frame budget in microseconds is zero or exceeds `max_micros`.
    InvalidFrameBudgetMicros {
        /// Requested frame budget in microseconds.
        budget_micros: u64,
        /// Maximum admissible frame budget in microseconds.
        max_micros: u64,
    },
    /// Worker thread count is zero.
    ZeroWorkerThreads,
}

impl fmt::Display for EngineConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroViewportDimension { width, height } => {
                write!(
                    f,
                    "viewport dimensions must be positive, got {width}x{height}"
                )
            }
            Self::InvalidTickRateHz {
                tick_rate_hz,
                max_hz,
            } => {
                write!(
                    f,
                    "tick rate must be positive and at most {max_hz} Hz, got {tick_rate_hz}"
                )
            }
            Self::InvalidFrameBudgetMicros {
                budget_micros,
                max_micros,
            } => {
                write!(
                    f,
                    "frame budget must be positive and at most {max_micros} us, got {budget_micros}"
                )
            }
            Self::ZeroWorkerThreads => write!(f, "worker thread pool requires a positive count"),
        }
    }
}

impl std::error::Error for EngineConfigError {}

/// Refusal returned when world expansion violates causal epoch ordering or prerequisites.
#[derive(Clone, Debug, PartialEq)]
pub enum WorldEpochError {
    /// Cosmic scale factor lies outside `(0, 1]`.
    InvalidScaleFactor {
        /// Observed cosmic scale factor.
        scale_factor: f64,
    },
    /// Attempted to step backward in cosmological time.
    NonMonotonicExpansion {
        /// Current world scale factor.
        current_scale: f64,
        /// Requested target scale factor.
        requested_scale: f64,
    },
    /// Attempted to synthesize higher-order entities before precursor epoch completion.
    MissingPrecursorEpoch {
        /// Epoch required before the requested synthesis stage.
        required_epoch: &'static str,
        /// Current active world epoch.
        actual_epoch: &'static str,
    },
    /// Differentiated Builder guilds have not yet emerged from ecological evolution.
    BuildersNotYetDifferentiated {
        /// Ecological biomass index observed at the attempted build time.
        ecological_maturity: f64,
        /// Minimum ecological maturity required for Builder differentiation.
        minimum_required: f64,
    },
}

impl fmt::Display for WorldEpochError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidScaleFactor { scale_factor } => {
                write!(
                    f,
                    "cosmic scale factor must be positive and at most unit scale, got {scale_factor}"
                )
            }
            Self::NonMonotonicExpansion {
                current_scale,
                requested_scale,
            } => write!(
                f,
                "world expansion is monotonic: cannot regress from {current_scale} to {requested_scale}"
            ),
            Self::MissingPrecursorEpoch {
                required_epoch,
                actual_epoch,
            } => write!(
                f,
                "stage requires precursor epoch {required_epoch}, active epoch is {actual_epoch}"
            ),
            Self::BuildersNotYetDifferentiated {
                ecological_maturity,
                minimum_required,
            } => write!(
                f,
                "builder guilds require ecological maturity >= {minimum_required}, observed {ecological_maturity}"
            ),
        }
    }
}

impl std::error::Error for WorldEpochError {}

/// Refusal returned when a proposed architectural structure violates static equilibrium or utility.
#[derive(Clone, Debug, PartialEq)]
pub enum StructuralEquilibriumError {
    /// Masonry thrust-line eccentricity exceeds Heyman's middle-third stability limit.
    ThrustLineOutsideMiddleThird {
        /// Ratio `eccentricity / thickness` observed in the arch or pier section.
        eccentricity_ratio: f64,
        /// Maximum admissible Heyman middle-third eccentricity ratio.
        max_admissible_ratio: f64,
    },
    /// Compressive safety factor against material crushing falls below admissible bound.
    InsufficientSafetyFactor {
        /// Computed compressive or overturning safety factor.
        safety_factor: f64,
        /// Minimum admissible safety factor.
        minimum_required: f64,
    },
    /// Multi-objective Builder engineering utility is non-positive after material cost.
    NonPositiveEngineeringUtility {
        /// Evaluated net utility score.
        net_utility: f64,
    },
    /// Span or height dimension is non-positive or non-finite.
    InvalidStructuralDimensions {
        /// Span in meters.
        span_meters: f64,
        /// Height in meters.
        height_meters: f64,
    },
}

impl fmt::Display for StructuralEquilibriumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThrustLineOutsideMiddleThird {
                eccentricity_ratio,
                max_admissible_ratio,
            } => write!(
                f,
                "masonry thrust line exits middle third: e/t = {eccentricity_ratio} > {max_admissible_ratio}"
            ),
            Self::InsufficientSafetyFactor {
                safety_factor,
                minimum_required,
            } => write!(
                f,
                "structural safety factor {safety_factor} is below minimum {minimum_required}"
            ),
            Self::NonPositiveEngineeringUtility { net_utility } => write!(
                f,
                "builder optimization refused structure with non-positive net utility {net_utility}"
            ),
            Self::InvalidStructuralDimensions {
                span_meters,
                height_meters,
            } => write!(
                f,
                "structural span ({span_meters}) and height ({height_meters}) must be finite and positive"
            ),
        }
    }
}

impl std::error::Error for StructuralEquilibriumError {}

/// Refusal returned when a physics integration step receives invalid timestep or body parameters.
#[derive(Clone, Debug, PartialEq)]
pub enum PhysicsStepError {
    /// Integration timestep is non-finite, zero, negative, or exceeds configured stability bound.
    InvalidTimeStepSeconds {
        /// Supplied delta time in seconds.
        dt_seconds: f64,
        /// Configured upper timestep bound in seconds.
        max_dt_seconds: f64,
    },
    /// Rigid body mass or collider dimension is non-positive or non-finite.
    InvalidBodyMassOrRadius {
        /// Mass in kilograms.
        mass_kg: f64,
        /// Bounding radius in meters.
        radius_meters: f64,
    },
}

impl fmt::Display for PhysicsStepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTimeStepSeconds {
                dt_seconds,
                max_dt_seconds,
            } => write!(
                f,
                "physics timestep must be positive and at most {max_dt_seconds} s, got {dt_seconds}"
            ),
            Self::InvalidBodyMassOrRadius {
                mass_kg,
                radius_meters,
            } => write!(
                f,
                "rigid body mass ({mass_kg} kg) and radius ({radius_meters} m) must be finite and positive"
            ),
        }
    }
}

impl std::error::Error for PhysicsStepError {}

/// Refusal returned when a mesh or glTF binary asset fails structural or oracle validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssetValidationError {
    /// Mesh has zero vertices or zero triangles.
    EmptyMeshTopology {
        /// Mesh identifier.
        mesh_name: String,
    },
    /// Triangle index buffer length is not a multiple of triangle arity or references out-of-bounds vertices.
    InvalidTriangleIndices {
        /// Mesh identifier.
        mesh_name: String,
        /// Number of indices supplied.
        index_count: usize,
        /// Number of vertices supplied.
        vertex_count: usize,
    },
    /// Vertex attribute arrays have mismatched lengths or non-unit/NaN normals.
    AttributeLengthMismatch {
        /// Mesh identifier.
        mesh_name: String,
        /// Diagnostic reason.
        detail: String,
    },
    /// Authoritative Khronos glTF parser rejected the serialized JSON or binary GLB asset.
    GltfOracleRejected {
        /// Diagnostic message returned by the `gltf` specification crate.
        oracle_message: String,
    },
}

impl fmt::Display for AssetValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMeshTopology { mesh_name } => {
                write!(f, "mesh `{mesh_name}` contains zero vertices or triangles")
            }
            Self::InvalidTriangleIndices {
                mesh_name,
                index_count,
                vertex_count,
            } => write!(
                f,
                "mesh `{mesh_name}` has invalid triangle indices ({index_count} indices for {vertex_count} vertices)"
            ),
            Self::AttributeLengthMismatch { mesh_name, detail } => {
                write!(f, "mesh `{mesh_name}` attribute mismatch: {detail}")
            }
            Self::GltfOracleRejected { oracle_message } => {
                write!(f, "Khronos glTF validation failed: {oracle_message}")
            }
        }
    }
}

impl std::error::Error for AssetValidationError {}

/// Refusal returned when a W3C WGSL shader module fails `naga` parsing or capability validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShaderValidationError {
    /// WGSL source failed lexical or grammatical parsing in `naga`.
    WgslParseRejected {
        /// Shader label.
        shader_label: String,
        /// Parser diagnostic emitted by `naga`.
        diagnostic: String,
    },
    /// Parsed shader module failed `naga` type, layout, or uniformity validation.
    WgslModuleInvalid {
        /// Shader label.
        shader_label: String,
        /// Validator diagnostic emitted by `naga`.
        diagnostic: String,
    },
    /// Expected entry point stage is missing from the shader module.
    MissingEntryPoint {
        /// Shader label.
        shader_label: String,
        /// Required entry point name.
        entry_point: String,
    },
}

impl fmt::Display for ShaderValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WgslParseRejected {
                shader_label,
                diagnostic,
            } => write!(f, "WGSL parse error in `{shader_label}`: {diagnostic}"),
            Self::WgslModuleInvalid {
                shader_label,
                diagnostic,
            } => write!(f, "WGSL validation error in `{shader_label}`: {diagnostic}"),
            Self::MissingEntryPoint {
                shader_label,
                entry_point,
            } => write!(
                f,
                "WGSL module `{shader_label}` is missing required entry point `{entry_point}`"
            ),
        }
    }
}

impl std::error::Error for ShaderValidationError {}

/// Refusal returned when encoding or decoding a deterministic binary world snapshot fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotCodecError {
    /// Binary payload is shorter than the fixed snapshot header and digest trailer.
    TruncatedSnapshot {
        /// Observed byte length.
        byte_len: usize,
    },
    /// Magic header or format version does not match the engine snapshot envelope.
    InvalidHeaderMagic,
    /// Embedded integrity digest does not match the payload bytes.
    IntegrityDigestMismatch {
        /// Expected hex digest recorded in trailer.
        expected_hex: String,
        /// Recomputed hex digest over payload.
        observed_hex: String,
    },
    /// JSON world state payload inside verified envelope is malformed.
    MalformedStatePayload {
        /// Parser diagnostic.
        detail: String,
    },
}

impl fmt::Display for SnapshotCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedSnapshot { byte_len } => {
                write!(f, "world snapshot truncated at {byte_len} bytes")
            }
            Self::InvalidHeaderMagic => write!(f, "invalid world snapshot magic or version"),
            Self::IntegrityDigestMismatch {
                expected_hex,
                observed_hex,
            } => write!(
                f,
                "snapshot digest mismatch: expected {expected_hex}, observed {observed_hex}"
            ),
            Self::MalformedStatePayload { detail } => {
                write!(f, "malformed snapshot payload: {detail}")
            }
        }
    }
}

impl std::error::Error for SnapshotCodecError {}

/// Refusal returned when skeletal rig topology, skin weights, or keyframe clips violate bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimationError {
    /// Skeleton contains zero joints or a joint whose parent violates topological ordering.
    InvalidJointHierarchy {
        /// Offending joint index.
        joint_index: usize,
        /// Declared parent index.
        parent_index: usize,
    },
    /// Four-joint linear-blend skinning weights do not sum to the Prism permille scale or reference an unknown joint.
    UnnormalizedOrInvalidSkinWeights {
        /// Sum of permille weights supplied.
        observed_sum: u64,
        /// Required permille scale (`1000`).
        expected_scale: u64,
    },
    /// Keyframe clip is empty, has mismatched joint counts, or exceeds the morph target channel cap.
    InvalidClipChannel {
        /// Diagnostic description.
        detail: String,
    },
}

impl fmt::Display for AnimationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJointHierarchy {
                joint_index,
                parent_index,
            } => write!(
                f,
                "invalid skeletal joint hierarchy at joint {joint_index} (parent {parent_index})"
            ),
            Self::UnnormalizedOrInvalidSkinWeights {
                observed_sum,
                expected_scale,
            } => write!(
                f,
                "skinning weights sum to {observed_sum}, expected {expected_scale} with valid joint indices"
            ),
            Self::InvalidClipChannel { detail } => {
                write!(f, "invalid animation clip or skinning binding: {detail}")
            }
        }
    }
}

impl std::error::Error for AnimationError {}

/// Refusal returned when shadow cascades, IBL probes, or GPU render-pass barrier graphs fail validation.
#[derive(Clone, Debug, PartialEq)]
pub enum RenderGraphError {
    /// Perspective near/far shadow bounds are non-positive, non-finite, or inverted (`far <= near`).
    InvalidShadowFrustumBounds {
        /// Near clip plane in meters.
        near_meters: f64,
        /// Far clip plane in meters.
        far_meters: f64,
    },
    /// Environment cubemap resolution is zero.
    ZeroProbeResolution,
    /// GPU render-pass barrier edge violates strict forward topological pass ordering.
    InvalidPassDependency {
        /// Producer pass index.
        producer_pass: usize,
        /// Consumer pass index.
        consumer_pass: usize,
    },
}

impl fmt::Display for RenderGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidShadowFrustumBounds {
                near_meters,
                far_meters,
            } => write!(
                f,
                "shadow cascade bounds require 0 < near < far, got [{near_meters}, {far_meters}]"
            ),
            Self::ZeroProbeResolution => {
                write!(f, "IBL environment probe base resolution must be positive")
            }
            Self::InvalidPassDependency {
                producer_pass,
                consumer_pass,
            } => write!(
                f,
                "invalid render graph dependency from pass {producer_pass} to pass {consumer_pass}"
            ),
        }
    }
}

impl std::error::Error for RenderGraphError {}

/// Refusal returned when GJK convex collision hulls or contact-solver inputs violate admissibility.
#[derive(Clone, Debug, PartialEq)]
pub enum CollisionError {
    /// Convex hull has zero vertices or contains non-finite coordinates.
    EmptyOrDegenerateHull {
        /// Number of vertices supplied.
        vertex_count: usize,
    },
    /// Contact solver received non-positive or non-finite effective mass or timestep.
    InvalidContactSolverInputs {
        /// Effective mass in kilograms.
        effective_mass_kg: f64,
        /// Step duration in seconds.
        dt_seconds: f64,
    },
}

impl fmt::Display for CollisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOrDegenerateHull { vertex_count } => {
                write!(
                    f,
                    "convex hull collider requires finite non-empty vertices, got {vertex_count}"
                )
            }
            Self::InvalidContactSolverInputs {
                effective_mass_kg,
                dt_seconds,
            } => write!(
                f,
                "contact solver requires positive finite mass ({effective_mass_kg}) and dt ({dt_seconds})"
            ),
        }
    }
}

impl std::error::Error for CollisionError {}

/// Refusal returned when terrain NavMesh extraction or A-star corridor pathfinding fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NavigationError {
    /// Source mesh yielded zero walkable triangles meeting the minimum slope normal threshold.
    EmptyWalkableNavMesh,
    /// Start or goal polygon index is out of bounds for the NavMesh.
    InvalidPolygonIndex {
        /// Requested polygon index.
        polygon_index: usize,
        /// Total polygon count in the NavMesh.
        polygon_count: usize,
    },
    /// Goal polygon is disconnected from start polygon or exceeded the bounded node budget.
    UnreachableGoal {
        /// Start polygon index.
        start_polygon: usize,
        /// Goal polygon index.
        goal_polygon: usize,
    },
}

impl fmt::Display for NavigationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyWalkableNavMesh => {
                write!(f, "terrain mesh contains zero walkable navigation polygons")
            }
            Self::InvalidPolygonIndex {
                polygon_index,
                polygon_count,
            } => write!(
                f,
                "navigation polygon index {polygon_index} out of bounds for count {polygon_count}"
            ),
            Self::UnreachableGoal {
                start_polygon,
                goal_polygon,
            } => write!(
                f,
                "no walkable navigation corridor from polygon {start_polygon} to {goal_polygon}"
            ),
        }
    }
}

impl std::error::Error for NavigationError {}
