use std::path::{Path, PathBuf};
use crate::manifest::Manifest;
use crate::checker::{Diagnostic, CheckResult};

/// JSON Schema for manifests, embedded at compile time.
static SCHEMA_JSON: &str = include_str!("../../../../schema/aria-manifest.schema.json");

/// Validate a manifest against the JSON Schema.
/// Returns diagnostics for each violation found.
pub fn check_schema(file: &Path, manifest: &Manifest) -> CheckResult {
    let schema_value: serde_json::Value = match serde_json::from_str(SCHEMA_JSON) {
        Ok(v) => v,
        Err(e) => return vec![Diagnostic::error(
            file, 0, 0,
            format!("Internal error: failed to parse embedded schema: {}", e),
        )],
    };

    let instance = match serde_json::to_value(manifest) {
        Ok(v) => v,
        Err(e) => return vec![Diagnostic::error(
            file, 0, 0,
            format!("Failed to serialize manifest for schema validation: {}", e),
        )],
    };

    match jsonschema::validate(&schema_value, &instance) {
        Ok(_) => vec![],
        Err(err) => vec![Diagnostic::error(
            file, 0, 0,
            format!("Schema violation: {}", err),
        )],
    }
}

/// Validate raw YAML text against the JSON Schema.
/// Used by the Salsa incremental database for LSP diagnostics.
pub fn check_schema_str(file: &Path, content: &str) -> CheckResult {
    let manifest = match Manifest::from_yaml(content) {
        Ok(m) => m,
        Err(e) => return vec![Diagnostic::error(
            file, 0, 0,
            format!("YAML parse error: {}", e),
        )],
    };
    check_schema(file, &manifest)
}
