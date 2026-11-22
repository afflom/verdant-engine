//! Pure `{0, 1}` MIR-lifted multi-gateway Stitch hosting, failover, HTTP readiness, and
//! stakeholder part routing predicates (`M25`, `VE-77`), included by `scalars.rs`.

use super::{
    eight_u32, five_u32, four_u32, hundred_u32, hundred_u64, six_u32, ten_u32, ten_u64,
    thousand_u32, three_u32, two_u32, two_u64,
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

/// Pure MIR-lifted multi-gateway deployment adapter completeness (`adapters >= 4`: GitHub, Cloudflare, Netlify, IPFS).
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
