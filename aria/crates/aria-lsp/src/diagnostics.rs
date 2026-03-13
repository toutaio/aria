use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tower_lsp::lsp_types::Url;

use aria_core::{checker::Diagnostic, db::AriaDatabase, db::ManifestValidation};

/// Validate a .manifest.yaml document and return diagnostics.
/// Updates the Salsa incremental database with the new content.
pub fn validate_document(
    db: &Arc<Mutex<AriaDatabase>>,
    uri: &Url,
    text: &str,
) -> Vec<Diagnostic> {
    if !uri.path().ends_with(".manifest.yaml") {
        return vec![];
    }

    let path = match uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return vec![],
    };

    let mut guard = match db.lock() {
        Ok(g) => g,
        Err(_) => return vec![],
    };

    guard.set_manifest_file_content(path.clone(), Arc::new(text.to_string()));

    let mut diags = Vec::new();
    diags.extend(
        guard.schema_diagnostics(path.clone())
             .iter()
             .map(|r| r.to_diagnostic())
    );
    diags.extend(
        guard.naming_diagnostics(path)
             .iter()
             .map(|r| r.to_diagnostic())
    );

    diags
}
