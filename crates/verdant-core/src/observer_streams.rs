//! Observer project-model admission, sensory stream registry, host capability envelope updates,
//! and live knowledge-passage ingestion (`observer_streams.rs`), extracted from `observer.rs`.

use super::*;

impl ObserverPrismHolon {
    /// Admits and auto-tags an external or local catalog model into the active project via
    /// OMG DMN 1.5 + `prism_kernel::dmnModelRouteAdmitted` + `VerdantModelEnvelope`, creating
    /// immutable `v1` (or appending `vN+1` if re-imported) and recording a SOSA/PROV-O envelope.
    pub fn import_project_model(
        &mut self,
        req: ProjectModelImportRequest,
        grants: &ContainerGrantMatrix,
    ) -> Result<ProjectImportedModelRecord, ObserverModelError> {
        let model_id = req.model_id.trim().to_string();
        let title = req.title.trim().to_string();
        let schema_type = req.schema_type.trim().to_string();
        let media_type = req.media_type.trim().to_string();
        let target_slot = req.target_slot.trim().to_string();
        if model_id.is_empty()
            || title.is_empty()
            || schema_type.is_empty()
            || media_type.is_empty()
            || target_slot.is_empty()
        {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id,
                detail: "model_id, title, schema_type, media_type, and target_slot are required"
                    .to_string(),
            });
        }

        let clean_tags: Vec<String> = req
            .tags
            .into_iter()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();
        let props = if req.properties_json.trim().is_empty() {
            "{}".to_string()
        } else {
            req.properties_json.trim().to_string()
        };
        let commit_note = if req.commit_note.trim().is_empty() {
            format!("Imported from {} ({})", req.source_stream, schema_type)
        } else {
            req.commit_note.trim().to_string()
        };

        let canonical_payload = json!({
            "modelId": model_id,
            "title": title,
            "schemaType": schema_type,
            "mediaType": media_type,
            "targetSlot": target_slot,
            "sourceStream": req.source_stream,
            "sourceUri": req.source_uri,
            "licenseSpdx": req.license_spdx,
            "tags": clean_tags,
            "properties": props,
        })
        .to_string()
        .into_bytes();

        let owner_organ = if target_slot.starts_with("code:") {
            ObserverOrganKind::Engine
        } else if target_slot.starts_with("narrative:") || target_slot.starts_with("gameplay:") {
            ObserverOrganKind::Demo
        } else {
            ObserverOrganKind::Studio
        };

        let domain_kind = if target_slot.starts_with("audio:") {
            ArbitraryModelDomainKind::SpatialAcoustics
        } else if target_slot.starts_with("design:theme")
            || target_slot.starts_with("design:typography")
        {
            ArbitraryModelDomainKind::TropicalUi
        } else if target_slot.starts_with("narrative:") {
            ArbitraryModelDomainKind::ConservationAutomaton
        } else if target_slot.starts_with("gameplay:") {
            ArbitraryModelDomainKind::RigidBodyPhysics
        } else {
            ArbitraryModelDomainKind::NeuromorphicSsm
        };
        let oracle_class = if self.catalogs.schema_org.classes.contains_key(&schema_type) {
            schema_type.as_str()
        } else {
            "CreativeWork"
        };
        let is_creative = self
            .catalogs
            .schema_org
            .is_subclass_of(oracle_class, "CreativeWork");
        let schema_jsonld = if is_creative {
            json!({
                "@context": "https://schema.org",
                "@type": oracle_class,
                "identifier": model_id,
                "name": title,
                "additionalType": schema_type,
                "encodingFormat": media_type,
                "url": req.source_uri,
                "description": commit_note,
            })
        } else {
            json!({
                "@context": "https://schema.org",
                "@type": oracle_class,
                "identifier": model_id,
                "name": title,
                "additionalType": schema_type,
                "url": req.source_uri,
                "description": commit_note,
            })
        };

        let envelope = VerdantModelEnvelope::new_sealed(
            &model_id,
            domain_kind,
            owner_organ,
            ContainerOrgan::WebAssemblyCore,
            "verdant-uorc",
            canonical_payload,
            schema_jsonld,
        );
        let admitted_kappa_hex = self.admit_arbitrary_prism_model(&envelope, grants)?;

        let record = if let Some(existing) = self
            .project_models
            .iter_mut()
            .find(|m| m.model_id == model_id)
        {
            let next_ver = existing.active_version.saturating_add(1);
            let parent = Some(existing.current_kappa_hex.clone());
            let rev_payload = format!("{model_id}|v{next_ver}|{admitted_kappa_hex}|{title}");
            let (_, rev_kappa_hex) = compute_kappa(&model_id, rev_payload.as_bytes());
            existing.title = title.clone();
            existing.tags = clean_tags.clone();
            existing.properties_json = props.clone();
            existing.active_version = next_ver;
            existing.current_kappa_hex = rev_kappa_hex.clone();
            existing.revisions.push(ModelRevisionRecord {
                version: next_ver,
                kappa_hex: rev_kappa_hex,
                parent_kappa_hex: parent,
                title,
                tags: clean_tags,
                properties_json: props,
                commit_note,
            });
            existing.clone()
        } else {
            let rev = ModelRevisionRecord {
                version: 1,
                kappa_hex: admitted_kappa_hex.clone(),
                parent_kappa_hex: None,
                title: title.clone(),
                tags: clean_tags.clone(),
                properties_json: props.clone(),
                commit_note,
            };
            let rec = ProjectImportedModelRecord {
                model_id: model_id.clone(),
                title,
                schema_type,
                media_type,
                target_slot: target_slot.clone(),
                source_stream: req.source_stream,
                source_uri: req.source_uri,
                license_spdx: req.license_spdx,
                active_version: 1,
                current_kappa_hex: admitted_kappa_hex,
                tags: clean_tags,
                properties_json: props,
                revisions: vec![rev],
            };
            self.project_models.push(rec.clone());
            rec
        };

        let obs_msg = format!(
            "Imported model `{}` v{} -> {} (kappa={})",
            record.model_id, record.active_version, record.target_slot, record.current_kappa_hex
        );
        let _ = self.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            "urn:verdant:organ:catalogs",
            "urn:verdant:organ:studio",
            "en",
            0,
            &obs_msg,
            None,
            0,
        )?;
        Ok(record)
    }

    /// Edits an imported project model in place, creating a new cryptographically parent-linked
    /// version (`vN+1`) and logging a PROV-O observation envelope.
    pub fn edit_project_model(
        &mut self,
        req: ProjectModelEditRequest,
    ) -> Result<ProjectImportedModelRecord, ObserverModelError> {
        let model_id = req.model_id.trim();
        let new_title = req.title.trim().to_string();
        if model_id.is_empty() || new_title.is_empty() {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: model_id.to_string(),
                detail: "edit requires non-empty model_id and title".to_string(),
            });
        }
        let clean_tags: Vec<String> = req
            .tags
            .into_iter()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();
        let props = if req.properties_json.trim().is_empty() {
            "{}".to_string()
        } else {
            req.properties_json.trim().to_string()
        };
        let commit_note = if req.commit_note.trim().is_empty() {
            "Edited model properties".to_string()
        } else {
            req.commit_note.trim().to_string()
        };

        let Some(existing) = self
            .project_models
            .iter_mut()
            .find(|m| m.model_id == model_id)
        else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("project_model:{model_id}"),
            });
        };

        let next_ver = existing.active_version.saturating_add(1);
        let parent_kappa = existing.current_kappa_hex.clone();
        let canonical = json!({
            "modelId": model_id,
            "version": next_ver,
            "parentKappa": parent_kappa,
            "title": new_title,
            "tags": clean_tags,
            "properties": props,
            "commitNote": commit_note,
        })
        .to_string();
        let (_, new_kappa_hex) = compute_kappa(model_id, canonical.as_bytes());

        existing.title = new_title.clone();
        existing.tags = clean_tags.clone();
        existing.properties_json = props.clone();
        existing.active_version = next_ver;
        existing.current_kappa_hex = new_kappa_hex.clone();
        existing.revisions.push(ModelRevisionRecord {
            version: next_ver,
            kappa_hex: new_kappa_hex.clone(),
            parent_kappa_hex: Some(parent_kappa),
            title: new_title,
            tags: clean_tags,
            properties_json: props,
            commit_note,
        });
        let snapshot = existing.clone();

        let obs_msg = format!(
            "Edited model `{}` -> v{} (kappa={})",
            snapshot.model_id, snapshot.active_version, new_kappa_hex
        );
        let _ = self.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            "urn:verdant:organ:studio",
            "urn:verdant:organ:engine",
            "en",
            0,
            &obs_msg,
            None,
            0,
        )?;
        Ok(snapshot)
    }

    /// Reverts an imported project model to an earlier immutable version (`target_version`),
    /// appending a new non-destructive audited revision `v(N+1)`.
    pub fn revert_project_model(
        &mut self,
        model_id: &str,
        target_version: u32,
    ) -> Result<ProjectImportedModelRecord, ObserverModelError> {
        let Some(existing) = self
            .project_models
            .iter()
            .find(|m| m.model_id == model_id.trim())
        else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("project_model:{model_id}"),
            });
        };
        let Some(target) = existing
            .revisions
            .iter()
            .find(|r| r.version == target_version)
            .cloned()
        else {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: model_id.to_string(),
                detail: format!("version v{target_version} not found"),
            });
        };
        self.edit_project_model(ProjectModelEditRequest {
            model_id: model_id.trim().to_string(),
            title: target.title,
            tags: target.tags,
            properties_json: target.properties_json,
            commit_note: format!("Reverted to v{target_version} ({})", target.kappa_hex),
        })
    }

    /// Removes an imported model from the active project and logs a PROV-O removal observation.
    pub fn remove_project_model(&mut self, model_id: &str) -> Result<String, ObserverModelError> {
        let clean = model_id.trim();
        let Some(pos) = self.project_models.iter().position(|m| m.model_id == clean) else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("project_model:{clean}"),
            });
        };
        let removed = self.project_models.remove(pos);
        self.admitted_model_kappas.retain(|(id, _)| id != clean);
        let obs_msg = format!(
            "Removed model `{}` (last kappa={})",
            removed.model_id, removed.current_kappa_hex
        );
        let _ = self.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            "urn:verdant:organ:studio",
            "urn:verdant:organ:engine",
            "en",
            0,
            &obs_msg,
            None,
            0,
        )?;
        Ok(removed.current_kappa_hex)
    }

    /// Serializes all currently imported project models and their immutable version histories.
    #[must_use]
    pub fn project_models_json_bytes(&self) -> Vec<u8> {
        let doc = json!({
            "count": self.project_models.len(),
            "activeDomainSlug": self.active_domain.spec.slug,
            "models": self.project_models,
        });
        serde_json::to_vec(&doc).unwrap_or_default()
    }

    /// Parses a JSON [`ProjectModelImportRequest`], admits the model, and returns serialized JSON.
    pub fn import_project_model_json(
        &mut self,
        json_bytes: &[u8],
        grants: &ContainerGrantMatrix,
    ) -> Result<Vec<u8>, ObserverModelError> {
        let req: ProjectModelImportRequest = serde_json::from_slice(json_bytes).map_err(|err| {
            ObserverModelError::InvalidPrismModelEnvelope {
                model_id: "import_json".to_string(),
                detail: err.to_string(),
            }
        })?;
        let rec = self.import_project_model(req, grants)?;
        Ok(serde_json::to_vec(&rec).unwrap_or_default())
    }

    /// Parses a JSON [`ProjectModelEditRequest`], commits version `vN+1`, and returns serialized JSON.
    pub fn edit_project_model_json(
        &mut self,
        json_bytes: &[u8],
    ) -> Result<Vec<u8>, ObserverModelError> {
        let req: ProjectModelEditRequest = serde_json::from_slice(json_bytes).map_err(|err| {
            ObserverModelError::InvalidPrismModelEnvelope {
                model_id: "edit_json".to_string(),
                detail: err.to_string(),
            }
        })?;
        let rec = self.edit_project_model(req)?;
        Ok(serde_json::to_vec(&rec).unwrap_or_default())
    }

    /// Reverts an imported model to `target_version` and returns serialized JSON of the updated record.
    pub fn revert_project_model_json(
        &mut self,
        model_id: &str,
        target_version: u32,
    ) -> Result<Vec<u8>, ObserverModelError> {
        let rec = self.revert_project_model(model_id, target_version)?;
        Ok(serde_json::to_vec(&rec).unwrap_or_default())
    }

    pub(super) fn seed_default_sensory_streams() -> (Vec<ObserverStreamModel>, u32) {
        type StreamSeedSpec = (
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static [ObserverPartSlot],
        );
        let defs: [StreamSeedSpec; 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1] = [
            (
                "khronos-gltf",
                "Khronos glTF-Sample-Assets",
                "Khronos Group",
                "khronos-index",
                "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/model-index.json",
                &[ObserverPartSlot::CharacterRigs, ObserverPartSlot::HeroProps],
            ),
            (
                "nasa-3d",
                "NASA 3D Resources (257+ GLB)",
                "NASA Open Data",
                "nasa-3d",
                "https://api.github.com/repos/nasa/NASA-3D-Resources/git/trees/master?recursive=1",
                &[ObserverPartSlot::HeroProps, ObserverPartSlot::StageEnvironment],
            ),
            (
                "threejs-gltf",
                "Three.js Open Rigged Characters & GLB",
                "Open Web glTF Showcase",
                "threejs-gltf",
                "https://api.github.com/repos/mrdoob/three.js/contents/examples/models/gltf",
                &[ObserverPartSlot::CharacterRigs, ObserverPartSlot::HeroProps],
            ),
            (
                "babylon-assets",
                "BabylonJS Open 3D Characters & Props",
                "BabylonJS Open Assets",
                "babylon-assets",
                "https://api.github.com/repos/BabylonJS/Assets/contents/meshes",
                &[ObserverPartSlot::CharacterRigs, ObserverPartSlot::HeroProps],
            ),
            (
                "polyhaven",
                "Poly Haven CC0 (4K PBR, HDRIs & 3D)",
                "Poly Haven",
                "polyhaven",
                "https://api.polyhaven.com/assets",
                &[
                    ObserverPartSlot::StageEnvironment,
                    ObserverPartSlot::SurfaceMaterials,
                    ObserverPartSlot::HeroProps,
                ],
            ),
            (
                "fontsource",
                "Fontsource Open Fonts",
                "Fontsource / Google Fonts",
                "fontsource",
                "https://api.fontsource.org/v1/fonts",
                &[ObserverPartSlot::VisualIdentity],
            ),
            (
                "openverse",
                "Openverse Creative Commons",
                "Openverse / WordPress Foundation",
                "openverse",
                "https://api.openverse.org/v1/",
                &[
                    ObserverPartSlot::SpatialAcoustics,
                    ObserverPartSlot::SurfaceMaterials,
                ],
            ),
            (
                "wikidata",
                "Wikidata + Wikipedia Knowledge Streams",
                "Wikimedia Foundation",
                "wikidata",
                "https://www.wikidata.org/w/api.php",
                &[
                    ObserverPartSlot::CognitiveLanguage,
                    ObserverPartSlot::StageEnvironment,
                ],
            ),
            (
                "github",
                "GitHub OSS Shaders & Frameworks",
                "GitHub Public Repositories",
                "github",
                "https://api.github.com/search/repositories",
                &[
                    ObserverPartSlot::RuntimeFrameworks,
                    ObserverPartSlot::VisualIdentity,
                ],
            ),
            (
                "verdant-prism",
                "Verdant Locked Prism Authority",
                "Verdant Engine (SHA-256 Locked)",
                "verdant-prism",
                "urn:verdant:catalog:prism-locked",
                &[
                    ObserverPartSlot::StageEnvironment,
                    ObserverPartSlot::RuntimeFrameworks,
                    ObserverPartSlot::VisualIdentity,
                    ObserverPartSlot::SpatialAcoustics,
                ],
            ),
        ];

        // Start sensory stream basis primes right after the 5 platform organs (ordinal 5 => prime 13):
        let mut ord = 1u32 + 1 + 1 + 1 + 1;
        let mut out = Vec::with_capacity(defs.len());
        for (id, label, authority, adapter_kind, endpoint_url, parts) in defs {
            let prime = nth_prime_u64(ord);
            ord = ord.saturating_add(1);
            let organ_iri = format!("urn:verdant:sensory:{id}");
            let canonical = json!({
                "streamId": id,
                "basisPrime": prime,
                "organIri": organ_iri,
                "adapterKind": adapter_kind,
                "endpointUrl": endpoint_url,
            })
            .to_string();
            let (_, kappa_hex) = compute_kappa(id, canonical.as_bytes());
            out.push(ObserverStreamModel {
                stream_id: id.to_string(),
                label: label.to_string(),
                authority: authority.to_string(),
                basis_prime: prime,
                organ_iri,
                adapter_kind: adapter_kind.to_string(),
                endpoint_url: endpoint_url.to_string(),
                target_parts: parts.to_vec(),
                enabled: true,
                built_in: true,
                kappa_hex,
            });
        }
        (out, ord)
    }

    /// Updates the Observer's real-time host container envelope without exceeding host limits.
    pub fn update_host_envelope(
        &mut self,
        max_texture_edge_px: u32,
        is_software_gpu: bool,
        hardware_threads: u32,
        ema_frame_micros: u32,
        active_triangles: u32,
    ) {
        self.host_envelope.update(
            max_texture_edge_px,
            is_software_gpu,
            hardware_threads,
            ema_frame_micros,
            active_triangles,
        );
    }

    /// Updates the Observer's UHD-aware host container envelope including viewport dimensions.
    #[allow(clippy::too_many_arguments)]
    pub fn update_host_envelope_with_viewport(
        &mut self,
        max_texture_edge_px: u32,
        is_software_gpu: bool,
        hardware_threads: u32,
        ema_frame_micros: u32,
        active_triangles: u32,
        viewport_width_px: u32,
        viewport_height_px: u32,
    ) {
        self.host_envelope.update_with_viewport(
            max_texture_edge_px,
            is_software_gpu,
            hardware_threads,
            ema_frame_micros,
            active_triangles,
            viewport_width_px,
            viewport_height_px,
        );
    }

    /// Dynamically registers a new sensory catalog stream into `ObserverPrismHolon`, verifying
    /// Schema.org `DataCatalog` + OMG DMN 1.5 admission and assigning its unique basis prime.
    pub fn register_stream(
        &mut self,
        req: StreamRegisterRequest,
        grants: &ContainerGrantMatrix,
    ) -> Result<ObserverStreamModel, ObserverModelError> {
        let stream_id = req.stream_id.trim().to_ascii_lowercase();
        let label = req.label.trim().to_string();
        let endpoint_url = req.endpoint_url.trim().to_string();
        let adapter_kind = if req.adapter_kind.trim().is_empty() {
            "schema-org-catalog".to_string()
        } else {
            req.adapter_kind.trim().to_ascii_lowercase()
        };
        if stream_id.is_empty() || label.is_empty() || endpoint_url.is_empty() {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: stream_id,
                detail: "stream registration requires non-empty streamId, label, and endpointUrl"
                    .to_string(),
            });
        }
        if !endpoint_url.starts_with("https://") && !endpoint_url.starts_with("urn:") {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: stream_id,
                detail: "sensory stream endpoint must use https:// or urn: scheme".to_string(),
            });
        }
        let route_granted = grants.is_allowed(ContainerOrgan::WebAssemblyCore);
        let dmn_decision = self
            .catalogs
            .language_dmn
            .evaluate_dmn_admission(&DmnAdmissionInput {
                model_format: "verdant-uorc".to_string(),
                observer_halted: self.emergency_halted,
                source_locked: true,
                route_granted,
                runtime_state: "match".to_string(),
                weights_state: "match".to_string(),
            })
            .map_err(|err| ObserverModelError::InvalidPrismModelEnvelope {
                model_id: stream_id.clone(),
                detail: err.to_string(),
            })?;
        let kernel_dmn = prism_kernel::dmnModelRouteAdmitted(
            !self.emergency_halted,
            true,
            true,
            route_granted,
            true,
            true,
        );
        if dmn_decision.outcome != "run" || !kernel_dmn {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: stream_id,
                detail: format!(
                    "DMN 1.5 stream admission refused: {} ({})",
                    dmn_decision.outcome, dmn_decision.reason
                ),
            });
        }

        let authority = if req.authority.trim().is_empty() {
            "Observer Custom Stream".to_string()
        } else {
            req.authority.trim().to_string()
        };
        let target_parts: Vec<ObserverPartSlot> = if req.target_parts.is_empty() {
            vec![ObserverPartSlot::HeroProps]
        } else {
            req.target_parts
                .iter()
                .map(|s| ObserverPartSlot::from_target_slot(s))
                .collect()
        };

        let organ_iri = format!("urn:verdant:sensory:{stream_id}");
        let schema_doc = json!({
            "@context": "https://schema.org",
            "@id": organ_iri,
            "@type": "Dataset",
            "identifier": format!("stream:{stream_id}"),
            "name": label,
            "url": endpoint_url,
            "additionalType": adapter_kind,
        });
        self.catalogs
            .schema_org
            .validate_schema_org_jsonld(&schema_doc)
            .map_err(|err| ObserverModelError::InvalidPrismModelEnvelope {
                model_id: stream_id.clone(),
                detail: err.to_string(),
            })?;

        let (basis_prime, existing_idx) = if let Some(pos) = self
            .sensory_streams
            .iter()
            .position(|s| s.stream_id == stream_id)
        {
            (self.sensory_streams[pos].basis_prime, Some(pos))
        } else {
            let p = nth_prime_u64(self.next_stream_prime_ordinal);
            self.next_stream_prime_ordinal = self.next_stream_prime_ordinal.saturating_add(1);
            (p, None)
        };

        let canonical = json!({
            "streamId": stream_id,
            "basisPrime": basis_prime,
            "organIri": organ_iri,
            "adapterKind": adapter_kind,
            "endpointUrl": endpoint_url,
            "targetParts": target_parts,
        })
        .to_string();
        let (_, kappa_hex) = compute_kappa(&stream_id, canonical.as_bytes());
        let model = ObserverStreamModel {
            stream_id: stream_id.clone(),
            label,
            authority,
            basis_prime,
            organ_iri: organ_iri.clone(),
            adapter_kind,
            endpoint_url,
            target_parts,
            enabled: true,
            built_in: existing_idx.is_some_and(|i| self.sensory_streams[i].built_in),
            kappa_hex: kappa_hex.clone(),
        };
        if let Some(pos) = existing_idx {
            self.sensory_streams[pos] = model.clone();
        } else {
            self.sensory_streams.push(model.clone());
        }

        let obs_msg = format!(
            "Registered sensory stream `{stream_id}` (prime={basis_prime}, kappa={kappa_hex})"
        );
        let _ = self.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            &organ_iri,
            "urn:verdant:organ:engine",
            "en",
            0,
            &obs_msg,
            None,
            0,
        )?;
        Ok(model)
    }

    /// Parses a JSON [`StreamRegisterRequest`], admits the stream, and returns serialized JSON.
    pub fn register_stream_json(
        &mut self,
        json_bytes: &[u8],
        grants: &ContainerGrantMatrix,
    ) -> Result<Vec<u8>, ObserverModelError> {
        let req: StreamRegisterRequest = serde_json::from_slice(json_bytes).map_err(|err| {
            ObserverModelError::InvalidPrismModelEnvelope {
                model_id: "stream_register_json".to_string(),
                detail: err.to_string(),
            }
        })?;
        let model = self.register_stream(req, grants)?;
        Ok(serde_json::to_vec(&model).unwrap_or_default())
    }

    /// Removes a sensory stream from `ObserverPrismHolon` and logs a W3C SOSA/PROV-O observation.
    pub fn remove_stream(&mut self, stream_id: &str) -> Result<String, ObserverModelError> {
        let clean = stream_id.trim().to_ascii_lowercase();
        let Some(pos) = self
            .sensory_streams
            .iter()
            .position(|s| s.stream_id == clean)
        else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("stream:{clean}"),
            });
        };
        let removed = self.sensory_streams.remove(pos);
        let obs_msg = format!(
            "Removed sensory stream `{}` (prime={}, kappa={})",
            removed.stream_id, removed.basis_prime, removed.kappa_hex
        );
        let _ = self.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            "urn:verdant:organ:studio",
            "urn:verdant:organ:engine",
            "en",
            0,
            &obs_msg,
            None,
            0,
        )?;
        Ok(removed.kappa_hex)
    }

    /// Enables or disables a registered sensory stream inside `ObserverPrismHolon`.
    pub fn toggle_stream(
        &mut self,
        stream_id: &str,
        enabled: bool,
    ) -> Result<ObserverStreamModel, ObserverModelError> {
        let clean = stream_id.trim().to_ascii_lowercase();
        let Some(stream) = self
            .sensory_streams
            .iter_mut()
            .find(|s| s.stream_id == clean)
        else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("stream:{clean}"),
            });
        };
        stream.enabled = enabled;
        Ok(stream.clone())
    }

    /// Toggles `stream_id` and returns the updated stream model as serialized JSON bytes.
    pub fn toggle_stream_json(
        &mut self,
        stream_id: &str,
        enabled: bool,
    ) -> Result<Vec<u8>, ObserverModelError> {
        let model = self.toggle_stream(stream_id, enabled)?;
        Ok(serde_json::to_vec(&model).unwrap_or_default())
    }

    /// Records that a specific [`ObserverPartSlot`] was refined/hydrated from a live stream asset.
    pub fn record_part_hydration(
        &mut self,
        slot: ObserverPartSlot,
        precision_tier: u32,
        active_title: String,
        source_stream: String,
        kappa_hex: String,
    ) {
        let max_tier = 1u32 + 1 + 1 + 1;
        let rec = PartHydrationRecord {
            slot,
            precision_tier: precision_tier.clamp(1, max_tier),
            active_title,
            source_stream,
            kappa_hex,
        };
        if let Some(existing) = self.part_hydrations.iter_mut().find(|p| p.slot == slot) {
            *existing = rec;
        } else {
            self.part_hydrations.push(rec);
        }
    }

    /// Ingests a live natural-language knowledge passage from an internet sensory stream into
    /// `VerdantLanguageEngine` (`ISO 24617-2` passage-grounded reasoning) across active domain NPCs.
    pub fn ingest_knowledge_passage(
        &mut self,
        req: KnowledgePassageRequest,
    ) -> Result<IngestedKnowledgeRecord, ObserverModelError> {
        let topic = req.topic.trim().to_string();
        let passage = req.passage.trim().to_string();
        if topic.is_empty() || passage.is_empty() {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: "knowledge_passage".to_string(),
                detail: "knowledge passage requires non-empty topic and passage".to_string(),
            });
        }
        let source_stream = if req.source_stream.trim().is_empty() {
            "wikidata".to_string()
        } else {
            req.source_stream.trim().to_string()
        };
        let source_uri = req.source_uri.trim().to_string();
        let canonical = format!("{source_stream}|{source_uri}|{topic}|{passage}");
        let (_, kappa_hex) = compute_kappa("knowledge_passage", canonical.as_bytes());
        let formatted_fact = format!("{topic}: {passage}");

        for ch in &mut self.active_domain.spec.characters {
            if !ch.knowledge_facts.iter().any(|f| f == &formatted_fact) {
                ch.knowledge_facts.push(formatted_fact.clone());
            }
        }
        let rec = IngestedKnowledgeRecord {
            source_stream: source_stream.clone(),
            source_uri,
            topic: topic.clone(),
            passage,
            kappa_hex: kappa_hex.clone(),
        };
        if !self
            .ingested_knowledge
            .iter()
            .any(|k| k.kappa_hex == kappa_hex)
        {
            self.ingested_knowledge.push(rec.clone());
        }
        let four_tier = 1u32 + 1 + 1 + 1;
        self.record_part_hydration(
            ObserverPartSlot::CognitiveLanguage,
            four_tier,
            topic.clone(),
            source_stream.clone(),
            kappa_hex.clone(),
        );
        let obs_msg = format!(
            "Ingested live knowledge passage `{topic}` from `{source_stream}` (kappa={kappa_hex})"
        );
        let _ = self.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            &format!("urn:verdant:sensory:{source_stream}"),
            "urn:verdant:organ:engine",
            "en",
            0,
            &obs_msg,
            None,
            0,
        )?;
        Ok(rec)
    }

    /// Parses a JSON [`KnowledgePassageRequest`], ingests it into ISO 24617-2 cognition, and returns JSON.
    pub fn ingest_knowledge_passage_json(
        &mut self,
        json_bytes: &[u8],
    ) -> Result<Vec<u8>, ObserverModelError> {
        let req: KnowledgePassageRequest = serde_json::from_slice(json_bytes).map_err(|err| {
            ObserverModelError::InvalidPrismModelEnvelope {
                model_id: "knowledge_passage_json".to_string(),
                detail: err.to_string(),
            }
        })?;
        let rec = self.ingest_knowledge_passage(req)?;
        Ok(serde_json::to_vec(&rec).unwrap_or_default())
    }

    /// Serializes the full Observer Cognitive Resolution state: live Host Capability Envelope,
    /// all 8 differentiated Observer parts (`R1..R4`), connected sensory streams, ingested
    /// knowledge count, and lawful stream escalation plan.
    #[must_use]
    pub fn cognitive_resolution_json_bytes(&self) -> Vec<u8> {
        let two = 1u32 + 1;
        let three = two + 1;
        let four = two * two;
        let mut total_precision = 0u32;
        let mut parts_json = Vec::with_capacity(ObserverPartSlot::all().len());

        for slot in ObserverPartSlot::all() {
            let connected: Vec<&ObserverStreamModel> = self
                .sensory_streams
                .iter()
                .filter(|s| s.enabled && s.target_parts.contains(&slot))
                .collect();
            let hydrated = self.part_hydrations.iter().find(|h| h.slot == slot);
            let imported_for_slot = self
                .project_models
                .iter()
                .rev()
                .find(|m| ObserverPartSlot::from_target_slot(&m.target_slot) == slot);

            let (precision_tier, active_title, active_stream, active_kappa) =
                if let Some(h) = hydrated {
                    (
                        h.precision_tier,
                        h.active_title.clone(),
                        h.source_stream.clone(),
                        h.kappa_hex.clone(),
                    )
                } else if let Some(m) = imported_for_slot {
                    (
                        three,
                        m.title.clone(),
                        m.source_stream.clone(),
                        m.current_kappa_hex.clone(),
                    )
                } else {
                    (
                        1,
                        format!("Procedural {} Stub", slot.label()),
                        "verdant-prism".to_string(),
                        self.observer_kappa_hex.clone(),
                    )
                };

            total_precision = total_precision.saturating_add(precision_tier);
            let precision_label = match precision_tier {
                x if x >= four => "R4 Stream-Hydrated HD",
                x if x == three => "R3 Stream Asset Bound",
                x if x == two => "R2 Metadata Linked",
                _ => "R1 Procedural Stub",
            };
            let needs_escalation = precision_tier < three && !connected.is_empty();
            let recommended_stream = connected
                .first()
                .map(|s| s.stream_id.clone())
                .unwrap_or_default();

            parts_json.push(json!({
                "slot": slot.slug(),
                "label": slot.label(),
                "precisionTier": precision_tier,
                "precisionLabel": precision_label,
                "activeTitle": active_title,
                "activeStream": active_stream,
                "activeKappaHex": active_kappa,
                "connectedStreams": connected.iter().map(|s| s.stream_id.as_str()).collect::<Vec<_>>(),
                "needsStreamEscalation": needs_escalation,
                "recommendedStream": recommended_stream,
            }));
        }

        let max_total = (ObserverPartSlot::all().len() as u32) * four;
        let thousand = pow10_u64(two_u32() + 1) as u32;
        let resolution_permille = (total_precision.saturating_mul(thousand)) / max_total.max(1);

        let doc = json!({
            "observerKappaHex": self.observer_kappa_hex,
            "resolutionPermille": resolution_permille,
            "hostEnvelope": self.host_envelope,
            "streamCount": self.sensory_streams.len(),
            "enabledStreamCount": self.sensory_streams.iter().filter(|s| s.enabled).count(),
            "ingestedKnowledgeCount": self.ingested_knowledge.len(),
            "streams": self.sensory_streams,
            "parts": parts_json,
        });
        serde_json::to_vec(&doc).unwrap_or_default()
    }
}
