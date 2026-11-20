//! Second-Order Cybernetic Mutual-Attention Loop, Embedded `coded_gemm` Language/Intent Router,
//! and Capability-Gated Browser Container API Saturation (`cybernetics.rs`).
//!
//! Realizes Verdant Engine's neuromorphic architecture where the world observes both itself and
//! its player (`PlayerObserverSample <-> WorldAttentionDirective`) in exact integer arithmetic
//! (`step_neuromorphic_lif_ssm_ring`) and routes natural-language + behavioral intents through
//! compressed `uor_matmul::codec` weights (`infer_coded_slm_intent_logits`) with zero cloud APIs.

use serde::{Deserialize, Serialize};
use verdant_tensor::{
    infer_coded_slm_intent_logits, step_neuromorphic_lif_ssm_ring, CODED_TABLE_LEN,
};

use crate::error::CyberneticError;
use crate::prism_kernel;

/// Standard W3C browser container organs saturated by Verdant Engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ContainerOrgan {
    /// Core WebAssembly execution sandbox (always sandboxed).
    WebAssemblyCore,
    /// `OffscreenCanvas` WebGL2 + `16x16` software tile viewport (always sandboxed).
    OffscreenCanvasViewport,
    /// Offline-capable `ServiceWorker` deploy cache (always sandboxed).
    ServiceWorkerCache,
    /// `AudioWorklet` 128-frame spatial modal synthesizer (requires gesture grant).
    AudioWorkletSynth,
    /// Origin Private File System (`OPFS`) persistent `.uorc` cache (requires grant).
    OpfsAssetCache,
    /// W3C `Gamepad` controller input (requires gesture grant).
    GamepadController,
    /// Mobile `DeviceOrientation` gyroscope look (requires permission grant).
    DeviceOrientationSensor,
    /// Non-extractable `WebCrypto` `<prism-frame>` `Prism id` session signer.
    WebCryptoPrismId,
    /// Same-origin `BroadcastChannel` instant multi-tab rollback mesh.
    BroadcastChannelTabs,
    /// Authenticated `WebRTC` DataChannel + `PRISM_RELAY` Stitch peer mesh (explicit grant).
    WebRtcStitchMesh,
    /// Optional local `WebNN` hardware tensor accelerator (explicit grant).
    WebNnAccelerator,
}

impl ContainerOrgan {
    /// Canonical human/protocol name of this container organ.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WebAssemblyCore => "wasm-core",
            Self::OffscreenCanvasViewport => "offscreen-canvas",
            Self::ServiceWorkerCache => "service-worker",
            Self::AudioWorkletSynth => "audio-worklet",
            Self::OpfsAssetCache => "opfs-storage",
            Self::GamepadController => "gamepad-hid",
            Self::DeviceOrientationSensor => "device-orientation",
            Self::WebCryptoPrismId => "webcrypto-prism-id",
            Self::BroadcastChannelTabs => "broadcast-channel",
            Self::WebRtcStitchMesh => "webrtc-stitch-mesh",
            Self::WebNnAccelerator => "webnn-local",
        }
    }

    /// Whether this organ operates purely inside the default origin sandbox without external prompts.
    #[must_use]
    pub const fn is_always_sandboxed(self) -> bool {
        matches!(
            self,
            Self::WebAssemblyCore
                | Self::OffscreenCanvasViewport
                | Self::ServiceWorkerCache
                | Self::WebCryptoPrismId
                | Self::BroadcastChannelTabs
        )
    }
}

/// Capability-grant state modeled on `UOR-Foundation/foundry` (`FK-05..09`: denial always wins).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganGrantRecord {
    /// Whether the visitor has explicitly denied this organ (`denial wins`).
    pub explicitly_denied: bool,
    /// Whether the visitor has explicitly granted this organ.
    pub explicitly_granted: bool,
}

/// Capability-grant table tracking all browser container organs.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerGrantMatrix {
    records: std::collections::BTreeMap<ContainerOrgan, OrganGrantRecord>,
}

impl ContainerGrantMatrix {
    /// Grants `organ` unless previously explicitly denied (`revoke` clears denial only on explicit reset).
    pub fn grant(&mut self, organ: ContainerOrgan) {
        let entry = self.records.entry(organ).or_default();
        entry.explicitly_granted = true;
    }

    /// Explicitly denies `organ`; per `Foundry` `FK-07`, denial overrides any prior or concurrent grant.
    pub fn deny(&mut self, organ: ContainerOrgan) {
        let entry = self.records.entry(organ).or_default();
        entry.explicitly_denied = true;
    }

    /// Evaluates `prism_kernel::containerOrganAllowed` for `organ`.
    #[must_use]
    pub fn is_allowed(&self, organ: ContainerOrgan) -> bool {
        let rec = self.records.get(&organ).copied().unwrap_or_default();
        prism_kernel::containerOrganAllowed(
            rec.explicitly_denied,
            rec.explicitly_granted,
            organ.is_always_sandboxed(),
        )
    }

    /// Enforces that `organ` is permitted, returning `CyberneticError::ContainerOrganDenied` otherwise.
    pub fn require_organ(&self, organ: ContainerOrgan) -> Result<(), CyberneticError> {
        if self.is_allowed(organ) {
            Ok(())
        } else {
            Err(CyberneticError::ContainerOrganDenied {
                organ_name: organ.as_str(),
            })
        }
    }
}

/// Per-frame sensory-motor observation of the player (`8` channels matching `observerSalienceDim`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerObserverSample {
    /// Entity slot currently intersected by the player's central gaze/pointer BVH ray (`0..entity_slots`).
    pub gazed_entity_slot: usize,
    /// Continuous dwell duration on `gazed_entity_slot` in milliseconds.
    pub gaze_dwell_millis: u32,
    /// Camera angular pan/tilt speed in milli-radians per second.
    pub camera_angular_speed_milli: u32,
    /// Player or vehicle locomotion speed in milli-meters per second.
    pub locomotion_speed_milli: u32,
    /// Input cadence regularity / hesitation measure in milli-units (`0..=1000`).
    pub cadence_focus_milli: u32,
    /// Signed listener-to-emitter azimuth alignment in milli-units (`-1000..=1000`).
    pub audio_alignment_milli: i32,
    /// Active UI focus slot (`0` when focused on the 3D world canvas).
    pub ui_focus_slot: u32,
    /// Frame delta in milliseconds.
    pub frame_dt_millis: u32,
}

impl PlayerObserverSample {
    /// Packs the observation into the canonical `8`-channel `i32` drive vector.
    #[must_use]
    pub fn to_drive_vector(self) -> Vec<i32> {
        let dim = prism_kernel::observerSalienceDim().unwrap_or(1) as usize;
        let scale = prism_kernel::sHundred().unwrap_or(1) as i32;
        let mut v = Vec::with_capacity(dim);
        v.push(((self.gazed_entity_slot as i32) + 1).saturating_mul(scale));
        v.push(self.gaze_dwell_millis.min(u32::MAX / (1 + 1)) as i32);
        v.push(self.camera_angular_speed_milli.min(u32::MAX / (1 + 1)) as i32);
        v.push(self.locomotion_speed_milli.min(u32::MAX / (1 + 1)) as i32);
        v.push(self.cadence_focus_milli.min(u32::MAX / (1 + 1)) as i32);
        v.push(self.audio_alignment_milli);
        v.push((self.ui_focus_slot as i32).saturating_mul(scale));
        v.push((self.frame_dt_millis as i32).saturating_mul(scale));
        v.truncate(dim);
        while v.len() < dim {
            v.push(0);
        }
        v
    }
}

/// Closed-loop world actuation emitted when the neuromorphic world turns its attention to the player.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldAttentionDirective {
    /// Primary entity slot selected by mutual-attention salience.
    pub attended_entity_slot: usize,
    /// Recommended progressive `UORC` LOD rank (`0..=3`) steered by observer focus.
    pub foveated_lod_rank: u32,
    /// Target yaw angle in milli-radians for the world's Observer Core gimbal to face the player.
    pub observer_gimbal_yaw_milli: i64,
    /// Harmonic excitation boost in permille (`0..=1000`) for attended modal audio emitters.
    pub modal_harmonic_boost_permille: u32,
    /// Peripheral UI calm flag (`true` during rapid locomotion, `false` during focused inspection).
    pub calm_peripheral_ui: bool,
    /// LIF neuron spike bitmask emitted on this tick.
    pub spike_mask: u64,
    /// Per-entity salience distribution in milli-units.
    pub entity_salience_milli: Vec<i64>,
}

/// Rollback-deterministic second-order cybernetic loop (`kernel10NeuromorphicSalienceSsm`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CyberneticObserverLoop {
    state_dim: usize,
    entity_slots: usize,
    /// Recurrent LIF membrane potentials (`state_dim`) stored in exact integer milli-units.
    pub membrane_milli: Vec<i32>,
    a_recurrent_q: Vec<i32>,
    b_observer_q: Vec<i32>,
    c_salience_q: Vec<i32>,
}

impl CyberneticObserverLoop {
    /// Synthesizes a deterministic `entity_slots`-target neuromorphic observer loop from `{0, 1}`.
    pub fn new(entity_slots: usize) -> Result<Self, CyberneticError> {
        let state_dim = prism_kernel::observerSalienceDim().unwrap_or(1) as usize;
        if entity_slots == 0 || state_dim == 0 {
            return Err(CyberneticError::InvalidObserverDimensions {
                detail: "entity_slots and observerSalienceDim must be positive".to_string(),
            });
        }

        let unit_q =
            (prism_kernel::sSixteen().unwrap_or(1) * prism_kernel::sSixteen().unwrap_or(1)) as i32;
        let cross_q = (prism_kernel::sThirtyTwo().unwrap_or(1)) as i32;

        let mut a_recurrent_q = vec![0_i32; state_dim * state_dim];
        for r in 0..state_dim {
            for c in 0..state_dim {
                a_recurrent_q[r * state_dim + c] = if r == c {
                    unit_q
                } else if (r + 1) % state_dim == c {
                    cross_q
                } else if (c + 1) % state_dim == r {
                    -cross_q
                } else {
                    0
                };
            }
        }

        let mut b_observer_q = vec![0_i32; state_dim * state_dim];
        for r in 0..state_dim {
            for c in 0..state_dim {
                b_observer_q[r * state_dim + c] = if r == c {
                    unit_q * (prism_kernel::sTwo() as i32)
                } else if (r + c).is_multiple_of(prism_kernel::sTwo() as usize) {
                    cross_q
                } else {
                    0
                };
            }
        }

        let mut c_salience_q = vec![0_i32; entity_slots * state_dim];
        let s_four = prism_kernel::sFour().unwrap_or(1) as i32;
        for slot in 0..entity_slots {
            for r in 0..state_dim {
                let focus_match = r == (slot % state_dim);
                c_salience_q[slot * state_dim + r] = if focus_match {
                    unit_q * s_four
                } else {
                    cross_q
                };
            }
        }

        Ok(Self {
            state_dim,
            entity_slots,
            membrane_milli: vec![0_i32; state_dim],
            a_recurrent_q,
            b_observer_q,
            c_salience_q,
        })
    }

    /// Advances the coupled player-world observer state by one deterministic tick.
    pub fn step_observation(
        &mut self,
        sample: PlayerObserverSample,
        player_yaw_milli: i64,
    ) -> Result<WorldAttentionDirective, CyberneticError> {
        let drive = sample.to_drive_vector();
        let mut out = step_neuromorphic_lif_ssm_ring(
            &self.membrane_milli,
            &drive,
            &self.a_recurrent_q,
            &self.b_observer_q,
            &self.c_salience_q,
            self.entity_slots,
        )
        .map_err(|_e| CyberneticError::InvalidObserverDimensions {
            #[cfg(not(target_arch = "wasm32"))]
            detail: _e.to_string(),
            #[cfg(target_arch = "wasm32")]
            detail: String::new(),
        })?;

        self.membrane_milli = out
            .next_membrane_milli
            .iter()
            .map(|&v| v.clamp(0, i64::from(i32::MAX)) as i32)
            .collect();

        // Boost the directly gazed entity by dwell salience so sustained player attention locks cleanly:
        let gazed = sample.gazed_entity_slot % self.entity_slots;
        let dwell_bonus = i64::from(sample.gaze_dwell_millis);
        out.entity_salience_milli[gazed] =
            out.entity_salience_milli[gazed].saturating_add(dwell_bonus);

        let mut best_slot = 0usize;
        let mut best_val = out.entity_salience_milli[0];
        for (idx, &val) in out.entity_salience_milli.iter().enumerate().skip(1) {
            if val > best_val {
                best_val = val;
                best_slot = idx;
            }
        }

        let max_lod = prism_kernel::wasmMaxLodRank() as u32;
        let high_motion_threshold = (prism_kernel::skinWeightScale().unwrap_or(1)
            * prism_kernel::sFour().unwrap_or(1)) as u32;
        let calm_peripheral_ui = sample.locomotion_speed_milli > high_motion_threshold
            && sample.gaze_dwell_millis < (prism_kernel::sHundred().unwrap_or(1) as u32);
        let foveated_lod_rank = if sample.gaze_dwell_millis > 0 || !calm_peripheral_ui {
            max_lod
        } else {
            max_lod.saturating_sub(1)
        };
        let scale = prism_kernel::skinWeightScale().unwrap_or(1);
        let boost = (best_val.max(0) as u64).min(scale) as u32;

        Ok(WorldAttentionDirective {
            attended_entity_slot: best_slot,
            foveated_lod_rank,
            observer_gimbal_yaw_milli: -player_yaw_milli,
            modal_harmonic_boost_permille: boost,
            calm_peripheral_ui,
            spike_mask: out.spike_mask,
            entity_salience_milli: out.entity_salience_milli,
        })
    }
}

/// Verified engine and Studio intent categories emitted by the embedded `coded_gemm` SLM router (`0..16`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineIntentKind {
    /// Orient camera and neuromorphic Observer Core for mutual inspection (`0`).
    FocusObserverCore,
    /// Engage the 4-wheel raycast suspension vehicle controller (`1`).
    DriveSuspensionVehicle,
    /// Switch viewport to the pure WebAssembly `16x16` software tile PBR rasterizer (`2`).
    SwitchTileRasterizer,
    /// Switch viewport to hardware `OffscreenCanvas` WebGL2 PBR rendering (`3`).
    SwitchWebGl2Hardware,
    /// Stream progressive `.uorc` frames to lossless `LOD3` via `uorc_decompress` + `coded_gemm` (`4`).
    StreamLosslessUorc,
    /// Elevate world geometry synthesis to ultra-high-definition `LOD4` (`5`).
    StreamUltraHdLod4,
    /// Fragment active world state into 2-of-3 `afflom/stitch` parity shards and sync peers (`6`).
    ShardWorldToStitch,
    /// Solve and overlay the formal 2D/3D UI & WCAG compliance bounds (`7`).
    InspectUiCompliance,
    /// Apply a deterministic XPBD physics impulse to articulated joints and rigid bodies (`8`).
    StepPhysicsImpulse,
    /// Open the Producer / Systems Dashboard workspace (`9`).
    OpenRoleDashboard,
    /// Open the Code & Project Scaffolder workspace (`10`).
    OpenRoleCode,
    /// Open the Design, Outliner & Brand Tokens workspace (`11`).
    OpenRoleDesign,
    /// Open the Spatial & Sabine Modal Audio workspace (`12`).
    OpenRoleAudio,
    /// Open the Cinematography & Camera Choreography Director workspace (`13`).
    OpenRoleDirector,
    /// Open the Gameplay, NPC Directives & Tournament workspace (`14`).
    OpenRoleGameplay,
    /// Advance Chronicle narrative interaction or toggle possessed Explorer camera (`15`).
    NarrativeInteract,
}

impl EngineIntentKind {
    /// Canonical protocol slug returned to callers and the Studio command bar.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::FocusObserverCore => "focus-observer",
            Self::DriveSuspensionVehicle => "drive-vehicle",
            Self::SwitchTileRasterizer => "render-tile-wasm",
            Self::SwitchWebGl2Hardware => "render-webgl2",
            Self::StreamLosslessUorc => "stream-uorc-lod3",
            Self::StreamUltraHdLod4 => "stream-uorc-lod4",
            Self::ShardWorldToStitch => "stitch-shard-sync",
            Self::InspectUiCompliance => "inspect-formal-ui",
            Self::StepPhysicsImpulse => "step-xpbd-physics",
            Self::OpenRoleDashboard => "open-role-dashboard",
            Self::OpenRoleCode => "open-role-code",
            Self::OpenRoleDesign => "open-role-design",
            Self::OpenRoleAudio => "open-role-audio",
            Self::OpenRoleDirector => "open-role-director",
            Self::OpenRoleGameplay => "open-role-gameplay",
            Self::NarrativeInteract => "narrative-interact",
        }
    }

    fn from_index(idx: usize) -> Self {
        let i2 = crate::math::two_u32() as usize;
        let i3 = crate::math::three_u32() as usize;
        let i4 = crate::math::four_u32() as usize;
        let i5 = crate::math::five_u32() as usize;
        let i6 = crate::math::three_u32().wrapping_mul(crate::math::two_u32()) as usize;
        let i7 = crate::math::eight_u32().wrapping_sub(1_u32) as usize;
        let i8 = crate::math::eight_u32() as usize;
        let i9 = crate::math::eight_u32().wrapping_add(1_u32) as usize;
        let i10 = crate::math::ten_u32() as usize;
        let i11 = crate::math::ten_u32().wrapping_add(1_u32) as usize;
        let i12 = crate::math::ten_u32().wrapping_add(crate::math::two_u32()) as usize;
        let i13 = crate::math::ten_u32().wrapping_add(crate::math::three_u32()) as usize;
        let i14 = crate::math::ten_u32().wrapping_add(crate::math::four_u32()) as usize;
        if idx == 0 {
            EngineIntentKind::FocusObserverCore
        } else if idx == 1 {
            EngineIntentKind::DriveSuspensionVehicle
        } else if idx == i2 {
            EngineIntentKind::SwitchTileRasterizer
        } else if idx == i3 {
            EngineIntentKind::SwitchWebGl2Hardware
        } else if idx == i4 {
            EngineIntentKind::StreamLosslessUorc
        } else if idx == i5 {
            EngineIntentKind::StreamUltraHdLod4
        } else if idx == i6 {
            EngineIntentKind::ShardWorldToStitch
        } else if idx == i7 {
            EngineIntentKind::InspectUiCompliance
        } else if idx == i8 {
            EngineIntentKind::StepPhysicsImpulse
        } else if idx == i9 {
            EngineIntentKind::OpenRoleDashboard
        } else if idx == i10 {
            EngineIntentKind::OpenRoleCode
        } else if idx == i11 {
            EngineIntentKind::OpenRoleDesign
        } else if idx == i12 {
            EngineIntentKind::OpenRoleAudio
        } else if idx == i13 {
            EngineIntentKind::OpenRoleDirector
        } else if idx == i14 {
            EngineIntentKind::OpenRoleGameplay
        } else {
            EngineIntentKind::NarrativeInteract
        }
    }
}

/// Verified proposal produced by `CodedSlmDirector::route_prompt` (`ProposalAdmission`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineProposal {
    /// Admitted engine intent.
    pub intent: EngineIntentKind,
    /// Normalized confidence score in milli-units (`0..=1000`).
    pub score_milli: u64,
    /// Raw integer logit from `coded_gemm`.
    pub raw_logit: i64,
    /// Human-readable explanation of the routed action.
    pub summary: String,
}

/// Embedded zero-decode language and intent director backed by `uor_matmul::codec` (`kernel11`).
#[derive(Clone, Debug)]
pub struct CodedSlmDirector {
    embed_dim: usize,
    hidden_dim: usize,
    intent_count: usize,
    codebook: [i32; CODED_TABLE_LEN],
    w1_packed_nibbles: Vec<u8>,
    w2_grid_codes: Vec<u16>,
}

impl Default for CodedSlmDirector {
    fn default() -> Self {
        Self::new()
    }
}

impl CodedSlmDirector {
    /// Constructs the deterministic 16-route `Packed<Grid<16>, 2>` + `Grid<16>` intent model from `{0, 1}`.
    #[must_use]
    pub fn new() -> Self {
        let intent_count = CODED_TABLE_LEN;
        let embed_dim = CODED_TABLE_LEN * (1 + 1);
        let hidden_dim = CODED_TABLE_LEN * (1 + 1);
        let mid = (CODED_TABLE_LEN / (1 + 1)) as i32;
        let step = prism_kernel::sSixteen().unwrap_or(1) as i32;
        let codebook: [i32; CODED_TABLE_LEN] = core::array::from_fn(|i| ((i as i32) - mid) * step);

        let zero_code = mid as u8;
        let pos_strong = (CODED_TABLE_LEN - 1) as u8;
        let pos_mild = (mid as usize + (1 + 1)) as u8;
        let nibble_shift = prism_kernel::sFour().unwrap_or(1) as u8;

        // Layer 1 (`embed_dim x hidden_dim`, packed 2 nibbles per byte):
        // Routes lexical concept channel `r` (`0..16`) strongly into hidden neuron `r` and subword context channel `r + 16` mildly.
        let half_cols = hidden_dim / (1 + 1);
        let mut w1_packed_nibbles = vec![0_u8; embed_dim * half_cols];
        for r in 0..embed_dim {
            for pair in 0..half_cols {
                let c_lo = pair * (1 + 1);
                let c_hi = c_lo + 1;
                let code_lo = if r == c_lo {
                    pos_strong
                } else if (r % intent_count) == (c_lo % intent_count) {
                    pos_mild
                } else {
                    zero_code
                };
                let code_hi = if r == c_hi {
                    pos_strong
                } else if (r % intent_count) == (c_hi % intent_count) {
                    pos_mild
                } else {
                    zero_code
                };
                w1_packed_nibbles[r * half_cols + pair] = code_lo | (code_hi << nibble_shift);
            }
        }

        // Layer 2 (`hidden_dim x intent_count`, `Grid<16>` codes):
        let mut w2_grid_codes = vec![u16::from(zero_code); hidden_dim * intent_count];
        for h in 0..hidden_dim {
            for k in 0..intent_count {
                w2_grid_codes[h * intent_count + k] = if (h % intent_count) == k {
                    u16::from(pos_strong)
                } else {
                    u16::from(zero_code)
                };
            }
        }

        Self {
            embed_dim,
            hidden_dim,
            intent_count,
            codebook,
            w1_packed_nibbles,
            w2_grid_codes,
        }
    }

    fn encode_prompt_features(
        &self,
        prompt: &str,
        observer: Option<&PlayerObserverSample>,
    ) -> Vec<i32> {
        let lower = prompt.trim().to_ascii_lowercase();
        let boost =
            (prism_kernel::sSixteen().unwrap_or(1) * prism_kernel::sSixteen().unwrap_or(1)) as i32;
        let mut feats = vec![0_i32; self.embed_dim];

        let concept_keywords: [&[&str]; CODED_TABLE_LEN] = [
            &[
                "observer",
                "focus",
                "gaze",
                "attention",
                "core",
                "look",
                "gimbal",
                "holon",
                "duck",
                "portrait",
            ],
            &[
                "vehicle",
                "car",
                "drive",
                "wheel",
                "suspension",
                "chassis",
                "rover",
            ],
            &["tile", "software", "raster", "cpu", "16x16", "fallback"],
            &[
                "webgl", "webgl2", "hardware", "gpu", "shader", "fast", "pbr",
            ],
            &[
                "uorc", "stream", "lod3", "lossless", "decode", "glb", "import", "asset",
            ],
            &["lod4", "ultra", "uhd", "subdivide", "masonry", "vault"],
            &[
                "stitch",
                "shard",
                "parity",
                "peer",
                "multiplayer",
                "sync",
                "prism",
            ],
            &[
                "ui",
                "wcag",
                "contrast",
                "layout",
                "diegetic",
                "compliance",
                "touch",
            ],
            &[
                "physics", "joint", "hinge", "impulse", "pendulum", "step", "sim", "xpbd",
            ],
            &[
                "dashboard",
                "producer",
                "requirements",
                "traceability",
                "telos",
                "kpi",
            ],
            &[
                "create",
                "scaffold",
                "new project",
                "code",
                "rust",
                "crate",
                "starter",
            ],
            &["design", "outliner", "hierarchy", "brand", "tokens"],
            &[
                "audio",
                "sabine",
                "reverb",
                "binaural",
                "room",
                "modal",
                "acoustics",
                "sound",
            ],
            &[
                "director",
                "cinematography",
                "choreography",
                "shot",
                "cut",
                "orbit",
            ],
            &[
                "gameplay",
                "campaign",
                "tournament",
                "npc",
                "directive",
                "arena",
                "patrol",
            ],
            &[
                "interact",
                "chronicle",
                "archivist",
                "narrative",
                "quest",
                "keystone",
                "podium",
                "camera",
                "1st",
                "first person",
                "3rd",
                "third person",
                "explorer",
            ],
        ];

        for (slot, words) in concept_keywords.iter().enumerate() {
            for kw in *words {
                if lower.contains(kw) {
                    feats[slot] = feats[slot].saturating_add(boost);
                }
            }
        }

        // Subword / character n-gram hash fallback into upper channels (`16..32`) so novel phrasings still project deterministically:
        let offset_base = self.intent_count;
        for (idx, byte) in lower.bytes().enumerate() {
            if byte.is_ascii_alphanumeric() {
                let bucket =
                    offset_base + ((usize::from(byte).wrapping_add(idx)) % self.intent_count);
                feats[bucket] = feats[bucket]
                    .saturating_add(boost / (prism_kernel::sEight().unwrap_or(1) as i32));
            }
        }

        if let Some(obs) = observer {
            let att_bucket = offset_base + (obs.gazed_entity_slot % self.intent_count);
            let dwell_q =
                (obs.gaze_dwell_millis / (prism_kernel::sTen().unwrap_or(1) as u32)) as i32;
            feats[att_bucket] = feats[att_bucket].saturating_add(dwell_q);
        }

        feats
    }

    /// Routes a natural-language prompt and optional live `PlayerObserverSample` through
    /// `infer_coded_slm_intent_logits` (`coded_gemm`) and enforces `prism_kernel::proposalAdmitted`.
    pub fn route_prompt(
        &self,
        prompt: &str,
        observer: Option<&PlayerObserverSample>,
        grants: &ContainerGrantMatrix,
    ) -> Result<EngineProposal, CyberneticError> {
        let trimmed = prompt.trim();
        if trimmed.is_empty() {
            return Err(CyberneticError::ProposalRejected {
                summary: String::new(),
                score_milli: 0,
            });
        }

        let feats = self.encode_prompt_features(trimmed, observer);
        let logits = infer_coded_slm_intent_logits(
            &feats,
            1,
            self.embed_dim,
            self.hidden_dim,
            self.intent_count,
            &self.codebook,
            &self.w1_packed_nibbles,
            &self.w2_grid_codes,
        )
        .map_err(|_e| CyberneticError::InvalidObserverDimensions {
            #[cfg(not(target_arch = "wasm32"))]
            detail: _e.to_string(),
            #[cfg(target_arch = "wasm32")]
            detail: String::new(),
        })?;

        let mut best_idx = 0usize;
        let mut best_logit = logits[0];
        let mut pos_sum = 0_i128;
        for (idx, &val) in logits.iter().enumerate() {
            if val > best_logit {
                best_logit = val;
                best_idx = idx;
            }
            if val > 0 {
                pos_sum += i128::from(val);
            }
        }

        let scale = prism_kernel::skinWeightScale().unwrap_or(1);
        let score_milli = if best_logit <= 0 || pos_sum <= 0 {
            0
        } else {
            ((i128::from(best_logit) * i128::from(scale)) / pos_sum).min(i128::from(scale)) as u64
        };

        let intent = EngineIntentKind::from_index(best_idx);
        let required_organ = match intent {
            EngineIntentKind::SwitchWebGl2Hardware => ContainerOrgan::OffscreenCanvasViewport,
            EngineIntentKind::ShardWorldToStitch => ContainerOrgan::BroadcastChannelTabs,
            _ => ContainerOrgan::WebAssemblyCore,
        };
        let organ_ok = grants.is_allowed(required_organ);
        let min_threshold = prism_kernel::sFifty().unwrap_or(1);
        let invariants_hold = best_logit > 0;

        let admitted =
            prism_kernel::proposalAdmitted(score_milli, min_threshold, invariants_hold, organ_ok);
        if !admitted {
            if !organ_ok {
                return Err(CyberneticError::ContainerOrganDenied {
                    organ_name: required_organ.as_str(),
                });
            }
            return Err(CyberneticError::ProposalRejected {
                summary: trimmed.to_string(),
                score_milli,
            });
        }

        Ok(EngineProposal {
            intent,
            score_milli,
            raw_logit: best_logit,
            summary: format!("admitted:{} ({score_milli}/1000)", intent.slug()),
        })
    }
}
