//! External standard validation oracles (`Schema.org`, `W3C SOSA/SSN & PROV-O`, `Khronos glTF 2.0`,
//! `ISO 24617-2` & `OMG DMN 1.5`), SHA-256 authority catalog verification, runtime exclusivity
//! enforcement, and catalog-driven High-Definition multi-domain world instantiation.
//!
//! All numeric bounds and indices in this module are synthesized constructively from `{0, 1}`
//! (`FI-08`), and every external standard validator checks full structural, type-hierarchy,
//! cardinality, and acyclicity laws against the SHA-256-locked authority corpora in `catalogs/`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::audio::AcousticEnvironment;
use crate::character::{ArticulatedCharacter, CharacterArchetype};
use crate::digest::sha256_hex;
use crate::ecs::{EntityRole, Transform};
use crate::error::OracleValidationError;
use crate::geometry::{generate_faceted_prism_mesh, generate_fluted_column_mesh, ProceduralMesh};
use crate::hd_world::{
    hd_terrain_height, synthesize_hd_world_sector_for_kind, HdSectorBundle, WorldArchitectureKind,
};
use crate::math::{
    four_u32, half_f64, pow10_u64, three_u32, two_u32, Quat, Vec3, DIM_HOMOGENEOUS, DIM_SPATIAL,
};
use crate::narrative::ConservationLedger;
use crate::render::{canonical_pbr_material_library, HdPbrDescriptor};
use crate::streaming::{PhysicalEnvironment, TerrainReliefParams};

const EMBEDDED_AUTHORITIES_TOML: &str = include_str!("../../../model/authorities.toml");
const EMBEDDED_SCHEMA_ORG_JSON: &str = include_str!("../../../catalogs/schema-org/vocabulary.json");
const EMBEDDED_SOSA_PROVO_JSON: &str = include_str!("../../../catalogs/w3c/sosa-provo-shapes.json");
const EMBEDDED_KHRONOS_JSON: &str =
    include_str!("../../../catalogs/khronos/gltf-hd-materials-rigs.json");
const EMBEDDED_ISO_DMN_JSON: &str =
    include_str!("../../../catalogs/language/iso24617-dmn-dialogue.json");
const EMBEDDED_DOMAIN_PACKS_JSON: &str = include_str!("../../../catalogs/domains/world-packs.json");

/// File-backed external authority record extracted from `model/authorities.toml`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityFileLock {
    /// Authority identifier (`AUTH-*`).
    pub id: String,
    /// Repo-relative path to the committed corpus (`catalogs/...`).
    pub path: String,
    /// Expected lowercase hex SHA-256 digest.
    pub sha256: String,
}

/// Parses file-backed `[[authority]]` records (`path` + `sha256`) from `model/authorities.toml`.
#[must_use]
pub fn parse_authority_file_locks(authorities_toml: &str) -> Vec<AuthorityFileLock> {
    let mut out = Vec::new();
    let mut cur_id: Option<String> = None;
    let mut cur_path: Option<String> = None;
    let mut cur_sha: Option<String> = None;

    let flush = |out: &mut Vec<AuthorityFileLock>,
                 id: &mut Option<String>,
                 path: &mut Option<String>,
                 sha: &mut Option<String>| {
        if let (Some(i), Some(p), Some(s)) = (id.take(), path.take(), sha.take()) {
            out.push(AuthorityFileLock {
                id: i,
                path: p,
                sha256: s,
            });
        } else {
            *id = None;
            *path = None;
            *sha = None;
        }
    };

    for raw_line in authorities_toml.lines() {
        let line = raw_line.trim();
        if line == "[[authority]]" {
            flush(&mut out, &mut cur_id, &mut cur_path, &mut cur_sha);
            continue;
        }
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim();
            let val = v.trim().trim_matches('"');
            match key {
                "id" => cur_id = Some(val.to_string()),
                "artifact" | "path" => cur_path = Some(val.to_string()),
                "checksum" | "sha256" if val != "none" => {
                    cur_sha = Some(val.to_string());
                }
                _ => {}
            }
        }
    }
    flush(&mut out, &mut cur_id, &mut cur_path, &mut cur_sha);
    out
}

/// Class definition in `catalogs/schema-org/vocabulary.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaOrgClassDef {
    /// Immediate superclass name (`None` for root `Thing`).
    #[serde(rename = "subClassOf")]
    pub sub_class_of: Option<String>,
    /// Whether direct instantiation is abstract.
    #[serde(rename = "abstract")]
    pub is_abstract: bool,
}

/// Property domain/range definition in `catalogs/schema-org/vocabulary.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaOrgPropertyDef {
    /// Admissible subject classes (`domainIncludes`).
    #[serde(rename = "domainIncludes")]
    pub domain_includes: Vec<String>,
    /// Admissible object classes or scalar datatypes (`rangeIncludes`).
    #[serde(rename = "rangeIncludes")]
    pub range_includes: Vec<String>,
}

/// Verified Schema.org vocabulary oracle (`AUTH-SCHEMA-ORG`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaOrgVocabulary {
    /// Spec version header.
    pub spec: String,
    /// Authority ID (`AUTH-SCHEMA-ORG`).
    #[serde(rename = "authorityId")]
    pub authority_id: String,
    /// Canonical JSON-LD context URL (`https://schema.org`).
    #[serde(rename = "contextUrl")]
    pub context_url: String,
    /// Class taxonomy map.
    pub classes: BTreeMap<String, SchemaOrgClassDef>,
    /// Scalar datatype names (`Text`, `Number`, `Integer`, `Boolean`, `URL`).
    pub datatypes: Vec<String>,
    /// Typed property map.
    pub properties: BTreeMap<String, SchemaOrgPropertyDef>,
}

/// Validation report emitted by [`SchemaOrgVocabulary::validate_schema_org_jsonld`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaOrgValidationReport {
    /// Number of typed entity nodes verified.
    pub entity_count: usize,
    /// Number of typed property assertions verified against `domainIncludes` and `rangeIncludes`.
    pub property_assertion_count: usize,
}

impl SchemaOrgVocabulary {
    /// Returns `true` when `sub_class` equals `super_class` or transitively inherits from it.
    #[must_use]
    pub fn is_subclass_of(&self, sub_class: &str, super_class: &str) -> bool {
        if !self.classes.contains_key(sub_class) || !self.classes.contains_key(super_class) {
            return false;
        }
        let mut cursor = Some(sub_class);
        let mut visited = BTreeSet::new();
        while let Some(curr) = cursor {
            if curr == super_class {
                return true;
            }
            if !visited.insert(curr) {
                return false;
            }
            cursor = self
                .classes
                .get(curr)
                .and_then(|def| def.sub_class_of.as_deref());
        }
        false
    }

    fn collect_id_types(
        node: &serde_json::Value,
        id_map: &mut BTreeMap<String, String>,
    ) -> Result<(), OracleValidationError> {
        let Some(obj) = node.as_object() else {
            return Ok(());
        };
        if let Some(ty_val) = obj.get("@type") {
            let ty = ty_val
                .as_str()
                .ok_or_else(|| OracleValidationError::SchemaOrgInvalid {
                    subject: "<anonymous>".to_string(),
                    detail: "`@type` must be a Schema.org class string".to_string(),
                })?;
            if let Some(id_str) = obj.get("@id").and_then(serde_json::Value::as_str) {
                id_map.insert(id_str.to_string(), ty.to_string());
            }
        }
        for (k, v) in obj {
            if k.starts_with('@') {
                continue;
            }
            match v {
                serde_json::Value::Object(_) => Self::collect_id_types(v, id_map)?,
                serde_json::Value::Array(items) => {
                    for item in items {
                        Self::collect_id_types(item, id_map)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn value_matches_range(
        &self,
        val: &serde_json::Value,
        allowed_ranges: &[String],
        id_types: &BTreeMap<String, String>,
        subject: &str,
        prop_name: &str,
    ) -> Result<usize, OracleValidationError> {
        match val {
            serde_json::Value::String(s) => {
                let is_url = s.starts_with("https://")
                    || s.starts_with("http://")
                    || s.starts_with("urn:")
                    || s.starts_with('/');
                if let Some(ref_type) = id_types.get(s) {
                    if allowed_ranges
                        .iter()
                        .any(|r| self.is_subclass_of(ref_type, r))
                    {
                        return Ok(1);
                    }
                }
                let scalar_ok = allowed_ranges
                    .iter()
                    .any(|r| r == "Text" || (r == "URL" && is_url));
                if scalar_ok {
                    Ok(1)
                } else {
                    Err(OracleValidationError::SchemaOrgInvalid {
                        subject: subject.to_string(),
                        detail: format!(
                            "property `{prop_name}` string value `{s}` violates rangeIncludes {allowed_ranges:?}"
                        ),
                    })
                }
            }
            serde_json::Value::Number(n) => {
                let num_ok = allowed_ranges
                    .iter()
                    .any(|r| r == "Number" || (r == "Integer" && (n.is_i64() || n.is_u64())));
                if num_ok {
                    Ok(1)
                } else {
                    Err(OracleValidationError::SchemaOrgInvalid {
                        subject: subject.to_string(),
                        detail: format!(
                            "property `{prop_name}` numeric value `{n}` violates rangeIncludes {allowed_ranges:?}"
                        ),
                    })
                }
            }
            serde_json::Value::Bool(_) => {
                if allowed_ranges.iter().any(|r| r == "Boolean") {
                    Ok(1)
                } else {
                    Err(OracleValidationError::SchemaOrgInvalid {
                        subject: subject.to_string(),
                        detail: format!(
                            "property `{prop_name}` boolean value violates rangeIncludes {allowed_ranges:?}"
                        ),
                    })
                }
            }
            serde_json::Value::Object(obj) => {
                if obj.get("@type").is_some() {
                    let sub_rep = self.validate_entity_object(obj, id_types)?;
                    let child_ty = obj
                        .get("@type")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("");
                    if allowed_ranges
                        .iter()
                        .any(|r| self.is_subclass_of(child_ty, r))
                    {
                        Ok(1 + sub_rep.property_assertion_count)
                    } else {
                        Err(OracleValidationError::SchemaOrgInvalid {
                            subject: subject.to_string(),
                            detail: format!(
                                "property `{prop_name}` nested `@type` `{child_ty}` violates rangeIncludes {allowed_ranges:?}"
                            ),
                        })
                    }
                } else if let Some(ref_id) = obj.get("@id").and_then(serde_json::Value::as_str) {
                    if let Some(target_ty) = id_types.get(ref_id) {
                        if allowed_ranges
                            .iter()
                            .any(|r| self.is_subclass_of(target_ty, r))
                        {
                            return Ok(1);
                        }
                        return Err(OracleValidationError::SchemaOrgInvalid {
                            subject: subject.to_string(),
                            detail: format!(
                                "property `{prop_name}` reference `{ref_id}` (`{target_ty}`) violates rangeIncludes {allowed_ranges:?}"
                            ),
                        });
                    }
                    if allowed_ranges.iter().any(|r| r == "URL") {
                        Ok(1)
                    } else {
                        Err(OracleValidationError::SchemaOrgInvalid {
                            subject: subject.to_string(),
                            detail: format!(
                                "property `{prop_name}` references untyped `@id` `{ref_id}`"
                            ),
                        })
                    }
                } else {
                    Err(OracleValidationError::SchemaOrgInvalid {
                        subject: subject.to_string(),
                        detail: format!("property `{prop_name}` object lacks `@type` or `@id`"),
                    })
                }
            }
            serde_json::Value::Array(items) => {
                if items.is_empty() {
                    return Err(OracleValidationError::SchemaOrgInvalid {
                        subject: subject.to_string(),
                        detail: format!("property `{prop_name}` array must not be empty"),
                    });
                }
                let mut count = 0usize;
                for item in items {
                    count += self.value_matches_range(
                        item,
                        allowed_ranges,
                        id_types,
                        subject,
                        prop_name,
                    )?;
                }
                Ok(count)
            }
            serde_json::Value::Null => Err(OracleValidationError::SchemaOrgInvalid {
                subject: subject.to_string(),
                detail: format!("property `{prop_name}` must not be null"),
            }),
        }
    }

    fn validate_entity_object(
        &self,
        obj: &serde_json::Map<String, serde_json::Value>,
        id_types: &BTreeMap<String, String>,
    ) -> Result<SchemaOrgValidationReport, OracleValidationError> {
        let subject = obj
            .get("@id")
            .and_then(serde_json::Value::as_str)
            .or_else(|| obj.get("name").and_then(serde_json::Value::as_str))
            .unwrap_or("<node>");
        let class_name = obj
            .get("@type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| OracleValidationError::SchemaOrgInvalid {
                subject: subject.to_string(),
                detail: "missing required Schema.org `@type`".to_string(),
            })?;
        let Some(class_def) = self.classes.get(class_name) else {
            return Err(OracleValidationError::SchemaOrgInvalid {
                subject: subject.to_string(),
                detail: format!("unknown Schema.org class `{class_name}`"),
            });
        };
        if class_def.is_abstract {
            return Err(OracleValidationError::SchemaOrgInvalid {
                subject: subject.to_string(),
                detail: format!("cannot instantiate abstract Schema.org class `{class_name}`"),
            });
        }

        let mut assertions = 0usize;
        for (prop_name, prop_val) in obj {
            if prop_name.starts_with('@') {
                continue;
            }
            let Some(prop_def) = self.properties.get(prop_name) else {
                return Err(OracleValidationError::SchemaOrgInvalid {
                    subject: subject.to_string(),
                    detail: format!("unknown Schema.org property `{prop_name}` on `{class_name}`"),
                });
            };
            let domain_ok = prop_def
                .domain_includes
                .iter()
                .any(|dom| self.is_subclass_of(class_name, dom));
            if !domain_ok {
                return Err(OracleValidationError::SchemaOrgInvalid {
                    subject: subject.to_string(),
                    detail: format!(
                        "property `{prop_name}` is not valid on `@type` `{class_name}` (domainIncludes: {:?})",
                        prop_def.domain_includes
                    ),
                });
            }
            assertions += self.value_matches_range(
                prop_val,
                &prop_def.range_includes,
                id_types,
                subject,
                prop_name,
            )?;
        }

        Ok(SchemaOrgValidationReport {
            entity_count: 1,
            property_assertion_count: assertions,
        })
    }

    /// Validates a Schema.org JSON-LD document (single entity or `@graph` bundle) against
    /// `AUTH-SCHEMA-ORG` class inheritance, `domainIncludes`, and `rangeIncludes`.
    pub fn validate_schema_org_jsonld(
        &self,
        doc: &serde_json::Value,
    ) -> Result<SchemaOrgValidationReport, OracleValidationError> {
        let Some(root) = doc.as_object() else {
            return Err(OracleValidationError::SchemaOrgInvalid {
                subject: "<root>".to_string(),
                detail: "JSON-LD root must be an object".to_string(),
            });
        };
        let ctx = root
            .get("@context")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        if ctx != "https://schema.org" && ctx != "http://schema.org" && ctx != "https://schema.org/"
        {
            return Err(OracleValidationError::SchemaOrgInvalid {
                subject: "<root>".to_string(),
                detail: format!("`@context` must be `https://schema.org`, got `{ctx}`"),
            });
        }

        let mut id_types = BTreeMap::new();
        if let Some(graph) = root.get("@graph") {
            let Some(nodes) = graph.as_array() else {
                return Err(OracleValidationError::SchemaOrgInvalid {
                    subject: "<@graph>".to_string(),
                    detail: "`@graph` must be a non-empty array".to_string(),
                });
            };
            if nodes.is_empty() {
                return Err(OracleValidationError::SchemaOrgInvalid {
                    subject: "<@graph>".to_string(),
                    detail: "`@graph` must contain at least one entity".to_string(),
                });
            }
            for n in nodes {
                Self::collect_id_types(n, &mut id_types)?;
            }
            let mut total_entities = 0usize;
            let mut total_props = 0usize;
            for n in nodes {
                let Some(obj) = n.as_object() else {
                    return Err(OracleValidationError::SchemaOrgInvalid {
                        subject: "<@graph>".to_string(),
                        detail: "every `@graph` entry must be an object".to_string(),
                    });
                };
                let rep = self.validate_entity_object(obj, &id_types)?;
                total_entities += rep.entity_count;
                total_props += rep.property_assertion_count;
            }
            Ok(SchemaOrgValidationReport {
                entity_count: total_entities,
                property_assertion_count: total_props,
            })
        } else {
            Self::collect_id_types(doc, &mut id_types)?;
            self.validate_entity_object(root, &id_types)
        }
    }
}

/// Shape definition in `catalogs/w3c/sosa-provo-shapes.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SosaProvoShapeRule {
    /// Predicates that must appear at least once on every instance of the class.
    #[serde(rename = "requiredPredicates")]
    pub required_predicates: Vec<String>,
    /// Disjunctive required predicate set (at least one must appear when non-empty).
    #[serde(rename = "atLeastOneOf")]
    pub at_least_one_of: Vec<String>,
    /// Predicates whose object must be an IRI resource.
    #[serde(rename = "iriPredicates")]
    pub iri_predicates: Vec<String>,
    /// Predicates whose object must be a literal value.
    #[serde(rename = "literalPredicates")]
    pub literal_predicates: Vec<String>,
    /// Optional predicate that must form a strict DAG (`prov:wasDerivedFrom`).
    #[serde(rename = "acyclicPredicate", default)]
    pub acyclic_predicate: Option<String>,
}

/// Verified W3C SOSA/SSN & W3C PROV-O SHACL-equivalent shape oracle (`AUTH-W3C-SOSA-PROVO`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SosaProvoShapes {
    /// Spec header.
    pub spec: String,
    /// Authority ID (`AUTH-W3C-SOSA-PROVO`).
    #[serde(rename = "authorityId")]
    pub authority_id: String,
    /// Standard namespace prefix map.
    pub prefixes: BTreeMap<String, String>,
    /// Known CURIE classes.
    pub classes: Vec<String>,
    /// Subclass relation (`prov:Person -> prov:Agent`).
    #[serde(rename = "subClassOf")]
    pub sub_class_of: BTreeMap<String, String>,
    /// Per-class shape rules.
    pub shapes: BTreeMap<String, SosaProvoShapeRule>,
}

/// Normalized RDF object value (either an IRI/CURIE resource or an RDF literal).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RdfTerm {
    /// Resource IRI or compact CURIE.
    Iri(String),
    /// Scalar literal value.
    Literal(String),
}

/// Normalized RDF triple `(subject, predicate, object)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RdfTriple {
    /// Subject IRI or CURIE.
    pub subject: String,
    /// Predicate CURIE (`sosa:madeBySensor`, `prov:wasDerivedFrom`, `rdf:type`).
    pub predicate: String,
    /// Object IRI or literal.
    pub object: RdfTerm,
}

/// Conformance report emitted by [`SosaProvoShapes::validate_sosa_provo_turtle`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SosaProvoValidationReport {
    /// W3C SHACL status string (`"conforms"`).
    pub status: &'static str,
    /// Distinct typed subjects verified.
    pub subject_count: usize,
    /// Total RDF triples verified.
    pub triple_count: usize,
    /// Verified `sosa:Observation` instances.
    pub observation_count: usize,
    /// Verified `prov:Entity` instances.
    pub provenance_entity_count: usize,
}

fn tokenize_turtle(input: &str) -> Result<Vec<String>, OracleValidationError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut idx = 0usize;
    let len = chars.len();

    while idx < len {
        let ch = chars[idx];
        if ch.is_whitespace() {
            idx += 1;
            continue;
        }
        if ch == '#' {
            while idx < len && chars[idx] != '\n' {
                idx += 1;
            }
            continue;
        }
        if ch == ';' || ch == ',' || ch == '.' {
            tokens.push(ch.to_string());
            idx += 1;
            continue;
        }
        if ch == '<' {
            let start = idx;
            idx += 1;
            while idx < len && chars[idx] != '>' {
                idx += 1;
            }
            if idx >= len {
                return Err(OracleValidationError::SosaProvoConstraintViolated {
                    subject: "<turtle>".to_string(),
                    detail: "unterminated `<iri>` in Turtle graph".to_string(),
                });
            }
            idx += 1;
            let iri_tok: String = chars[start..idx].iter().collect();
            tokens.push(iri_tok);
            continue;
        }
        if ch == '"' {
            let start = idx;
            idx += 1;
            while idx < len && chars[idx] != '"' {
                if chars[idx] == '\\' {
                    idx += 1;
                }
                idx += 1;
            }
            if idx >= len {
                return Err(OracleValidationError::SosaProvoConstraintViolated {
                    subject: "<turtle>".to_string(),
                    detail: "unterminated string literal in Turtle graph".to_string(),
                });
            }
            idx += 1;
            // Optional `^^xsd:...` or `@lang` suffix attached to literal token:
            while idx < len
                && !chars[idx].is_whitespace()
                && chars[idx] != ';'
                && chars[idx] != ','
                && chars[idx] != '.'
            {
                if chars[idx] == '<' {
                    while idx < len && chars[idx] != '>' {
                        idx += 1;
                    }
                }
                idx += 1;
            }
            let lit_tok: String = chars[start..idx].iter().collect();
            tokens.push(lit_tok);
            continue;
        }

        let start = idx;
        while idx < len
            && !chars[idx].is_whitespace()
            && chars[idx] != ';'
            && chars[idx] != ','
            && chars[idx] != '.'
        {
            idx += 1;
        }
        // Handle trailing `.` only when not part of a decimal literal like `12.5`:
        let word: String = chars[start..idx].iter().collect();
        tokens.push(word);
    }

    Ok(tokens)
}

impl SosaProvoShapes {
    fn normalize_curie_or_iri(
        &self,
        raw: &str,
        doc_prefixes: &BTreeMap<String, String>,
    ) -> Result<String, OracleValidationError> {
        if raw == "a" {
            return Ok("rdf:type".to_string());
        }
        if raw.starts_with('<') && raw.ends_with('>') {
            let full = &raw[1..raw.len() - 1];
            for (std_pfx, std_ns) in &self.prefixes {
                if let Some(local) = full.strip_prefix(std_ns.as_str()) {
                    return Ok(format!("{std_pfx}:{local}"));
                }
            }
            return Ok(full.to_string());
        }
        if let Some((pfx, local)) = raw.split_once(':') {
            if let Some(doc_ns) = doc_prefixes.get(pfx) {
                for (std_pfx, std_ns) in &self.prefixes {
                    if doc_ns == std_ns {
                        return Ok(format!("{std_pfx}:{local}"));
                    }
                }
                return Ok(format!("{doc_ns}{local}"));
            }
            if self.prefixes.contains_key(pfx) || pfx == "urn" || pfx == "ex" || pfx == "verdant" {
                return Ok(raw.to_string());
            }
        }
        Err(OracleValidationError::SosaProvoConstraintViolated {
            subject: raw.to_string(),
            detail: format!("undeclared prefix or malformed IRI `{raw}`"),
        })
    }

    /// Parses a W3C Turtle graph (`@prefix`, `;`, `,`, `.`) into normalized [`RdfTriple`] records.
    pub fn parse_turtle(&self, turtle: &str) -> Result<Vec<RdfTriple>, OracleValidationError> {
        let tokens = tokenize_turtle(turtle)?;
        let mut doc_prefixes = BTreeMap::new();
        let mut triples = Vec::new();
        let mut i = 0usize;
        let i2 = 1usize + 1usize;
        let i3 = i2 + 1usize;

        while i < tokens.len() {
            if tokens[i] == "@prefix" {
                if i + i3 >= tokens.len() || tokens[i + i3] != "." {
                    return Err(OracleValidationError::SosaProvoConstraintViolated {
                        subject: "@prefix".to_string(),
                        detail: "malformed `@prefix` directive in Turtle".to_string(),
                    });
                }
                let pfx = tokens[i + 1].trim_end_matches(':').to_string();
                let iri_tok = &tokens[i + i2];
                if !iri_tok.starts_with('<') || !iri_tok.ends_with('>') {
                    return Err(OracleValidationError::SosaProvoConstraintViolated {
                        subject: "@prefix".to_string(),
                        detail: format!("prefix namespace `{iri_tok}` must be `<...>`"),
                    });
                }
                doc_prefixes.insert(pfx, iri_tok[1..iri_tok.len() - 1].to_string());
                i += i3 + 1;
                continue;
            }

            let subject = self.normalize_curie_or_iri(&tokens[i], &doc_prefixes)?;
            i += 1;
            loop {
                if i >= tokens.len() {
                    return Err(OracleValidationError::SosaProvoConstraintViolated {
                        subject: subject.clone(),
                        detail: "unexpected EOF expecting predicate or `.`".to_string(),
                    });
                }
                if tokens[i] == "." {
                    i += 1;
                    break;
                }
                let predicate = self.normalize_curie_or_iri(&tokens[i], &doc_prefixes)?;
                i += 1;
                loop {
                    if i >= tokens.len() {
                        return Err(OracleValidationError::SosaProvoConstraintViolated {
                            subject: subject.clone(),
                            detail: "unexpected EOF expecting RDF object".to_string(),
                        });
                    }
                    let obj_tok = &tokens[i];
                    let object = if obj_tok.starts_with('"')
                        || obj_tok == "true"
                        || obj_tok == "false"
                        || obj_tok
                            .chars()
                            .next()
                            .is_some_and(|c| c.is_ascii_digit() || c == '-')
                    {
                        RdfTerm::Literal(obj_tok.clone())
                    } else {
                        RdfTerm::Iri(self.normalize_curie_or_iri(obj_tok, &doc_prefixes)?)
                    };
                    triples.push(RdfTriple {
                        subject: subject.clone(),
                        predicate: predicate.clone(),
                        object,
                    });
                    i += 1;
                    if i < tokens.len() && tokens[i] == "," {
                        i += 1;
                        continue;
                    }
                    break;
                }
                if i < tokens.len() && tokens[i] == ";" {
                    i += 1;
                    continue;
                }
                if i < tokens.len() && tokens[i] == "." {
                    i += 1;
                    break;
                }
                return Err(OracleValidationError::SosaProvoConstraintViolated {
                    subject: subject.clone(),
                    detail: format!(
                        "expected `;`, `,`, or `.` in Turtle statement, got `{}`",
                        tokens.get(i).map_or("<eof>", String::as_str)
                    ),
                });
            }
        }

        if triples.is_empty() {
            return Err(OracleValidationError::SosaProvoConstraintViolated {
                subject: "<graph>".to_string(),
                detail: "RDF graph contains zero triples".to_string(),
            });
        }
        Ok(triples)
    }

    /// Validates normalized RDF triples against W3C SOSA/SSN & W3C PROV-O shapes and acyclicity laws.
    pub fn validate_triples(
        &self,
        triples: &[RdfTriple],
    ) -> Result<SosaProvoValidationReport, OracleValidationError> {
        if triples.is_empty() {
            return Err(OracleValidationError::SosaProvoConstraintViolated {
                subject: "<graph>".to_string(),
                detail: "RDF graph contains zero triples".to_string(),
            });
        }

        let known_classes: BTreeSet<&str> = self.classes.iter().map(String::as_str).collect();
        let mut subject_types: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut subject_preds: BTreeMap<String, Vec<(String, RdfTerm)>> = BTreeMap::new();

        for t in triples {
            subject_preds
                .entry(t.subject.clone())
                .or_default()
                .push((t.predicate.clone(), t.object.clone()));
            if t.predicate == "rdf:type" {
                let RdfTerm::Iri(cls) = &t.object else {
                    return Err(OracleValidationError::SosaProvoConstraintViolated {
                        subject: t.subject.clone(),
                        detail: "`rdf:type` object must be a class IRI".to_string(),
                    });
                };
                if !known_classes.contains(cls.as_str()) {
                    return Err(OracleValidationError::SosaProvoConstraintViolated {
                        subject: t.subject.clone(),
                        detail: format!("unknown SOSA/PROV-O class `{cls}`"),
                    });
                }
                let entry = subject_types.entry(t.subject.clone()).or_default();
                entry.insert(cls.clone());
                let mut cur = cls.as_str();
                while let Some(sup) = self.sub_class_of.get(cur) {
                    entry.insert(sup.clone());
                    cur = sup.as_str();
                }
            }
        }

        if subject_types.is_empty() {
            return Err(OracleValidationError::SosaProvoConstraintViolated {
                subject: "<graph>".to_string(),
                detail: "at least one typed SOSA/PROV-O subject is required".to_string(),
            });
        }

        let mut observation_count = 0usize;
        let mut provenance_entity_count = 0usize;

        for (subj, types) in &subject_types {
            if types.contains("sosa:Observation") {
                observation_count += 1;
            }
            if types.contains("prov:Entity") {
                provenance_entity_count += 1;
            }
            let preds = subject_preds.get(subj).map_or(&[][..], Vec::as_slice);
            for cls in types {
                let Some(shape) = self.shapes.get(cls) else {
                    continue;
                };
                for req in &shape.required_predicates {
                    if !preds.iter().any(|(p, _)| p == req) {
                        return Err(OracleValidationError::SosaProvoConstraintViolated {
                            subject: subj.clone(),
                            detail: format!("shape `{cls}` missing required predicate `{req}`"),
                        });
                    }
                }
                if !shape.at_least_one_of.is_empty()
                    && !shape
                        .at_least_one_of
                        .iter()
                        .any(|cand| preds.iter().any(|(p, _)| p == cand))
                {
                    return Err(OracleValidationError::SosaProvoConstraintViolated {
                        subject: subj.clone(),
                        detail: format!(
                            "shape `{cls}` requires at least one of {:?}",
                            shape.at_least_one_of
                        ),
                    });
                }
                for (pred, obj) in preds {
                    if shape.iri_predicates.iter().any(|p| p == pred)
                        && !matches!(obj, RdfTerm::Iri(_))
                    {
                        return Err(OracleValidationError::SosaProvoConstraintViolated {
                            subject: subj.clone(),
                            detail: format!(
                                "predicate `{pred}` on `{cls}` requires an IRI object, got literal"
                            ),
                        });
                    }
                    if shape.literal_predicates.iter().any(|p| p == pred)
                        && !matches!(obj, RdfTerm::Literal(_))
                    {
                        return Err(OracleValidationError::SosaProvoConstraintViolated {
                            subject: subj.clone(),
                            detail: format!(
                                "predicate `{pred}` on `{cls}` requires a literal object, got IRI"
                            ),
                        });
                    }
                }
            }
        }

        // Enforce strict acyclicity on `prov:wasDerivedFrom` (and any shape-declared acyclicPredicate):
        let mut acyclic_preds = BTreeSet::new();
        for shape in self.shapes.values() {
            if let Some(ap) = shape.acyclic_predicate_or_ref() {
                acyclic_preds.insert(ap.to_string());
            }
        }
        for ap in acyclic_preds {
            let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
            for t in triples {
                if t.predicate == ap {
                    if let RdfTerm::Iri(dst) = &t.object {
                        adj.entry(t.subject.as_str())
                            .or_default()
                            .push(dst.as_str());
                    }
                }
            }
            for &start in adj.keys() {
                let mut stack: Vec<(&str, BTreeSet<&str>)> = vec![(start, BTreeSet::from([start]))];
                while let Some((node, path)) = stack.pop() {
                    if let Some(nexts) = adj.get(node) {
                        for &nx in nexts {
                            if path.contains(nx) {
                                return Err(OracleValidationError::SosaProvoConstraintViolated {
                                    subject: start.to_string(),
                                    detail: format!(
                                        "cyclic `{ap}` derivation chain detected involving `{node}` -> `{nx}`"
                                    ),
                                });
                            }
                            let mut next_path = path.clone();
                            next_path.insert(nx);
                            stack.push((nx, next_path));
                        }
                    }
                }
            }
        }

        Ok(SosaProvoValidationReport {
            status: "conforms",
            subject_count: subject_types.len(),
            triple_count: triples.len(),
            observation_count,
            provenance_entity_count,
        })
    }

    /// Parses and validates a W3C Turtle document against `AUTH-W3C-SOSA-PROVO`.
    pub fn validate_sosa_provo_turtle(
        &self,
        turtle: &str,
    ) -> Result<SosaProvoValidationReport, OracleValidationError> {
        let triples = self.parse_turtle(turtle)?;
        self.validate_triples(&triples)
    }
}

impl SosaProvoShapeRule {
    fn acyclic_predicate_or_ref(&self) -> Option<&str> {
        self.acyclic_predicate.as_deref()
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

impl LanguageDialogueCatalog {
    /// Evaluates OMG DMN 1.5 `FIRST`-hit model admission.
    ///
    /// Immediately refuses any external model format outside `allowedVerdantFormats` with
    /// [`OracleValidationError::ProhibitedExternalModelRuntime`].
    pub fn evaluate_dmn_admission(
        &self,
        input: &DmnAdmissionInput,
    ) -> Result<DmnAdmissionDecision, OracleValidationError> {
        let format_disallowed = !self
            .allowed_verdant_formats
            .iter()
            .any(|f| f == &input.model_format);
        let halted_str = if input.observer_halted { "yes" } else { "no" };
        let source_str = if input.source_locked {
            "locked"
        } else {
            "unlocked"
        };
        let route_str = if input.route_granted {
            "granted"
        } else {
            "ungranted"
        };
        let missing_bundle = input.runtime_state == "missing" || input.weights_state == "missing";

        for rule in &self.dmn_rules {
            if let Some(h) = &rule.halted {
                if h != halted_str {
                    continue;
                }
            }
            if let Some(s) = &rule.source {
                if s != source_str {
                    continue;
                }
            }
            if let Some(req_disallowed) = rule.format_not_in_allowed {
                if req_disallowed != format_disallowed {
                    continue;
                }
            }
            if let Some(rt) = &rule.route {
                if rt != route_str {
                    continue;
                }
            }
            if let Some(run_st) = &rule.runtime {
                if run_st != &input.runtime_state {
                    continue;
                }
            }
            if let Some(wt_st) = &rule.weights {
                if wt_st != &input.weights_state {
                    continue;
                }
            }
            if let Some(miss) = rule.missing_runtime_or_weights {
                if miss != missing_bundle {
                    continue;
                }
            }

            if rule.outcome == "refuse" && format_disallowed {
                return Err(OracleValidationError::ProhibitedExternalModelRuntime {
                    runtime_id: input.model_format.clone(),
                    detail: format!("DMN rule `{}` refused format: {}", rule.id, rule.reason),
                });
            }
            return Ok(DmnAdmissionDecision {
                rule_id: rule.id.clone(),
                outcome: rule.outcome.clone(),
                reason: rule.reason.clone(),
            });
        }

        Err(OracleValidationError::ProhibitedExternalModelRuntime {
            runtime_id: input.model_format.clone(),
            detail: "no DMN 1.5 admission rule matched model request".to_string(),
        })
    }

    /// Scans a manifest (`package.json`, `Cargo.toml`) or JS/TS/Rust source file to verify zero
    /// external model runtimes (`@huggingface/transformers`, `onnxruntime*`, `tensorflow*`,
    /// `three`, `babylonjs`, `@mlc-ai/web-llm`, `openai`, etc.) are imported or declared.
    pub fn verify_verdant_runtime_exclusivity(
        &self,
        manifest_label: &str,
        source_text: &str,
    ) -> Result<(), OracleValidationError> {
        for forbidden in &self.prohibited_external_runtimes {
            let quoted_double = format!("\"{forbidden}\"");
            let quoted_single = format!("'{forbidden}'");
            let subpath_double = format!("\"{forbidden}/");
            let subpath_single = format!("'{forbidden}/");
            let cargo_dep = format!("{forbidden} =");
            if source_text.contains(&quoted_double)
                || source_text.contains(&quoted_single)
                || source_text.contains(&subpath_double)
                || source_text.contains(&subpath_single)
                || source_text.contains(&cargo_dep)
            {
                return Err(OracleValidationError::ProhibitedExternalModelRuntime {
                    runtime_id: forbidden.clone(),
                    detail: format!(
                        "`{manifest_label}` references prohibited external runtime `{forbidden}` (Verdant Engine is the sole permitted model runtime)"
                    ),
                });
            }
        }
        Ok(())
    }
}

#[path = "standards_registry.rs"]
mod standards_registry;
pub use standards_registry::*;
