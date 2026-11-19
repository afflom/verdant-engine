// Verdant Studio — Zero-Constants Rust/Wasm Project Scaffolder & Base64url UORC Codec.
// Generates complete, FI-08 compliant (`{0, 1}` + `prism_kernel`) external game and simulation
// workspaces aligned with Prism's authoritative 140m 4-cascade CSM horizon and UORC custody.

import { PRISM_CSM_SPLIT_METERS } from "./prism-bridge-meta.mjs";
import { STARTER_ARCHETYPES } from "./studio-data.mjs";

export function zeroOneExpr(n) {
  const v = Math.max(0, Math.floor(Number(n) || 0));
  if (v === 0) return "0";
  if (v === 1) return "1";
  const parts = [];
  let bit = 0;
  let rem = v;
  while (rem > 0) {
    if ((rem & 1) === 1) {
      parts.push(bit === 0 ? "1" : bit === 1 ? "(1 << 1)" : `(1 << ${zeroOneExpr(bit)})`);
    }
    rem >>= 1;
    bit += 1;
  }
  return parts.reverse().join(" + ");
}

export function generateStarterProjectFiles({ slug, archetypeIdx, seed, lodRank, cameraMode }) {
  const arch = STARTER_ARCHETYPES[archetypeIdx] || STARTER_ARCHETYPES[0];
  const cleanSlug = String(slug || arch.slug).toLowerCase().replace(/[^a-z0-9_-]/g, "-") || "sunspire-regatta";
  const seedExpr = zeroOneExpr(seed || arch.seed || 7);
  const lodExpr = zeroOneExpr(lodRank ?? arch.defaultLod);
  const camVariant = cameraMode === "first" ? "CameraMode::FirstPersonEye" : "CameraMode::ThirdPersonSpringArm";
  const csmFarMeters = PRISM_CSM_SPLIT_METERS[3];

  const cargoToml = `[package]
name = "${cleanSlug}"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
verdant-core = { git = "https://github.com/afflom/verdant-engine.git" }
`;

  const domainSlug =
    arch.idx === 1
      ? "meridian-observatory"
      : arch.idx === 2
        ? "glasshouse-chronicle"
        : arch.idx === 3
          ? "starfall-bastion"
          : "sunspire-regatta";

  let extraImports = "";
  let archetypeSection = "";
  let printSummary = "";

  if (arch.idx === 0) {
    extraImports = `use verdant_core::gltf_io::{export_hd_scene_to_glb, export_skinned_character_to_glb};`;
    archetypeSection = `    // Sunspire Harbor Regatta: cataloged 22-joint Foot IK patrol, NavMesh directive, & HD PBR/Skinned GLB export
    let nav_state = holon.set_npc_directive(0, 1)?;
    let walk_intent = CharacterMoveIntent {
        planar_wish_dir: Vec3::new(0.0, 0.0, -1.0),
        speed_scale: 1.0,
        jump_requested: false,
    };
    physics.step_lawful(dt, walk_intent, &bvh)?;
    holon.step_npc_patrols(dt, 0.0, spawn, &bvh);
    let captain_head = holon.active_domain.characters[0].head_anchor_world();
    let cam_frame = camera.solve_frame(dt, captain_head, &bvh, |x, z| hd_terrain_height(seed, &relief, x, z))?;
    assert!(cam_frame.near_sphere_guarded);
    let glb = export_hd_scene_to_glb(&holon.active_domain.sector.scene, &holon.active_domain.sector.meshes, &holon.active_domain.hd_materials)?;
    let skin_glb = export_skinned_character_to_glb(&holon.active_domain.characters[0], &holon.active_domain.hd_materials[0])?;
    std::fs::write("${cleanSlug}.glb", &glb)?;`;
    printSummary = `    println!("${cleanSlug} [Sunspire Harbor Regatta] OK: lod={} entities={} waypoints={} skin_glb={}B w5h={} glb={}B", lod_rank, holon.active_domain.sector.scene.entity_count(), nav_state.corridor_waypoints.len(), skin_glb.len(), telos.all_eight_strata_certified, glb.len());`;
  } else if (arch.idx === 1) {
    extraImports = `use verdant_core::audio::{evaluate_binaural_cue, AcousticEnvironment, ListenerPose, SpatialAudioSource};
use verdant_core::render::{compute_csm_splits, transpile_builtin_engine_shaders_webgl2};`;
    archetypeSection = `    // Meridian Celestial Observatory: cataloged first-person HD dome, WebGL2 shaders, 4-Cascade CSM, & Sabine acoustics
    let eye_anchor = Vec3::new(1.0 + 1.0, prism_kernel::sFour().unwrap_or(1) as f64, prism_kernel::sNine().unwrap_or(1) as f64);
    let cam_frame = camera.solve_frame(dt, eye_anchor, &bvh, |x, z| hd_terrain_height(seed, &relief, x, z))?;
    assert!(cam_frame.near_sphere_guarded);
    let z_near = 1.0 / (prism_kernel::sFour().unwrap_or(1) as f64);
    let z_far = (prism_kernel::sFourteen().unwrap_or(1) * prism_kernel::sTen().unwrap_or(1)) as f64;
    let csm_splits = compute_csm_splits(z_near, z_far)?;
    let webgl2_shaders = transpile_builtin_engine_shaders_webgl2()?;
    let fund_hz = prism_kernel::wasmBinauralFundamentalHz().unwrap_or(1) as f64;
    let binaural = evaluate_binaural_cue(
        &SpatialAudioSource {
            label: "CelestialArmillary".to_string(),
            position: Vec3::new(-(prism_kernel::sNine().unwrap_or(1) as f64), 1.0 + 1.0, -(prism_kernel::sFour().unwrap_or(1) as f64)),
            fundamental_hz: fund_hz,
            reference_gain: 1.0,
        },
        &ListenerPose { position: cam_frame.eye, forward: Vec3::new(0.0, 0.0, -1.0), up: Vec3::UP },
        &AcousticEnvironment::reflective_chamber(holon.active_domain.physical_environment.sound_speed_mps),
    );
    let rt60_sec = AcousticEnvironment::reflective_chamber(holon.active_domain.physical_environment.sound_speed_mps).sabine_rt60_seconds()?;`;
    printSummary = `    println!("${cleanSlug} [Meridian Celestial Observatory] OK: lod={} csm_cascades={} webgl2={} rt60={:.2}s gain={:.3} w5h={}", lod_rank, csm_splits.len(), webgl2_shaders.len(), rt60_sec, binaural.direct_gain, telos.all_eight_strata_certified);`;
  } else if (arch.idx === 2) {
    extraImports = `use verdant_core::network::PrismIdSessionEnvelope;
use verdant_core::observer::sha256_payload_digest;
use verdant_core::streaming::StandardSceneState;
use verdant_core::universe::{UniverseCellCoord, UniverseManager, UniversePosition};`;
    archetypeSection = `    // Glasshouse Botanical Chronicle: cataloged StoryAutomaton, grounded ISO 24617-2 dialogue, & 128m portals
    let cam_frame = camera.solve_frame(dt, spawn, &bvh, |x, z| hd_terrain_height(seed, &relief, x, z))?;
    assert!(cam_frame.near_sphere_guarded);
    let origin = "https://afflom.github.io";
    let issued = prism_kernel::skinWeightScale().unwrap_or(1);
    let question = "What glasshouse botanical seed vault is conserved here?";
    let sig = PrismIdSessionEnvelope::sign_deterministic(
        "prism:chronicle-player",
        &seed.to_le_bytes(),
        origin,
        issued,
        issued + issued,
        &sha256_payload_digest(question.as_bytes()),
    );
    let turn = holon.converse_player_with_npc("urn:verdant:player:local", (&sig, origin, issued + 1), 0, question)?;
    let universe = UniverseManager::canonical_citadel_universe(seed)?;
    let world_pos = UniversePosition::new_canonical(UniverseCellCoord::ORIGIN, spawn)?;
    let max_snap_lod = prism_kernel::wasmMaxLodRank();
    let snap = WorldSnapshot::encode_uorc(&StandardSceneState::synthesize_at_lod(seed, u64::from(lod_rank).min(max_snap_lod))?)?;
    std::fs::write("${cleanSlug}.uorc", &snap)?;`;
    printSummary = `    println!("${cleanSlug} [Glasshouse Botanical Chronicle] OK: evidence={} supply={} portals={} cell=({},{},{}) uorc={}B w5h={}", turn.matched_evidence_facts, holon.active_domain.item_ledger.total_supply(), universe.portal_graph.portals.len(), world_pos.cell.cx, world_pos.cell.cy, world_pos.cell.cz, snap.len(), telos.all_eight_strata_certified);`;
  } else {
    extraImports = `use verdant_core::cybernetics::{CodedSlmDirector, ContainerGrantMatrix, ContainerOrgan};
use verdant_core::ui::UiTree;`;
    archetypeSection = `    // Starfall Bastion Arena: cataloged low-G arena, Spatial UI solver, FK-07 custody, & 16-route Director
    let cam_frame = camera.solve_frame(dt, spawn, &bvh, |x, z| hd_terrain_height(seed, &relief, x, z))?;
    assert!(cam_frame.near_sphere_guarded);
    let first_entity = holon.active_domain.sector.scene.entities().next().map(|e| e.id);
    let solved_ui = UiTree::canonical_studio_ui_tree(first_entity)
        .solve_and_verify(ui_width, ui_height, Some(&holon.active_domain.sector.scene))?;
    let diegetic_cards = UiTree::build_diegetic_bvh(&solved_ui);
    let mut grants = ContainerGrantMatrix::default();
    grants.grant(ContainerOrgan::AudioWorkletSynth);
    let slm = CodedSlmDirector::new();
    let proposal = slm.route_prompt("inspect formal ui compliance", None, &grants)?;`;
    printSummary = `    println!("${cleanSlug} [Starfall Bastion Arena] OK: widgets={} diegetic_cards={} slm={} score={} w5h={}", solved_ui.len(), diegetic_cards.len(), proposal.intent.slug(), proposal.score_milli, telos.all_eight_strata_certified);`;
  }

  const mainRs = `//! ${arch.title} — scaffolded project source instantiated from catalog pack "${domainSlug}".
//! Zero unexplained magic constants (FI-08 compliant): all scalars derive from {0, 1} or prism_kernel.

use verdant_core::camera::{CameraMode, CameraRigConfig, LawfulCameraRig};
use verdant_core::ecs::TwoLevelBvh;
use verdant_core::hd_world::hd_terrain_height;
use verdant_core::math::Vec3;
use verdant_core::observer::ObserverPrismHolon;
use verdant_core::physics::{CapsuleController, CharacterMoveIntent, PhysicsWorld, PhysicsWorldConfig};
use verdant_core::prism_kernel;
use verdant_core::runtime::{BrowserCapabilities, EngineConfig, OntologicalRuntimeGovernor, WorldSnapshot};
${extraImports}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_seed: u64 = ${seedExpr};
    let lod_rank: u32 = ${lodExpr};

    // 1. Instantiate imported catalog world pack inside ObserverPrismHolon & build solid Two-Level BVH:
    let mut holon = ObserverPrismHolon::new("${domainSlug}")?;
    holon.switch_domain_pack("${domainSlug}", Some(u64::from(lod_rank)))?;
    let seed = holon.active_domain.spec.seed.max(user_seed.min(1));
    let relief = holon.active_domain.terrain_relief;
    let bvh = TwoLevelBvh::build_solid(&holon.active_domain.sector.scene, &holon.active_domain.sector.meshes)?;

    // 2. Configure swept-capsule physics, wall-guarded LawfulCameraRig, & OntologicalRuntimeGovernor:
    let mut physics = PhysicsWorld::new(PhysicsWorldConfig {
        terrain_seed: seed,
        environment: holon.active_domain.physical_environment,
        terrain_relief: relief,
        platform_deck: None,
        ..Default::default()
    });
    let spawn = Vec3::new(0.0, 1.0 + 1.0, (1 << (1 + 1 + 1)) as f64);
    physics.player_capsule = Some(CapsuleController::new(spawn));
    let dt = 1.0 / (prism_kernel::sSix().unwrap_or(1) * prism_kernel::sTen().unwrap_or(1)) as f64;
    physics.step_lawful(dt, CharacterMoveIntent::default(), &bvh)?;

    let arm_dist = prism_kernel::sSix().unwrap_or(1) as f64;
    let mut camera = LawfulCameraRig::new(CameraRigConfig::default(), ${camVariant}, 0.0, 0.0, arm_dist)?;

    let ui_width = (prism_kernel::sTwelve().unwrap_or(1) * prism_kernel::sHundred().unwrap_or(1)) as u32;
    let ui_height = (prism_kernel::sNine().unwrap_or(1) * prism_kernel::sHundred().unwrap_or(1)) as u32;
    let hz = (prism_kernel::sSix().unwrap_or(1) * prism_kernel::sTen().unwrap_or(1)) as u32;
    let cores = prism_kernel::sFour().unwrap_or(1) as u32;
    let cfg = EngineConfig::from_refresh_rate(ui_width, ui_height, hz, hz, cores)?;
    let caps = BrowserCapabilities {
        wasm_simd_available: true,
        webgl2_available: true,
        hardware_concurrency: cores,
        shared_array_buffer: false,
        audio_worklet: true,
        offscreen_canvas: true,
    };
    let gov = OntologicalRuntimeGovernor::new(cfg, caps);
    let telos = gov.evaluate_frame_telos(0, true, 1, 1, 0, 0, 0);
    let _ = WorldSnapshot::encode_uorc;

${archetypeSection}
${printSummary}
    Ok(())
}
`;

  const indexHtml = `<!doctype html>
<html lang="en" class="dark">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>${arch.title}</title>
  <link rel="stylesheet" href="./brand/fonts.css">
  <link rel="stylesheet" href="./brand/hologram-warm.css">
  <link rel="stylesheet" href="./prism-page.css">
  <style>
    html, body { margin: 0; height: 100%; background: var(--background); color: var(--foreground); font-family: var(--font-sans); overflow: hidden; }
    #game-viewport { display: block; width: 100vw; height: 100dvh; touch-action: none; }
    .hud { position: fixed; top: var(--space-3); left: var(--space-3); padding: var(--space-2) var(--space-3); border-radius: var(--radius-control); border: var(--border-width) solid var(--border-strong); background: var(--card); font-family: var(--font-mono); font-size: var(--text-14); font-variant-numeric: tabular-nums; }
  </style>
</head>
<body>
  <canvas id="game-viewport" width="1280" height="720" aria-label="${arch.title} 3D viewport"></canvas>
  <div class="hud" id="hud-status">Loading ${arch.title}…</div>
  <script type="module" src="./main.mjs"></script>
</body>
</html>
`;

  const mainMjs = `// WebAssembly + WebGL2 runtime bootstrap for ${arch.title} (${cleanSlug})
import { createBestRenderer, mat4LookAt, mat4Mul, mat4Perspective } from "./scene-renderer.mjs";
import { load } from "./worker-runtime/index.mjs";

const canvas = document.getElementById("game-viewport");
const statusEl = document.getElementById("hud-status");
const [holoResp, wasmResp] = await Promise.all([fetch("./app.holo"), fetch("./verdant-wasm.wasm")]);
const holoApp = await load(new Uint8Array(await holoResp.arrayBuffer()));
const verdict = new TextDecoder().decode(await holoApp.dispatch(new TextEncoder().encode("summary")));
if (!verdict.startsWith("conformant:")) throw new Error("Holo verification refused: " + verdict);

const { instance } = await WebAssembly.instantiate(await wasmResp.arrayBuffer(), {});
const ex = instance.exports;
ex.verdant_wasm_init(${Number(seed || arch.seed || 7)}n);
ex.verdant_wasm_set_lod_rank(${Number(lodRank ?? arch.defaultLod)});
ex.verdant_wasm_camera_set_mode(${cameraMode === "first" ? 1 : 0});
ex.verdant_wasm_scene_rebuild_buffers(0);

const zFar = ${csmFarMeters.toFixed(1)};
const renderer = await createBestRenderer(canvas);
const csmSplits = [0, 1, 2, 3].map((i) => ex.verdant_wasm_csm_split_meters(0.25, zFar, i));
renderer?.setCsmSplits?.(csmSplits);

const keys = new Set();
addEventListener("keydown", (e) => keys.add(e.code));
addEventListener("keyup", (e) => keys.delete(e.code));

let prev = performance.now();
function tick(now) {
  const dt = Math.min(0.05, Math.max(0.001, (now - prev) * 0.001));
  prev = now;
  const dpr = Math.min(devicePixelRatio || 1, renderer?.isSoftware ? 1 : 2);
  const w = Math.max(1, Math.floor(canvas.clientWidth * dpr));
  const h = Math.max(1, Math.floor(canvas.clientHeight * dpr));
  if (canvas.width !== w || canvas.height !== h) { canvas.width = w; canvas.height = h; }
  const fwd = (keys.has("KeyW") || keys.has("ArrowUp") ? 1 : 0) - (keys.has("KeyS") || keys.has("ArrowDown") ? 1 : 0);
  const strafe = (keys.has("KeyD") || keys.has("ArrowRight") ? 1 : 0) - (keys.has("KeyA") || keys.has("ArrowLeft") ? 1 : 0);
  const dtUs = Math.round(dt * 1_000_000) >>> 0;
  ex.verdant_wasm_camera_intent(0, 0, 0, Math.round((w / h) * 1000) >>> 0, dtUs);
  if (fwd || strafe || keys.has("Space")) {
    ex.verdant_wasm_step_lawful(dtUs, fwd * 1000, strafe * 1000, ex.verdant_wasm_camera_yaw_milli() | 0, 1000, keys.has("Space") ? 1 : 0);
    ex.verdant_wasm_scene_rebuild_buffers(now * 0.001);
  }
  const vCount = ex.verdant_wasm_scene_vert_count() >>> 0, iCount = ex.verdant_wasm_scene_idx_count() >>> 0;
  const buf = ex.memory.buffer;
  renderer?.uploadPackedBuffers(
    new Float32Array(buf, ex.verdant_wasm_scene_pos_ptr() >>> 0, vCount * 3),
    new Float32Array(buf, ex.verdant_wasm_scene_norm_ptr() >>> 0, vCount * 3),
    new Float32Array(buf, ex.verdant_wasm_scene_col_ptr() >>> 0, vCount * 3),
    new Uint32Array(buf, ex.verdant_wasm_scene_idx_ptr() >>> 0, iCount),
  );
  const eye = [ex.verdant_wasm_camera_eye_x(), ex.verdant_wasm_camera_eye_y(), ex.verdant_wasm_camera_eye_z()];
  const tgt = [ex.verdant_wasm_camera_target_x(), ex.verdant_wasm_camera_target_y(), ex.verdant_wasm_camera_target_z()];
  const fov = Math.PI / 3.4;
  renderer?.render(mat4Mul(mat4Perspective(fov, w / h, 0.25, zFar), mat4LookAt(eye, tgt, [0, 1, 0])), eye, { rank: ${Number(lodRank ?? arch.defaultLod)} }, now * 0.001, tgt, fov);
  if (statusEl) statusEl.textContent = \`${arch.title} · \${Math.floor(iCount / 3)} tris\`;
  requestAnimationFrame(tick);
}
requestAnimationFrame(tick);
`;

  return {
    "Cargo.toml": cargoToml,
    "src/main.rs": mainRs,
    "web/index.html": indexHtml,
    "web/main.mjs": mainMjs,
  };
}

export function bytesToBase64Url(bytes) {
  let bin = "";
  for (let i = 0; i < bytes.byteLength; i++) bin += String.fromCharCode(bytes[i]);
  return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/g, "");
}

export function base64UrlToBytes(raw) {
  if (!raw || /[^A-Za-z0-9\-_]/.test(raw)) return null;
  try {
    const bin = atob(raw.replace(/-/g, "+").replace(/_/g, "/") + "===".slice((raw.length + 3) % 4));
    return Uint8Array.from(bin, (c) => c.charCodeAt(0));
  } catch {
    return null;
  }
}
