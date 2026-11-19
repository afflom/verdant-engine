// Verdant Studio — Authoritative Role Catalog, Capability Organs, & DTCG Modal Stylesheet.
// Extracted from `site/studio.mjs` so declarative role specifications, imported standard models,
// and non-self-referential external project templates remain cleanly decoupled from DOM & audio hooks.

import {
  PRISM_MODAL_BAR_DENOM,
  PRISM_MODAL_BAR_NUMERS,
  PRISM_TERRAIN_WALKABLE_POLYGONS,
} from "./prism-bridge-meta.mjs";

export const STUDIO_ROLE_KEYS = Object.freeze([
  "dashboard",
  "code",
  "design",
  "audio",
  "narrative",
  "director",
  "gameplay",
]);

export const STUDIO_CSS = `
dialog.ve-modal {
  width: min(96vw, 72rem);
  max-height: min(90dvh, 56rem);
  margin: auto;
  padding: 0;
  border: var(--border-width) solid var(--border-strong);
  border-radius: var(--radius);
  background: var(--popover);
  color: var(--popover-foreground);
  box-shadow: var(--shadow-overlay);
  overflow: hidden;
  font-family: var(--font-sans);
}
html.immersive dialog.ve-modal {
  background: color-mix(in oklab, var(--popover) 94%, transparent);
}
dialog.ve-modal::backdrop {
  background: var(--scrim);
}
dialog.ve-modal.ve-docked {
  width: min(92vw, 34rem);
  max-height: min(82dvh, 48rem);
  margin: auto var(--space-3) var(--space-3) auto;
}
dialog.ve-modal.ve-docked::backdrop {
  background: transparent;
}
.ve-m-shell {
  display: flex;
  flex-direction: column;
  max-height: min(90dvh, 56rem);
}
dialog.ve-modal.ve-docked .ve-m-shell {
  max-height: min(82dvh, 48rem);
}
.ve-inline-studio .ve-m-shell {
  max-height: none;
  border: var(--border-width) solid var(--border-strong);
  border-radius: var(--radius);
  background: var(--card);
  overflow: hidden;
}
.ve-inline-studio .ve-m-body {
  overflow-y: visible;
}
.ve-role-standards {
  padding-bottom: var(--space-2);
  border-bottom: var(--border-width) solid var(--border);
  font-family: var(--font-mono);
  font-size: var(--text-14);
  color: var(--muted-foreground);
  line-height: 1.45;
}
.ve-role-standards a {
  color: var(--foreground);
  text-decoration: underline;
  text-underline-offset: 0.2rem;
}
.ve-m-head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  border-bottom: var(--border-width) solid var(--border);
  background: var(--sidebar);
}
.ve-m-title {
  margin: 0;
  font-family: var(--font-display);
  font-size: var(--text-18);
  font-weight: 600;
  letter-spacing: var(--tracking-display);
  color: var(--foreground);
}
.ve-m-sub {
  margin: 0.125rem 0 0;
  font-family: var(--font-mono);
  font-size: var(--text-14);
  font-variant-numeric: tabular-nums;
  color: var(--muted-foreground);
}
.ve-m-observer-bar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  border-bottom: var(--border-width) solid var(--border);
  background: var(--card);
  font-size: var(--text-14);
  font-variant-numeric: tabular-nums;
}
.ve-m-observer-meta {
  font-family: var(--font-mono);
  font-size: var(--text-14);
  color: var(--muted-foreground);
  font-variant-numeric: tabular-nums;
}
.ve-m-tabs {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-2) var(--space-4);
  border-bottom: var(--border-width) solid var(--border);
  background: var(--muted);
}
.ve-m-btn,
.ve-m-tab,
.ve-m-peek,
.ve-m-select,
.ve-m-input {
  min-height: var(--touch);
  min-width: var(--touch);
  padding: 0 var(--space-3);
  border-radius: var(--radius-control);
  border: var(--border-width) solid var(--border-strong);
  background: var(--card);
  color: var(--card-foreground);
  font-family: var(--font-sans);
  font-size: var(--text-14);
  font-weight: 500;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  white-space: nowrap;
}
.ve-m-peek {
  margin-left: auto;
  font-family: var(--font-mono);
}
.ve-m-peek[aria-pressed="true"] {
  background: var(--brand);
  border-color: var(--brand);
  color: var(--brand-foreground);
}
.ve-m-btn,
.ve-m-tab,
.ve-m-peek,
.ve-m-select {
  cursor: pointer;
}
.ve-m-btn:hover,
.ve-m-tab:hover,
.ve-m-peek:hover {
  background: var(--accent);
  color: var(--accent-foreground);
}
.ve-m-tab[aria-selected="true"],
.ve-m-btn.primary {
  background: var(--primary);
  border-color: var(--primary);
  color: var(--primary-foreground);
  font-weight: 600;
}
.ve-m-btn.primary:hover {
  background: var(--primary-hover);
}
.ve-m-btn.brand {
  background: var(--brand);
  border-color: var(--brand);
  color: var(--brand-foreground);
  font-weight: 600;
}
.ve-m-body {
  padding: var(--space-4);
  overflow-y: auto;
  display: grid;
  gap: var(--space-4);
}
.ve-m-panel {
  display: grid;
  gap: var(--space-4);
}
.ve-m-panel[hidden] {
  display: none;
}
.ve-m-card {
  border: var(--border-width) solid var(--border);
  border-radius: var(--radius);
  background: var(--card);
  color: var(--card-foreground);
  padding: var(--space-4);
  display: grid;
  gap: var(--space-3);
}
.ve-m-card h3 {
  margin: 0;
  font-family: var(--font-display);
  font-size: var(--text-16);
  font-weight: 600;
}
.ve-m-card p {
  margin: 0;
  font-size: var(--text-14);
  line-height: 1.5;
  color: var(--muted-foreground);
}
.ve-m-row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: center;
}
.ve-m-grid2 {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
  gap: var(--space-3);
}
.ve-m-kpi-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(11rem, 1fr));
  gap: var(--space-3);
  padding: var(--space-1) 0;
}
.ve-m-kpi {
  padding: var(--space-2) 0 0;
  border-top: var(--border-width) solid var(--border-strong);
  background: transparent;
  display: grid;
  gap: 0.125rem;
  font-variant-numeric: tabular-nums;
}
.ve-m-kpi span {
  font-size: var(--text-14);
  color: var(--muted-foreground);
}
.ve-m-kpi strong {
  font-family: var(--font-mono);
  font-size: var(--text-16);
  color: var(--foreground);
}
.ve-m-field {
  display: grid;
  gap: var(--space-1);
}
.ve-m-field label {
  font-family: var(--font-mono);
  font-size: var(--text-14);
  color: var(--muted-foreground);
}
.ve-m-canvas {
  width: 100%;
  height: auto;
  border-radius: var(--radius-control);
  border: var(--border-width) solid var(--border-strong);
  background: var(--muted);
  display: block;
}
.ve-m-log {
  margin: 0;
  padding: var(--space-3) 0 0;
  border: 0;
  border-top: var(--border-width) solid var(--border);
  border-radius: 0;
  background: transparent;
  color: var(--foreground);
  font-family: var(--font-mono);
  font-size: var(--text-14);
  font-variant-numeric: tabular-nums;
  line-height: 1.45;
  white-space: pre-wrap;
  overflow-x: auto;
  max-height: 18rem;
}
.ve-step-list {
  display: grid;
  gap: 0;
  border-top: var(--border-width) solid var(--border);
}
.ve-step-item {
  border-bottom: var(--border-width) solid var(--border);
  background: transparent;
  padding: var(--space-3) 0;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
}
.ve-step-copy {
  flex: 1 1 18rem;
  display: grid;
  gap: 0.125rem;
}
.ve-step-title {
  font-weight: 600;
  font-size: var(--text-14);
  color: var(--foreground);
}
.ve-step-meta {
  font-family: var(--font-mono);
  font-size: var(--text-14);
  font-variant-numeric: tabular-nums;
  color: var(--muted-foreground);
}
.ve-badge {
  display: inline-flex;
  align-items: center;
  padding: 0.125rem var(--space-2);
  border-radius: var(--radius-control);
  border: var(--border-width) solid var(--success-border);
  background: var(--success-subtle);
  color: var(--success);
  font-family: var(--font-mono);
  font-size: var(--text-14);
  font-variant-numeric: tabular-nums;
}
html:not(.dark) .ve-badge {
  background: var(--card);
}
.ve-table-wrap {
  overflow-x: auto;
  border: var(--border-width) solid var(--border);
  border-radius: var(--radius-control);
}
.ve-table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--text-14);
  font-variant-numeric: tabular-nums;
}
.ve-table th,
.ve-table td {
  padding: var(--space-2) var(--space-3);
  text-align: left;
  border-bottom: var(--border-width) solid var(--border);
}
.ve-table th {
  font-family: var(--font-mono);
  color: var(--muted-foreground);
  background: var(--muted);
}
.ve-organ-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(18rem, 1fr));
  column-gap: var(--space-4);
  row-gap: 0;
  border-top: var(--border-width) solid var(--border);
}
.ve-organ-item {
  border-bottom: var(--border-width) solid var(--border);
  padding: var(--space-2) 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
  background: transparent;
}
.ve-organ-meta {
  display: grid;
  gap: 0.125rem;
}
.ve-organ-name {
  font-family: var(--font-mono);
  font-size: var(--text-14);
  font-weight: 600;
}
.ve-organ-state {
  font-size: var(--text-14);
  color: var(--muted-foreground);
}
.ve-swatch-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(9rem, 1fr));
  gap: var(--space-2);
}
.ve-swatch {
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-control);
  border: var(--border-width) solid var(--border-strong);
  font-family: var(--font-mono);
  font-size: var(--text-14);
}
`;

export const CONTAINER_ORGANS = Object.freeze([
  { idx: 0, slug: "wasm-core", label: "WebAssembly Sandbox", defaultAllowed: true },
  { idx: 1, slug: "offscreen-canvas", label: "OffscreenCanvas Viewport", defaultAllowed: true },
  { idx: 2, slug: "service-worker", label: "ServiceWorker Offline Cache", defaultAllowed: true },
  { idx: 3, slug: "audio-worklet", label: "AudioWorklet 128f Synth", defaultAllowed: false },
  { idx: 4, slug: "opfs-storage", label: "OPFS Persistent Asset Cache", defaultAllowed: false },
  { idx: 5, slug: "gamepad-hid", label: "Gamepad HID Controller", defaultAllowed: false },
  { idx: 6, slug: "device-orientation", label: "DeviceOrientation Pose", defaultAllowed: false },
  { idx: 7, slug: "webcrypto-prism-id", label: "WebCrypto Prism Identity", defaultAllowed: true },
  { idx: 8, slug: "broadcast-channel", label: "BroadcastChannel Multi-Tab", defaultAllowed: true },
  { idx: 9, slug: "webrtc-stitch-mesh", label: "WebRTC Stitch Peer Mesh", defaultAllowed: false },
  { idx: 10, slug: "webnn-local", label: "WebNN Local Tensor Probe", defaultAllowed: false },
]);

// Four non-self-referential external application & game projects authored inside Verdant Studio:
export const STARTER_ARCHETYPES = Object.freeze([
  {
    idx: 0,
    slug: "sunspire-regatta",
    title: "Sunspire Harbor Regatta",
    name: "Sunspire Harbor Regatta (3rd-Person Coastal Adventure)",
    seed: 7,
    defaultLod: 3,
    cameraMode: "third",
    desc: "22-joint Foot IK mariner + NPC dock engineer patrol, swept TwoLevelBvh harbour basin physics, near-sphere spring-arm camera, and binary glTF 2.0 export.",
  },
  {
    idx: 1,
    slug: "meridian-observatory",
    title: "Meridian Celestial Observatory",
    name: "Meridian Celestial Observatory (1st-Person Architectural Sim)",
    seed: 256,
    defaultLod: 4,
    cameraMode: "first",
    desc: "1st-person head-masked eye rig, LOD4 ultra-high-definition stone arch & dome masonry, 4-cascade CSM shadow splits, and Sabine chamber acoustics.",
  },
  {
    idx: 2,
    slug: "glasshouse-chronicle",
    title: "Glasshouse Botanical Chronicle",
    name: "Glasshouse Botanical Chronicle (Conservation Narrative RPG)",
    seed: 108,
    defaultLod: 3,
    cameraMode: "third",
    desc: "Deadlock-free tropical StoryAutomaton, exact integer seed-bank ConservationLedger, grounded NPC dialogue, and 128m floating-origin UniversePosition portals.",
  },
  {
    idx: 3,
    slug: "starfall-bastion",
    title: "Starfall Bastion Arena",
    name: "Starfall Bastion Arena (Tactical Spatial UI & Tournament)",
    seed: 42,
    defaultLod: 3,
    cameraMode: "landmark",
    desc: "Screen-space + diegetic 3D tactical UiTree solved over tropical (max, +) with 48px targets, WCAG 2.2 AA contrast, FK-07 custody, and UORC replay snapshots.",
  },
]);

export const ROLE_REQUIREMENTS_TRACE = Object.freeze([
  {
    roleKey: "dashboard",
    roleLabel: "Dashboard (Producer)",
    standard: "ISO/IEC/IEEE 42010 · ISO 25010 · W5H Telos",
    requirement: "8-stratum 16.67ms frame budget, zero unmodeled features, FK-07 denial-wins custody",
    holoQuery: "scene:telos",
  },
  {
    roleKey: "code",
    roleLabel: "Code (Developer)",
    standard: "FI-08 {0,1} Rust · Swept CCD · XPBD",
    requirement: "Zero-penetration capsule sweeps, TwoLevelBvh, XPBD joints, SHA-256-sealed .uorc project export",
    holoQuery: "scene:lawful",
  },
  {
    roleKey: "design",
    roleLabel: "Design (Artist)",
    standard: "Alina Wheeler · W3C DTCG 2025.10 · glTF 2.0 · WCAG 2.2",
    requirement: "Progressive LOD0-LOD4 .uorc, .glb I/O, tropical (max,+) 48px non-overlapping UI",
    holoQuery: "scene:ui",
  },
  {
    roleKey: "audio",
    roleLabel: "Audio (Acoustics)",
    standard: "Euler-Bernoulli · Sabine RT60 · Woodworth ITD",
    requirement: `5-mode harmonic bar (${PRISM_MODAL_BAR_NUMERS.join(":")}/${PRISM_MODAL_BAR_DENOM}), 4-ch Hadamard FDN, spherical-head binaural`,
    holoQuery: "scene:acoustics",
  },
  {
    roleKey: "narrative",
    roleLabel: "Narrative (Writer)",
    standard: "Tropical StoryAutomaton · ConservationLedger",
    requirement: "Deadlock-free quest states, conserved token supply, 128m 3-sector Universe portals",
    holoQuery: "scene:narrative",
  },
  {
    roleKey: "director",
    roleLabel: "Director (Cinematography)",
    standard: "LawfulCameraRig · 4-Cascade CSM · CDMX SLM",
    requirement: "Near-sphere wall guard (>=290mm), multi-shot keyframe choreography, local coded_gemm cues",
    holoQuery: "scene:studio",
  },
  {
    roleKey: "gameplay",
    roleLabel: "Gameplay (Modes & NPCs)",
    standard: "22-Joint Foot IK · Q15 LIF/SSM · Stitch 2-of-3",
    requirement: `Campaign/Arena/Tournament rules, ${PRISM_TERRAIN_WALKABLE_POLYGONS.toLocaleString()}-poly NavMesh NPC directives, stride-8 rollback & parity`,
    holoQuery: "scene:character",
  },
]);

export const DIRECTOR_SHOTS = Object.freeze([
  { id: "shot-0", title: "01. Establishing Harbor Crane", rig: "Orbit", target: [2.2, 3.8, 9.2], dist: 18.5, yawMilli: 460, pitchMilli: 260 },
  { id: "shot-1", title: "02. Parabolic Arch Low-Angle", rig: "Orbit", target: [7.5, 1.8, -4.0], dist: 12.5, yawMilli: 590, pitchMilli: 190 },
  { id: "shot-2", title: "03. Character Tracking Close-Up", rig: "3rd IK", possess: true, target: [0, 1.6, 11.5], dist: 5.8, yawMilli: -580, pitchMilli: 260 },
  { id: "shot-3", title: "04. Articulated Rig Overhead", rig: "Orbit", target: [-6.5, 1.8, 9.0], dist: 9.8, yawMilli: -540, pitchMilli: 420 },
  { id: "shot-4", title: "05. South Terrace Diegetic Framing", rig: "Orbit", target: [6.0, 2.0, 8.0], dist: 10.5, yawMilli: 520, pitchMilli: 290 },
  { id: "shot-5", title: "06. Story Podium Two-Shot", rig: "Orbit", target: [1.5, 2.2, -1.0], dist: 7.4, yawMilli: 340, pitchMilli: 240 },
]);

export const ACOUSTIC_ROOMS = Object.freeze([
  { slug: "stone-hall", name: "Vaulted Stone Hall", volumeM3: 1800, absorptionMilliM2: 96000 },
  { slug: "harbor-amphitheater", name: "Harbor Amphitheater", volumeM3: 4200, absorptionMilliM2: 310000 },
  { slug: "glasshouse-atrium", name: "Glasshouse Botanical Atrium", volumeM3: 950, absorptionMilliM2: 68000 },
  { slug: "open-courtyard", name: "Open Basin Courtyard", volumeM3: 6800, absorptionMilliM2: 820000 },
]);

export const GAME_MODES = Object.freeze([
  {
    slug: "campaign",
    name: "Campaign",
    subtitle: "3-Sector Co-op Chronicle & Persistent Conservation",
    roundSec: 900,
    maxPlayers: 4,
    scoring: "Exact ConservationLedger Supply (12 Relics Conserved)",
    hudTopology: "Atmospheric Quest Objective + Guarded 3rd-Person IK",
    cameraMode: "third",
    holoQuery: "scene:narrative",
  },
  {
    slug: "arena",
    name: "Arena",
    subtitle: "3-Beacon Time Trial & Patrol Mover Deflection",
    roundSec: 180,
    maxPlayers: 8,
    scoring: "Beacon Harmonization Time + Patrol Deflection Bonus",
    hudTopology: "2 Perimeter Corner Clusters (>= 88% Open Viewport)",
    cameraMode: "third",
    holoQuery: "scene:lawful",
  },
  {
    slug: "tournament",
    name: "Tournament",
    subtitle: "Deterministic Seed Bracket & Signed Replay Verification",
    roundSec: 300,
    maxPlayers: 16,
    scoring: "Zero-Sum Elo Conservation + Stride-8 Rollback Hash Parity",
    hudTopology: "Spectator Orbit + Split Match Clock + Stitch 2-of-3 Seal",
    cameraMode: "landmark",
    holoQuery: "scene:rollback",
  },
]);

export const NPC_ROSTER = [
  {
    id: "archivist",
    name: "Archivist",
    archetype: "Lore Keeper",
    nodeIdx: 5,
    coords: [1.5, 2.2, -1.0],
    stats: "22 LBS Joints · Speed 3.2 m/s · Perception 14m · Salience Slot 5",
    directives: ["Guard Story Podium", "Guide Player to Portals", "Audit Relic Ledger", "Observe Courtyard"],
    activeDirective: 0,
  },
  {
    id: "bridge-engineer",
    name: "Dock Engineer",
    archetype: "Structural Patrol",
    nodeIdx: 1,
    coords: [7.5, 1.8, -4.0],
    stats: "22 LBS Joints + Two-Bone Foot IK · Speed 4.4 m/s · Perception 18m · Salience Slot 1",
    directives: ["Patrol Stone Arch Span", "Inspect Suspension Track", "Escort Player Across Basin", "Hold North Abutment"],
    activeDirective: 0,
  },
  {
    id: "observatory-keeper",
    name: "Observatory Keeper",
    archetype: "Celestial Guide",
    nodeIdx: 4,
    coords: [6.0, 2.0, 8.0],
    stats: "22 LBS Joints + Gaze Tracking · Speed 3.8 m/s · Perception 24m · Salience Slot 4",
    directives: ["Survey South Terrace", "Calibrate Spatial UI", "Signal Tournament Round", "Track Horizon"],
    activeDirective: 0,
  },
];
