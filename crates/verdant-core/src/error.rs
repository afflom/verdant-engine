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

/// Refusal returned when progressive multi-LOD scene streaming or UORC framing fails.
#[derive(Clone, Debug, PartialEq)]
pub enum StreamingError {
    /// Requested LOD rank exceeds the maximum verified engine LOD rank.
    InvalidLodRank {
        /// Requested LOD rank.
        requested_rank: u64,
        /// Maximum admissible LOD rank (`prism_kernel::wasmMaxLodRank()`).
        max_rank: u64,
    },
    /// Attempted to regress in LOD rank or scale during progressive UORC streaming.
    NonMonotonicLodTransition {
        /// Active scene LOD rank.
        current_rank: u64,
        /// Requested target LOD rank.
        requested_rank: u64,
    },
    /// Authoritative `uorc` encoder, verifier, or decoder refused the archive or stream.
    UorcArchiveRejected {
        /// Diagnostic message emitted by `uorc`.
        detail: String,
    },
    /// Decoded progressive LOD object count or digest does not match the stream manifest.
    LodPayloadMismatch {
        /// Diagnostic detail.
        detail: String,
    },
}

impl fmt::Display for StreamingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLodRank {
                requested_rank,
                max_rank,
            } => write!(
                f,
                "streaming LOD rank must be at most {max_rank}, got {requested_rank}"
            ),
            Self::NonMonotonicLodTransition {
                current_rank,
                requested_rank,
            } => write!(
                f,
                "progressive UORC streaming is monotonic: cannot regress from LOD{current_rank} to LOD{requested_rank}"
            ),
            Self::UorcArchiveRejected { detail } => {
                write!(f, "UORC archive stream rejected: {detail}")
            }
            Self::LodPayloadMismatch { detail } => {
                write!(f, "progressive LOD payload mismatch: {detail}")
            }
        }
    }
}

impl std::error::Error for StreamingError {}

pub use verdant_tensor::TensorComputeError;

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

/// Refusal returned when a 2D or diegetic 3D UI tree violates layout, focus reachability,
/// coarse touch-target (`>= 48px`), or WCAG 2.2 AA contrast bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiComplianceError {
    /// UI tree is empty or references an invalid child/parent widget index.
    InvalidUiHierarchy {
        /// Diagnostic description.
        detail: String,
    },
    /// Interactive widget resolved below `prism_kernel::uiMinTouchTargetCssPx()` (`48x48` CSS px).
    SubMinimumTouchTarget {
        /// Offending widget label.
        widget_label: String,
        /// Observed width in CSS pixels.
        width_px: u32,
        /// Observed height in CSS pixels.
        height_px: u32,
        /// Required minimum edge in CSS pixels.
        min_edge_px: u32,
    },
    /// Foreground/background contrast fell below WCAG 2.2 AA (`4500` text or `3000` non-text milli).
    InsufficientContrastRatio {
        /// Offending widget label.
        widget_label: String,
        /// Observed contrast ratio in milli-units.
        observed_milli: u64,
        /// Required contrast ratio in milli-units.
        required_milli: u64,
    },
    /// Two sibling widgets in the same container produced overlapping bounding rectangles.
    SiblingBoxOverlap {
        /// First widget label.
        widget_a: String,
        /// Second widget label.
        widget_b: String,
    },
    /// Interactive widget cannot be reached from the root focus widget via keyboard/gamepad traversal.
    UnreachableFocusWidget {
        /// Unreachable widget label.
        widget_label: String,
    },
}

impl fmt::Display for UiComplianceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUiHierarchy { detail } => {
                write!(f, "invalid UI widget hierarchy: {detail}")
            }
            Self::SubMinimumTouchTarget {
                widget_label,
                width_px,
                height_px,
                min_edge_px,
            } => write!(
                f,
                "interactive UI widget `{widget_label}` ({width_px}x{height_px}px) is smaller than {min_edge_px}px target"
            ),
            Self::InsufficientContrastRatio {
                widget_label,
                observed_milli,
                required_milli,
            } => write!(
                f,
                "UI widget `{widget_label}` contrast {observed_milli}/1000 is below required {required_milli}/1000"
            ),
            Self::SiblingBoxOverlap { widget_a, widget_b } => write!(
                f,
                "sibling UI widgets `{widget_a}` and `{widget_b}` overlap"
            ),
            Self::UnreachableFocusWidget { widget_label } => write!(
                f,
                "interactive UI widget `{widget_label}` is unreachable in the focus graph"
            ),
        }
    }
}

impl std::error::Error for UiComplianceError {}

/// Refusal returned when a browser container organ lacks an explicit capability grant or an
/// embedded neural/SLM proposal fails Prism admission invariants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CyberneticError {
    /// Browser container API organ is ungranted or explicitly denied (`denial wins`).
    ContainerOrganDenied {
        /// Organ identifier.
        organ_name: &'static str,
    },
    /// Embedded `coded_gemm` SLM proposal fell below the confidence threshold or violated invariants.
    ProposalRejected {
        /// Prompt or intent summary.
        summary: String,
        /// Observed confidence score in milli-units.
        score_milli: u64,
    },
    /// Observer salience vector or neuromorphic state dimensions are invalid.
    InvalidObserverDimensions {
        /// Diagnostic detail.
        detail: String,
    },
}

impl fmt::Display for CyberneticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContainerOrganDenied { organ_name } => write!(
                f,
                "container organ `{organ_name}` refused by capability-grant lattice"
            ),
            Self::ProposalRejected {
                summary,
                score_milli,
            } => write!(
                f,
                "cybernetic proposal `{summary}` rejected at score {score_milli}/1000"
            ),
            Self::InvalidObserverDimensions { detail } => {
                write!(f, "invalid cybernetic observer dimensions: {detail}")
            }
        }
    }
}

impl std::error::Error for CyberneticError {}

/// Refusal returned when a `Prism id` signed journal envelope, peer observation, or `afflom/stitch`
/// 2-of-3 parity fragment recovery fails verification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StitchCustodyError {
    /// Fragment indices are out of `0..3`, identical (`duplicateIndexCannotRecover`), or have mismatched manifests/lengths.
    IncompatibleFragmentPair {
        /// Left fragment index.
        left_index: u8,
        /// Right fragment index.
        right_index: u8,
        /// Diagnostic detail.
        detail: String,
    },
    /// Fragment payload digest or reconstructed object SHA-256 root did not match commitment.
    PayloadDigestMismatch {
        /// Diagnostic detail.
        detail: String,
    },
    /// `Prism id` (`prism/identity/4`) session signature envelope is missing, expired, or forged.
    InvalidJournalSignature {
        /// Diagnostic detail.
        detail: String,
    },
    /// Peer observation is unauthenticated, non-monotonic in sequence, or expired.
    StalePeerObservation {
        /// Peer slot identifier.
        peer_id: u64,
    },
}

impl fmt::Display for StitchCustodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleFragmentPair {
                left_index,
                right_index,
                detail,
            } => write!(
                f,
                "Stitch fragment pair ({left_index}, {right_index}) incompatible: {detail}"
            ),
            Self::PayloadDigestMismatch { detail } => {
                write!(f, "Stitch SHA-256 commitment mismatch: {detail}")
            }
            Self::InvalidJournalSignature { detail } => {
                write!(f, "Prism ID journal signature refused: {detail}")
            }
            Self::StalePeerObservation { peer_id } => {
                write!(
                    f,
                    "stale or unauthenticated peer observation for peer {peer_id}"
                )
            }
        }
    }
}

impl std::error::Error for StitchCustodyError {}

/// Refusal returned when a lawful camera rig receives invalid optical frustum parameters.
#[derive(Clone, Debug, PartialEq)]
pub enum CameraRigError {
    /// Vertical field-of-view, aspect ratio, or clip plane distances are non-finite or out of bounds.
    InvalidOpticalFrustum {
        /// Vertical FOV in radians.
        fov_y_radians: f64,
        /// Viewport aspect ratio (`width / height`).
        aspect_ratio: f64,
        /// Near clip plane in meters.
        z_near_meters: f64,
        /// Far clip plane in meters.
        z_far_meters: f64,
    },
    /// Desired spring-arm orbit distance is negative, non-finite, or exceeds `max_orbit_distance_meters`.
    InvalidSpringArmDistance {
        /// Requested orbit distance in meters.
        requested_meters: f64,
        /// Maximum admissible orbit distance in meters.
        max_meters: f64,
    },
}

impl fmt::Display for CameraRigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpticalFrustum {
                fov_y_radians,
                aspect_ratio,
                z_near_meters,
                z_far_meters,
            } => write!(
                f,
                "invalid camera optical frustum: fov={fov_y_radians}, aspect={aspect_ratio}, near={z_near_meters}, far={z_far_meters}"
            ),
            Self::InvalidSpringArmDistance {
                requested_meters,
                max_meters,
            } => write!(
                f,
                "spring-arm distance must lie in [0, {max_meters}] m, got {requested_meters}"
            ),
        }
    }
}

impl std::error::Error for CameraRigError {}
