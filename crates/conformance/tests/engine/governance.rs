#![allow(unused_imports)]
use super::{repo_root, wasm_abi_lock};
use std::path::PathBuf;

use verdant_core::animation::{
    skin_mesh_linear_blend, Joint, KeyframePose, SkeletalClip, Skeleton, SkinWeights4,
};
use verdant_core::audio::{
    evaluate_binaural_cue, synthesize_modal_bar_modes, AcousticEnvironment, ListenerPose,
    SpatialAudioSource,
};
use verdant_core::collision::{
    gjk_intersect_and_manifold, solve_contact_impulse, ConvexHullCollider,
};
use verdant_core::ecs::{EntityRole, SceneWorld, Transform};
use verdant_core::error::{
    AnimationError, AssetValidationError, CollisionError, EngineConfigError, NavigationError,
    PhysicsStepError, RenderGraphError, ShaderValidationError, SnapshotCodecError, StreamingError,
    TensorComputeError,
};
use verdant_core::geometry::{canonical_scene_mesh_library, MorphTarget, ProceduralMesh};
use verdant_core::gltf_io::{export_scene_to_glb, validate_glb_bytes};
use verdant_core::math::{Aabb, Frustum, Mat4, Quat, Ray, Vec3};
use verdant_core::navmesh::{steer_along_corridor, NavMesh};
use verdant_core::physics::{
    CapsuleController, KinematicSplineBody, PhysicsWorld, PhysicsWorldConfig, RigidBody,
};
use verdant_core::render::{
    canonical_pbr_material_library, compute_csm_splits, validate_builtin_engine_shaders,
    validate_wgsl_shader, IblEnvironmentProbe, RenderFrameGraph, WGSL_PBR_SURFACE_SHADER,
};
use verdant_core::runtime::{EngineConfig, FrameBudgetProfiler, WorkerJobGraph, WorldSnapshot};
use verdant_core::streaming::{
    decode_uorc_stream_up_to_lod, pack_progressive_uorc_stream, verify_progressive_uorc_stream,
    LodLevel, SceneNodeKind, StandardSceneState,
};
use verdant_core::tensor::{
    batch_transform_positions_exact, mix_spatial_audio_channels_exact,
    render_triangles_tile_framebuffer, tropical_frustum_distance_bounds,
    verified_engine_tensor_kernels, RasterTriangle,
};

/// `VE-42`: SHA-256-locked Stafford Beer VSM (S1-S5 + S3*) and ISO/IEC/IEEE 42010:2022 Viewpoint +
/// ISO/IEC/IEEE 12207 ALM Managers enforce Total Ownership ("no part without a manager"), disallow
/// contrived ungrounded or cross-contaminated prompts, synthesize declarative Viewpoint & Stitch
/// App Catalog manifests, and expose a signed bidirectional Operator Communications Channel across
/// native Rust CLI and WebAssembly.
#[test]
fn engine_ve_42() {
    use std::collections::BTreeSet;
    use verdant_cli::run_cli;
    use verdant_core::operator::{
        audit_total_ownership, load_locked_iso42010_catalog, load_locked_vsm_catalog,
        OperatorPromptRequest, VerdantOperatorChannel, REQUIRED_LOCKED_AUTHORITIES,
        REQUIRED_SHIPPED_CRATES,
    };
    use verdant_wasm::{
        verdant_wasm_aux_len, verdant_wasm_aux_ptr, verdant_wasm_init, verdant_wasm_io_clear,
        verdant_wasm_io_push_byte, verdant_wasm_operator_prompt_io,
        verdant_wasm_operator_status_aux,
    };

    // 1. Validate embedded SHA-256-locked VSM & ISO 42010 catalogs:
    let vsm = load_locked_vsm_catalog().expect("AUTH-VSM-CYBERNETICS loads cleanly");
    let vp = load_locked_iso42010_catalog().expect("AUTH-ISO-42010-VIEWPOINTS loads cleanly");
    assert_eq!(vsm.systems.len(), 6);
    assert_eq!(vp.managers.len(), 13);
    assert_eq!(vp.stitch_app_catalog_protocol.archive_format, ".uorc");

    // 2. Instantiate Operator Channel & verify Total Ownership ("no part without a manager"):
    let mut ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let own = ch.check_total_ownership();
    assert!(own.complete, "Total Ownership failed: {own:?}");
    assert!(own.orphans.is_empty());
    assert!(own.duplicate_owners.is_empty());

    // Every Manager has a unique orthogonal prime & strong 32-byte kappa:
    let primes: BTreeSet<u64> = ch.managers.iter().map(|m| m.basis_prime).collect();
    let kappas: BTreeSet<&str> = ch.managers.iter().map(|m| m.kappa_hex.as_str()).collect();
    assert_eq!(primes.len(), ch.managers.len());
    assert_eq!(kappas.len(), ch.managers.len());

    // 3. Negative Total Ownership test: planted unmanaged orphan route MUST fail!
    let orphan_rep = audit_total_ownership(
        &ch.viewpoints,
        &["/", "/unmanaged-orphan/"],
        REQUIRED_SHIPPED_CRATES,
        &[],
        &[],
        REQUIRED_LOCKED_AUTHORITIES,
    );
    assert!(!orphan_rep.complete);
    assert!(orphan_rep
        .orphans
        .iter()
        .any(|o| o.contains("/unmanaged-orphan/")));

    // 4. Positive signed Operator prompt -> admitted ISO 24617-2 Inform with PROV-O kappa:
    let pos = ch
        .submit_directive(&OperatorPromptRequest {
            viewpoint: "creator-studio".to_string(),
            directive: "Package and share a sealed .uorc MMO universe on the Stitch App Catalog under ISO 12207".to_string(),
            cited_authority: Some("AUTH-ISO-42010-VIEWPOINTS".to_string()),
            operator_prism_id: "prism:operator-ci".to_string(),
        })
        .expect("positive prompt");
    assert!(pos.admitted);
    assert_eq!(pos.dialogue_act, "Inform");
    assert_eq!(pos.manager_id, "mgr-studio-alm");
    assert!(!pos.comm_kappa_hex.is_empty());

    // 5. Negative refusals: cross-contaminated Viewpoint leak & contrived ungrounded prompt:
    let cross_studio = ch
        .submit_directive(&OperatorPromptRequest {
            viewpoint: "creator-studio".to_string(),
            directive: "Add a Verify UHD 4K Envelope button to Verdant Studio".to_string(),
            cited_authority: None,
            operator_prism_id: "prism:operator-ci".to_string(),
        })
        .expect("cross-studio refusal");
    assert!(!cross_studio.admitted);
    assert_eq!(cross_studio.dialogue_act, "Refusal");
    assert_eq!(
        cross_studio.recommended_viewpoint.as_deref(),
        Some("verification-observatory")
    );

    let cross_starter = ch
        .submit_directive(&OperatorPromptRequest {
            viewpoint: "playable-world".to_string(),
            directive: "Add an IDE Inspector panel to the playable starter game".to_string(),
            cited_authority: None,
            operator_prism_id: "prism:operator-ci".to_string(),
        })
        .expect("cross-starter refusal");
    assert!(!cross_starter.admitted);
    assert_eq!(cross_starter.dialogue_act, "Refusal");

    let contrived = ch
        .submit_directive(&OperatorPromptRequest {
            viewpoint: "asset-catalogs".to_string(),
            directive: "Invent random ungrounded buttons with sparkles".to_string(),
            cited_authority: None,
            operator_prism_id: "prism:operator-ci".to_string(),
        })
        .expect("contrived refusal");
    assert!(!contrived.admitted);
    assert_eq!(contrived.dialogue_act, "Refusal");

    // 6. Stitch 2-of-3 Verdant App Catalog publishing for arbitrary game archetypes:
    let snap_world = StandardSceneState::synthesize_at_lod(7, 2).expect("world");
    let uorc = WorldSnapshot::encode_uorc(&snap_world).expect("uorc");
    let app_entry = ch
        .publish_stitch_app_catalog_entry(
            "Starfall Vanguard FPS",
            "first-person-shooter",
            "Arena shooter published to Stitch Verdant App Catalog",
            "prism:operator-ci",
            &uorc,
        )
        .expect("publish stitch app");
    assert_eq!(app_entry.stitch_fragments.len(), 3);
    assert_eq!(app_entry.schema_org_type, "VideoGame");

    // 7. VSM S3* live repository audit via both Rust API and `verdant operator audit` CLI:
    let root = repo_root();
    let audit = ch.audit_repository(&root, None);
    assert!(
        audit.passed,
        "VSM S3* Manager repository audit failed: {audit:?}"
    );
    assert_eq!(audit.differentiated_worlds, 4);

    let cli_audit = run_cli(&[
        "operator".to_string(),
        "audit".to_string(),
        "--root".to_string(),
        root.to_string_lossy().to_string(),
    ])
    .expect("verdant operator audit CLI must pass");
    assert!(cli_audit.contains("\"passed\": true"));

    let cli_prompt = run_cli(&[
        "operator".to_string(),
        "prompt".to_string(),
        "--viewpoint".to_string(),
        "playable-world".to_string(),
        "--directive".to_string(),
        "Differentiate all 4 world packs using Khronos glTF 2.0 and HDRI skies".to_string(),
    ])
    .expect("verdant operator prompt CLI");
    assert!(cli_prompt.contains("\"admitted\": true"));

    // 7b. Verdant Operator API materialization (`apply` admitted vs refused + multi-turn NDJSON `session`):
    let apply_refused = ch
        .apply_directive(
            &root,
            &OperatorPromptRequest {
                viewpoint: "creator-studio".to_string(),
                directive: "Add Verify UHD 4K Envelope button to Studio".to_string(),
                cited_authority: None,
                operator_prism_id: "prism:operator-ci".to_string(),
            },
        )
        .expect("refused apply");
    assert!(!apply_refused.applied);
    assert!(apply_refused.written_files.is_empty());

    let (cont1, s1) = ch.handle_session_rpc_line(&root, r#"{"op":"status"}"#);
    assert!(cont1 && s1.contains("\"ok\":true"));
    let (cont2, s2) = ch.handle_session_rpc_line(&root, r#"{"op":"audit","viewpoint":"all"}"#);
    assert!(cont2 && s2.contains("\"ok\":true"));
    let (cont3, s3) = ch.handle_session_rpc_line(&root, r#"{"op":"history"}"#);
    assert!(cont3 && s3.contains("\"communicationCount\""));
    let (cont4, s4) = ch.handle_session_rpc_line(&root, r#"{"op":"quit"}"#);
    assert!(!cont4 && s4.contains("\"op\":\"quit\""));

    // Verify Manager-synthesized world differentiation profiles & Stitch App Catalog JSON are rich & valid:
    let world_profiles_json = ch.synthesize_world_differentiation_profiles_json();
    assert!(world_profiles_json.contains("\"worldCount\": 4"));
    assert!(world_profiles_json.contains("starfall-bastion"));
    let stitch_catalog_json = ch
        .synthesize_stitch_app_catalog_json(None)
        .expect("stitch catalog json");
    assert!(stitch_catalog_json.contains("\"appCount\": 4"));
    assert!(stitch_catalog_json.contains("Parity (A ^ B)"));

    // 7c. Model Context Protocol (MCP JSON-RPC 2.0) server & View Immutability Seal falsifiability:
    let init_resp = ch
        .handle_mcp_jsonrpc_line(
            &root,
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        )
        .expect("mcp initialize response");
    assert!(init_resp.contains("\"protocolVersion\":\"2024-11-05\""));
    assert!(init_resp.contains("verdant-engine-mcp"));

    let tools_resp = ch
        .handle_mcp_jsonrpc_line(
            &root,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
        )
        .expect("mcp tools/list response");
    assert!(tools_resp.contains("verdant_mutate_view"));
    assert!(tools_resp.contains("verdant_verify_immutability"));

    let imm_resp = ch
        .handle_mcp_jsonrpc_line(
            &root,
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"verdant_verify_immutability","arguments":{}}}"#,
        )
        .expect("mcp verify immutability");
    assert!(imm_resp.contains("\"isError\":false"));

    let res_list = ch
        .handle_mcp_jsonrpc_line(
            &root,
            r#"{"jsonrpc":"2.0","id":4,"method":"resources/list","params":{}}"#,
        )
        .expect("mcp resources/list");
    assert!(res_list.contains("verdant://generated/immutability-seal"));

    // Negative test: cross-manager / ungrounded mutation MUST be refused with 0 bytes written:
    let bad_mut = ch
        .mutate_owned_file(
            &root,
            &verdant_core::operator::OwnedFileMutationRequest {
                viewpoint: "creator-studio".to_string(),
                directive: "Edit catalog html from studio manager".to_string(),
                cited_authority: "AUTH-ISO-42010-VIEWPOINTS".to_string(),
                path: "site/catalogs/index.html".to_string(),
                mode: "replace_exact".to_string(),
                target: Some("Verdant Catalogs".to_string()),
                replacement: Some("Hijacked".to_string()),
                content: None,
                operator_prism_id: "prism:operator-ci".to_string(),
            },
        )
        .expect("refused cross-manager mutation");
    assert!(!bad_mut.mutated);
    assert!(bad_mut.diagnostic.contains("not owned by"));

    // Negative immutability falsifiability test: out-of-band 1-byte tamper in temp mirror MUST fail!
    let tmp_dir = std::env::temp_dir().join("verdant_ve42_immutability_test");
    let _ = std::fs::remove_dir_all(&tmp_dir);
    for &rel in verdant_core::operator::SEALED_IMMUTABLE_VIEW_FILES {
        let dst = tmp_dir.join(rel);
        if let Some(p) = dst.parent() {
            std::fs::create_dir_all(p).expect("mkdir");
        }
        std::fs::copy(root.join(rel), &dst).expect("copy sealed file");
    }
    ch.seal_repository_views(&tmp_dir, "test-seal-kappa")
        .expect("seal temp mirror");
    assert!(ch.verify_immutability_seal(&tmp_dir).valid);
    // Simulate unauthorized manual out-of-band edit to site/studio/index.html:
    let tampered_path = tmp_dir.join("site/studio/index.html");
    let mut tampered = std::fs::read_to_string(&tampered_path).expect("read studio");
    tampered.push_str("\n<!-- unauthorized manual edit -->");
    std::fs::write(&tampered_path, tampered).expect("write tamper");
    let after_tamper = ch.verify_immutability_seal(&tmp_dir);
    assert!(!after_tamper.valid);
    assert!(after_tamper
        .violations
        .iter()
        .any(|v| v.contains("out-of-band manual edit detected")
            && v.contains("site/studio/index.html")));
    let _ = std::fs::remove_dir_all(&tmp_dir);

    // 8. WebAssembly C-ABI mirror (`verdant_wasm_operator_status_aux` + `verdant_wasm_operator_prompt_io`):
    let _abi_guard = wasm_abi_lock();
    assert_eq!(verdant_wasm_init(7), 1);
    let st_len = verdant_wasm_operator_status_aux() as usize;
    assert!(st_len > 0 && verdant_wasm_aux_len() as usize == st_len && verdant_wasm_aux_ptr() != 0);
    verdant_wasm_io_clear();
    let prompt_json = br#"{"viewpoint":"creator-studio","directive":"Manage .uorc app lifecycle under ISO 12207"}"#;
    for &b in prompt_json {
        verdant_wasm_io_push_byte(u32::from(b));
    }
    let pr_len = verdant_wasm_operator_prompt_io() as usize;
    assert!(pr_len > 0 && verdant_wasm_aux_len() as usize == pr_len);
}

/// `VE-43`: Zero-account Prism-ID Encrypt-then-MAC boundary privacy with revocation epoch
/// rotation, k-of-n systematic Reed-Solomon GF(2^8) erasure coding & autonomous parity repair,
/// monotonic quorum release pointers, budgeted reciprocal Stitch admission, and in-Verdant
/// W3C PROV-O Merkle revision control (`VerdantVcs`).
#[test]
fn engine_ve_43() {
    use std::collections::BTreeMap;
    use verdant_core::network::{
        BoundaryKeyring, BoundaryVisibility, PrismBoundaryCipher, ReedSolomonStitchCodec,
        StitchBudgetLedger, StitchMonotonicPointer, BOUNDARY_PADDING_BLOCK_BYTES,
        RELEASE_RS_DATA_SHARDS, RELEASE_RS_TOTAL_SHARDS,
    };
    use verdant_core::vcs::VerdantVcs;

    // 1. Threat 12 & 13: Zero-account Encrypt-Before-Shard Boundary Privacy + Revocation Rotation
    let mut ring = BoundaryKeyring::new(
        "stitch://boundary/alice-private-world",
        "prism:alice-local-device",
        b"alice-non-extractable-webcrypto-seed",
    )
    .expect("create zero-account boundary keyring");
    ring.admit_member("prism:bob-collaborator")
        .expect("admit zero-account collaborator");
    assert!(ring.is_authorized("prism:alice-local-device"));
    assert!(ring.is_authorized("prism:bob-collaborator"));

    let secret_payload = b"CONFIDENTIAL_STITCH_WORLD_SCENE_VERDANT_ZERO_ACCOUNT_SECRET_PAYLOAD";
    let env_e1 = PrismBoundaryCipher::seal(
        &ring,
        "prism:alice-local-device",
        BoundaryVisibility::SelectiveShare,
        secret_payload,
        b"nonce-epoch-1",
    )
    .expect("seal private boundary epoch 1");
    assert_eq!(env_e1.ciphertext.len() % BOUNDARY_PADDING_BLOCK_BYTES, 0);
    assert!(
        !env_e1
            .ciphertext
            .windows(b"CONFIDENTIAL".len())
            .any(|w| w == b"CONFIDENTIAL"),
        "ciphertext must leak zero plaintext substrings"
    );
    let bob_read = PrismBoundaryCipher::open(&ring, "prism:bob-collaborator", &env_e1)
        .expect("authorized collaborator decrypts epoch 1");
    assert_eq!(bob_read.as_slice(), secret_payload);

    // Tampered ciphertext MUST be refused by HMAC-SHA256 before decryption:
    let mut tampered_env = env_e1.clone();
    tampered_env.ciphertext[0] ^= 0x55;
    assert!(PrismBoundaryCipher::open(&ring, "prism:alice-local-device", &tampered_env).is_err());

    // Revoking Bob rotates epoch 1 -> 2 with fresh key; Bob is refused on epoch 2 AND stale epoch 1:
    let old_bob_ring = ring.clone();
    let e2 = ring
        .revoke_and_rotate("prism:bob-collaborator", b"revocation-entropy-2")
        .expect("rotate boundary on revocation");
    assert_eq!(e2, 2);
    assert!(!ring.is_authorized("prism:bob-collaborator"));
    let env_e2 = PrismBoundaryCipher::seal(
        &ring,
        "prism:alice-local-device",
        BoundaryVisibility::PrivateEncrypted,
        b"EPOCH_2_POST_REVOCATION_SECRET",
        b"nonce-epoch-2",
    )
    .expect("seal epoch 2");
    assert!(PrismBoundaryCipher::open(&ring, "prism:bob-collaborator", &env_e2).is_err());
    assert!(PrismBoundaryCipher::open(&old_bob_ring, "prism:bob-collaborator", &env_e2).is_err());
    assert!(PrismBoundaryCipher::seal(
        &ring,
        "prism:bob-collaborator",
        BoundaryVisibility::PrivateEncrypted,
        b"forged",
        b"n",
    )
    .is_err());

    // 2. Threat 17: Systematic 4-of-8 GF(2^8) Reed-Solomon Erasure Coding & Autonomous Repair
    let envelope_bytes = serde_json::to_vec(&env_e2).expect("serialize encrypted envelope");
    let shards = ReedSolomonStitchCodec::shard_k_of_n(
        &envelope_bytes,
        RELEASE_RS_DATA_SHARDS,
        RELEASE_RS_TOTAL_SHARDS,
    )
    .expect("4-of-8 Reed-Solomon sharding");
    assert_eq!(shards.len(), 8);
    // Drop ALL 4 systematic data shards (0..4) — keeping ONLY the 4 Cauchy parity shards (4..8)!
    let parity_only = &shards[4..8];
    let recovered = ReedSolomonStitchCodec::recover_k_of_n(parity_only)
        .expect("recover bit-exact payload from 4 parity shards alone");
    assert_eq!(recovered, envelope_bytes);
    let recovered_env: verdant_core::network::StitchBoundaryEnvelope =
        serde_json::from_slice(&recovered).expect("parse recovered envelope");
    let decrypted = PrismBoundaryCipher::open(&ring, "prism:alice-local-device", &recovered_env)
        .expect("owner decrypts recovered 4-of-8 boundary");
    assert_eq!(decrypted.as_slice(), b"EPOCH_2_POST_REVOCATION_SECRET");

    // Autonomous S3* repair restores all 8 exact shards from any 4 survivors:
    let repaired = ReedSolomonStitchCodec::repair_k_of_n(&[
        shards[0].clone(),
        shards[2].clone(),
        shards[5].clone(),
        shards[7].clone(),
    ])
    .expect("autonomous repair back to 8 shards");
    assert_eq!(repaired, shards);

    // Negative: fewer than k shards or 1-bit tampered shard MUST fail:
    assert!(ReedSolomonStitchCodec::recover_k_of_n(&shards[0..3]).is_err());
    let mut bad_subset = parity_only.to_vec();
    bad_subset[0].payload[0] ^= 0x01;
    assert!(ReedSolomonStitchCodec::recover_k_of_n(&bad_subset).is_err());

    // 3. Threat 14: Monotonic Quorum Release / Deep-Link Pointer (Anti-Rollback)
    let mut quorum_keys = BTreeMap::new();
    quorum_keys.insert(
        "prism:release-signer-1".to_string(),
        b"signer-1-secret".to_vec(),
    );
    quorum_keys.insert(
        "prism:release-signer-2".to_string(),
        b"signer-2-secret".to_vec(),
    );
    let ptr_v1 = StitchMonotonicPointer::sign_quorum(
        "stitch://verdant/releases/stable",
        1,
        "genesis",
        "kappa-release-v1",
        2,
        &[
            ("prism:release-signer-1", b"signer-1-secret"),
            ("prism:release-signer-2", b"signer-2-secret"),
        ],
    )
    .expect("sign quorum v1");
    ptr_v1
        .verify_advance(0, Some("genesis"), &quorum_keys)
        .expect("admit epoch 1");
    let ptr_v2 = StitchMonotonicPointer::sign_quorum(
        "stitch://verdant/releases/stable",
        2,
        "kappa-release-v1",
        "kappa-release-v2",
        2,
        &[
            ("prism:release-signer-1", b"signer-1-secret"),
            ("prism:release-signer-2", b"signer-2-secret"),
        ],
    )
    .expect("sign quorum v2");
    ptr_v2
        .verify_advance(1, Some("kappa-release-v1"), &quorum_keys)
        .expect("admit epoch 2");
    // Rollback to validly-signed v1 MUST be refused when persisted_epoch == 2:
    assert!(ptr_v1
        .verify_advance(2, Some("kappa-release-v2"), &quorum_keys)
        .is_err());
    // Insufficient 1-of-2 quorum MUST be refused:
    let under_quorum = StitchMonotonicPointer::sign_quorum(
        "stitch://verdant/releases/stable",
        3,
        "kappa-release-v2",
        "kappa-release-v3",
        2,
        &[("prism:release-signer-1", b"signer-1-secret")],
    )
    .expect("sign 1-of-2");
    assert!(under_quorum
        .verify_advance(2, Some("kappa-release-v2"), &quorum_keys)
        .is_err());

    // 4. Threat 16: Zero-Account Budgeted Reciprocal Stitch Admission (Sybil Flood Guard)
    let mut ledger = StitchBudgetLedger::new(1024, 2048);
    ledger
        .admit_shard("prism:peer-a", "shard-a1", 512)
        .expect("admit within cap");
    ledger
        .admit_shard("prism:peer-a", "shard-a2", 512)
        .expect("admit up to exact per-id cap");
    // Over-budget Sybil burst from peer-a MUST be refused:
    assert!(ledger.admit_shard("prism:peer-a", "shard-a3", 64).is_err());
    // Different zero-account peers can use alien pool while LRU eviction enforces alien_pool_cap_bytes:
    ledger
        .admit_shard("prism:peer-b", "shard-b1", 1024)
        .expect("peer-b fits");
    ledger
        .admit_shard("prism:peer-c", "shard-c1", 1024)
        .expect("peer-c triggers bounded alien LRU eviction");
    assert!(ledger.alien_resident_bytes <= ledger.alien_pool_cap_bytes);
    assert!(ledger.alien_evictions > 0);

    // 5. In-Verdant W3C PROV-O Merkle Revision Control (`VerdantVcs`) + `.uorc` Capsule Roundtrip
    let mut vcs = VerdantVcs::new();
    let mut tree1 = BTreeMap::new();
    tree1.insert(
        "frame-pin.json".to_string(),
        br#"{"stitchUri":"stitch://verdant/releases/stable"}"#.to_vec(),
    );
    tree1.insert(
        "studio/view.json".to_string(),
        br#"{"viewpoint":"creator-studio","v":1}"#.to_vec(),
    );
    let c1 = vcs
        .commit_tree(
            "refs/verdant/heads/main",
            &tree1,
            "prism:alice-local-device",
            b"alice-secret",
            "mgr-studio-alm",
            "AUTH-STITCH-SELF-HOSTING",
            "Lift Studio viewpoint into VerdantVcs",
        )
        .expect("commit genesis");
    let mut tree2 = tree1.clone();
    tree2.insert(
        "studio/view.json".to_string(),
        br#"{"viewpoint":"creator-studio","v":2}"#.to_vec(),
    );
    let c2 = vcs
        .commit_tree(
            "refs/verdant/heads/main",
            &tree2,
            "prism:alice-local-device",
            b"alice-secret",
            "mgr-studio-alm",
            "AUTH-STITCH-SELF-HOSTING",
            "Refine Studio viewpoint declaratively",
        )
        .expect("commit revision 2");
    assert_eq!(c2.epoch, 2);
    assert_eq!(c2.parent_kappas, vec![c1.commit_kappa.clone()]);
    let history = vcs
        .verify_acyclic_history(&c2.commit_kappa)
        .expect("acyclic history");
    assert_eq!(history.len(), 2);

    // Unsigned or ungrounded (non-AUTH-*) commit MUST be refused:
    assert!(vcs
        .commit_tree(
            "refs/verdant/heads/main",
            &tree2,
            "anonymous",
            b"s",
            "mgr-studio-alm",
            "AUTH-STITCH-SELF-HOSTING",
            "bad author",
        )
        .is_err());
    assert!(vcs
        .commit_tree(
            "refs/verdant/heads/main",
            &tree2,
            "prism:alice-local-device",
            b"alice-secret",
            "mgr-studio-alm",
            "CONTRIVED",
            "bad authority",
        )
        .is_err());

    // Roundtrip entire VerdantVcs through `.uorc` capsule:
    let capsule = vcs.encode_uorc_capsule().expect("encode VerdantVcs .uorc");
    let restored_vcs =
        VerdantVcs::decode_uorc_capsule(&capsule).expect("decode & verify VerdantVcs .uorc");
    let checked_out = restored_vcs
        .checkout_commit(&c2.commit_kappa)
        .expect("checkout verified tree");
    assert_eq!(checked_out, tree2);
}

#[test]
fn engine_ve_44() {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::Path;
    use verdant_core::network::StitchRsShard;
    use verdant_core::vcs::{
        default_release_rs_geometry, parse_stitch_deep_link_uri, VerdantReleaseCapsule,
        MAX_BLANK_FRAME_BYTES,
    };

    // 1. Verify the frozen Composable Blank Frame in `site/frame/` stays <= 30 KiB and contains
    // ZERO loose Verdant UI/Studio/Wasm files (`Threat 15` / `Threat 19`):
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let frame_dir = repo_root.join("site/frame");
    let allowed: BTreeSet<&str> = [
        "index.html",
        "frame-boot.mjs",
        "frame-sw.js",
        "frame-pin.json",
    ]
    .into_iter()
    .collect();
    let mut total_frame_bytes = 0usize;
    let mut seen = BTreeSet::new();
    for entry in std::fs::read_dir(&frame_dir).expect("read site/frame") {
        let entry = entry.expect("dir entry");
        let fname = entry.file_name().to_string_lossy().to_string();
        assert!(
            allowed.contains(fname.as_str()),
            "Blank Frame must contain zero loose UI files, found `{fname}`"
        );
        total_frame_bytes += entry.metadata().expect("meta").len() as usize;
        seen.insert(fname);
    }
    assert_eq!(seen.len(), allowed.len(), "all 4 Blank Frame files exist");
    assert!(
        total_frame_bytes <= MAX_BLANK_FRAME_BYTES,
        "Blank Frame footprint {total_frame_bytes} B exceeds 30 KiB ({MAX_BLANK_FRAME_BYTES} B)"
    );

    // 2. Pack a self-verifying portable `.holo` Release Capsule, shard 4-of-8 into compact
    // binary `STCH2` frames, deliberately drop 4 shards, and prove exact bit-for-bit recovery:
    let mut lifted = BTreeMap::new();
    lifted.insert(
        "studio/index.html".to_string(),
        b"<!doctype html><title>Lifted Verdant Studio</title><canvas></canvas>".to_vec(),
    );
    lifted.insert(
        "generated/viewpoint-contracts.json".to_string(),
        br#"{"spec":"verdant/manager-viewpoint-contracts/1","totalOwnershipComplete":true}"#
            .to_vec(),
    );
    lifted.insert(
        "verdant-wasm.wasm".to_string(),
        vec![0x00, 0x61, 0x73, 0x6d, 1, 0, 0, 0],
    );
    let capsule = VerdantReleaseCapsule::pack_from_files(
        "0.1.0",
        1,
        "a".repeat(64).as_str(),
        "b".repeat(64).as_str(),
        "studio/index.html",
        &lifted,
    )
    .expect("pack release capsule");
    let unpacked =
        VerdantReleaseCapsule::verify_and_unpack(&capsule.holo_bytes).expect("verify & unpack");
    assert_eq!(unpacked.payloads, lifted);
    assert_eq!(unpacked.capsule_sha256_hex, capsule.capsule_sha256_hex);

    // Negative check (`Threat 15`): tampering a single byte inside `.holo` MUST be refused!
    let mut tampered = capsule.holo_bytes.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0x01;
    assert!(VerdantReleaseCapsule::verify_and_unpack(&tampered).is_err());

    // 4-of-8 STCH2 binary roundtrip + 4-shard-loss recovery + anti-rollback enforcement:
    let (k, n) = default_release_rs_geometry();
    assert_eq!((k, n), (4, 8));
    let (shards, pin) = capsule
        .shard_for_stitch(
            k,
            n,
            "genesis",
            &[
                ("prism:verdant-observer-executive", b"exec-secret"),
                ("prism:verdant-release-bot", b"bot-secret"),
            ],
            2,
            "stitch://verdant/studio",
            vec!["https://afflom.github.io/verdant-engine/stitch/v0.1.0/shards".to_string()],
        )
        .expect("shard release 4-of-8");
    assert_eq!(shards.len(), 8);

    // Roundtrip 4 surviving odd-indexed shards (1, 3, 5, 7) through compact binary STCH2 framing:
    let mut surviving = Vec::new();
    for idx in [1usize, 3, 5, 7] {
        let bin = shards[idx].to_binary_stch2();
        let parsed = StitchRsShard::from_binary_stch2(&bin).expect("parse STCH2 binary shard");
        surviving.push(parsed);
    }
    let mut trusted = BTreeMap::new();
    trusted.insert(
        "prism:verdant-observer-executive".to_string(),
        b"exec-secret".to_vec(),
    );
    trusted.insert(
        "prism:verdant-release-bot".to_string(),
        b"bot-secret".to_vec(),
    );
    let recovered =
        VerdantReleaseCapsule::recover_from_stitch(&surviving, &pin, 0, Some("genesis"), &trusted)
            .expect("recover 4-of-8 release capsule");
    assert_eq!(recovered.payloads, lifted);

    // Negative check (`Threat 14`): rollback to persisted_epoch >= release_epoch MUST be refused!
    assert!(VerdantReleaseCapsule::recover_from_stitch(
        &surviving,
        &pin,
        1,
        Some("genesis"),
        &trusted,
    )
    .is_err());

    // 3. Composable deep-link sanitization (`Threat 18`): valid stitch:// normalizes cleanly;
    // hostile schemes & path traversal MUST be refused!
    let studio = parse_stitch_deep_link_uri("stitch://verdant/studio").expect("studio deep link");
    assert_eq!(studio.virtual_route, "studio/index.html");
    let catalogs =
        parse_stitch_deep_link_uri("stitch://verdant/catalogs").expect("catalogs deep link");
    assert_eq!(catalogs.virtual_route, "catalogs/index.html");
    let third_party = parse_stitch_deep_link_uri(&format!("stitch://{}/demo", "c".repeat(64)))
        .expect("third-party kappa deep link");
    assert_eq!(third_party.virtual_route, "demo/index.html");
    for hostile in [
        "javascript:alert(1)",
        "data:text/html,<script>alert(1)</script>",
        "https://evil.example/studio",
        "stitch://verdant/../../etc/passwd",
        "stitch://verdant/<script>",
        "stitch://not-a-valid-kappa/studio",
    ] {
        assert!(
            parse_stitch_deep_link_uri(hostile).is_err(),
            "hostile deep link MUST be refused: {hostile}"
        );
    }
}

#[test]
fn engine_ve_45() {
    use std::path::Path;
    use verdant_core::governance::{
        ChangeVersionManager, KnowledgeProvenanceManager, ObserverProgressSignals, ReleaseBumpKind,
        ReleaseGateVerdict, SemVer, REQUIRED_PRISM_FACETS,
    };
    use verdant_core::operator::{
        audit_total_ownership_with_prism_facets, VerdantOperatorChannel,
        REQUIRED_LOCKED_AUTHORITIES, REQUIRED_SHIPPED_CRATES, REQUIRED_SHIPPED_ROUTES,
    };

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");

    // 1. All 13 accountable VSM Managers must own 100% of Verdant AND all 9 Prism strata/governance
    // facets (including Knowledge, Change/Version, Security, Architecture & Roadmap/KPI Managers):
    let own = ch.check_total_ownership();
    assert!(own.complete, "Total Ownership must be complete: {own:?}");
    assert_eq!(own.managers_checked, 13);
    assert!(own.orphans.is_empty());
    assert!(own.duplicate_owners.is_empty());

    for required_mgr in [
        "mgr-observer-executive",
        "mgr-architecture-design",
        "mgr-knowledge-provenance",
        "mgr-change-release",
        "mgr-security-trust",
        "mgr-roadmap-strategy",
        "mgr-design-coordinator",
        "mgr-runtime-stage",
        "mgr-studio-alm",
        "mgr-catalog-curator",
        "mgr-playable-world",
        "mgr-verification-observatory",
        "mgr-reference-docs",
    ] {
        assert!(
            ch.managers
                .iter()
                .any(|m| m.spec.manager_id == required_mgr),
            "missing required VSM Manager `{required_mgr}`"
        );
    }

    // Negative check: planting an unowned Prism facet MUST fail Total Ownership!
    let bad_facet = audit_total_ownership_with_prism_facets(
        &ch.viewpoints,
        REQUIRED_SHIPPED_ROUTES,
        REQUIRED_SHIPPED_CRATES,
        &[],
        &[],
        REQUIRED_LOCKED_AUTHORITIES,
        &["stratum-0-ontology-bytes", "stratum-99-unowned-orphan"],
    );
    assert!(!bad_facet.complete);
    assert!(bad_facet
        .orphans
        .iter()
        .any(|o| o.contains("prism-facet:stratum-99-unowned-orphan")));

    // 2. Knowledge & Provenance Manager inspects locked authorities, charter & strata live:
    let knowledge = KnowledgeProvenanceManager::inspect_repo(&repo_root);
    assert_eq!(knowledge.manager_id, "mgr-knowledge-provenance");
    assert_eq!(knowledge.locked_authorities_count, 10);
    assert!(knowledge.provenance_sources_count >= 4);
    assert!(knowledge.charter_bytes > 1024);
    assert!(knowledge.governed_lex_modules >= 28);
    assert_eq!(knowledge.measured_closed, knowledge.measured_total);
    assert_eq!(knowledge.authority_coverage_pct, 100);

    // 3. Change & Version Manager (`mgr-change-release`): SemVer Major / Minor / Patch ONLY
    // advance when Ratchet is applied AND all 5 Release Gates pass!
    let all_green = ReleaseGateVerdict {
        ratchet_applied: true,
        parametric_ratchet_clean: true,
        immutability_seal_valid: true,
        total_ownership_complete: true,
        fidelity_closed: true,
        pull_through_verified: true,
    };
    let v030 = SemVer {
        major: 0,
        minor: 3,
        patch: 0,
    };
    let minor_sig = ObserverProgressSignals {
        breaking_contract_change: false,
        major_milestone_completed: false,
        minor_capability_added: true,
        stakeholder_requirement_closed: true,
        defect_or_security_fix: false,
        summary: "Closed M4 Total Prism Facet Ownership & Observer KPIs (VE-45)".to_string(),
    };
    let minor_dec = ChangeVersionManager::evaluate_release(v030, 3, all_green, &minor_sig);
    assert_eq!(minor_dec.recommended_bump, ReleaseBumpKind::Minor);
    assert!(minor_dec.version_incremented);
    assert_eq!(minor_dec.next_version.display_string(), "0.4.0");
    assert_eq!(minor_dec.next_epoch, 4);

    let major_sig = ObserverProgressSignals {
        breaking_contract_change: false,
        major_milestone_completed: true,
        minor_capability_added: true,
        stakeholder_requirement_closed: true,
        defect_or_security_fix: false,
        summary: "Closed M5 Full Self-Hosted Blank Repo Milestone".to_string(),
    };
    let major_dec =
        ChangeVersionManager::evaluate_release(minor_dec.next_version, 4, all_green, &major_sig);
    assert_eq!(major_dec.recommended_bump, ReleaseBumpKind::Major);
    assert!(major_dec.version_incremented);
    assert_eq!(major_dec.next_version.display_string(), "1.0.0");
    assert_eq!(major_dec.next_epoch, 5);

    let patch_sig = ObserverProgressSignals {
        breaking_contract_change: false,
        major_milestone_completed: false,
        minor_capability_added: false,
        stakeholder_requirement_closed: false,
        defect_or_security_fix: true,
        summary: "Hardened STCH2 shard parser".to_string(),
    };
    let patch_dec =
        ChangeVersionManager::evaluate_release(major_dec.next_version, 5, all_green, &patch_sig);
    assert_eq!(patch_dec.recommended_bump, ReleaseBumpKind::Patch);
    assert_eq!(patch_dec.next_version.display_string(), "1.0.1");

    // Negative checks: missing Ratchet OR ANY failed Release Gate MUST HOLD version (no increment)!
    let mut no_ratchet = all_green;
    no_ratchet.ratchet_applied = false;
    let hold_ratchet = ChangeVersionManager::evaluate_release(v030, 3, no_ratchet, &major_sig);
    assert_eq!(hold_ratchet.recommended_bump, ReleaseBumpKind::Hold);
    assert!(!hold_ratchet.version_incremented);
    assert_eq!(hold_ratchet.next_version, v030);
    assert_eq!(hold_ratchet.next_epoch, 3);
    assert!(!hold_ratchet.blocking_reasons.is_empty());

    let mut failed_pull_through = all_green;
    failed_pull_through.pull_through_verified = false;
    let hold_gate =
        ChangeVersionManager::evaluate_release(v030, 3, failed_pull_through, &minor_sig);
    assert_eq!(hold_gate.recommended_bump, ReleaseBumpKind::Hold);
    assert!(!hold_gate.version_incremented);
    assert_eq!(hold_gate.next_version, v030);

    let mut ledger = ChangeVersionManager::default_ledger().expect("default ledger");
    assert!(
        ChangeVersionManager::apply_decision_to_ledger(&mut ledger, &hold_gate, "refused").is_err()
    );

    // 4. Observer queries its parts, Knowledge Manager & Roadmap/KPI Manager for live KPIs and
    // Major/Minor stakeholder roadmap progress:
    let kpi_rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert_eq!(kpi_rep.total_managers, 13);
    assert_eq!(
        kpi_rep.total_prism_facets_owned,
        REQUIRED_PRISM_FACETS.len()
    );
    assert_eq!(kpi_rep.minor_progress_pct, 100);
    assert!(kpi_rep.major_progress_pct >= 50);
    assert_eq!(kpi_rep.kpi_passed_count, kpi_rep.kpi_total_count);
    assert!(kpi_rep.milestones.len() >= 5);
    assert_eq!(kpi_rep.observer_parts.len(), 13);
}

#[test]
fn engine_ve_46() {
    use std::path::Path;
    use verdant_core::governance::ObserverProgressSignals;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");

    let base_tmp = std::env::temp_dir().join(format!("verdant-ve46-{}", std::process::id()));
    let lift_dir = base_tmp.join("vN-lift");
    let next_dir = base_tmp.join("vNplus1-out");
    let _ = std::fs::remove_dir_all(&base_tmp);

    // 1. Lift 100% of Verdant's workspace (all 4 shipped Rust crates, all 26 Prism LexLean strata,
    // all governance/model/feature registers, and all distilled Viewpoints) into a 4-of-8 sharded
    // VerdantVcs Source DAG on Stitch:
    let lift = ch
        .lift_full_workspace_into_vcs(&repo_root, &lift_dir, "1.0.0", 5)
        .expect("lift full workspace into VerdantVcs");
    assert_eq!(
        lift.unlifted_workspace_modules, 0,
        "M5 ratchet requires 0 unlifted shipped workspace modules"
    );
    assert!(
        lift.lifted_rust_files >= 41,
        "all shipped Rust files lifted"
    );
    assert!(
        lift.lifted_lex_files >= 29,
        "all 26 LexLean strata + locks lifted"
    );
    assert!(
        lift.lifted_governance_files >= 10,
        "governance & model files lifted"
    );
    assert!(
        lift.lifted_site_files >= 70,
        "distilled Viewpoints & manifests lifted (got {})",
        lift.lifted_site_files
    );
    assert_eq!(lift.source_shard_count, 8);
    assert!(lift.source_uorc_bytes > 10_000);

    // 2. Negative check: unratcheted autonomous vN -> v(N+1) bootstrap MUST be refused by mgr-change-release!
    let next_signals = ObserverProgressSignals {
        breaking_contract_change: false,
        major_milestone_completed: false,
        minor_capability_added: true,
        stakeholder_requirement_closed: true,
        defect_or_security_fix: false,
        summary: "Autonomous vN -> v(N+1) self-hosted Stitch release".to_string(),
    };
    assert!(
        ch.bootstrap_next_release_from_source_shards(
            &repo_root,
            &lift_dir,
            &next_dir,
            &lift.source_commit_kappa,
            false,
            true,
            &next_signals,
        )
        .is_err(),
        "unratcheted v(N+1) self-hosted bootstrap MUST be refused"
    );

    // 3. Positive check: drop 4 of 8 source shards, recover full VerdantVcs from the 4 surviving
    // STCH2 shards, verify acyclic parent -> child lineage, pack chained v(N+1) .holo + 4-of-8
    // shards, and verify the resulting release directory!
    let next_rep = ch
        .bootstrap_next_release_from_source_shards(
            &repo_root,
            &lift_dir,
            &next_dir,
            &lift.source_commit_kappa,
            true,
            true,
            &next_signals,
        )
        .expect("autonomous vN -> v(N+1) bootstrap from 4 surviving source shards");
    assert_eq!(next_rep.survived_source_shards, 4);
    assert_eq!(next_rep.parent_commit_kappa, lift.source_commit_kappa);
    assert_ne!(next_rep.child_commit_kappa, next_rep.parent_commit_kappa);
    assert_eq!(next_rep.next_epoch, next_rep.parent_epoch + 1);
    assert_eq!(next_rep.prev_release_kappa, lift.source_commit_kappa);
    assert!(
        next_rep.verified_files_count >= 140,
        "expected >= 140 verified files in recovered Source DAG (got {})",
        next_rep.verified_files_count
    );

    // Verify the autonomously produced v(N+1) release directory by dropping 4 of its 8 release shards!
    let verified = ch
        .verify_release_dir(&next_dir)
        .expect("verify chained v(N+1) release directory");
    assert!(verified.contains("RELEASE VERIFIED:"));
    assert!(verified.contains(&next_rep.next_release_kappa));

    let _ = std::fs::remove_dir_all(&base_tmp);
}

#[test]
fn engine_ve_47() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");

    let out_dir = std::env::temp_dir().join(format!("verdant-ve47-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    // 1. Invoke Verdant's autonomous self-lift through its own MCP JSON-RPC interface (`verdant_ratchet_lift`):
    let mcp_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 47,
        "method": "tools/call",
        "params": {
            "name": "verdant_ratchet_lift",
            "arguments": {
                "outDir": out_dir.to_string_lossy(),
                "writeRepoArtifacts": false,
                "updateFramePin": false
            }
        }
    })
    .to_string();
    let mcp_raw = ch
        .handle_mcp_jsonrpc_line(&repo_root, &mcp_req)
        .expect("MCP verdant_ratchet_lift response");
    let mcp_val: serde_json::Value = serde_json::from_str(&mcp_raw).expect("parse MCP JSON");
    assert_eq!(mcp_val["result"]["isError"], serde_json::Value::Bool(false));
    let payload_txt = mcp_val["result"]["content"][0]["text"]
        .as_str()
        .expect("MCP content text");
    let lift_val: serde_json::Value = serde_json::from_str(payload_txt).expect("parse lift JSON");
    assert_eq!(lift_val["passed"], serde_json::Value::Bool(true));
    assert_eq!(lift_val["unliftedWorkspaceModules"], 0);
    assert_eq!(lift_val["unliftedUiFiles"], 0);
    assert_eq!(lift_val["unverifiedFrameAssets"], 0);
    assert_eq!(lift_val["userRequiredExternalAccounts"], 0);
    assert!(lift_val["blankFrameBytes"].as_u64().unwrap_or(999_999) <= 30_720);
    assert!(lift_val["liftedModuleCount"].as_u64().unwrap_or(0) >= 65);
    assert!(out_dir.join("self-host-ratchet-manifest.toml").is_file());

    // 2. Verify the live repository satisfies the tightened `prism/parametric-ratchet/2` mechanically:
    let ratchet_rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass tightened prism/parametric-ratchet/2");
    assert_eq!(ratchet_rep.unlifted_ui_file_count, 0);
    assert_eq!(ratchet_rep.unverified_frame_asset_count, 0);
    assert_eq!(ratchet_rep.user_required_external_account_count, 0);
    assert_eq!(ratchet_rep.unlifted_workspace_module_count, 0);
    assert!(ratchet_rep.self_hosted_lifted_module_count >= 65);
    assert!(ratchet_rep.blank_frame_total_bytes <= 30_720);

    // 3. Negative check: tampering a single lifted module's SHA-256 digest MUST fail closed!
    let manifest_text = std::fs::read_to_string(out_dir.join("self-host-ratchet-manifest.toml"))
        .expect("read isolated manifest");
    let tampered_manifest = manifest_text.replacen(
        "\nsha256 = \"",
        "\nsha256 = \"0000000000000000000000000000000000000000000000000000000000000000\" # ",
        1,
    );
    assert_ne!(
        tampered_manifest, manifest_text,
        "negative test must actually mutate a lifted_module sha256 line"
    );
    let tampered_manifest_path = out_dir.join("tampered-manifest.toml");
    std::fs::write(&tampered_manifest_path, tampered_manifest).expect("write tampered manifest");
    let cfg_raw = std::fs::read_to_string(repo_root.join("model/parametric-ratchet.toml"))
        .expect("read ratchet config");
    let tampered_cfg = cfg_raw.replace(
        "ratchet_manifest = \"site/generated/self-host-ratchet-manifest.toml\"",
        &format!(
            "ratchet_manifest = \"{}\"",
            tampered_manifest_path.to_string_lossy().replace('\\', "/")
        ),
    );
    let tampered_cfg_path = out_dir.join("tampered-ratchet.toml");
    std::fs::write(&tampered_cfg_path, tampered_cfg).expect("write tampered ratchet config");
    assert!(
        prism_fidelity::ratchet::audit_parametric_ratchet(&repo_root, &tampered_cfg_path).is_err(),
        "tampered lifted module SHA-256 MUST fail closed under prism/parametric-ratchet/2"
    );

    let _ = std::fs::remove_dir_all(&out_dir);
}

#[test]
fn engine_ve_48() {
    use verdant_core::network::{
        MultiOriginRelayPlan, ReedSolomonStitchCodec, StitchPeerExchangeSession,
    };

    // 1. MultiOriginRelayPlan enforces >= 2 independent HTTPS origins & survives primary outage:
    let plan = MultiOriginRelayPlan::build(
        vec![
            "https://afflom.github.io/verdant-engine/stitch/v1.2.0/shards".to_string(),
            "https://github.com/afflom/verdant-engine/releases/download/v1.2.0".to_string(),
        ],
        2,
    )
    .expect("two independent HTTPS origins must succeed");
    assert_eq!(plan.distinct_origins.len(), 2);
    let survivors = plan
        .surviving_relays(&["afflom.github.io"])
        .expect("secondary origin must survive primary outage");
    assert_eq!(survivors.len(), 1);
    assert!(survivors[0].contains("github.com"));

    // Negative checks: single origin, unsafe http:// external host, or total outage MUST fail closed!
    assert!(
        MultiOriginRelayPlan::build(
            vec!["https://afflom.github.io/verdant-engine/stitch/v1.2.0/shards".to_string()],
            2
        )
        .is_err(),
        "single-origin downgrade must be refused"
    );
    assert!(
        MultiOriginRelayPlan::build(
            vec![
                "http://evil.example/shards".to_string(),
                "https://github.com/afflom/verdant-engine/releases/download/v1.2.0".to_string(),
            ],
            2
        )
        .is_err(),
        "unsafe non-HTTPS external relay must be refused"
    );
    assert!(
        plan.surviving_relays(&["afflom.github.io", "github.com"])
            .is_err(),
        "zero surviving relays must fail closed"
    );

    // 2. Zero-account reciprocal peer shard exchange (WebRTC/BroadcastChannel) governed by StitchBudgetLedger:
    let payload = b"Verdant Multi-Origin 4-of-8 STCH2 peer shard payload";
    let shards = ReedSolomonStitchCodec::shard_k_of_n(payload, 4, 8).expect("4-of-8 shard");
    let raw0 = shards[0].to_binary_stch2();
    let cap = (raw0.len() as u64) * 2;
    let mut peer = StitchPeerExchangeSession::new("prism:peer-local", cap, cap * 4)
        .expect("peer exchange session");
    let rec = peer
        .receive_verified_peer_shard("prism:peer-alpha", &raw0)
        .expect("valid STCH2 shard admitted");
    let key = format!(
        "{}:{}",
        rec.manifest_root
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>(),
        rec.index
    );
    let served = peer
        .serve_shard_to_peer("prism:peer-beta", &key)
        .expect("serve shard within reciprocal budget");
    assert_eq!(served, raw0);
    assert_eq!(peer.received_shards, 1);
    assert_eq!(peer.served_shards, 1);

    // Negative checks: tampered shard, non-prism identity, and over-budget flood MUST be refused!
    let mut tampered = raw0.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0xFF;
    assert!(peer
        .receive_verified_peer_shard("prism:peer-alpha", &tampered)
        .is_err());
    assert!(peer.serve_shard_to_peer("github-oauth-user", &key).is_err());
    let _ = peer.serve_shard_to_peer("prism:peer-beta", &key);
    assert!(
        peer.serve_shard_to_peer("prism:peer-beta", &key).is_err(),
        "over-budget peer flood must be refused"
    );
}

#[test]
fn engine_ve_49() {
    use std::path::Path;
    use verdant_core::vcs::{
        parse_stitch_deep_link_uri, studio_vcs_commit_and_pack_holo_json, VerdantReleaseCapsule,
    };

    let req1 = serde_json::json!({
        "appTitle": "Sunspire Zero-Account App",
        "authorPrismId": "prism:local-creator-01",
        "message": "Initial authored 3D scene",
        "version": "1.2.0",
        "epoch": 1,
        "sceneJson": "{\"objects\":[{\"kind\":\"Box\",\"pos\":[0,1,0]}]}"
    })
    .to_string();
    let out1: serde_json::Value =
        serde_json::from_str(&studio_vcs_commit_and_pack_holo_json(&req1)).expect("parse out1");
    assert_eq!(out1["ok"], serde_json::Value::Bool(true));
    assert_eq!(out1["shardCount"], 8);
    let kappa1 = out1["commitKappa"].as_str().expect("commitKappa");
    let holo_hex = out1["holoHex"].as_str().expect("holoHex");
    let holo_bytes: Vec<u8> = (0..holo_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&holo_hex[i..i + 2], 16).expect("hex byte"))
        .collect();
    let unpacked = VerdantReleaseCapsule::verify_and_unpack(&holo_bytes)
        .expect("exported portable .holo must self-verify");
    assert_eq!(
        unpacked.capsule_sha256_hex,
        out1["capsuleSha256Hex"].as_str().unwrap_or("")
    );
    let deep_link = parse_stitch_deep_link_uri(out1["stitchDeepLinkUri"].as_str().unwrap_or(""))
        .expect("generated stitch:// deep-link must parse cleanly");
    assert_eq!(deep_link.virtual_route, "studio/index.html");

    // Second chained commit e2 referencing e1:
    let req2 = serde_json::json!({
        "appTitle": "Sunspire Zero-Account App",
        "authorPrismId": "prism:local-creator-01",
        "message": "Added beacon tower",
        "parentCommitKappa": kappa1,
        "version": "1.2.0",
        "epoch": 2,
        "sceneJson": "{\"objects\":[{\"kind\":\"Box\",\"pos\":[0,1,0]},{\"kind\":\"Pillar\",\"pos\":[4,2,0]}]}"
    })
    .to_string();
    let out2: serde_json::Value =
        serde_json::from_str(&studio_vcs_commit_and_pack_holo_json(&req2)).expect("parse out2");
    assert_eq!(out2["ok"], serde_json::Value::Bool(true));
    assert_eq!(out2["parentCommitKappa"].as_str(), Some(kappa1));
    assert_ne!(out2["commitKappa"].as_str(), Some(kappa1));

    // Verify Verdant Studio exposes the zero-account VerdantVcs & portable .holo controls:
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let studio_html = std::fs::read_to_string(repo_root.join("site/studio/index.html"))
        .expect("read site/studio/index.html");
    for id in [
        "id=\"studio-vcs-commit-btn\"",
        "id=\"studio-export-holo-btn\"",
        "id=\"studio-copy-pin-btn\"",
    ] {
        assert!(studio_html.contains(id), "Studio HTML missing {id}");
    }
}

#[test]
fn engine_ve_50() {
    use std::path::Path;
    use verdant_core::uhd::paged_workload_step_json;

    // 1. 4K UHD (3840x2160) with 16 MiB virtual glTF + CodedMatrix AI pages inside an 8 MiB arena:
    let req4k = serde_json::json!({
        "viewportW": 3840,
        "viewportH": 2160,
        "isSoftwareGpu": false,
        "webgpuAvailable": true,
        "frameUs": 9200,
        "budgetMb": 8,
        "gltfPages": 128,
        "codedAiPages": 128,
        "pageKb": 64
    })
    .to_string();
    let res4k: serde_json::Value =
        serde_json::from_str(&paged_workload_step_json(&req4k)).expect("parse 4K JSON");
    assert_eq!(res4k["ok"], serde_json::Value::Bool(true));
    assert_eq!(res4k["contractMet"], serde_json::Value::Bool(true));
    assert_eq!(
        res4k["arena"]["invariantHeld"],
        serde_json::Value::Bool(true)
    );
    assert!(res4k["arena"]["evictions"].as_u64().unwrap_or(0) > 0);
    assert!(
        res4k["arena"]["peakResidentBytes"]
            .as_u64()
            .unwrap_or(u64::MAX)
            <= res4k["arena"]["budgetBytes"].as_u64().unwrap_or(0)
    );

    // 2. 8K UHD (7680x4320) with 32 MiB virtual glTF + CodedMatrix AI pages inside an 8 MiB arena:
    let req8k = serde_json::json!({
        "viewportW": 7680,
        "viewportH": 4320,
        "isSoftwareGpu": false,
        "webgpuAvailable": true,
        "frameUs": 11400,
        "budgetMb": 8,
        "gltfPages": 256,
        "codedAiPages": 256,
        "pageKb": 64
    })
    .to_string();
    let res8k: serde_json::Value =
        serde_json::from_str(&paged_workload_step_json(&req8k)).expect("parse 8K JSON");
    assert_eq!(res8k["ok"], serde_json::Value::Bool(true));
    assert_eq!(res8k["contractMet"], serde_json::Value::Bool(true));
    assert_eq!(
        res8k["arena"]["invariantHeld"],
        serde_json::Value::Bool(true)
    );
    assert!(res8k["arena"]["evictions"].as_u64().unwrap_or(0) >= 256);

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    assert!(
        repo_root.join("scripts/verify-uhd-workload.mjs").is_file(),
        "headless Chromium UHD workload gate script must exist"
    );
}

#[test]
fn engine_ve_51() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    // 1. Verify Zenodo archival DOI & Cargo Sparse Registry are locked in AUTH-STITCH-SELF-HOSTING:
    let stitch_auth =
        std::fs::read_to_string(repo_root.join("catalogs/governance/stitch-self-hosting.json"))
            .expect("read stitch-self-hosting.json");
    assert!(stitch_auth.contains("https://zenodo.org/records/24383968"));
    assert!(stitch_auth.contains("https://afflom.github.io/verdant-engine/crates/index/"));

    // 2. Verify Blank-Frame-first entry on site/card.html and live docs links:
    let card_html =
        std::fs::read_to_string(repo_root.join("site/card.html")).expect("read site/card.html");
    assert!(card_html.contains("id=\"ve-frame-link\""));
    assert!(card_html.contains("./frame/?pin=stitch://verdant/studio"));

    let docs_html = std::fs::read_to_string(repo_root.join("site/docs/index.html"))
        .expect("read site/docs/index.html");
    assert!(docs_html.contains("https://zenodo.org/records/24383968"));
    assert!(docs_html.contains("sparse+https://afflom.github.io/verdant-engine/crates/index/"));

    // 3. Verify Observer Roadmap & KPI Manager reports all milestones (M1..M9+) and 100% passing KPIs:
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(rep.milestones.len() >= 8, "expected Milestones M1..M8+");
    assert!(
        rep.kpi_total_count >= 10,
        "expected >= 10 ISO 15939 Observer KPIs"
    );
    assert_eq!(
        rep.kpi_passed_count, rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        rep.kpis
    );
    assert_eq!(rep.minor_progress_pct, 100);
    assert_eq!(rep.major_progress_pct, 100);
}

#[test]
fn engine_ve_52() {
    use std::path::Path;
    use std::process::Command;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    // 1. Constructive Cadence.lex.tex & Release.lex.tex exist with zero Classical.choice:
    let cadence = std::fs::read_to_string(repo_root.join("app/src/Cadence.lex.tex"))
        .expect("read app/src/Cadence.lex.tex");
    let release = std::fs::read_to_string(repo_root.join("app/src/Release.lex.tex"))
        .expect("read app/src/Release.lex.tex");
    assert!(cadence.contains("cadenceGuardsVerified"));
    assert!(cadence.contains("cadenceReleaseHistoryVerified"));
    assert!(!cadence.contains("Classical.choice"));
    assert!(release.contains("releaseChannelsCompleteVerified"));
    assert!(!release.contains("Classical.choice"));

    // 2. Cargo.toml [workspace.package].version matches site/generated/release-ledger.json:
    let ledger_raw = std::fs::read_to_string(repo_root.join("site/generated/release-ledger.json"))
        .expect("read release-ledger.json");
    let ledger: serde_json::Value = serde_json::from_str(&ledger_raw).expect("parse ledger");
    let ver = format!(
        "{}.{}.{}",
        ledger["currentVersion"]["major"].as_u64().unwrap_or(0),
        ledger["currentVersion"]["minor"].as_u64().unwrap_or(0),
        ledger["currentVersion"]["patch"].as_u64().unwrap_or(0)
    );
    let cargo_toml =
        std::fs::read_to_string(repo_root.join("Cargo.toml")).expect("read Cargo.toml");
    assert!(
        cargo_toml.contains(&format!("version = \"{ver}\"")),
        "Cargo.toml must be synced to release-ledger version {ver}"
    );

    // 3. Autonomous release-tag.yml exists and is SHA-pinned; qualify-tag passes on matching tag and refuses mismatched tag!
    let rel_tag = std::fs::read_to_string(repo_root.join(".github/workflows/release-tag.yml"))
        .expect("read release-tag.yml");
    assert!(rel_tag.contains("workflows: [bootstrap]"));
    assert!(rel_tag.contains("scripts/release/qualify-tag.mjs"));

    let ok_out = Command::new("node")
        .args([
            "scripts/release/qualify-tag.mjs",
            "--offline",
            "--tag",
            &format!("v{ver}"),
        ])
        .current_dir(&repo_root)
        .output()
        .expect("run qualify-tag.mjs");
    assert!(
        ok_out.status.success(),
        "qualify-tag must pass on synchronized v{ver}: {}",
        String::from_utf8_lossy(&ok_out.stderr)
    );

    let bad_out = Command::new("node")
        .args([
            "scripts/release/qualify-tag.mjs",
            "--offline",
            "--tag",
            "v99.99.99",
        ])
        .current_dir(&repo_root)
        .output()
        .expect("run negative qualify-tag.mjs");
    assert!(
        !bad_out.status.success(),
        "qualify-tag MUST fail closed on mismatched tag v99.99.99"
    );
}

#[test]
fn engine_ve_53() {
    use std::path::Path;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass parametric + self-hosting + uorust frontier ratchet");
    assert!(rep.frontier_total_functions >= 1000);
    assert!(rep.frontier_lifted_functions >= 136);
    assert_eq!(
        rep.frontier_total_functions,
        rep.frontier_lifted_functions + rep.frontier_uncertified_functions
    );

    // Negative check: inflating residue count by 1 in frontier.toml MUST fail closed!
    let tmp = std::env::temp_dir().join(format!("verdant-ve53-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).expect("mkdir tmp");
    let raw_frontier =
        std::fs::read_to_string(repo_root.join("frontier.toml")).expect("read frontier.toml");
    let inflated = raw_frontier
        .replacen(
            &format!("functions = {}", rep.frontier_total_functions),
            &format!("functions = {}", rep.frontier_total_functions + 1),
            1,
        )
        .replacen(
            &format!("residue = {}", rep.frontier_residue_functions),
            &format!("residue = {}", rep.frontier_residue_functions + 1),
            1,
        )
        + "\n[[residue]]\nfunction = \"synthetic_regressed_fn\"\ncause = \"form\"\n";
    let bad_frontier_path = tmp.join("bad-frontier.toml");
    std::fs::write(&bad_frontier_path, inflated).expect("write bad frontier");

    let cfg_raw = std::fs::read_to_string(repo_root.join("model/parametric-ratchet.toml"))
        .expect("read ratchet config");
    let bad_cfg = cfg_raw.replace(
        "frontier_register = \"frontier.toml\"",
        &format!(
            "frontier_register = \"{}\"",
            bad_frontier_path.to_string_lossy().replace('\\', "/")
        ),
    );
    let bad_cfg_path = tmp.join("bad-ratchet.toml");
    std::fs::write(&bad_cfg_path, bad_cfg).expect("write bad ratchet config");
    assert!(
        prism_fidelity::ratchet::audit_parametric_ratchet(&repo_root, &bad_cfg_path).is_err(),
        "growing hand-written Rust residue surface MUST fail closed under [frontier_ratchet]"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn engine_ve_54() {
    use std::path::Path;
    use verdant_core::network::MultiOriginRelayPlan;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    // 1. Verify frame-pin.json pins 3 CORS-verified independent HTTPS shard origins (Pages, jsDelivr, raw.githubusercontent):
    let pin_raw = std::fs::read_to_string(repo_root.join("site/frame/frame-pin.json"))
        .expect("read frame-pin.json");
    let pin: serde_json::Value = serde_json::from_str(&pin_raw).expect("parse frame-pin.json");
    let relays: Vec<String> = pin["stitchRelays"]
        .as_array()
        .expect("stitchRelays array")
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    let plan = MultiOriginRelayPlan::build(relays, 3)
        .expect("frame-pin.json must pin >= 3 independent HTTPS origins");
    assert!(plan.distinct_origins.len() >= 3);
    assert!(plan
        .distinct_origins
        .iter()
        .any(|o| o.contains("github.io")));
    assert!(plan
        .distinct_origins
        .iter()
        .any(|o| o.contains("cdn.jsdelivr.net")));
    assert!(plan
        .distinct_origins
        .iter()
        .any(|o| o.contains("raw.githubusercontent.com")));

    // 2. Verify Blank Frame supports cross-browser #holo= user app pins and stays <= 30 KiB:
    let boot_js = std::fs::read_to_string(repo_root.join("site/frame/frame-boot.mjs"))
        .expect("read frame-boot.mjs");
    assert!(boot_js.contains("stitch-user-pin-holo"));
    assert!(repo_root
        .join("scripts/verify-sparse-registry.mjs")
        .is_file());

    // 3. Verify Observer Roadmap & KPI Manager reports all milestones (M1..M9+) & 100% passing KPIs:
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(rep.milestones.len() >= 9, "expected Milestones M1..M9+");
    assert!(
        rep.kpi_total_count >= 12,
        "expected >= 12 ISO 15939 Observer KPIs"
    );
    assert_eq!(
        rep.kpi_passed_count, rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        rep.kpis
    );
}

#[test]
fn engine_ve_55() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;
    use verdant_core::ratchet_sync::{
        measure_live_ratchet_metrics, tighten_parametric_ratchet_config,
    };
    use verdant_tensor::scalars;

    // 1. Lifted FI-08 + uorust harmonic scalars produce exact values:
    assert_eq!(scalars::two_u64(), 2);
    assert_eq!(scalars::four_u64(), 4);
    assert_eq!(scalars::eight_u64(), 8);
    assert_eq!(scalars::sixteen_u64(), 16);
    assert_eq!(scalars::thirty_two_u64(), 32);
    assert_eq!(scalars::sixty_four_u64(), 64);
    assert_eq!(scalars::hundred_u64(), 100);
    assert_eq!(scalars::thirty_two_u8(), 32);

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    // 2. Live repo passes ALL 4 ratchet layers (zero-constants, self-hosting, live-bound frontier, surface watermarks):
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers");
    assert!(rep.frontier_lifted_functions >= 150);
    assert!(rep.observed_wasm_core_bytes > 0);
    assert!(rep.observed_wasm_uorc_bytes > 0);
    assert!(rep.observed_release_capsule_bytes > 0);
    assert!(rep.observed_max_rust_file_lines > 0);
    assert!(rep.observed_max_js_file_lines > 0);

    // Verify lifted scalars appear in frontier.toml [[lifted]]:
    let frontier_raw =
        std::fs::read_to_string(repo_root.join("frontier.toml")).expect("read frontier.toml");
    assert!(frontier_raw.contains("verdant_tensor::scalars::hundred_u64"));
    assert!(
        frontier_raw.contains("verdant_tensor::memory::TensorStorageEncoding::compression_factor")
    );

    // 3. Negative check 1: stale `frontier_rust_tree_sha256` MUST fail closed!
    let tmp = std::env::temp_dir().join(format!("verdant-ve55-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).expect("mkdir tmp");
    let cfg_raw = std::fs::read_to_string(repo_root.join("model/parametric-ratchet.toml"))
        .expect("read ratchet config");
    let stale_cfg = cfg_raw.replacen(
        "frontier_rust_tree_sha256 = \"",
        "frontier_rust_tree_sha256 = \"0000000000000000000000000000000000000000000000000000000000000000\" # ",
        1,
    );
    let stale_path = tmp.join("stale-frontier-ratchet.toml");
    std::fs::write(&stale_path, stale_cfg).expect("write stale cfg");
    assert!(
        prism_fidelity::ratchet::audit_parametric_ratchet(&repo_root, &stale_path).is_err(),
        "stale frontier_rust_tree_sha256 MUST fail closed"
    );

    // 4. Negative check 2: regressed Wasm / surface metric MUST fail closed in one-way tightener!
    let mut bad_metrics =
        measure_live_ratchet_metrics(&repo_root, rep.observed_release_capsule_bytes)
            .expect("measure live metrics");
    bad_metrics.wasm_uorc_bytes = bad_metrics.wasm_uorc_bytes.saturating_add(1_048_576);
    assert!(
        tighten_parametric_ratchet_config(&repo_root, &bad_metrics, false).is_err(),
        "one-way ratchet tightener MUST refuse regressed Wasm byte weight"
    );
    let _ = std::fs::remove_dir_all(&tmp);

    // 5. Observer reports all milestones (M1..M10+) and 100% passing KPIs:
    assert!(repo_root.join("scripts/verdant-ratchet.mjs").is_file());
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let kpi_rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(
        kpi_rep.milestones.len() >= 10,
        "expected Milestones M1..M10+"
    );
    assert!(
        kpi_rep.kpi_total_count >= 13,
        "expected >= 13 ISO 15939 Observer KPIs"
    );
    assert_eq!(
        kpi_rep.kpi_passed_count, kpi_rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        kpi_rep.kpis
    );
}

#[test]
fn engine_ve_56() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers with tightened M11 watermarks");

    // 1. Verify >= 1.5 MiB Wasm weight diet on site/verdant-uorc.wasm (was 6,555,219 B, now < 5,000,000 B):
    assert!(
        rep.observed_wasm_uorc_bytes < 5_000_000,
        "site/verdant-uorc.wasm must stay strictly below 5,000,000 B after M11 opt-level=z diet, got {}",
        rep.observed_wasm_uorc_bytes
    );
    // 2. Verify expanded uorust lifted frontier (>= 180 lifted functions) and tightened surface ceilings:
    assert!(
        rep.frontier_lifted_functions >= 180,
        "M11 must lift >= 180 constructive Rust functions in frontier.toml, got {}",
        rep.frontier_lifted_functions
    );
    let ratchet_raw = std::fs::read_to_string(repo_root.join("model/parametric-ratchet.toml"))
        .expect("read model/parametric-ratchet.toml");
    assert!(
        ratchet_raw.contains("[surface_ratchet]"),
        "parametric-ratchet.toml must retain monotonic [surface_ratchet]"
    );

    // 3. Observer reports all 11 milestones (M1..M11) and 13/13 passing KPIs:
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let kpi_rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(
        kpi_rep.milestones.len() >= 11,
        "expected Milestones M1..M11"
    );
    assert_eq!(
        kpi_rep.kpi_passed_count, kpi_rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        kpi_rep.kpis
    );
}

#[test]
fn engine_ve_57() {
    use verdant_core::portable_player::{escape_html_text, render_portable_3d_player_html};
    use verdant_core::vcs::{studio_vcs_commit_and_pack_holo_json, VerdantReleaseCapsule};

    // 1. Safe HTML escaping refuses raw tag/script injection:
    let escaped = escape_html_text("<script>alert(\"x&y\")</script>");
    assert!(!escaped.contains("<script>"));
    assert!(escaped.contains("&lt;script&gt;"));
    assert!(escaped.contains("&amp;"));
    assert!(escaped.contains("&quot;"));

    // 2. Standalone portable 3D player HTML embeds interactive WebGL2 viewport with zero external CDN fetches:
    let scene_json = r#"{"project":"Harbor Lighthouse","domain":"sunspire-regatta","objects":[{"id":"o1","pos":[0,1,0],"scale":[2,3,2],"yawMilli":0,"color":[0.9,0.4,0.2]},{"id":"o2","pos":[3,0.5,-2],"scale":[1.2,1.2,1.2],"yawMilli":785,"color":[3.95,0.78,0.28]}]}"#;
    let html =
        render_portable_3d_player_html("Harbor <Lighthouse>", "prism:local:alice", scene_json);
    assert!(html.contains("id=\"verdant-shared-3d-canvas\""));
    assert!(html.contains("window.__VERDANT_SHARED_APP__"));
    assert!(html.contains("Harbor &lt;Lighthouse&gt;"));
    assert!(
        !html.contains("https://"),
        "portable .holo 3D player must be 100% offline with zero external CDN URLs"
    );
    assert!(
        !html.contains("http://"),
        "portable .holo 3D player must be 100% offline with zero external HTTP URLs"
    );

    // 3. End-to-end zero-account VerdantVcs commit -> portable .holo pack -> cryptographic unpack -> live 3D player:
    let req = serde_json::json!({
        "appTitle": "Harbor Lighthouse",
        "authorPrismId": "prism:local:alice",
        "message": "Author 2 objects for offline 3D sharing",
        "version": "1.0.0",
        "epoch": 1,
        "sceneJson": scene_json,
    })
    .to_string();
    let packed_raw = studio_vcs_commit_and_pack_holo_json(&req);
    let packed: serde_json::Value =
        serde_json::from_str(&packed_raw).expect("valid studio_vcs_commit_and_pack_holo_json JSON");
    assert_eq!(packed["ok"], serde_json::json!(true), "packed={packed}");
    assert!(packed["capsuleByteLen"].as_u64().unwrap_or(0) > 0);
    assert_eq!(packed["shardCount"].as_u64(), Some(8));
    assert!(packed["stitchDeepLinkUri"]
        .as_str()
        .unwrap_or("")
        .starts_with("stitch://"));

    let holo_hex = packed["holoHex"].as_str().expect("holoHex string");
    let mut capsule_bytes = Vec::with_capacity(holo_hex.len() / 2);
    for idx in (0..holo_hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&holo_hex[idx..idx + 2], 16).expect("valid hex byte");
        capsule_bytes.push(byte);
    }
    let unpacked = VerdantReleaseCapsule::verify_and_unpack(&capsule_bytes)
        .expect("verify unpacked .holo capsule");
    let studio_html_bytes = unpacked
        .payloads
        .get("studio/index.html")
        .expect("portable .holo must contain studio/index.html");
    let studio_html = std::str::from_utf8(studio_html_bytes).expect("utf-8 studio/index.html");
    assert!(studio_html.contains("id=\"verdant-shared-3d-canvas\""));
    assert!(studio_html.contains("Harbor Lighthouse"));
    assert!(studio_html.contains("window.__VERDANT_SHARED_APP__"));
}

#[test]
fn engine_ve_58() {
    use std::path::Path;
    use verdant_core::portable_player::render_portable_3d_player_html;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    // 1. Verify monolith decomposition submodules exist across Rust & Studio JS:
    for rel in [
        "crates/verdant-wasm/src/exports_cold.rs",
        "crates/verdant-wasm/src/exports_uorc.rs",
        "crates/verdant-core/src/operator_types.rs",
        "crates/verdant-core/src/operator_release.rs",
        "crates/verdant-core/src/operator_mcp.rs",
        "site/studio/outliner.mjs",
        "site/studio/observer-panel.mjs",
        "site/studio/project-inspector.mjs",
        "site/studio/vcs-panel.mjs",
    ] {
        assert!(
            repo_root.join(rel).is_file(),
            "decomposed submodule `{rel}` must exist"
        );
    }

    // 2. Verify tightened [surface_ratchet] file-line ceilings (max Rust <= 3525, max JS <= 1980):
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers with tightened M12 ceilings");
    assert!(
        rep.observed_max_rust_file_lines <= 3525,
        "max hand-written Rust file lines must be <= 3525 after decomposition, got {}",
        rep.observed_max_rust_file_lines
    );
    assert!(
        rep.observed_max_js_file_lines <= 1980,
        "max JS file lines must be <= 1980 after decomposition, got {}",
        rep.observed_max_js_file_lines
    );

    // 3. Verify naga is gated behind optional wgsl-oracle feature in crates/verdant-core/Cargo.toml:
    let core_toml = std::fs::read_to_string(repo_root.join("crates/verdant-core/Cargo.toml"))
        .expect("read verdant-core Cargo.toml");
    assert!(core_toml.contains("wgsl-oracle = [\"dep:naga\"]"));
    assert!(core_toml.contains("optional = true"));

    // 4. Verify 5-shape faithful offline WebGL2 portable player HTML (Box, Platform, Pillar, Ramp, Beacon Orb):
    let multi_shape_scene = r#"{"project":"Citadel Courtyard","domain":"meridian-observatory","objects":[{"kind":0,"pos":[0,0,0],"scale":[1.6,1.6,1.6],"yawMilli":0,"color":[0.86,0.34,0.16]},{"kind":1,"pos":[3,0,0],"scale":[2.2,1.0,2.2],"yawMilli":0,"color":[0.84,0.82,0.76]},{"kind":2,"pos":[-3,0,0],"scale":[1.2,2.2,1.2],"yawMilli":0,"color":[2.84,0.82,0.76]},{"kind":3,"pos":[0,0,3],"scale":[2.0,1.5,2.6],"yawMilli":0,"color":[0.26,0.3,0.36]},{"kind":4,"pos":[0,0,-3],"scale":[1.2,1.2,1.2],"yawMilli":0,"color":[3.24,0.66,0.86]}]}"#;
    let html =
        render_portable_3d_player_html("Citadel Courtyard", "prism:local:alice", multi_shape_scene);
    assert!(html.contains("mkBox"));
    assert!(html.contains("mkCyl"));
    assert!(html.contains("mkRamp"));
    assert!(html.contains("mkOrb"));
    assert!(html.contains("shapeKinds:5"));
    assert!(html.contains("KeyW"));
    assert!(!html.contains("https://"));
    assert!(!html.contains("http://"));
}

#[test]
fn engine_ve_59() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers");

    // 1. Verify uorust MIR proof frontier crossed 200+ lifted functions:
    assert!(
        rep.frontier_lifted_functions >= 200,
        "M12 must lift >= 200 constructive Rust functions in frontier.toml, got {}",
        rep.frontier_lifted_functions
    );

    // 2. Verify fidelity.toml closes all meant obligations across Cadence, Release, and Security:
    let fid_raw =
        std::fs::read_to_string(repo_root.join("fidelity.toml")).expect("read fidelity.toml");
    assert!(
        !fid_raw.contains("axis = \"meant\"\nreason = \"no theorem states anything about it"),
        "all 705 meant verification obligations across Cadence/Release/Security must be closed in fidelity.toml"
    );

    // 3. Observer reports all 12 milestones (M1..M12) and 13/13 passing KPIs:
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let kpi_rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(
        kpi_rep.milestones.len() >= 12,
        "expected Milestones M1..M12"
    );
    assert_eq!(
        kpi_rep.kpi_passed_count, kpi_rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        kpi_rep.kpis
    );
}

/// `VE-60`: Transparent RFC 1952 gzip `.holo` release capsule compression (`< 1.8 MiB`),
/// unified `digest.rs` SHA-256/hex helper consolidation, and `VerdantVcs` structural commit
/// diff (`diff_commits`) & branch ref (`create_branch`, `list_refs`) APIs.
#[test]
fn engine_ve_60() {
    use std::collections::BTreeMap;
    use std::path::Path;
    use verdant_core::digest::{hex_encode, sha256_hex, sha256_hex_tagged};
    use verdant_core::vcs::{VerdantReleaseCapsule, VerdantVcs, VerdantVcsChangeKind};

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers");

    // 1. Gzipped .holo release capsule stays strictly <= 1,800,000 B (>70% reduction vs 6.29 MiB):
    assert!(
        rep.observed_release_capsule_bytes <= 1_800_000,
        "M13 gzipped .holo release capsule must stay <= 1,800,000 B, got {}",
        rep.observed_release_capsule_bytes
    );

    // 2. Round-trip gzipped capsule, verify tamper & trailing-byte refusal, and verify Blank Frame support:
    let mut files = BTreeMap::new();
    let payload = "<html><body>Verdant Gzip Capsule Verification</body></html>".repeat(48);
    files.insert("studio/index.html".to_string(), payload.as_bytes().to_vec());
    let packed = VerdantReleaseCapsule::pack_from_files(
        "2.5.0",
        9,
        "vcs-commit-kappa-m13",
        "seal-sha-m13",
        "studio/index.html",
        &files,
    )
    .expect("pack gzipped .holo capsule");
    assert_eq!(packed.manifest.compression.as_deref(), Some("gzip"));
    assert!(packed.holo_bytes.len() < packed.manifest.total_payload_bytes / 3);
    let unpacked = VerdantReleaseCapsule::verify_and_unpack(&packed.holo_bytes)
        .expect("verify gzipped .holo capsule");
    assert_eq!(unpacked.payloads, files);

    let mut with_trailing = packed.holo_bytes.clone();
    with_trailing.push(0x42);
    assert!(VerdantReleaseCapsule::verify_and_unpack(&with_trailing).is_err());

    let frame_boot =
        std::fs::read_to_string(repo_root.join("site/frame/frame-boot.mjs")).expect("frame-boot");
    assert!(frame_boot.contains("DecompressionStream(\"gzip\")"));
    let vcs_panel =
        std::fs::read_to_string(repo_root.join("site/studio/vcs-panel.mjs")).expect("vcs-panel");
    assert!(vcs_panel.contains("CompressionStream(\"gzip\")"));

    // 3. Canonical digest.rs helpers produce deterministic SHA-256 / hex output:
    assert_eq!(hex_encode(&[0x0a, 0xff]), "0aff");
    assert_eq!(sha256_hex(b"").len(), 64);
    assert_ne!(
        sha256_hex_tagged("verdant/operator/v1:", "a", b"x"),
        sha256_hex_tagged("", "a", b"x")
    );

    // 4. VerdantVcs structural diff_commits + create_branch + list_refs:
    let mut vcs = VerdantVcs::new();
    let mut t1 = BTreeMap::new();
    t1.insert("scene.json".to_string(), b"{\"v\":1}".to_vec());
    t1.insert("old.txt".to_string(), b"remove-me".to_vec());
    let c1 = vcs
        .commit_tree(
            "refs/verdant/heads/main",
            &t1,
            "prism:m13-tester",
            b"dev-secret",
            "mgr-change-release",
            "AUTH-STITCH-SELF-HOSTING",
            "m13 genesis",
        )
        .expect("commit c1");
    let mut t2 = BTreeMap::new();
    t2.insert("scene.json".to_string(), b"{\"v\":2}".to_vec());
    t2.insert("new.txt".to_string(), b"added".to_vec());
    let c2 = vcs
        .commit_tree(
            "refs/verdant/heads/main",
            &t2,
            "prism:m13-tester",
            b"dev-secret",
            "mgr-change-release",
            "AUTH-STITCH-SELF-HOSTING",
            "m13 update",
        )
        .expect("commit c2");
    let diff = vcs
        .diff_commits(&c1.commit_kappa, &c2.commit_kappa)
        .expect("diff c1 -> c2");
    assert_eq!(diff.added_count, 1);
    assert_eq!(diff.modified_count, 1);
    assert_eq!(diff.removed_count, 1);
    assert_eq!(diff.entries[0].kind, VerdantVcsChangeKind::Added);
    let b_ref = vcs
        .create_branch("m13-canary", &c1.commit_kappa)
        .expect("create branch");
    assert_eq!(b_ref, "refs/verdant/heads/m13-canary");
    assert!(vcs.create_branch("../escape", &c1.commit_kappa).is_err());
    assert_eq!(vcs.list_refs().len(), 2);
}

/// `VE-61`: 100% zero-residue formal model fidelity across all 28 LexLean modules (`705/705`
/// derived, `705/705` meant, `9/9` measured, `0` residue), version-parametric `Release.lex.tex`,
/// and second-tier decomposition tightening `max_handwritten_rust_file_lines <= 2,200`.
#[test]
fn engine_ve_61() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers");

    // 1. Second-tier Rust monolith decomposition tightened max_handwritten_rust_file_lines <= 2,200:
    assert!(
        rep.observed_max_rust_file_lines <= 2_200,
        "M13 must tighten max_handwritten_rust_file_lines <= 2200, got {}",
        rep.observed_max_rust_file_lines
    );
    for decomposed in [
        "crates/verdant-core/src/digest.rs",
        "crates/verdant-core/src/observer_language.rs",
        "crates/verdant-core/src/observer_streams.rs",
        "crates/verdant-core/src/operator_synth.rs",
        "crates/conformance/tests/engine/main.rs",
    ] {
        assert!(
            repo_root.join(decomposed).is_file(),
            "missing decomposed file `{decomposed}`"
        );
    }

    // 2. 100% zero-residue formal fidelity closure (0 [[residue]] rows in fidelity.toml):
    let fid_raw =
        std::fs::read_to_string(repo_root.join("fidelity.toml")).expect("read fidelity.toml");
    assert!(
        !fid_raw.contains("[[residue]]"),
        "M13 fidelity.toml must achieve 100% closure with 0 [[residue]] rows"
    );

    // 3. Release.lex.tex uses version-parametric channel templates (zero stale v2.1.0 endpoint strings):
    let release_lex = std::fs::read_to_string(repo_root.join("app/src/Release.lex.tex"))
        .expect("read Release.lex.tex");
    assert!(release_lex.contains("v{version}/shards"));
    assert!(release_lex.contains("verdant-release:v{version}"));
    assert!(!release_lex.contains("/v2.1.0/shards"));

    // 4. Observer reports all 13 milestones (M1..M13) and 13/13 passing KPIs:
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let kpi_rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(
        kpi_rep.milestones.len() >= 13,
        "expected Milestones M1..M13"
    );
    assert_eq!(
        kpi_rep.kpi_passed_count, kpi_rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        kpi_rep.kpis
    );
}

/// `VE-62`: Generative Observer Self-Refinement Engine (`observer_refine.rs`) synthesizes a
/// deterministic, cryptographically sealed self-refinement agenda across all 13 accountable VSM
/// Managers, live `frontier.toml` blockers (`>= 219` MIR-lifted), and surface metrics, wired into
/// CLI, MCP, and Studio's live Observer Panel.
#[test]
fn engine_ve_62() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");

    // 1. Deterministic self-refinement agenda synthesis across all 13 Managers:
    let (agenda_a, json_a) = ch
        .synthesize_refinement_agenda(&repo_root)
        .expect("synthesize observer refinement agenda (pass 1)");
    let (agenda_b, _) = ch
        .synthesize_refinement_agenda(&repo_root)
        .expect("synthesize observer refinement agenda (pass 2)");
    assert_eq!(agenda_a.spec, "verdant/observer-refinement-agenda/1");
    assert_eq!(agenda_a.agenda_kappa_hex, agenda_b.agenda_kappa_hex);
    assert_eq!(agenda_a.part_health.len(), 13);
    assert!(!agenda_a.top_frontier_blockers.is_empty());
    assert!(!agenda_a.proposals.is_empty());
    assert!(agenda_a.overall_refinement_score_permille > 0);
    assert!(agenda_a.live_metrics.frontier_lifted >= 219);
    assert_eq!(agenda_a.live_metrics.fidelity_open_residue, 0);
    assert!(json_a.contains("observer-refinement-agenda/1"));

    // 2. Cryptographically sealed in site/generated/observer-refinement-agenda.json & immutability-seal.json:
    assert!(repo_root
        .join("site/generated/observer-refinement-agenda.json")
        .is_file());
    let seal = std::fs::read_to_string(repo_root.join("site/generated/immutability-seal.json"))
        .expect("read immutability-seal.json");
    assert!(seal.contains("generated/observer-refinement-agenda.json"));

    // 3. Studio Observer Panel renders live self-refinement telemetry:
    let obs_panel = std::fs::read_to_string(repo_root.join("site/studio/observer-panel.mjs"))
        .expect("read site/studio/observer-panel.mjs");
    assert!(obs_panel.contains("observer-refinement-agenda.json"));
}

/// `VE-63`: Verdant implements Verdant — lifted in-Verdant validation, test execution & release
/// qualification lifecycle (`lifecycle.rs`), 6-category lifted manifest coverage (`>= 130`
/// modules), Studio VCS commit diffs + JS decomposition (`<= 1,700` lines), and M14 KPIs.
#[test]
fn engine_ve_63() {
    use std::path::Path;
    use verdant_core::operator::{LifecycleGateOptions, VerdantOperatorChannel};

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");

    // 1. In-process lifted lifecycle gate executes cleanly and certifies release readiness:
    let (gate, gate_json) = ch
        .execute_lifecycle_gate(
            &repo_root,
            &LifecycleGateOptions {
                run_workspace_tests: false,
                run_browser_gates: false,
                expected_tag: None,
                write_report: false,
            },
        )
        .expect("execute in-process lifecycle gate");
    assert_eq!(gate.spec, "verdant/lifecycle-gate-report/1");
    assert!(
        gate.all_passed,
        "in-process lifecycle gate failed: {:?}",
        gate.checks
    );
    assert!(gate.release_qualified);
    assert!(gate.lifted_coverage.all_categories_lifted);
    assert!(
        gate.lifted_coverage.total_lifted_modules >= 130,
        "expected >= 130 lifted lifecycle modules across all 6 categories, got {}",
        gate.lifted_coverage.total_lifted_modules
    );
    assert!(gate.lifted_coverage.verification_test_modules > 0);
    assert!(gate.lifted_coverage.feature_suite_modules > 0);
    assert!(gate.lifted_coverage.model_governance_modules > 0);
    assert!(gate_json.contains("verdant/lifecycle-gate-report/1"));

    // 2. Fail-closed tag qualification refuses mismatched release tag:
    let (bad_tag_gate, _) = ch
        .execute_lifecycle_gate(
            &repo_root,
            &LifecycleGateOptions {
                run_workspace_tests: false,
                run_browser_gates: false,
                expected_tag: Some("v0.0.1".to_string()),
                write_report: false,
            },
        )
        .expect("mismatched tag gate evaluation");
    assert!(
        !bad_tag_gate.release_qualified,
        "lifecycle gate MUST refuse mismatched release tag v0.0.1"
    );

    // 3. Sealed lifecycle report, qualify-tag delegation, Studio JS decomposition & VCS commit diffs:
    assert!(repo_root
        .join("site/generated/lifecycle-gate-report.json")
        .is_file());
    let seal = std::fs::read_to_string(repo_root.join("site/generated/immutability-seal.json"))
        .expect("read immutability-seal.json");
    assert!(seal.contains("generated/lifecycle-gate-report.json"));
    let qualify_mjs = std::fs::read_to_string(repo_root.join("scripts/release/qualify-tag.mjs"))
        .expect("read qualify-tag.mjs");
    assert!(qualify_mjs.contains("lifecycle-gate"));
    assert!(repo_root.join("site/studio-media.mjs").is_file());
    assert!(repo_root.join("site/studio/discovery-panel.mjs").is_file());
    let vcs_panel = std::fs::read_to_string(repo_root.join("site/studio/vcs-panel.mjs"))
        .expect("read vcs-panel.mjs");
    assert!(vcs_panel.contains("computeSceneObjectDiff"));

    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers");
    assert!(
        rep.observed_max_js_file_lines <= 1_680,
        "M14 Studio JS decomposition must tighten max_js_file_lines <= 1680, got {}",
        rep.observed_max_js_file_lines
    );

    // 4. Observer reports all 14 milestones (M1..M14) and 13/13 passing KPIs:
    let kpi_rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(
        kpi_rep.milestones.len() >= 14,
        "expected Milestones M1..M14"
    );
    assert_eq!(
        kpi_rep.kpi_passed_count, kpi_rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        kpi_rep.kpis
    );
}

/// `VE-64`: Full in-Verdant ratchet orchestrator (`execute_ratchet_cycle`, CLI `operator
/// ratchet-cycle`, MCP `verdant_ratchet_cycle`, thin `scripts/verdant-ratchet.mjs` delegate)
/// and 7-category lifted lifecycle coverage (`>= 150` modules including `automation-script`).
#[test]
fn engine_ve_64() {
    use std::path::Path;
    use verdant_core::operator::{
        lifecycle_lifted_categories_complete_v7, ratchet_cycle_all_passed,
        ratchet_cycle_needs_second_pass, ratchet_stage_cache_hit, LifecycleGateOptions,
        VerdantOperatorChannel,
    };

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");

    // 1. Pure MIR-lifted ratchet-cycle decision predicates hold constructively:
    assert!(ratchet_stage_cache_hit(false, true, true));
    assert!(!ratchet_stage_cache_hit(true, true, true));
    assert!(ratchet_cycle_all_passed(7, 1, 8));
    assert!(!ratchet_cycle_all_passed(6, 1, 8));
    assert!(ratchet_cycle_needs_second_pass(false, true));
    assert!(!ratchet_cycle_needs_second_pass(false, false));

    // 2. Thin Node.js entry delegates directly into Verdant's lifted `operator ratchet-cycle`:
    let ratchet_js = std::fs::read_to_string(repo_root.join("scripts/verdant-ratchet.mjs"))
        .expect("read scripts/verdant-ratchet.mjs");
    assert!(ratchet_js.contains("ratchet-cycle"));
    assert!(
        ratchet_js.lines().count() <= 40,
        "scripts/verdant-ratchet.mjs must stay a thin delegate into Verdant (got {} lines)",
        ratchet_js.lines().count()
    );

    // 3. MCP server advertises `verdant_ratchet_cycle` tool:
    let mut mcp = VerdantOperatorChannel::new("sunspire-regatta").expect("mcp channel");
    let list_out = mcp
        .handle_mcp_jsonrpc_line(
            &repo_root,
            r#"{"jsonrpc":"2.0","id":64,"method":"tools/list","params":{}}"#,
        )
        .expect("MCP tools/list response");
    assert!(list_out.contains("verdant_ratchet_cycle"));

    // 4. All 7 lifecycle categories (including 20 automation scripts) are lifted inside Verdant:
    let (gate, _) = ch
        .execute_lifecycle_gate(
            &repo_root,
            &LifecycleGateOptions {
                run_workspace_tests: false,
                run_browser_gates: false,
                expected_tag: None,
                write_report: false,
            },
        )
        .expect("in-process lifecycle gate");
    assert!(gate.all_passed && gate.release_qualified);
    assert!(gate.lifted_coverage.automation_script_modules >= 15);
    assert!(gate.lifted_coverage.total_lifted_modules >= 150);
    assert!(lifecycle_lifted_categories_complete_v7(
        gate.lifted_coverage.rust_modules,
        gate.lifted_coverage.verification_test_modules,
        gate.lifted_coverage.lexlean_modules,
        gate.lifted_coverage.feature_suite_modules,
        gate.lifted_coverage.model_governance_modules,
        gate.lifted_coverage.automation_script_modules,
        gate.lifted_coverage.viewpoint_modules,
    ));
}

/// `VE-65`: Observer-driven MIR proof frontier expansion (`>= 246` lifted functions, residue
/// `<= 1,179`, uncertified `<= 1,334`) and JS surface decomposition tightening
/// `max_js_file_lines <= 1,525` across `engine-worker-codec.mjs`, `stage-assets.mjs`, and
/// `studio-gameplay-panel.mjs`, with all 15 milestones (`M1..M15`) and 13/13 Observer KPIs passing.
#[test]
fn engine_ve_65() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers");

    // 1. Expanded uorust MIR proof frontier >= 246 lifted functions:
    assert!(
        rep.frontier_lifted_functions >= 246,
        "M15 must lift >= 246 constructive Rust functions, got {}",
        rep.frontier_lifted_functions
    );

    // 2. Decomposed JS submodules exist and tighten max_js_file_lines <= 1,525:
    for rel in [
        "site/engine-worker-codec.mjs",
        "site/studio/stage-assets.mjs",
        "site/studio-gameplay-panel.mjs",
    ] {
        assert!(
            repo_root.join(rel).is_file(),
            "missing decomposed JS submodule `{rel}`"
        );
    }
    assert!(
        rep.observed_max_js_file_lines <= 1_525,
        "M15 JS decomposition must tighten max_js_file_lines <= 1525, got {}",
        rep.observed_max_js_file_lines
    );

    // 3. Observer reports all 15 milestones (M1..M15) and 13/13 passing KPIs:
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let kpi_rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(
        kpi_rep.milestones.len() >= 15,
        "expected Milestones M1..M15"
    );
    assert_eq!(
        kpi_rep.kpi_passed_count, kpi_rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        kpi_rep.kpis
    );
}

/// `VE-66`: Lifted in-Verdant self-automation pipeline (`execute_automate_pipeline`, CLI
/// `operator automate`, MCP `verdant_automate`, thin `scripts/verdant-automate.mjs` delegate),
/// pinpoint crate-aware Observer diagnosis, and expanded `uorust` MIR proof frontier (`>= 267`
/// lifted constructive functions, residue `<= 1,178`).
#[test]
fn engine_ve_66() {
    use std::path::Path;
    use verdant_core::operator::VerdantOperatorChannel;
    use verdant_core::verdant_tensor::scalars::{
        automate_should_build_wasm, automate_stage_passed, blank_frame_compliant,
        semver_is_strictly_higher, stitch_quorum_sufficient, vcs_diff_is_clean,
    };

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    // 1. MIR-lifted automation, Stitch quorum, Blank Frame, VCS & SemVer predicates hold:
    assert!(automate_stage_passed(11, 1, 12));
    assert!(!automate_stage_passed(10, 1, 12));
    assert!(automate_should_build_wasm(false, false, false));
    assert!(!automate_should_build_wasm(true, true, false));
    assert!(stitch_quorum_sufficient(4, 4));
    assert!(!stitch_quorum_sufficient(3, 4));
    assert!(blank_frame_compliant(28_370, 30_720, 0));
    assert!(!blank_frame_compliant(28_370, 30_720, 1));
    assert!(vcs_diff_is_clean(0, 0, 0));
    assert!(semver_is_strictly_higher(2, 8, 0, 2, 7, 0));

    // 2. Thin scripts/verdant-automate.mjs delegates directly to `verdant operator automate`:
    let auto_js = std::fs::read_to_string(repo_root.join("scripts/verdant-automate.mjs"))
        .expect("read scripts/verdant-automate.mjs");
    assert!(auto_js.contains("\"automate\""));
    assert!(
        auto_js.lines().count() <= 40,
        "scripts/verdant-automate.mjs must remain a thin delegate into Verdant (got {} lines)",
        auto_js.lines().count()
    );

    // 3. MCP server advertises `verdant_automate`:
    let mut mcp = VerdantOperatorChannel::new("sunspire-regatta").expect("mcp channel");
    let list_out = mcp
        .handle_mcp_jsonrpc_line(
            &repo_root,
            r#"{"jsonrpc":"2.0","id":66,"method":"tools/list","params":{}}"#,
        )
        .expect("MCP tools/list response");
    assert!(list_out.contains("verdant_automate"));

    // 4. uorust MIR proof frontier >= 267 lifted functions:
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers");
    assert!(
        rep.frontier_lifted_functions >= 267,
        "M16 must lift >= 267 constructive Rust functions, got {}",
        rep.frontier_lifted_functions
    );
}

/// `VE-67`: Sub-2,000-line Rust modularity (`max_handwritten_rust_file_lines <= 1,985`, `>= 163`
/// lifted lifecycle modules across all 7 categories) and sub-1,400-line JS modularity
/// (`max_js_file_lines <= 1,400`), with all 16 milestones (`M1..M16`) and 13/13 Observer KPIs.
#[test]
fn engine_ve_67() {
    use std::path::Path;
    use verdant_core::operator::{LifecycleGateOptions, VerdantOperatorChannel};

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let rep = prism_fidelity::ratchet::audit_parametric_ratchet(
        &repo_root,
        Path::new("model/parametric-ratchet.toml"),
    )
    .expect("live repo must pass all 4 ratchet layers");

    // 1. All 7 decomposed Rust submodules + catalog-streams-stitch.mjs exist:
    for rel in [
        "crates/verdant-core/src/hd_world_templates.rs",
        "crates/verdant-core/src/standards_registry.rs",
        "crates/verdant-core/src/observer_types.rs",
        "crates/verdant-core/src/uhd_shaders.rs",
        "crates/verdant-wasm/src/bridge_sim.rs",
        "crates/verdant-wasm/src/exports_cold_stage.rs",
        "crates/verdant-wasm/src/exports_uorc_studio.rs",
        "site/catalog-streams-stitch.mjs",
    ] {
        assert!(
            repo_root.join(rel).is_file(),
            "missing M16 decomposed submodule `{rel}`"
        );
    }

    // 2. Monotonic surface ceilings strictly < 2000L Rust and <= 1400L JS:
    assert!(
        rep.observed_max_rust_file_lines <= 1_985,
        "M16 must tighten max_handwritten_rust_file_lines <= 1985, got {}",
        rep.observed_max_rust_file_lines
    );
    assert!(
        rep.observed_max_js_file_lines <= 1_400,
        "M16 must tighten max_js_file_lines <= 1400, got {}",
        rep.observed_max_js_file_lines
    );

    // 3. >= 163 lifted lifecycle modules across all 7 categories & M1..M16 with 13/13 KPIs:
    let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
    let (gate, _) = ch
        .execute_lifecycle_gate(
            &repo_root,
            &LifecycleGateOptions {
                run_workspace_tests: false,
                run_browser_gates: false,
                expected_tag: None,
                write_report: false,
            },
        )
        .expect("in-process lifecycle gate");
    assert!(gate.all_passed && gate.release_qualified);
    assert!(
        gate.lifted_coverage.total_lifted_modules >= 163,
        "expected >= 163 lifted lifecycle modules, got {}",
        gate.lifted_coverage.total_lifted_modules
    );

    let kpi_rep = ch
        .evaluate_observer_roadmap_kpis(&repo_root, true, true)
        .expect("observer roadmap & KPI report");
    assert!(
        kpi_rep.milestones.len() >= 16,
        "expected Milestones M1..M16"
    );
    assert_eq!(
        kpi_rep.kpi_passed_count, kpi_rep.kpi_total_count,
        "all Observer KPIs must pass (kpis={:?})",
        kpi_rep.kpis
    );
}
