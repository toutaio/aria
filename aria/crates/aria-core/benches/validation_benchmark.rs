use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aria_core::{Manifest, SemanticGraph, checkers};
use std::path::PathBuf;

/// Generate N synthetic valid L1 manifests in memory.
fn generate_manifests(n: usize) -> Vec<(PathBuf, Manifest)> {
    let verbs = ["validate", "transform", "compute", "generate", "encode", "decode", "hash", "compare"];
    let domains = ["auth", "user", "billing", "notification", "payment"];

    (0..n).map(|i| {
        let domain = domains[i % domains.len()];
        let verb = verbs[i % verbs.len()];
        let entity = format!("entity{}", i);
        let yaml = format!(r#"manifest:
  id: "{domain}.sub{i}.{verb}.{entity}"
  version: "1.0.0"
  schema_version: "1.0"
  identity:
    purpose: "Synthetic benchmark manifest {i}"
    domain: "{domain}"
    subdomain: "sub{i}"
    verb: "{verb}"
    entity: "{entity}"
  layer:
    declared: L1
    inferred: L1
  contract:
    input:
      type: "InputType{i}"
    output:
      success: "SuccessType{i}"
      failure: "DomainError.FAIL"
    side_effects: NONE
    idempotent: true
    deterministic: true
  dependencies:
    - id: "{domain}.entity{i}"
      layer: L0
      stability: STABLE
  context_budget:
    to_use: 100
    to_modify: 300
    to_extend: 500
    to_replace: 700
  test_contract:
    scenarios:
      - scenario: "Happy path passes"
    coverage_required: true
  stability: STABLE
  lifecycle:
    phase: STABLE
    stable_since: "2024-01-01T00:00:00Z"
  manifest_provenance:
    derived_by: STATIC_ANALYSIS
    reviewed_by: REVIEWER_AGENT
    approved_at: "2024-01-01T00:00:00Z"
"#);
        let manifest = Manifest::from_yaml(&yaml).expect("synthetic manifest must be valid");
        (PathBuf::from(format!("bench/{}.manifest.yaml", i)), manifest)
    }).collect()
}

fn benchmark_validation(c: &mut Criterion) {
    let manifests = generate_manifests(200);

    c.bench_function("validate 200 manifests (naming)", |b| {
        b.iter(|| {
            let pairs: Vec<(&std::path::Path, &Manifest)> = manifests
                .iter()
                .map(|(p, m)| (p.as_path(), m))
                .collect();
            let result = checkers::naming::check_naming_slice(black_box(&pairs));
            result
        })
    });
}

criterion_group!(benches, benchmark_validation);
criterion_main!(benches);
