#[cfg(test)]
mod naming_tests {
    use rstest::rstest;
    use std::path::Path;
    use crate::manifest::*;
    use crate::checkers::naming::check_naming_slice;

    fn make_l1_manifest(id: &str, verb: &str, declared_layer: Layer, inferred_layer: Layer) -> Manifest {
        Manifest {
            manifest: ManifestBody {
                id: id.to_string(),
                version: "1.0.0".to_string(),
                schema_version: "1.0".to_string(),
                identity: Identity {
                    purpose: "test".to_string(),
                    domain: "auth".to_string(),
                    subdomain: "token".to_string(),
                    verb: verb.to_string(),
                    entity: "sig".to_string(),
                },
                layer: LayerSection { declared: declared_layer, inferred: inferred_layer },
                contract: Contract {
                    input: ContractInput { type_name: "T".to_string(), constraints: vec![] },
                    output: ContractOutput {
                        success: "S".to_string(),
                        failure: "AuthError.FAIL".to_string(),
                    },
                    side_effects: SideEffects::None,
                    idempotent: true,
                    deterministic: true,
                },
                type_state: None,
                dependencies: vec![],
                composition: None,
                saga_participant: None,
                context_budget: None,
                test_contract: None,
                behavioral_contract: None,
                stability: Stability::Stable,
                lifecycle: Lifecycle {
                    phase: LifecyclePhase::Stable,
                    candidate_since: None,
                    stable_since: None,
                    deprecated_since: None,
                    sunset_at: None,
                    migration_aru: None,
                    tombstoned_at: None,
                },
                health_contract: None,
                diagnostic_surface: None,
                manifest_provenance: ManifestProvenance {
                    derived_by: "STATIC_ANALYSIS".to_string(),
                    reviewed_by: "REVIEWER_AGENT".to_string(),
                    approved_at: "2024-01-01T00:00:00Z".to_string(),
                    bundle_version: None,
                },
            }
        }
    }

    #[rstest]
    #[case("auth.token.validate.sig", "validate", Layer::L1, Layer::L1, false)]
    #[case("auth.token.transform.sig", "transform", Layer::L1, Layer::L1, false)]
    #[case("auth.token.validate.sig", "validate", Layer::L2, Layer::L1, true)] // mismatch
    #[case("auth.token.execute.flow", "execute", Layer::L1, Layer::L3, true)]  // mismatch
    fn test_layer_consistency(
        #[case] id: &str,
        #[case] verb: &str,
        #[case] declared: Layer,
        #[case] inferred: Layer,
        #[case] expect_error: bool,
    ) {
        let manifest = make_l1_manifest(id, verb, declared.clone(), inferred.clone());
        let path = std::path::PathBuf::from("test.manifest.yaml");
        let results = check_naming_slice(&[(&path, &manifest)]);
        let errors: Vec<_> = results.iter().filter(|d| matches!(d.severity, crate::checker::Severity::Error)).collect();
        if expect_error {
            assert!(!errors.is_empty(), "Expected error for {id} declared={declared} inferred={inferred}");
        } else {
            assert!(errors.is_empty(), "Expected no error for {id}: {:?}", errors);
        }
    }

    #[test]
    fn test_unknown_verb_fails() {
        let manifest = make_l1_manifest("auth.token.frobulate.sig", "frobulate", Layer::L1, Layer::L1);
        let path = std::path::PathBuf::from("test.manifest.yaml");
        let results = check_naming_slice(&[(&path, &manifest)]);
        assert!(results.iter().any(|d| d.message.contains("Unknown verb")));
    }

    #[test]
    fn test_duplicate_address_fails() {
        let m1 = make_l1_manifest("auth.token.validate.sig", "validate", Layer::L1, Layer::L1);
        let m2 = make_l1_manifest("auth.token.validate.sig", "validate", Layer::L1, Layer::L1);
        let p1 = std::path::PathBuf::from("a.manifest.yaml");
        let p2 = std::path::PathBuf::from("b.manifest.yaml");
        let results = check_naming_slice(&[(&p1, &m1), (&p2, &m2)]);
        assert!(results.iter().any(|d| d.message.contains("duplicate") || d.message.contains("Duplicate")));
    }

    #[test]
    fn test_invalid_error_type_fails() {
        let mut manifest = make_l1_manifest("auth.token.validate.sig", "validate", Layer::L1, Layer::L1);
        manifest.manifest.contract.output.failure = "generic_error".to_string();
        let path = std::path::PathBuf::from("test.manifest.yaml");
        let results = check_naming_slice(&[(&path, &manifest)]);
        assert!(results.iter().any(|d| d.message.contains("Domain")));
    }

    fn make_manifest_with_id(id: &str) -> Manifest {
        // Use a verb consistent with L1 so layer checks pass
        make_l1_manifest(id, "validate", Layer::L1, Layer::L1)
    }

    // ── Segment casing tests ──────────────────────────────────────────────────

    #[rstest]
    // Valid: all-lowercase entity
    #[case("url.link.create.original", false)]
    // Valid: camelCase entity
    #[case("url.link.create.fromOriginal", false)]
    #[case("url.store.resolve.shortCode", false)]
    #[case("auth.session.create.fromToken", false)]
    // Invalid: kebab-case entity
    #[case("url.link.create.from-original", true)]
    #[case("url.store.resolve.short-code", true)]
    // Invalid: PascalCase entity (reserved for L0 types)
    #[case("url.link.create.FromOriginal", true)]
    // Invalid: uppercase domain
    #[case("URL.link.create.fromOriginal", true)]
    // Invalid: uppercase subdomain
    #[case("url.Link.create.fromOriginal", true)]
    // Invalid: underscore in domain
    #[case("url_short.link.create.fromOriginal", true)]
    // Invalid: underscore in entity
    #[case("url.link.create.from_original", true)]
    fn test_segment_casing(#[case] id: &str, #[case] expect_error: bool) {
        let manifest = make_manifest_with_id(id);
        let path = std::path::PathBuf::from("test.manifest.yaml");
        let results = check_naming_slice(&[(&path, &manifest)]);
        let errors: Vec<_> = results.iter()
            .filter(|d| matches!(d.severity, crate::checker::Severity::Error))
            .filter(|d| d.message.contains("kebab-case") || d.message.contains("camelCase") || d.message.contains("entity"))
            .collect();
        if expect_error {
            assert!(!errors.is_empty(), "Expected casing error for id '{id}'");
        } else {
            assert!(errors.is_empty(), "Expected no casing error for id '{id}': {:?}", errors);
        }
    }
}

#[cfg(test)]
mod cycle_tests {
    use std::path::PathBuf;
    use crate::manifest::*;
    use crate::graph::SemanticGraph;

    fn manifest_with_deps(id: &str, layer: Layer, deps: Vec<&str>) -> (PathBuf, Manifest) {
        let path = PathBuf::from(format!("{}.manifest.yaml", id.replace('.', "-")));
        let manifest = Manifest {
            manifest: ManifestBody {
                id: id.to_string(),
                version: "1.0.0".to_string(),
                schema_version: "1.0".to_string(),
                identity: Identity {
                    purpose: "test".to_string(),
                    domain: id.split('.').next().unwrap_or("test").to_string(),
                    subdomain: "sub".to_string(),
                    verb: "validate".to_string(),
                    entity: "thing".to_string(),
                },
                layer: LayerSection { declared: layer.clone(), inferred: layer },
                contract: Contract {
                    input: ContractInput { type_name: "T".to_string(), constraints: vec![] },
                    output: ContractOutput { success: "S".to_string(), failure: "DomainError.FAIL".to_string() },
                    side_effects: SideEffects::None,
                    idempotent: true,
                    deterministic: true,
                },
                type_state: None,
                dependencies: deps.into_iter().map(|d| Dependency {
                    id: d.to_string(),
                    layer: Layer::L1,
                    version_pin: None,
                    stability: Stability::Stable,
                }).collect(),
                composition: None,
                saga_participant: None,
                context_budget: None,
                test_contract: None,
                behavioral_contract: None,
                stability: Stability::Stable,
                lifecycle: Lifecycle {
                    phase: LifecyclePhase::Stable,
                    candidate_since: None,
                    stable_since: None,
                    deprecated_since: None,
                    sunset_at: None,
                    migration_aru: None,
                    tombstoned_at: None,
                },
                health_contract: None,
                diagnostic_surface: None,
                manifest_provenance: ManifestProvenance {
                    derived_by: "STATIC_ANALYSIS".to_string(),
                    reviewed_by: "REVIEWER_AGENT".to_string(),
                    approved_at: "2024-01-01T00:00:00Z".to_string(),
                    bundle_version: None,
                },
            }
        };
        (path, manifest)
    }

    #[test]
    fn no_cycle_in_dag() {
        let manifests = vec![
            manifest_with_deps("a.sub.validate.x", Layer::L1, vec!["b.sub.validate.y"]),
            manifest_with_deps("b.sub.validate.y", Layer::L1, vec![]),
        ];
        let graph = SemanticGraph::build(&manifests);
        let diagnostics = graph.check_cycles();
        assert!(diagnostics.is_empty(), "Expected no cycles: {:?}", diagnostics);
    }

    #[test]
    fn cycle_detected() {
        let manifests = vec![
            manifest_with_deps("a.sub.validate.x", Layer::L1, vec!["b.sub.validate.y"]),
            manifest_with_deps("b.sub.validate.y", Layer::L1, vec!["a.sub.validate.x"]),
        ];
        let graph = SemanticGraph::build(&manifests);
        let diagnostics = graph.check_cycles();
        assert!(!diagnostics.is_empty(), "Expected cycle to be detected");
    }
}

#[cfg(test)]
mod incremental_tests {
    use std::path::PathBuf;
    use std::sync::Arc;
    use crate::db::{AriaDatabase, ManifestValidation};

    fn make_l1_yaml(id: &str, verb: &str) -> String {
        format!(r#"manifest:
  id: "{id}"
  version: "1.0.0"
  schema_version: "1.0"
  identity:
    purpose: "test"
    domain: "auth"
    subdomain: "token"
    verb: "{verb}"
    entity: "thing"
  layer:
    declared: L1
    inferred: L1
  contract:
    input:
      type: "T"
    output:
      success: "S"
      failure: "AuthError.FAIL"
    side_effects: NONE
    idempotent: true
    deterministic: true
  dependencies:
    - id: "auth.entity"
      layer: L0
      stability: STABLE
  context_budget:
    to_use: 100
    to_modify: 300
    to_extend: 500
    to_replace: 700
  test_contract:
    scenarios:
      - scenario: "Test"
    coverage_required: true
  stability: STABLE
  lifecycle:
    phase: STABLE
    stable_since: "2024-01-01T00:00:00Z"
  manifest_provenance:
    derived_by: STATIC_ANALYSIS
    reviewed_by: REVIEWER_AGENT
    approved_at: "2024-01-01T00:00:00Z"
"#)
    }

    #[test]
    fn incremental_rebuild_only_affects_changed_manifest() {
        let mut db = AriaDatabase::new();
        let paths: Vec<PathBuf> = (0..10)
            .map(|i| PathBuf::from(format!("manifest{i}.yaml")))
            .collect();

        // Load 10 manifests
        for (i, path) in paths.iter().enumerate() {
            let yaml = make_l1_yaml(
                &format!("auth.sub{i}.validate.thing{i}"),
                "validate",
            );
            db.set_manifest_file_content(path.clone(), Arc::new(yaml));
        }

        // Run all diagnostics — should all be empty for valid manifests
        for path in &paths {
            let diags = db.naming_diagnostics(path.clone());
            assert!(diags.is_empty(), "Expected no diagnostics for {}: {:?}", path.display(), *diags);
        }

        // Mutate one manifest to have an invalid verb
        let mutated_path = paths[3].clone();
        let bad_yaml = make_l1_yaml("auth.sub3.frobulate.thing3", "frobulate");
        db.set_manifest_file_content(mutated_path.clone(), Arc::new(bad_yaml));

        // Only the mutated manifest should now have diagnostics
        let mutated_diags = db.naming_diagnostics(mutated_path);
        assert!(!mutated_diags.is_empty(), "Expected error after mutation");

        // Others should still be clean
        for (i, path) in paths.iter().enumerate() {
            if i == 3 { continue; }
            let diags = db.naming_diagnostics(path.clone());
            assert!(diags.is_empty(), "Unexpected diagnostics for non-mutated {}", path.display());
        }
    }
}
