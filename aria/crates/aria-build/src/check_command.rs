use std::path::Path;
use anyhow::Result;
use serde_json::{json, Value};
use aria_core::{
    Manifest, SemanticGraph,
    checker::{Diagnostic, Severity},
    checkers::{check_schema, check_naming, check_cycles, check_cross_domain, check_type_compatibility},
};
use crate::cli::OutputFormat;

pub struct CheckConfig {
    pub compliance_level: u8,
    pub format: OutputFormat,
}

/// Run all enabled checks based on compliance level.
/// Returns (diagnostics, exit_code) — exit code 0 means pass.
pub fn run_check(
    manifests: &[(std::path::PathBuf, Manifest)],
    parse_errors: &[(std::path::PathBuf, String)],
    config: &CheckConfig,
) -> (Vec<Diagnostic>, i32) {
    let mut all_diags: Vec<Diagnostic> = vec![];

    // Add parse errors as diagnostics
    for (path, msg) in parse_errors {
        all_diags.push(Diagnostic::error(path, 0, 0, format!("YAML parse error: {}", msg)));
    }

    // Level 0: Schema validation
    if config.compliance_level >= 0 {
        for (path, manifest) in manifests {
            let diags = check_schema(path, manifest);
            all_diags.extend(diags);
        }
    }

    // Level 1: Naming enforcement
    if config.compliance_level >= 1 {
        let diags = check_naming(manifests);
        all_diags.extend(diags);
    }

    // Level 2+: Type graph checks
    if config.compliance_level >= 2 {
        let graph = SemanticGraph::build(manifests);
        all_diags.extend(check_cycles(&graph));
        all_diags.extend(check_cross_domain(&graph));
        all_diags.extend(check_type_compatibility(&graph));
    }

    let has_errors = all_diags.iter().any(|d| matches!(d.severity, Severity::Error));
    let exit_code = if has_errors { 1 } else { 0 };
    (all_diags, exit_code)
}

/// Format diagnostics for output.
pub fn format_diagnostics(
    diagnostics: &[Diagnostic],
    manifests_checked: usize,
    compliance_level: u8,
    format: &OutputFormat,
) -> String {
    match format {
        OutputFormat::Json => {
            let items: Vec<Value> = diagnostics.iter().map(|d| json!({
                "severity": format!("{}", d.severity),
                "file": d.file.display().to_string(),
                "line": d.line,
                "col": d.col,
                "message": d.message,
            })).collect();

            let error_count = diagnostics.iter().filter(|d| matches!(d.severity, Severity::Error)).count();
            let warn_count = diagnostics.iter().filter(|d| matches!(d.severity, Severity::Warn)).count();

            serde_json::to_string_pretty(&json!({
                "manifests_checked": manifests_checked,
                "compliance_level": compliance_level,
                "errors": error_count,
                "warnings": warn_count,
                "diagnostics": items,
            })).unwrap_or_default()
        }
        OutputFormat::Text => {
            let mut output = String::new();
            for d in diagnostics {
                output.push_str(&d.format_cli());
                output.push('\n');
            }
            output
        }
    }
}
