//! Unified SHA-256-verified catalog bundle registry (`standards_registry.rs`).

use super::*;

/// Cataloged Khronos glTF 2.0 HD material record in `catalogs/khronos/gltf-hd-materials-rigs.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogHdMaterial {
    /// Material ID (`MAT-*`).
    pub id: String,
    /// Material name.
    pub name: String,
    /// Schema.org class (`Product`).
    #[serde(rename = "schemaType")]
    pub schema_type: String,
    /// Base RGBA factor.
    #[serde(rename = "baseColorFactor")]
    pub base_color_factor: [f32; DIM_HOMOGENEOUS],
    /// Metallic factor.
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: f32,
    /// Perceptual roughness factor.
    #[serde(rename = "roughnessFactor")]
    pub roughness_factor: f32,
    /// Linear RGB emissive factor.
    #[serde(rename = "emissiveFactor")]
    pub emissive_factor: [f32; DIM_SPATIAL],
    /// `KHR_materials_emissive_strength`.
    #[serde(rename = "emissiveStrength")]
    pub emissive_strength: f32,
    /// `KHR_materials_ior`.
    pub ior: f32,
    /// `KHR_materials_specular`.
    #[serde(rename = "specularFactor")]
    pub specular_factor: f32,
    /// `KHR_materials_transmission`.
    #[serde(rename = "transmissionFactor")]
    pub transmission_factor: f32,
    /// `KHR_materials_clearcoat`.
    #[serde(rename = "clearcoatFactor")]
    pub clearcoat_factor: f32,
    /// `KHR_materials_clearcoat` roughness.
    #[serde(rename = "clearcoatRoughnessFactor")]
    pub clearcoat_roughness_factor: f32,
    /// Double-sided flag.
    #[serde(rename = "doubleSided")]
    pub double_sided: bool,
}

impl CatalogHdMaterial {
    /// Converts this catalog entry to an engine [`HdPbrDescriptor`].
    #[must_use]
    pub fn to_hd_descriptor(&self) -> HdPbrDescriptor {
        HdPbrDescriptor {
            id: self.id.clone(),
            name: self.name.clone(),
            base_color_factor: self.base_color_factor,
            metallic_factor: self.metallic_factor,
            roughness_factor: self.roughness_factor,
            emissive_factor: self.emissive_factor,
            emissive_strength: self.emissive_strength,
            ior: self.ior,
            specular_factor: self.specular_factor,
            transmission_factor: self.transmission_factor,
            clearcoat_factor: self.clearcoat_factor,
            clearcoat_roughness_factor: self.clearcoat_roughness_factor,
            double_sided: self.double_sided,
        }
    }
}

/// Anthropometric 22-joint humanoid rig profile in `catalogs/khronos/gltf-hd-materials-rigs.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogHumanoidRig {
    /// Rig ID (`RIG-HUMANOID-22-*`).
    pub id: String,
    /// Required joint count (`22`).
    #[serde(rename = "jointCount")]
    pub joint_count: usize,
    /// Stature scale in permille (`1000 == 1.0x`).
    #[serde(rename = "heightScalePermille")]
    pub height_scale_permille: u64,
    /// Clavicle/shoulder breadth scale in permille (`1000 == 1.0x`).
    #[serde(rename = "shoulderScalePermille")]
    pub shoulder_scale_permille: u64,
    /// Gait cadence in milli-Hz.
    #[serde(rename = "strideCadenceMilliHz")]
    pub stride_cadence_milli_hz: u64,
    /// Distinctive head/torso attachment identifier.
    #[serde(rename = "crestAttachment")]
    pub crest_attachment: String,
}

/// Verified Khronos glTF 2.0 HD Materials & Humanoid Rigs catalog (`AUTH-KHRONOS-GLTF2`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KhronosHdCatalog {
    /// Spec header.
    pub spec: String,
    /// Authority ID (`AUTH-KHRONOS-GLTF2`).
    #[serde(rename = "authorityId")]
    pub authority_id: String,
    /// Ratified Khronos glTF 2.0 extension names.
    pub extensions: Vec<String>,
    /// High-definition optical PBR materials.
    pub materials: Vec<CatalogHdMaterial>,
    /// Anthropometric 22-joint humanoid rigs.
    pub rigs: Vec<CatalogHumanoidRig>,
}

/// Physical & acoustic environment profile of a cataloged domain pack.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogEnvironmentSpec {
    /// Environment name.
    pub name: String,
    /// Schema.org place class (`BodyOfWater`, `Landform`, `CivicStructure`).
    #[serde(rename = "schemaType")]
    pub schema_type: String,
    /// Water level in millimeters (`-950` -> `-0.95m`).
    #[serde(rename = "waterLevelMetersMilli")]
    pub water_level_meters_milli: i64,
    /// Terrain ridge amplitude scale in permille.
    #[serde(rename = "ridgeAmplitudePermille")]
    pub ridge_amplitude_permille: u64,
    /// Basin depth scale in permille.
    #[serde(rename = "basinDepthPermille")]
    pub basin_depth_permille: u64,
    /// Sky zenith RGB in percent (`0..=100`).
    #[serde(rename = "skyRgbPercent")]
    pub sky_rgb_percent: [u8; DIM_SPATIAL],
    /// Horizon fog RGB in percent (`0..=100`).
    #[serde(rename = "fogRgbPercent")]
    pub fog_rgb_percent: [u8; DIM_SPATIAL],
    /// Sabine enclosure volume in `m^3`.
    #[serde(rename = "acousticVolumeM3")]
    pub acoustic_volume_m3: u64,
    /// Equivalent acoustic absorption area in milli-`m^2`.
    #[serde(rename = "acousticAbsorptionMilliM2")]
    pub acoustic_absorption_milli_m2: u64,
}

/// Domain-specific architectural/scientific/arena 3D model placement in a domain pack.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogModelPlacement {
    /// Model ID (`MDL-*`).
    pub id: String,
    /// Entity/mesh name.
    pub name: String,
    /// Parametric generator kind (`"fluted_column"` or `"faceted_prism"`).
    pub primitive: String,
    /// Bound `MAT-*` identifier from `AUTH-KHRONOS-GLTF2`.
    #[serde(rename = "materialId")]
    pub material_id: String,
    /// `[sx, sy, sz]` scale in permille (`1000 == 1.0`).
    #[serde(rename = "scalePermille")]
    pub scale_permille: [u64; DIM_SPATIAL],
    /// `[dx, dy, dz]` offset in decimeters (`10 == 1.0m`).
    #[serde(rename = "offsetDecimeters")]
    pub offset_decimeters: [i64; DIM_SPATIAL],
}

/// Conserved item specification in a domain pack.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogItemSpec {
    /// Item ID (`ITEM-*`).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Material ID (`MAT-*`).
    #[serde(rename = "materialId")]
    pub material_id: String,
    /// Conserved initial supply minted into the domain's `ConservationLedger`.
    #[serde(rename = "initialSupply")]
    pub initial_supply: u64,
}

/// Differentiated NPC/agent character profile in a domain pack.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogCharacterProfile {
    /// Character ID (`CHAR-*`).
    pub id: String,
    /// Character name (`Captain Soren`, `Astronomer Lyra`, `Archivist Rowan`, `Marshal Vesper`).
    pub name: String,
    /// Occupational role (`Schema.org` `jobTitle`).
    pub role: String,
    /// Anthropometric rig ID (`RIG-HUMANOID-22-*`).
    #[serde(rename = "rigId")]
    pub rig_id: String,
    /// Primary material ID (`MAT-*`).
    #[serde(rename = "materialId")]
    pub material_id: String,
    /// Planar/vertical spawn coordinates in decimeters.
    #[serde(rename = "spawnDecimeters")]
    pub spawn_decimeters: [i64; DIM_SPATIAL],
    /// Patrol radius in decimeters on the walkable `NavMesh`.
    #[serde(rename = "patrolRadiusDecimeters")]
    pub patrol_radius_decimeters: u64,
    /// Domain-specific actionable NPC directives.
    pub directives: Vec<String>,
    /// Grounded factual passages for epistemic Q&A (outside facts abstain with `"Unknown"`).
    #[serde(rename = "knowledgeFacts")]
    pub knowledge_facts: Vec<String>,
}

/// Complete imported multi-domain world pack (`DOM-*`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainPackSpec {
    /// Domain ID (`DOM-*`).
    pub id: String,
    /// Starter/URL slug (`sunspire-regatta`, `meridian-observatory`, `glasshouse-chronicle`, `starfall-bastion`).
    pub slug: String,
    /// Human-readable title.
    pub title: String,
    /// Schema.org root class (`VideoGame` or `SoftwareApplication`).
    #[serde(rename = "schemaType")]
    pub schema_type: String,
    /// Deterministic world seed.
    pub seed: u64,
    /// Default progressive LOD rank (`0..=4`).
    #[serde(rename = "defaultLodRank")]
    pub default_lod_rank: u64,
    /// Default lawful camera mode (`"first"`, `"third"`, `"orbit"`).
    #[serde(rename = "cameraMode")]
    pub camera_mode: String,
    /// Physical & acoustic environment specification.
    pub environment: CatalogEnvironmentSpec,
    /// Domain-specific 3D landmark models.
    pub models: Vec<CatalogModelPlacement>,
    /// Conserved world items.
    pub items: Vec<CatalogItemSpec>,
    /// Differentiated character/agent profiles.
    pub characters: Vec<CatalogCharacterProfile>,
}

/// Verified Multi-Domain World, Character, Item & Automation catalog (`AUTH-DOMAIN-PACKS`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainPacksCatalog {
    /// Spec header.
    pub spec: String,
    /// Authority ID (`AUTH-DOMAIN-PACKS`).
    #[serde(rename = "authorityId")]
    pub authority_id: String,
    /// Differentiated domain packs.
    pub domains: Vec<DomainPackSpec>,
}

/// Live Verdant Engine instantiation of a cataloged [`DomainPackSpec`], complete with differentiated
/// HD 3D geometry, Khronos PBR descriptors, articulated characters, conserved item ledger, and
/// oracle-verified Schema.org JSON-LD + W3C SOSA/PROV-O provenance.
#[derive(Clone, Debug)]
pub struct InstantiatedDomainPack {
    /// Source catalog specification.
    pub spec: DomainPackSpec,
    /// Synthesized HD 3D sector (`SceneWorld` + landmark meshes).
    pub sector: HdSectorBundle,
    /// Domain-adjusted physical environment.
    pub physical_environment: PhysicalEnvironment,
    /// Domain-adjusted terrain relief.
    pub terrain_relief: TerrainReliefParams,
    /// Domain-adjusted ISO 3382-1 Sabine acoustic enclosure.
    pub acoustic_environment: AcousticEnvironment,
    /// Ordered Khronos HD PBR descriptors bound to `sector.scene` materials.
    pub hd_materials: Vec<HdPbrDescriptor>,
    /// Differentiated 22-joint characters spawned onto the domain's walkable terrain.
    pub characters: Vec<ArticulatedCharacter>,
    /// Exact integer `ConservationLedger` initialized with the domain's conserved items.
    pub item_ledger: ConservationLedger,
    /// Coupled Schema.org `@graph` JSON-LD document verified by `SchemaOrgVocabulary`.
    pub schema_org_jsonld: serde_json::Value,
    /// W3C SOSA & PROV-O Turtle provenance graph verified by `SosaProvoShapes`.
    pub sosa_provo_turtle: String,
}

/// Unified SHA-256-verified registry of all external standard oracles and imported catalogs.
#[derive(Clone, Debug)]
pub struct CatalogBundleRegistry {
    /// Verified `AUTH-*` file SHA-256 locks.
    pub file_locks: Vec<AuthorityFileLock>,
    /// Schema.org v28.1 vocabulary oracle.
    pub schema_org: SchemaOrgVocabulary,
    /// W3C SOSA/SSN & W3C PROV-O shape oracle.
    pub sosa_provo: SosaProvoShapes,
    /// Khronos glTF 2.0 HD materials & 22-joint humanoid rigs catalog.
    pub khronos_hd: KhronosHdCatalog,
    /// ISO 24617-2 dialogue acts, OMG DMN 1.5 admission, & runtime exclusivity catalog.
    pub language_dmn: LanguageDialogueCatalog,
    /// Multi-domain world, character, item, & automation packs.
    pub domain_packs: DomainPacksCatalog,
}

impl CatalogBundleRegistry {
    // Conformance corpora are verified by the model and the tensor tests.
    // Runtime catalogs must still supply bytes for every other file lock.
    fn catalog_locks(authorities: &str) -> Vec<AuthorityFileLock> {
        parse_authority_file_locks(authorities)
            .into_iter()
            .filter(|lock| !lock.path.starts_with("tests/oracles/"))
            .collect()
    }

    /// Loads and cross-validates the embedded compile-time catalogs against `model/authorities.toml`
    /// SHA-256 locks (usable inside pure `wasm32-unknown-unknown` with zero filesystem I/O).
    pub fn embedded() -> Result<Self, OracleValidationError> {
        let mut raw_files = BTreeMap::new();
        raw_files.insert(
            "catalogs/schema-org/vocabulary.json",
            EMBEDDED_SCHEMA_ORG_JSON.as_bytes(),
        );
        raw_files.insert(
            "catalogs/w3c/sosa-provo-shapes.json",
            EMBEDDED_SOSA_PROVO_JSON.as_bytes(),
        );
        raw_files.insert(
            "catalogs/khronos/gltf-hd-materials-rigs.json",
            EMBEDDED_KHRONOS_JSON.as_bytes(),
        );
        raw_files.insert(
            "catalogs/language/iso24617-dmn-dialogue.json",
            EMBEDDED_ISO_DMN_JSON.as_bytes(),
        );
        raw_files.insert(
            "catalogs/domains/world-packs.json",
            EMBEDDED_DOMAIN_PACKS_JSON.as_bytes(),
        );
        #[cfg(not(target_arch = "wasm32"))]
        {
            raw_files.insert(
                "catalogs/governance/vsm-viable-system.json",
                crate::operator::LOCKED_VSM_CATALOG_JSON.as_bytes(),
            );
            raw_files.insert(
                "catalogs/governance/iso42010-viewpoints.json",
                crate::operator::LOCKED_ISO42010_VIEWPOINTS_JSON.as_bytes(),
            );
            raw_files.insert(
                "catalogs/governance/iso25010-tensor-scaling.json",
                crate::operator::LOCKED_ISO25010_TENSOR_SCALING_JSON.as_bytes(),
            );
            raw_files.insert(
                "catalogs/governance/stitch-self-hosting.json",
                crate::operator::LOCKED_STITCH_SELF_HOSTING_JSON.as_bytes(),
            );
            raw_files.insert(
                "catalogs/governance/change-roadmap-kpi.json",
                crate::governance::LOCKED_CHANGE_ROADMAP_KPI_JSON.as_bytes(),
            );
        }
        Self::from_locked_bytes(EMBEDDED_AUTHORITIES_TOML, &raw_files)
    }

    /// Reads `model/authorities.toml` and every committed `catalogs/**` file directly from
    /// `repo_dir`, verifying SHA-256 byte parity and cross-catalog referential integrity.
    pub fn load_from_repo_dir(repo_dir: &Path) -> Result<Self, OracleValidationError> {
        let auth_path = repo_dir.join("model").join("authorities.toml");
        let auth_toml = std::fs::read_to_string(&auth_path).map_err(|err| {
            OracleValidationError::CatalogIntegrityMismatch {
                authority_id: "model/authorities.toml".to_string(),
                detail: err.to_string(),
            }
        })?;
        let locks = Self::catalog_locks(&auth_toml);
        let mut owned_bytes: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for lock in &locks {
            let full_path = repo_dir.join(&lock.path);
            let bytes = std::fs::read(&full_path).map_err(|err| {
                OracleValidationError::CatalogIntegrityMismatch {
                    authority_id: lock.id.clone(),
                    detail: format!("failed to read `{}`: {err}", lock.path),
                }
            })?;
            owned_bytes.insert(lock.path.clone(), bytes);
        }
        let borrowed: BTreeMap<&str, &[u8]> = owned_bytes
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_slice()))
            .collect();
        Self::from_locked_bytes(&auth_toml, &borrowed)
    }

    /// Verifies SHA-256 digests against `authorities_toml` and cross-validates all five catalogs.
    pub fn from_locked_bytes(
        authorities_toml: &str,
        files_by_rel_path: &BTreeMap<&str, &[u8]>,
    ) -> Result<Self, OracleValidationError> {
        let file_locks = Self::catalog_locks(authorities_toml);
        let min_locks = (four_u32() as usize) + 1usize;
        if file_locks.len() < min_locks {
            return Err(OracleValidationError::CatalogIntegrityMismatch {
                authority_id: "model/authorities.toml".to_string(),
                detail: format!(
                    "expected at least {min_locks} file-locked authorities, found {}",
                    file_locks.len()
                ),
            });
        }

        for lock in &file_locks {
            #[cfg(target_arch = "wasm32")]
            if lock.path.starts_with("catalogs/governance/")
                && !files_by_rel_path.contains_key(lock.path.as_str())
            {
                continue;
            }
            let Some(&bytes) = files_by_rel_path.get(lock.path.as_str()) else {
                return Err(OracleValidationError::CatalogIntegrityMismatch {
                    authority_id: lock.id.clone(),
                    detail: format!("missing locked catalog bytes for `{}`", lock.path),
                });
            };
            let observed = sha256_hex(bytes);
            if observed != lock.sha256 {
                return Err(OracleValidationError::CatalogIntegrityMismatch {
                    authority_id: lock.id.clone(),
                    detail: format!(
                        "SHA-256 mismatch on `{}`: expected {}, observed {observed}",
                        lock.path, lock.sha256
                    ),
                });
            }
        }

        let parse_slice = |auth_id: &str, rel_path: &str| -> Result<&[u8], OracleValidationError> {
            files_by_rel_path.get(rel_path).copied().ok_or_else(|| {
                OracleValidationError::CatalogIntegrityMismatch {
                    authority_id: auth_id.to_string(),
                    detail: format!("missing `{rel_path}`"),
                }
            })
        };

        let schema_org: SchemaOrgVocabulary = serde_json::from_slice(parse_slice(
            "AUTH-SCHEMA-ORG",
            "catalogs/schema-org/vocabulary.json",
        )?)
        .map_err(|err| OracleValidationError::CatalogIntegrityMismatch {
            authority_id: "AUTH-SCHEMA-ORG".to_string(),
            detail: err.to_string(),
        })?;

        let sosa_provo: SosaProvoShapes = serde_json::from_slice(parse_slice(
            "AUTH-W3C-SOSA-PROVO",
            "catalogs/w3c/sosa-provo-shapes.json",
        )?)
        .map_err(|err| OracleValidationError::CatalogIntegrityMismatch {
            authority_id: "AUTH-W3C-SOSA-PROVO".to_string(),
            detail: err.to_string(),
        })?;

        let khronos_hd: KhronosHdCatalog = serde_json::from_slice(parse_slice(
            "AUTH-KHRONOS-GLTF2",
            "catalogs/khronos/gltf-hd-materials-rigs.json",
        )?)
        .map_err(|err| OracleValidationError::CatalogIntegrityMismatch {
            authority_id: "AUTH-KHRONOS-GLTF2".to_string(),
            detail: err.to_string(),
        })?;

        let language_dmn: LanguageDialogueCatalog = serde_json::from_slice(parse_slice(
            "AUTH-ISO-24617-DMN",
            "catalogs/language/iso24617-dmn-dialogue.json",
        )?)
        .map_err(|err| OracleValidationError::CatalogIntegrityMismatch {
            authority_id: "AUTH-ISO-24617-DMN".to_string(),
            detail: err.to_string(),
        })?;

        let domain_packs: DomainPacksCatalog = serde_json::from_slice(parse_slice(
            "AUTH-DOMAIN-PACKS",
            "catalogs/domains/world-packs.json",
        )?)
        .map_err(|err| OracleValidationError::CatalogIntegrityMismatch {
            authority_id: "AUTH-DOMAIN-PACKS".to_string(),
            detail: err.to_string(),
        })?;

        // Cross-check referential integrity across all 5 authorities:
        let expected_joints = crate::prism_kernel::humanoidJointCount().unwrap_or(1) as usize;
        let mat_ids: BTreeSet<&str> = khronos_hd.materials.iter().map(|m| m.id.as_str()).collect();
        let rig_ids: BTreeSet<&str> = khronos_hd.rigs.iter().map(|r| r.id.as_str()).collect();

        for rig in &khronos_hd.rigs {
            if rig.joint_count != expected_joints {
                return Err(OracleValidationError::CatalogIntegrityMismatch {
                    authority_id: "AUTH-KHRONOS-GLTF2".to_string(),
                    detail: format!(
                        "rig `{}` declares {} joints, expected {expected_joints}",
                        rig.id, rig.joint_count
                    ),
                });
            }
        }

        let mut seen_seeds = BTreeSet::new();
        for dom in &domain_packs.domains {
            if !seen_seeds.insert(dom.seed) {
                return Err(OracleValidationError::CatalogIntegrityMismatch {
                    authority_id: "AUTH-DOMAIN-PACKS".to_string(),
                    detail: format!("duplicate world seed {} in domain `{}`", dom.seed, dom.id),
                });
            }
            if !schema_org.classes.contains_key(&dom.schema_type)
                || !schema_org
                    .classes
                    .contains_key(&dom.environment.schema_type)
            {
                return Err(OracleValidationError::CatalogIntegrityMismatch {
                    authority_id: "AUTH-DOMAIN-PACKS".to_string(),
                    detail: format!("domain `{}` references unknown Schema.org class", dom.id),
                });
            }
            for mdl in &dom.models {
                if !mat_ids.contains(mdl.material_id.as_str()) {
                    return Err(OracleValidationError::CatalogIntegrityMismatch {
                        authority_id: "AUTH-DOMAIN-PACKS".to_string(),
                        detail: format!(
                            "model `{}` references unknown Khronos material `{}`",
                            mdl.id, mdl.material_id
                        ),
                    });
                }
            }
            for item in &dom.items {
                if !mat_ids.contains(item.material_id.as_str()) || item.initial_supply == 0 {
                    return Err(OracleValidationError::CatalogIntegrityMismatch {
                        authority_id: "AUTH-DOMAIN-PACKS".to_string(),
                        detail: format!("invalid item `{}` in domain `{}`", item.id, dom.id),
                    });
                }
            }
            for ch in &dom.characters {
                if !rig_ids.contains(ch.rig_id.as_str())
                    || !mat_ids.contains(ch.material_id.as_str())
                    || ch.directives.is_empty()
                    || ch.knowledge_facts.is_empty()
                {
                    return Err(OracleValidationError::CatalogIntegrityMismatch {
                        authority_id: "AUTH-DOMAIN-PACKS".to_string(),
                        detail: format!("invalid character `{}` in domain `{}`", ch.id, dom.id),
                    });
                }
            }
        }

        Ok(Self {
            file_locks,
            schema_org,
            sosa_provo,
            khronos_hd,
            language_dmn,
            domain_packs,
        })
    }

    /// Instantiates a differentiated High-Definition world, Khronos PBR palette, 22-joint NPC cast,
    /// conserved item ledger, Schema.org `@graph`, and W3C SOSA/PROV-O provenance from `domain_id_or_slug`.
    pub fn instantiate_domain_pack(
        &self,
        domain_id_or_slug: &str,
        lod_override: Option<u64>,
    ) -> Result<InstantiatedDomainPack, OracleValidationError> {
        let Some(dom) = self
            .domain_packs
            .domains
            .iter()
            .find(|d| d.id == domain_id_or_slug || d.slug == domain_id_or_slug)
        else {
            return Err(OracleValidationError::CatalogIntegrityMismatch {
                authority_id: "AUTH-DOMAIN-PACKS".to_string(),
                detail: format!("unknown domain pack `{domain_id_or_slug}`"),
            });
        };

        let max_hd_lod = u64::from(four_u32());
        let lod_rank = lod_override.unwrap_or(dom.default_lod_rank).min(max_hd_lod);
        let milli = pow10_u64(three_u32()) as f64;
        let deci = pow10_u64(1) as f64;
        let i2 = 1usize + 1usize;

        let mut physical_environment = PhysicalEnvironment::temperate_terrestrial();
        physical_environment.water_level_meters =
            (dom.environment.water_level_meters_milli as f64) / milli;

        let mut terrain_relief = TerrainReliefParams::standard_basin_valley();
        terrain_relief.hill_amplitude_meters *=
            (dom.environment.ridge_amplitude_permille as f64) / milli;
        terrain_relief.basin_depth_meters *= (dom.environment.basin_depth_permille as f64) / milli;

        let base_ac = AcousticEnvironment::reflective_chamber(physical_environment.sound_speed_mps);
        let eq_abs_m2 = ((dom.environment.acoustic_absorption_milli_m2 as f64) / milli).max(1.0);
        let four_f = f64::from(four_u32());
        let acoustic_environment = AcousticEnvironment {
            sound_speed_mps: physical_environment.sound_speed_mps,
            listener_head_radius_meters: base_ac.listener_head_radius_meters,
            enclosure_volume_m3: dom.environment.acoustic_volume_m3 as f64,
            boundary_surface_area_m2: eq_abs_m2 * four_f,
            mean_absorption_coeff: 1.0 / four_f,
        };

        let base_canon = canonical_pbr_material_library();
        let base_offset = base_canon.len();
        let mut hd_materials: Vec<HdPbrDescriptor> = base_canon
            .into_iter()
            .enumerate()
            .map(|(idx, m)| HdPbrDescriptor {
                id: format!("MAT-CANONICAL-{idx}"),
                name: m.name,
                base_color_factor: m.base_color_factor,
                metallic_factor: m.metallic_factor,
                roughness_factor: m.roughness_factor,
                emissive_factor: m.emissive_factor,
                emissive_strength: 1.0,
                ior: m.ior,
                specular_factor: half_f64() as f32,
                transmission_factor: if m.base_color_factor[DIM_SPATIAL] < 1.0 {
                    half_f64() as f32
                } else {
                    0.0
                },
                clearcoat_factor: 0.0,
                clearcoat_roughness_factor: 0.0,
                double_sided: m.double_sided,
            })
            .collect();
        for cat_mat in &self.khronos_hd.materials {
            hd_materials.push(cat_mat.to_hd_descriptor());
        }
        let mat_index_by_id: BTreeMap<&str, usize> = self
            .khronos_hd
            .materials
            .iter()
            .enumerate()
            .map(|(idx, m)| (m.id.as_str(), base_offset + idx))
            .collect();

        let arch_kind = WorldArchitectureKind::from_domain_slug(&dom.slug);
        let mut sector = synthesize_hd_world_sector_for_kind(
            dom.seed,
            &terrain_relief,
            physical_environment.water_level_meters,
            lod_rank,
            arch_kind,
        );

        // Spawn cataloged domain-specific HD landmark models onto the terrain:
        let half = half_f64();
        let col_segs = (four_u32() * four_u32()) as usize;
        let col_h_segs = (four_u32() * two_u32()) as usize;
        for mdl in &dom.models {
            let mut mesh: ProceduralMesh = if mdl.primitive == "fluted_column" {
                let mut m = generate_fluted_column_mesh(
                    half + half * half,
                    half,
                    1.0 + 1.0 + 1.0,
                    col_segs,
                    col_segs,
                    col_h_segs,
                );
                m.name = mdl.name.clone();
                m
            } else {
                let mut m = generate_faceted_prism_mesh(1.0, 1.0 + 1.0 + half);
                m.name = mdl.name.clone();
                m
            };
            let mat_slot = mat_index_by_id
                .get(mdl.material_id.as_str())
                .copied()
                .unwrap_or(0);
            let mat_col = hd_materials[mat_slot].base_color_factor;
            for vc in &mut mesh.colors {
                *vc = mat_col;
            }
            let mesh_slot = sector.meshes.len();
            let bounds = mesh.bounds();
            sector.meshes.push(mesh);

            let wx = (mdl.offset_decimeters[0] as f64) / deci;
            let dy = (mdl.offset_decimeters[1] as f64) / deci;
            let wz = (mdl.offset_decimeters[i2] as f64) / deci;
            let wy = hd_terrain_height(dom.seed, &terrain_relief, wx, wz) + dy;
            let sx = (mdl.scale_permille[0] as f64) / milli;
            let sy = (mdl.scale_permille[1] as f64) / milli;
            let sz = (mdl.scale_permille[i2] as f64) / milli;

            let ent_id = sector.scene.spawn(
                mdl.name.clone(),
                EntityRole::StaticArchitecture,
                None,
                Transform {
                    translation: Vec3::new(wx, wy, wz),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(sx, sy, sz),
                },
                bounds,
                (1.0, "AUTH-DOMAIN-PACKS"),
            );
            sector.scene.attach_renderable(ent_id, mesh_slot, mat_slot);
        }

        // Instantiate differentiated 22-joint characters from cataloged profiles & rigs:
        let mut characters = Vec::with_capacity(dom.characters.len());
        for ch_spec in &dom.characters {
            let archetype = match ch_spec.rig_id.as_str() {
                "RIG-HUMANOID-22-ARCHIVIST" => CharacterArchetype::Archivist,
                "RIG-HUMANOID-22-ENGINEER" => CharacterArchetype::BridgeEngineer,
                "RIG-HUMANOID-22-ASTRONOMER" => CharacterArchetype::ObservatoryKeeper,
                _ => CharacterArchetype::Explorer,
            };
            let cx = (ch_spec.spawn_decimeters[0] as f64) / deci;
            let cz = (ch_spec.spawn_decimeters[i2] as f64) / deci;
            let cy = hd_terrain_height(dom.seed, &terrain_relief, cx, cz);
            let mut ch = ArticulatedCharacter::new_humanoid(
                ch_spec.name.as_str(),
                archetype,
                Vec3::new(cx, cy, cz),
                lod_rank.max(1),
            )
            .map_err(|err| OracleValidationError::CatalogIntegrityMismatch {
                authority_id: "AUTH-KHRONOS-GLTF2".to_string(),
                detail: err.to_string(),
            })?;
            if let Some(rig_spec) = self.khronos_hd.rigs.iter().find(|r| r.id == ch_spec.rig_id) {
                let h_scale = (rig_spec.height_scale_permille as f32) / (milli as f32);
                let s_scale = (rig_spec.shoulder_scale_permille as f32) / (milli as f32);
                for pos in &mut ch.bind_mesh.positions {
                    pos[0] *= s_scale;
                    pos[1] *= h_scale;
                }
                ch.thigh_length_meters *= f64::from(h_scale);
                ch.calf_length_meters *= f64::from(h_scale);
            }
            characters.push(ch);
        }

        // Seed conserved items into the exact integer ConservationLedger:
        let mut item_ledger = ConservationLedger::new();
        for item in &dom.items {
            item_ledger
                .seed_account(format!("vault:{}", item.id), item.initial_supply)
                .map_err(|err| OracleValidationError::CatalogIntegrityMismatch {
                    authority_id: "AUTH-DOMAIN-PACKS".to_string(),
                    detail: err.to_string(),
                })?;
        }

        // Synthesize coupled Schema.org `@graph` JSON-LD and verify with `SchemaOrgVocabulary`:
        let place_iri = format!("urn:verdant:place:{}", dom.slug);
        let domain_iri = format!("urn:verdant:domain:{}", dom.id);
        let mut graph_nodes = Vec::new();
        graph_nodes.push(json!({
            "@id": place_iri,
            "@type": dom.environment.schema_type,
            "name": dom.environment.name,
            "identifier": format!("{}:env", dom.id)
        }));

        let mut model_refs = Vec::new();
        for mdl in &dom.models {
            let mdl_iri = format!("urn:verdant:model:{}", mdl.id);
            model_refs.push(json!({ "@id": mdl_iri }));
            graph_nodes.push(json!({
                "@id": mdl_iri,
                "@type": "3DModel",
                "name": mdl.name,
                "identifier": mdl.id,
                "encodingFormat": "model/gltf-binary",
                "isResizable": true,
                "material": mdl.material_id
            }));
        }

        let mut item_refs = Vec::new();
        for item in &dom.items {
            let item_iri = format!("urn:verdant:item:{}", item.id);
            item_refs.push(json!({ "@id": item_iri }));
            graph_nodes.push(json!({
                "@id": item_iri,
                "@type": "IndividualProduct",
                "name": item.name,
                "identifier": item.id,
                "serialNumber": format!("{}-SEED-{}", item.id, dom.seed),
                "material": item.material_id
            }));
        }

        let mut char_refs = Vec::new();
        for ch_spec in &dom.characters {
            let char_iri = format!("urn:verdant:character:{}", ch_spec.id);
            char_refs.push(json!({ "@id": char_iri }));
            graph_nodes.push(json!({
                "@id": char_iri,
                "@type": "Person",
                "name": ch_spec.name,
                "identifier": ch_spec.id,
                "jobTitle": ch_spec.role,
                "homeLocation": { "@id": place_iri },
                "knowsLanguage": ["en", "es", "fr", "ja"],
                "knowsAbout": ch_spec.knowledge_facts
            }));
        }

        let mut domain_node = json!({
            "@id": domain_iri,
            "@type": dom.schema_type,
            "name": dom.title,
            "identifier": dom.id,
            "applicationCategory": "3D Simulation & Interactive World",
            "operatingSystem": "WebAssembly (Verdant Engine)",
            "hasPart": model_refs,
            "character": char_refs,
            "location": { "@id": place_iri }
        });
        if dom.schema_type == "VideoGame" {
            domain_node["gameLocation"] = json!({ "@id": place_iri });
            domain_node["gameItem"] = json!(item_refs);
            domain_node["playMode"] = json!("CoOp");
        }
        graph_nodes.push(domain_node);

        let schema_org_jsonld = json!({
            "@context": "https://schema.org",
            "@graph": graph_nodes
        });
        self.schema_org
            .validate_schema_org_jsonld(&schema_org_jsonld)?;

        // Synthesize W3C SOSA & PROV-O Turtle provenance and verify with `SosaProvoShapes`:
        let sosa_provo_turtle = format!(
            "@prefix sosa: <http://www.w3.org/ns/sosa/> .\n\
             @prefix prov: <http://www.w3.org/ns/prov#> .\n\
             @prefix schema: <https://schema.org/> .\n\
             <urn:verdant:agent:engine> a prov:SoftwareAgent ; schema:name \"Verdant Engine Wasm Runtime\" .\n\
             <urn:verdant:activity:instantiate:{slug}> a prov:Activity ; prov:wasAssociatedWith <urn:verdant:agent:engine> .\n\
             <urn:verdant:catalog:domain-packs> a prov:Entity ; prov:wasAttributedTo <urn:verdant:agent:engine> .\n\
             <{domain_iri}> a prov:Entity ; prov:wasDerivedFrom <urn:verdant:catalog:domain-packs> ; prov:wasGeneratedBy <urn:verdant:activity:instantiate:{slug}> .\n\
             <urn:verdant:obs:boot:{slug}> a sosa:Observation ; sosa:madeBySensor <urn:verdant:sensor:observer-holon> ; sosa:hasFeatureOfInterest <{place_iri}> ; sosa:hasSimpleResult \"lod={lod_rank},seed={seed}\" .\n",
            slug = dom.slug,
            seed = dom.seed,
        );
        self.sosa_provo
            .validate_sosa_provo_turtle(&sosa_provo_turtle)?;

        Ok(InstantiatedDomainPack {
            spec: dom.clone(),
            sector,
            physical_environment,
            terrain_relief,
            acoustic_environment,
            hd_materials,
            characters,
            item_ledger,
            schema_org_jsonld,
            sosa_provo_turtle,
        })
    }
}

/// OMG DMN 1.5 decision rule in `catalogs/language/iso24617-dmn-dialogue.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DmnRuleEntry {
    /// Rule identifier (`R1-HALTED`..`R8-ADMIT-VERDANT`).
    pub id: String,
    /// Optional halted condition (`"yes"` / `"no"`).
    #[serde(default)]
    pub halted: Option<String>,
    /// Optional source lock condition (`"locked"` / `"unlocked"`).
    #[serde(default)]
    pub source: Option<String>,
    /// Optional flag matching when model format is outside `allowedVerdantFormats`.
    #[serde(rename = "formatNotInAllowed", default)]
    pub format_not_in_allowed: Option<bool>,
    /// Optional FK-07 route condition (`"granted"` / `"ungranted"`).
    #[serde(default)]
    pub route: Option<String>,
    /// Optional runtime digest condition (`"match"` / `"mismatch"`).
    #[serde(default)]
    pub runtime: Option<String>,
    /// Optional weights digest condition (`"match"` / `"mismatch"`).
    #[serde(default)]
    pub weights: Option<String>,
    /// Optional missing bundle condition.
    #[serde(rename = "missingRuntimeOrWeights", default)]
    pub missing_runtime_or_weights: Option<bool>,
    /// Decision outcome (`"run"`, `"ask"`, `"fetch"`, `"refuse"`).
    pub outcome: String,
    /// Machine-readable reason slug.
    pub reason: String,
}

/// ISO 24617-2 dialogue act entry in `catalogs/language/iso24617-dmn-dialogue.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IsoDialogueActEntry {
    /// Zero-based act index (`0..=7`).
    pub code: u8,
    /// ISO 24617-2 communicative function (`inform`, `whQuestion`, `ynQuestion`, `request`, `offer`, `confirm`, `disconfirm`, `abstain`).
    pub id: String,
    /// ISO dimension (`task`, `autoFeedback`, `epistemicSafety`).
    #[serde(rename = "isoDimension")]
    pub iso_dimension: String,
    /// Whether this dialogue act may propose conserved world state transitions.
    #[serde(rename = "mutatesWorld")]
    pub mutates_world: bool,
}

/// BCP-47 language starter profile in `catalogs/language/iso24617-dmn-dialogue.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageStarterProfile {
    /// BCP-47 tag (`en`, `es`, `fr`, `ja`).
    pub bcp47: String,
    /// Language display name.
    pub name: String,
    /// Wh-question starter tokens.
    #[serde(rename = "questionStarters")]
    pub question_starters: Vec<String>,
    /// Yes/No-question starter tokens.
    #[serde(rename = "ynStarters")]
    pub yn_starters: Vec<String>,
}

/// Cataloged Studio/Engine cybernetic intent route (`0..=15`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogIntentRoute {
    /// Zero-based route index (`0..=15`).
    pub index: usize,
    /// Canonical route slug.
    pub slug: String,
    /// Required `FK-07` container organ slug.
    pub organ: String,
    /// Lexical feature keywords feeding the `uor-matmul` `CodedSlmDirector`.
    pub keywords: Vec<String>,
}

/// Verified ISO 24617-2 Dialogue & OMG DMN 1.5 Model Admission catalog (`AUTH-ISO-24617-DMN`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageDialogueCatalog {
    /// Spec header.
    pub spec: String,
    /// Authority ID (`AUTH-ISO-24617-DMN`).
    #[serde(rename = "authorityId")]
    pub authority_id: String,
    /// External model/3D runtimes strictly prohibited in Verdant.
    #[serde(rename = "prohibitedExternalRuntimes")]
    pub prohibited_external_runtimes: Vec<String>,
    /// Native Verdant Engine model formats permitted for execution.
    #[serde(rename = "allowedVerdantFormats")]
    pub allowed_verdant_formats: Vec<String>,
    /// OMG DMN 1.5 hit policy (`"FIRST"`).
    #[serde(rename = "dmnHitPolicy")]
    pub dmn_hit_policy: String,
    /// Ordered `FIRST`-hit DMN rules.
    #[serde(rename = "dmnRules")]
    pub dmn_rules: Vec<DmnRuleEntry>,
    /// ISO 24617-2 communicative acts.
    #[serde(rename = "dialogueActs")]
    pub dialogue_acts: Vec<IsoDialogueActEntry>,
    /// Deterministic epistemic abstention token (`"Unknown"`).
    #[serde(rename = "abstentionToken")]
    pub abstention_token: String,
    /// Supported BCP-47 language profiles.
    pub languages: Vec<LanguageStarterProfile>,
    /// 16 cybernetic intent routes.
    #[serde(rename = "intentRoutes")]
    pub intent_routes: Vec<CatalogIntentRoute>,
}

/// Input context evaluated by the OMG DMN 1.5 `FIRST`-hit model admission gate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DmnAdmissionInput {
    /// Model format identifier (`"verdant-uorc"`, `"verdant-coded-matrix"`, `"verdant-holo"`, or external e.g. `"onnx"`).
    pub model_format: String,
    /// Whether the Observer emergency halt latch is active.
    pub observer_halted: bool,
    /// Whether the model authority is SHA-256 locked (`true` = `"locked"`, `false` = `"unlocked"`).
    pub source_locked: bool,
    /// Whether the required `FK-07` container organ route is granted.
    pub route_granted: bool,
    /// Runtime binary state (`"match"`, `"mismatch"`, or `"missing"`).
    pub runtime_state: String,
    /// Model weights state (`"match"`, `"mismatch"`, or `"missing"`).
    pub weights_state: String,
}

/// Decision returned by [`LanguageDialogueCatalog::evaluate_dmn_admission`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DmnAdmissionDecision {
    /// Matched rule ID (`R1-HALTED`..`R8-ADMIT-VERDANT`).
    pub rule_id: String,
    /// Outcome (`"run"`, `"ask"`, `"fetch"`, `"refuse"`).
    pub outcome: String,
    /// Reason slug.
    pub reason: String,
}
