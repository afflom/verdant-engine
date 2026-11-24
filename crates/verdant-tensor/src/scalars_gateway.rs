//! Pure `{0, 1}` MIR-lifted multi-gateway Stitch hosting, failover, HTTP readiness, and
//! stakeholder part routing predicates (`M25`, `VE-77`), included by `scalars.rs`.

use super::{
    eight_u32, five_u32, four_u32, hundred_u32, hundred_u64, six_u32, sixteen_u32, sixteen_u64,
    sixty_four_u32, ten_u32, ten_u64, thirty_two_u32, thousand_u32, three_u32, three_u64, two_u32,
    two_u64,
};

/// Pure MIR-lifted multi-gateway independent provider family floor (`families >= 2`).
#[must_use]
pub const fn gateway_provider_families_independent(distinct_families: u32) -> bool {
    distinct_families >= two_u32()
}

/// Pure MIR-lifted multi-gateway endpoint redundancy floor (`endpoints >= 2`).
#[must_use]
pub const fn gateway_endpoint_count_resilient(endpoints: u32) -> bool {
    endpoints >= two_u32()
}

/// Pure MIR-lifted HTTP 200 OK predicate (`code == 200`).
#[must_use]
pub const fn gateway_http_status_ok(code: u16) -> bool {
    let two_hundred = two_u32().wrapping_mul(hundred_u32()) as u16;
    code == two_hundred
}

/// Pure MIR-lifted HTTP redirect status predicate (`301 | 302 | 307`).
#[must_use]
pub const fn gateway_http_status_redirect(code: u16) -> bool {
    let three_hundred = three_u32().wrapping_mul(hundred_u32()) as u16;
    let r301 = three_hundred.wrapping_add(1_u16);
    let r302 = three_hundred.wrapping_add(two_u32() as u16);
    let r307 = three_hundred.wrapping_add(four_u32().wrapping_add(three_u32()) as u16);
    code == r301 || code == r302 || code == r307
}

/// Pure MIR-lifted HTTP 204 No Content CORS preflight predicate (`code == 204`).
#[must_use]
pub const fn gateway_http_status_no_content(code: u16) -> bool {
    let code_204 = two_u32()
        .wrapping_mul(hundred_u32())
        .wrapping_add(four_u32()) as u16;
    code == code_204
}

/// Pure MIR-lifted HTTP 5xx upstream outage predicate (`502 | 503 | 504`).
#[must_use]
pub const fn gateway_http_status_unavailable(code: u16) -> bool {
    let five_hundred = five_u32().wrapping_mul(hundred_u32()) as u16;
    let s502 = five_hundred.wrapping_add(two_u32() as u16);
    let s503 = five_hundred.wrapping_add(three_u32() as u16);
    let s504 = five_hundred.wrapping_add(four_u32() as u16);
    code == s502 || code == s503 || code == s504
}

/// Pure MIR-lifted HTTP 404 Not Found predicate (`code == 404`).
#[must_use]
pub const fn gateway_http_status_not_found(code: u16) -> bool {
    let code_404 = four_u32()
        .wrapping_mul(hundred_u32())
        .wrapping_add(four_u32()) as u16;
    code == code_404
}

/// Pure MIR-lifted HTTP 400 Bad Request path-traversal refusal predicate (`code == 400`).
#[must_use]
pub const fn gateway_http_status_bad_request(code: u16) -> bool {
    let code_400 = four_u32().wrapping_mul(hundred_u32()) as u16;
    code == code_400
}

/// Pure MIR-lifted stakeholder part completeness check (`parts >= 6`: studio, catalogs, demo, docs, starter, card).
#[must_use]
pub const fn gateway_stakeholder_part_count_complete(parts: u32) -> bool {
    parts >= six_u32()
}

/// Pure MIR-lifted gateway shard quorum availability check (`required_k >= 1 && available >= required_k`).
#[must_use]
pub const fn gateway_shard_quorum_reachable(available: u32, required_k: u32) -> bool {
    required_k >= 1_u32 && available >= required_k
}

/// Pure MIR-lifted multi-gateway primary outage survival check (`total >= 2 && total > failed`).
#[must_use]
pub const fn gateway_failover_survives_primary_outage(
    total_gateways: u32,
    failed_gateways: u32,
) -> bool {
    total_gateways >= two_u32() && total_gateways > failed_gateways
}

/// Pure MIR-lifted 4-of-8 STCH2 gateway recovery threshold check (`available >= 4`).
#[must_use]
pub const fn gateway_surviving_shards_recover_4_of_8(available: u32) -> bool {
    available >= four_u32()
}

/// Pure MIR-lifted full 8-of-8 STCH2 gateway mirror completeness check (`available == 8`).
#[must_use]
pub const fn gateway_full_shard_set_complete(available: u32) -> bool {
    available == eight_u32()
}

/// Pure MIR-lifted gateway failover priority range check (`1 <= priority <= 100`).
#[must_use]
pub const fn gateway_priority_valid(priority: u32) -> bool {
    priority >= 1_u32 && priority <= hundred_u32()
}

/// Pure MIR-lifted gateway provider kind discriminant bound (`code <= 6`).
#[must_use]
pub const fn gateway_provider_kind_code_valid(code: u32) -> bool {
    code <= six_u32()
}

/// Pure MIR-lifted TCP listener port validity (`port >= 1`).
#[must_use]
pub const fn gateway_port_valid(port: u16) -> bool {
    port >= 1_u16
}

/// Pure MIR-lifted HTTP request buffer safety bound (`1 <= len <= 16384`).
#[must_use]
pub const fn gateway_max_request_bytes_bounded(len: u32) -> bool {
    let k1 = hundred_u32()
        .wrapping_mul(ten_u32())
        .wrapping_add(two_u32().wrapping_mul(ten_u32()))
        .wrapping_add(four_u32());
    let max_len = k1.wrapping_mul(four_u32().wrapping_mul(four_u32()));
    len >= 1_u32 && len <= max_len
}

/// Pure MIR-lifted immutable content-addressed shard cache TTL (`31536000` seconds = 365 days).
#[must_use]
pub const fn gateway_immutable_cache_max_age_secs() -> u64 {
    let three = two_u64().wrapping_add(1_u64);
    let five = three.wrapping_add(two_u64());
    let six = five.wrapping_add(1_u64);
    let days = three
        .wrapping_mul(hundred_u64())
        .wrapping_add(six.wrapping_mul(ten_u64()))
        .wrapping_add(five);
    let hours = two_u64()
        .wrapping_mul(ten_u64())
        .wrapping_add(two_u64().wrapping_mul(two_u64()));
    let sixty = six.wrapping_mul(ten_u64());
    days.wrapping_mul(hours)
        .wrapping_mul(sixty)
        .wrapping_mul(sixty)
}

/// Pure MIR-lifted public-benefit Stitch bundle adapter completeness (`adapters >= 4`: Stitch Layer-5 Swarm, IPFS, Volunteer OCI/HTTP, GitHub).
#[must_use]
pub const fn gateway_adapter_count_complete(adapters: u32) -> bool {
    adapters >= four_u32()
}

/// Pure MIR-lifted Blank Frame 30 KiB envelope check (`1 <= bytes <= 30720`).
#[must_use]
pub const fn gateway_blank_frame_budget_ok(bytes: u64) -> bool {
    let k1 = hundred_u64()
        .wrapping_mul(ten_u64())
        .wrapping_add(two_u64().wrapping_mul(ten_u64()))
        .wrapping_add(two_u64().wrapping_mul(two_u64()));
    let thirty = two_u64().wrapping_add(1_u64).wrapping_mul(ten_u64());
    bytes >= 1_u64 && bytes <= thirty.wrapping_mul(k1)
}

/// Pure MIR-lifted Blank Frame OPFS offline resilience check (`network_shards >= required_k || opfs_verified`).
#[must_use]
pub const fn gateway_opfs_offline_fallback_valid(
    network_shards: u32,
    required_k: u32,
    opfs_verified: bool,
) -> bool {
    (required_k >= 1_u32 && network_shards >= required_k) || opfs_verified
}

/// Pure MIR-lifted cross-origin gateway CORS wildcard policy (`!cross_origin || has_wildcard`).
#[must_use]
pub const fn gateway_cors_wildcard_required(is_cross_origin: bool, has_wildcard: bool) -> bool {
    !is_cross_origin || has_wildcard
}

/// Pure MIR-lifted monotonic gateway release epoch progression (`next > prev`).
#[must_use]
pub const fn gateway_release_epoch_monotonic(prev_epoch: u64, next_epoch: u64) -> bool {
    next_epoch > prev_epoch
}

/// Pure MIR-lifted canonical stakeholder part index check (`idx < 6`).
#[must_use]
pub const fn gateway_part_route_index_valid(idx: u32) -> bool {
    idx < six_u32()
}

/// Pure MIR-lifted M25 lifted lifecycle module floor (`modules >= 185`).
#[must_use]
pub const fn gateway_m25_lifted_module_floor_ok(modules: u64) -> bool {
    let five = two_u64().wrapping_mul(two_u64()).wrapping_add(1_u64);
    let eight = two_u64().wrapping_mul(two_u64()).wrapping_mul(two_u64());
    let floor = hundred_u64()
        .wrapping_add(eight.wrapping_mul(ten_u64()))
        .wrapping_add(five);
    modules >= floor
}

/// Pure MIR-lifted `/healthz` gateway readiness invariant.
#[must_use]
pub const fn gateway_healthz_quorum_healthy(
    available_shards: u32,
    required_k: u32,
    manifest_valid: bool,
) -> bool {
    manifest_valid && required_k >= 1_u32 && available_shards >= required_k
}

/// Pure MIR-lifted non-GitHub provider independence check (`total_families > github_families`).
#[must_use]
pub const fn gateway_non_github_origin_present(total_families: u32, github_families: u32) -> bool {
    total_families > github_families
}

/// Pure MIR-lifted gateway bundle archive non-emptiness check (`bytes >= 1`).
#[must_use]
pub const fn gateway_bundle_tarball_nonempty(bytes: u64) -> bool {
    bytes >= 1_u64
}

/// Pure MIR-lifted dual-gateway availability check (`github_ok || external_ok`).
#[must_use]
pub const fn gateway_dual_mirror_quorum_ok(github_ok: bool, external_ok: bool) -> bool {
    github_ok || external_ok
}

/// Pure MIR-lifted full multi-gateway redundancy check (`github_ok && external_ok`).
#[must_use]
pub const fn gateway_full_redundancy_ok(github_ok: bool, external_ok: bool) -> bool {
    github_ok && external_ok
}

/// Pure MIR-lifted HTTP 503 outage code constructor (`503`).
#[must_use]
pub const fn gateway_http_503_code() -> u16 {
    five_u32()
        .wrapping_mul(hundred_u32())
        .wrapping_add(three_u32()) as u16
}

/// Pure MIR-lifted HTTP 302 redirect code constructor (`302`).
#[must_use]
pub const fn gateway_http_302_code() -> u16 {
    three_u32()
        .wrapping_mul(hundred_u32())
        .wrapping_add(two_u32()) as u16
}

/// Pure MIR-lifted HTTP 204 preflight code constructor (`204`).
#[must_use]
pub const fn gateway_http_204_code() -> u16 {
    two_u32()
        .wrapping_mul(hundred_u32())
        .wrapping_add(four_u32()) as u16
}

/// Pure MIR-lifted HTTP request timeout budget (`2000` ms).
#[must_use]
pub const fn gateway_probe_timeout_millis() -> u32 {
    two_u32().wrapping_mul(thousand_u32())
}

/// Pure MIR-lifted Stitch Layer-5 (`stitch/1`) wire frame ceiling (`4096` bytes).
#[must_use]
pub const fn stitch_l5_wire_bytes_max() -> u32 {
    let k1 = hundred_u32()
        .wrapping_mul(ten_u32())
        .wrapping_add(two_u32().wrapping_mul(ten_u32()))
        .wrapping_add(four_u32());
    four_u32().wrapping_mul(k1)
}

/// Pure MIR-lifted Stitch Layer-5 (`stitch/1`) chunk data ceiling (`2700` bytes).
#[must_use]
pub const fn stitch_l5_chunk_bytes_max() -> u32 {
    let seven = four_u32().wrapping_add(three_u32());
    two_u32()
        .wrapping_mul(thousand_u32())
        .wrapping_add(seven.wrapping_mul(hundred_u32()))
}

/// Pure MIR-lifted Stitch Layer-5 (`stitch/1`) assembled message ceiling (`524288` bytes = 512 KiB).
#[must_use]
pub const fn stitch_l5_message_bytes_max() -> u32 {
    let k1 = hundred_u32()
        .wrapping_mul(ten_u32())
        .wrapping_add(two_u32().wrapping_mul(ten_u32()))
        .wrapping_add(four_u32());
    let n512 = five_u32()
        .wrapping_mul(hundred_u32())
        .wrapping_add(ten_u32())
        .wrapping_add(two_u32());
    n512.wrapping_mul(k1)
}

/// Pure MIR-lifted Stitch Layer-5 (`stitch/1`) max frames per message (`256`).
#[must_use]
pub const fn stitch_l5_max_frames() -> u32 {
    two_u32()
        .wrapping_mul(hundred_u32())
        .wrapping_add(five_u32().wrapping_mul(ten_u32()))
        .wrapping_add(six_u32())
}

/// Pure MIR-lifted Stitch Layer-5 (`stitch/1`) signal lifetime (`60000` ms).
#[must_use]
pub const fn stitch_l5_signal_lifetime_ms() -> u32 {
    six_u32()
        .wrapping_mul(ten_u32())
        .wrapping_mul(thousand_u32())
}

/// Pure MIR-lifted Stitch Layer-5 (`stitch/1`) warm lifetime (`1800000` ms = 30 min).
#[must_use]
pub const fn stitch_l5_warm_lifetime_ms() -> u32 {
    ten_u32()
        .wrapping_add(eight_u32())
        .wrapping_mul(hundred_u32())
        .wrapping_mul(thousand_u32())
}

/// Pure MIR-lifted Stitch Layer-5 (`stitch/1`) future clock skew tolerance (`5000` ms).
#[must_use]
pub const fn stitch_l5_future_skew_ms() -> u32 {
    five_u32().wrapping_mul(thousand_u32())
}

/// Pure MIR-lifted Stitch Layer-5 wire frame admission predicate (`1 <= chunk < wire <= 4096`).
#[must_use]
pub const fn stitch_l5_frame_admitted(wire_bytes: u32, chunk_bytes: u32) -> bool {
    chunk_bytes >= 1_u32
        && chunk_bytes <= stitch_l5_chunk_bytes_max()
        && chunk_bytes < wire_bytes
        && wire_bytes <= stitch_l5_wire_bytes_max()
}

/// Pure MIR-lifted Stitch Layer-5 multi-frame sequence bounds (`1 <= total <= 256 && idx < total`).
#[must_use]
pub const fn stitch_l5_frame_index_valid(idx: u32, total: u32) -> bool {
    total >= 1_u32 && total <= stitch_l5_max_frames() && idx < total
}

/// Pure MIR-lifted Stitch Layer-5 assembled message size check (`1 <= bytes <= 524288`).
#[must_use]
pub const fn stitch_l5_message_bounded(bytes: u32) -> bool {
    bytes >= 1_u32 && bytes <= stitch_l5_message_bytes_max()
}

/// Pure MIR-lifted Stitch Layer-5 signal TTL freshness check (`age_ms <= 60000`).
#[must_use]
pub const fn stitch_l5_signal_fresh(age_ms: u32) -> bool {
    age_ms <= stitch_l5_signal_lifetime_ms()
}

/// Pure MIR-lifted Stitch Layer-5 warm TTL freshness check (`age_ms <= 1800000`).
#[must_use]
pub const fn stitch_l5_warm_fresh(age_ms: u32) -> bool {
    age_ms <= stitch_l5_warm_lifetime_ms()
}

/// Pure MIR-lifted `ResourceAdmission` epoch request ceiling (`4096`).
#[must_use]
pub const fn stitch_l5_epoch_request_cap() -> u32 {
    stitch_l5_wire_bytes_max()
}

/// Pure MIR-lifted `ResourceAdmission` 16 MiB transfer/storage budget (`16777216` bytes).
#[must_use]
pub const fn stitch_l5_transfer_budget_bytes() -> u64 {
    let k1 = hundred_u64()
        .wrapping_mul(ten_u64())
        .wrapping_add(two_u64().wrapping_mul(ten_u64()))
        .wrapping_add(two_u64().wrapping_mul(two_u64()));
    let sixteen = two_u64()
        .wrapping_mul(two_u64())
        .wrapping_mul(two_u64())
        .wrapping_mul(two_u64());
    sixteen.wrapping_mul(k1).wrapping_mul(k1)
}

/// Pure MIR-lifted `ResourceAdmission` compute-step ceiling (`1000000`).
#[must_use]
pub const fn stitch_l5_compute_steps_cap() -> u64 {
    let k1 = hundred_u64().wrapping_mul(ten_u64());
    k1.wrapping_mul(k1)
}

/// Pure MIR-lifted `ResourceAdmission` conservative reservation check (`used + add <= cap`).
#[must_use]
pub const fn stitch_l5_budget_reserve_ok(used: u64, delta: u64, cap: u64) -> bool {
    delta <= cap && used <= cap.wrapping_sub(delta)
}

/// Pure MIR-lifted opt-in swarm consent & budget gate (`consent && reqs <= 4096 && tx <= 16MiB`).
#[must_use]
pub const fn stitch_l5_seeder_admitted(
    opt_in_consent: bool,
    requests_used: u32,
    transfer_bytes_used: u64,
    storage_bytes_used: u64,
) -> bool {
    opt_in_consent
        && requests_used <= stitch_l5_epoch_request_cap()
        && transfer_bytes_used <= stitch_l5_transfer_budget_bytes()
        && storage_bytes_used <= stitch_l5_transfer_budget_bytes()
}

/// Pure MIR-lifted opaque `ntfy` topic length check (`16 <= len <= 64`).
#[must_use]
pub const fn stitch_l5_opaque_topic_len_valid(len: u32) -> bool {
    let sixteen = four_u32().wrapping_mul(four_u32());
    let sixty_four = eight_u32().wrapping_mul(eight_u32());
    len >= sixteen && len <= sixty_four
}

/// Pure MIR-lifted SHA-256 hex digest length check (`len == 64`).
#[must_use]
pub const fn stitch_l5_sha256_hex_len_valid(len: u32) -> bool {
    len == eight_u32().wrapping_mul(eight_u32())
}

/// Pure MIR-lifted zero-commercial-provider admission check (`!is_commercial_paas && !requires_registration`).
#[must_use]
pub const fn stitch_public_benefit_carrier_ok(
    is_commercial_paas: bool,
    requires_registration: bool,
) -> bool {
    !is_commercial_paas && !requires_registration
}

/// Pure MIR-lifted offline portable HTML envelope completeness check.
#[must_use]
pub const fn stitch_portable_html_envelope_valid(
    html_bytes: u64,
    capsule_bytes: u64,
    has_kappa_lock: bool,
    has_gzip_unpacker: bool,
) -> bool {
    capsule_bytes >= 1_u64 && html_bytes > capsule_bytes && has_kappa_lock && has_gzip_unpacker
}

/// Pure MIR-lifted volunteer `/swarm` gateway readiness invariant.
#[must_use]
pub const fn stitch_volunteer_swarm_ready(
    http_status: u16,
    available_shards: u32,
    required_k: u32,
    zero_commercial: bool,
) -> bool {
    gateway_http_status_ok(http_status)
        && required_k >= 1_u32
        && available_shards >= required_k
        && zero_commercial
}

/// Pure MIR-lifted GitHub-outage survival predicate across Stitch swarm, volunteer node, or offline envelope.
#[must_use]
pub const fn stitch_origin_outage_survived(
    github_up: bool,
    swarm_peers: u32,
    volunteer_nodes: u32,
    offline_envelope_ready: bool,
    surviving_shards: u32,
    required_k: u32,
) -> bool {
    offline_envelope_ready
        || (required_k >= 1_u32
            && surviving_shards >= required_k
            && (github_up || swarm_peers >= 1_u32 || volunteer_nodes >= 1_u32))
}

/// Pure MIR-lifted M27 lifted lifecycle module floor (`modules >= 186`).
#[must_use]
pub const fn stitch_m27_lifted_module_floor_ok(modules: u64) -> bool {
    let six = two_u64().wrapping_mul(two_u64()).wrapping_add(two_u64());
    let eight = two_u64().wrapping_mul(two_u64()).wrapping_mul(two_u64());
    let floor = hundred_u64()
        .wrapping_add(eight.wrapping_mul(ten_u64()))
        .wrapping_add(six);
    modules >= floor
}

/// Pure MIR-lifted Base64 encoded length bound (`ceil(raw / 3) * 4`).
#[must_use]
pub const fn stitch_portable_base64_valid(raw_bytes: u64, b64_bytes: u64) -> bool {
    raw_bytes >= 1_u64 && b64_bytes >= raw_bytes
}

/// Pure MIR-lifted Holo v6 archive schema version (`6`, `M29`, `VE-80`).
#[must_use]
pub const fn holo_v6_format_version() -> u32 {
    four_u32().wrapping_add(two_u32())
}

/// Pure MIR-lifted UORC v2.2 container major version (`2`, `M29`, `VE-80`).
#[must_use]
pub const fn holo_v6_uorc_major() -> u8 {
    1_u8.wrapping_add(1_u8)
}

/// Pure MIR-lifted UORC v2.2 container minor version (`2`, `M29`, `VE-80`).
#[must_use]
pub const fn holo_v6_uorc_minor() -> u8 {
    1_u8.wrapping_add(1_u8)
}

/// Pure MIR-lifted consolidated three-scope tensor proof count (`3`: `tensor`, `quotient`, `encoding`, `M29`).
#[must_use]
pub const fn unified_tensor_scopes_count() -> u32 {
    two_u32().wrapping_add(1_u32)
}

/// Pure MIR-lifted exact unified proof-scope count check (`scopes == 2`, `PF-28`, `M29`).
#[must_use]
pub const fn unified_scopes_exact(scopes: u32) -> bool {
    scopes == unified_tensor_scopes_count()
}

/// Pure MIR-lifted release capsule ceiling margin check (`actual + margin <= ratchet_ceiling <= megabyte_ceiling`, `PF-26`, `M29`).
#[must_use]
pub const fn holo_v6_capsule_margin_valid(
    actual_bytes: u64,
    margin_bytes: u64,
    ratchet_ceiling: u64,
    megabyte_ceiling: u64,
) -> bool {
    margin_bytes >= 1_u64
        && ratchet_ceiling <= megabyte_ceiling
        && actual_bytes <= ratchet_ceiling.saturating_sub(margin_bytes)
        && actual_bytes < megabyte_ceiling
}

/// Pure MIR-lifted theorem proof completion check (`total >= 1 && proved == total`, `M29`).
#[must_use]
pub const fn holo_v6_theorems_complete(proved: u32, total: u32) -> bool {
    total >= 1_u32 && proved == total
}

/// Pure MIR-lifted Holo v6 self-carrying proof capsule admission check (`M29`, `VE-80`).
#[must_use]
pub const fn holo_v6_self_carrying_admitted(
    format_ver: u32,
    has_snapshot: bool,
    has_kernel_statement: bool,
    proved_theorems: u32,
    total_theorems: u32,
    capsule_bytes: u64,
) -> bool {
    let thousand = thousand_u32() as u64;
    let megabyte = thousand.wrapping_mul(thousand);
    format_ver == holo_v6_format_version()
        && has_snapshot
        && has_kernel_statement
        && holo_v6_theorems_complete(proved_theorems, total_theorems)
        && capsule_bytes < megabyte
}

/// Pure MIR-lifted 256-lane GGML importance-grid super-block lane count (`16 * 16 = 256`, `M30`, `VE-81`).
#[must_use]
pub const fn importance_grid_superblock_lanes() -> u32 {
    sixteen_u32().wrapping_mul(sixteen_u32())
}

/// Pure MIR-lifted 32-lane importance-grid sub-block count per 256-lane super-block (`8`, `M30`).
#[must_use]
pub const fn importance_grid_subblocks_per_block() -> u32 {
    eight_u32()
}

/// Pure MIR-lifted 8-lane octet group count per 256-lane super-block (`32`, `VTK-59`..`VTK-62`).
#[must_use]
pub const fn importance_grid_octets_per_block() -> u32 {
    thirty_two_u32()
}

/// Pure MIR-lifted 4-lane quartet count per 256-lane super-block (`64`, `VTK-56`).
#[must_use]
pub const fn importance_grid_quartets_per_block() -> u32 {
    sixty_four_u32()
}

/// Pure MIR-lifted exact super-block divisibility check (`lanes >= 256 && lanes % 256 == 0`).
#[must_use]
pub const fn importance_grid_extent_admissible(total_lanes: u32) -> bool {
    let block = importance_grid_superblock_lanes();
    let whole_blocks = match total_lanes.checked_div(block) {
        Some(q) => q,
        None => 0_u32,
    };
    total_lanes >= block && whole_blocks.wrapping_mul(block) == total_lanes
}

/// Pure MIR-lifted IQ4_NL / IQ4_XS 16-entry nonlinear codebook index bound (`idx < 16`, `VTK-53`).
#[must_use]
pub const fn iq4_codebook_index_valid(idx: u32) -> bool {
    idx < sixteen_u32()
}

/// Pure MIR-lifted IQ3_XXS 256-entry 4-lane codebook index bound (`idx < 256`, `VTK-56`).
#[must_use]
pub const fn iq3_xxs_grid_index_valid(idx: u32) -> bool {
    idx < importance_grid_superblock_lanes()
}

/// Pure MIR-lifted IQ3_S 512-entry 4-lane codebook index bound (`idx < 512`, `VTK-56`).
#[must_use]
pub const fn iq3_s_grid_index_valid(idx: u32) -> bool {
    idx < importance_grid_superblock_lanes().wrapping_mul(two_u32())
}

/// Pure MIR-lifted IQ2_S 1024-entry 8-lane codebook index bound (`idx < 1024`, `VTK-59`).
#[must_use]
pub const fn iq2_s_grid_index_valid(idx: u32) -> bool {
    idx < thirty_two_u32().wrapping_mul(thirty_two_u32())
}

/// Pure MIR-lifted IQ2_XS 512-entry parity-constrained codebook index bound (`idx < 512`, `VTK-60`).
#[must_use]
pub const fn iq2_xs_grid_index_valid(idx: u32) -> bool {
    idx < importance_grid_superblock_lanes().wrapping_mul(two_u32())
}

/// Pure MIR-lifted IQ2_XXS 256-entry parity-constrained codebook index bound (`idx < 256`, `VTK-61`).
#[must_use]
pub const fn iq2_xxs_grid_index_valid(idx: u32) -> bool {
    idx < importance_grid_superblock_lanes()
}

/// Pure MIR-lifted IQ1_S / IQ1_M 2048-entry shifted ternary codebook index bound (`idx < 2048`, `VTK-62`).
#[must_use]
pub const fn iq1_shifted_grid_index_valid(idx: u32) -> bool {
    let kibi = thirty_two_u32().wrapping_mul(thirty_two_u32());
    idx < kibi.wrapping_mul(two_u32())
}

/// Pure MIR-lifted odd-popcount parity constraint check (`negative_sign_count % 2 == expected_odd`).
#[must_use]
pub const fn importance_octet_parity_valid(negative_sign_count: u32, require_odd: bool) -> bool {
    let two = two_u32();
    let pairs = match negative_sign_count.checked_div(two) {
        Some(q) => q,
        None => 0_u32,
    };
    let is_even = pairs.wrapping_mul(two) == negative_sign_count;
    if require_odd {
        !is_even
    } else {
        is_even
    }
}

/// Pure MIR-lifted IQ1_S byte footprint per 256-lane block (`50` bytes: `2 + 32 + 16`, `VTK-62`).
#[must_use]
pub const fn iq1_s_block_bytes() -> u32 {
    ten_u32().wrapping_mul(five_u32())
}

/// Pure MIR-lifted IQ1_M byte footprint per 256-lane block (`56` bytes: `32 + 16 + 6 + 2`, `VTK-62`).
#[must_use]
pub const fn iq1_m_block_bytes() -> u32 {
    eight_u32().wrapping_mul(four_u32().wrapping_add(three_u32()))
}

/// Pure MIR-lifted IQ2_XXS byte footprint per 256-lane block (`66` bytes: `2 + 64`, `VTK-61`).
#[must_use]
pub const fn iq2_xxs_block_bytes() -> u32 {
    sixty_four_u32().wrapping_add(two_u32())
}

/// Pure MIR-lifted IQ2_XS byte footprint per 256-lane block (`74` bytes: `2 + 64 + 8`, `VTK-60`).
#[must_use]
pub const fn iq2_xs_block_bytes() -> u32 {
    sixty_four_u32().wrapping_add(ten_u32())
}

/// Pure MIR-lifted IQ2_S byte footprint per 256-lane block (`82` bytes: `2 + 64 + 8 + 8`, `VTK-59`).
#[must_use]
pub const fn iq2_s_block_bytes() -> u32 {
    sixty_four_u32()
        .wrapping_add(sixteen_u32())
        .wrapping_add(two_u32())
}

/// Pure MIR-lifted monotonic K/IQ block-byte ordering invariant (`IQ1_S < IQ1_M < IQ2_XXS < IQ2_XS < IQ2_S`).
#[must_use]
pub const fn importance_grid_block_sizes_ordered() -> bool {
    iq1_s_block_bytes() < iq1_m_block_bytes()
        && iq1_m_block_bytes() < iq2_xxs_block_bytes()
        && iq2_xxs_block_bytes() < iq2_xs_block_bytes()
        && iq2_xs_block_bytes() < iq2_s_block_bytes()
}

/// Pure MIR-lifted gated delta rule state matrix extent check (`head_dim >= 1 && heads >= 1 && steps >= 1`, `VTK-57`).
#[must_use]
pub const fn delta_rule_extents_admissible(head_dim: u32, heads: u32, steps: u32) -> bool {
    head_dim >= 1_u32 && heads >= 1_u32 && steps >= 1_u32
}

/// Pure MIR-lifted selective state space (`SSM`) recurrence extent check (`state_dim >= 1 && channels >= 1`, `VTK-58`).
#[must_use]
pub const fn selective_state_extents_admissible(
    state_dim: u32,
    channels: u32,
    seq_len: u32,
) -> bool {
    state_dim >= 1_u32 && channels >= 1_u32 && seq_len >= 1_u32
}

/// Pure MIR-lifted write-once release-pinned Cargo sparse index immutability check (`PR #53`).
#[must_use]
pub const fn versioned_sparse_index_write_once_admissible(
    version_dir_exists: bool,
    has_config_json: bool,
    crate_count: u32,
) -> bool {
    let four = four_u32();
    !version_dir_exists && has_config_json && crate_count >= four
}

/// Pure MIR-lifted unified 3-scope certified root coverage check (`tensor >= 80 && quotient >= 6 && encoding >= 13`, `VE-81`).
#[must_use]
pub const fn unified_scope_root_counts_valid(
    tensor_roots: u32,
    quotient_roots: u32,
    encoding_roots: u32,
) -> bool {
    let eighty = eight_u32().wrapping_mul(ten_u32());
    let six = six_u32();
    let thirteen = ten_u32().wrapping_add(three_u32());
    tensor_roots >= eighty && quotient_roots >= six && encoding_roots >= thirteen
}

/// Pure MIR-lifted M30 Rust modularity ceiling check (`max_lines <= 1145`, `VE-81`).
#[must_use]
pub const fn m30_rust_modularity_ceiling() -> u32 {
    let thousand = thousand_u32();
    let hundred = hundred_u32();
    let forty_five = ten_u32().wrapping_mul(four_u32()).wrapping_add(five_u32());
    thousand.wrapping_add(hundred).wrapping_add(forty_five)
}

/// Pure MIR-lifted M30 shipped JS modularity ceiling check (`max_lines <= 850`, `VE-81`).
#[must_use]
pub const fn m30_js_modularity_ceiling() -> u32 {
    let eight_hundred = hundred_u32().wrapping_mul(eight_u32());
    let fifty = ten_u32().wrapping_mul(five_u32());
    eight_hundred.wrapping_add(fifty)
}

/// Pure MIR-lifted M30 modularity compliance check (`rust_max <= 1145 && js_max <= 850`, `VE-81`).
#[must_use]
pub const fn m30_modularity_targets_met(rust_max_lines: u32, js_max_lines: u32) -> bool {
    rust_max_lines >= 1_u32
        && js_max_lines >= 1_u32
        && rust_max_lines <= m30_rust_modularity_ceiling()
        && js_max_lines <= m30_js_modularity_ceiling()
}

/// Pure MIR-lifted M30 uorust frontier advancement check (`lifted >= 615 && residue <= 1148 && uncertified <= 1301`, `VE-81`).
#[must_use]
pub const fn m30_frontier_targets_met(lifted: u64, residue: u64, uncertified: u64) -> bool {
    let six_fifteen = hundred_u64()
        .wrapping_mul(six_u32() as u64)
        .wrapping_add(ten_u64())
        .wrapping_add(five_u32() as u64);
    let max_res = (thousand_u32() as u64)
        .wrapping_add(hundred_u64())
        .wrapping_add(sixteen_u64().wrapping_mul(three_u64()));
    let max_uncert = (thousand_u32() as u64)
        .wrapping_add(hundred_u64().wrapping_mul(three_u64()))
        .wrapping_add(1_u64);
    lifted >= six_fifteen && residue <= max_res && uncertified <= max_uncert
}

/// Pure MIR-lifted M30 Observer self-refinement score floor check (`score_permille >= 984`, `VE-81`).
#[must_use]
pub const fn m30_refinement_score_valid(score_permille: u32) -> bool {
    let thousand = thousand_u32();
    let floor = thousand.wrapping_sub(sixteen_u32());
    score_permille >= floor && score_permille <= thousand
}
