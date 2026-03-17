use sha2::{Sha256, Digest};
use serde_json::Value;
use crate::manifest::{ManifestBody, Composition};

/// Compute a canonical SHA-256 hash of a manifest's `composition:` section.
///
/// Uses deterministic JSON serialization (sorted keys) so that cosmetic YAML changes
/// (whitespace, comments, field reordering) do not produce false-positive stale detections.
///
/// Returns a 32-byte hash. Use `hex::encode(canonical_hash(...))` for display.
pub fn canonical_hash(manifest: &ManifestBody) -> [u8; 32] {
    // Serialize composition section to a normalized JSON form
    let json_value = match &manifest.composition {
        Some(comp) => {
            let v = serde_json::to_value(comp)
                .unwrap_or(Value::Null);
            sort_json_keys(v)
        }
        None => Value::Null,
    };

    let json_bytes = serde_json::to_vec(&json_value)
        .unwrap_or_default();

    let mut hasher = Sha256::new();
    hasher.update(&json_bytes);
    hasher.finalize().into()
}

/// Recursively sort all JSON object keys for deterministic serialization.
fn sort_json_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted: serde_json::Map<String, Value> = serde_json::Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                let v = map.get(&key).cloned().unwrap_or(Value::Null);
                sorted.insert(key, sort_json_keys(v));
            }
            Value::Object(sorted)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(sort_json_keys).collect()),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::*;

    fn minimal_manifest() -> ManifestBody {
        ManifestBody {
            id: "auth.token.validate.sig".into(),
            version: "1.0.0".into(),
            schema_version: "1.0".into(),
            identity: Identity {
                purpose: "test".into(),
                domain: "auth".into(),
                subdomain: "token".into(),
                verb: "validate".into(),
                entity: "sig".into(),
            },
            layer: LayerSection {
                declared: Layer::L1,
                inferred: Some(Layer::L1),
            },
            contract: Contract {
                input: ContractInput { type_name: "T".into(), constraints: vec![] },
                output: ContractOutput { success: "S".into(), failure: "AuthError.FAIL".into() },
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
                derived_by: "STATIC_ANALYSIS".into(),
                reviewed_by: "REVIEWER_AGENT".into(),
                approved_at: "2024-01-01T00:00:00Z".into(),
                bundle_version: None,
            },
        }
    }

    #[test]
    fn hash_is_deterministic() {
        let m = minimal_manifest();
        let h1 = canonical_hash(&m);
        let h2 = canonical_hash(&m);
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_changes_with_composition_change() {
        let mut m = minimal_manifest();
        let hash_without = canonical_hash(&m);

        m.composition = Some(Composition {
            pattern: CompositionPattern::Pipe,
            chain: Some(vec!["auth.token.validate.sig".into()]),
            ..Default::default()
        });

        let hash_with = canonical_hash(&m);
        assert_ne!(hash_without, hash_with);
    }
}
