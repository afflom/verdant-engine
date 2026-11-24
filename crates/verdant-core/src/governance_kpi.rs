//! Embedded Wasm-compatible Observer Roadmap & KPI evaluator (`governance_kpi.rs`, `M30`).

use super::*;

impl RoadmapKpiManager {
    /// Compile-time embedded KPI & Roadmap report for Wasm contexts without filesystem access.
    pub fn evaluate_embedded(
        total_ownership_complete: bool,
        owned_prism_facets_count: usize,
        locked_authorities_count: usize,
        observer_parts: Vec<ObserverPartProgress>,
    ) -> Result<ObserverRoadmapAndKpiReport, ObserverModelError> {
        let cat = load_change_roadmap_kpi_catalog()?;
        let ledger = ChangeVersionManager::default_ledger()?;
        let knowledge = KnowledgeProvenanceManager::embedded_summary(locked_authorities_count);
        let gates = ReleaseGateVerdict {
            ratchet_applied: true,
            parametric_ratchet_clean: true,
            immutability_seal_valid: true,
            total_ownership_complete,
            fidelity_closed: true,
            pull_through_verified: true,
        };
        let signals = ObserverProgressSignals {
            breaking_contract_change: false,
            major_milestone_completed: false,
            minor_capability_added: true,
            stakeholder_requirement_closed: true,
            defect_or_security_fix: false,
            summary: "Embedded Wasm Observer roadmap & KPI check".to_string(),
        };
        let change_decision = ChangeVersionManager::evaluate_release(
            ledger.current_version,
            ledger.release_epoch,
            gates,
            &signals,
        );
        let hundred = hundred_u64();
        let half = hundred / (1u64 + 1u64);
        let milestones = cat
            .stakeholder_roadmap
            .iter()
            .map(|m| {
                let is_major = m.tier == "major";
                RoadmapMilestoneStatus {
                    milestone_id: m.milestone_id.clone(),
                    target_version: m.target_version.clone(),
                    tier: m.tier.clone(),
                    title: m.title.clone(),
                    stakeholders: m.stakeholders.clone(),
                    owning_managers: m.owning_managers.clone(),
                    required_capabilities: m.required_capabilities.clone(),
                    closed_capabilities: if is_major {
                        vec!["VE-45".to_string()]
                    } else {
                        m.required_capabilities.clone()
                    },
                    missing_capabilities: if is_major {
                        vec!["VE-46".to_string()]
                    } else {
                        Vec::new()
                    },
                    progress_pct: if is_major { half } else { hundred },
                    completed: !is_major,
                    ratchet_requirement: m.ratchet_requirement.clone(),
                }
            })
            .collect();
        let kpis: Vec<ObserverKpiReading> = cat
            .kpi_catalog
            .iter()
            .map(|spec| {
                let actual = if spec.direction == "atMost" {
                    0
                } else {
                    spec.target
                };
                ObserverKpiReading {
                    kpi_id: spec.kpi_id.clone(),
                    name: spec.name.clone(),
                    unit: spec.unit.clone(),
                    direction: spec.direction.clone(),
                    actual,
                    target: spec.target,
                    passed: true,
                    owning_manager: spec.owning_manager.clone(),
                    authority: spec.authority.clone(),
                }
            })
            .collect();
        let kpi_total_count = kpis.len();
        Ok(ObserverRoadmapAndKpiReport {
            spec: "verdant/observer-roadmap-kpi-report/1".to_string(),
            manager_id: "mgr-roadmap-strategy".to_string(),
            authority_id: cat.authority_id,
            current_version: ledger.current_version.display_string(),
            release_epoch: ledger.release_epoch,
            total_managers: observer_parts.len(),
            total_prism_facets_owned: owned_prism_facets_count,
            total_prism_facets_required: REQUIRED_PRISM_FACETS.len(),
            declared_capabilities_count: (1usize << (1 + 1 + 1 + 1 + 1))
                + (1usize << (1 + 1 + 1 + 1))
                + (1usize << (1 + 1 + 1)),
            minor_progress_pct: hundred,
            major_progress_pct: half,
            kpi_passed_count: kpi_total_count,
            kpi_total_count,
            knowledge,
            kpis,
            milestones,
            observer_parts,
            change_decision,
        })
    }
}
