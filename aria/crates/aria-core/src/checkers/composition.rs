use std::path::Path;
use crate::manifest::Manifest;
use crate::checker::{Diagnostic, CheckResult};
use crate::canonical_hash::canonical_hash;
use hex;

/// Check if a generated file is stale by comparing the embedded manifest hash
/// in the generated file's header comment against the current manifest hash.
///
/// Generated files contain a header line:
///   `// @aria-manifest-hash: <sha256-hex>`
pub fn check_stale_generated_files(
    manifest_file: &Path,
    manifest: &Manifest,
    generated_file: &Path,
    generated_content: &str,
) -> CheckResult {
    let current_hash = hex::encode(canonical_hash(&manifest.manifest));

    // Extract embedded hash from generated file header
    let embedded_hash = generated_content
        .lines()
        .find(|l| l.contains("@aria-manifest-hash:"))
        .and_then(|l| l.split("@aria-manifest-hash:").nth(1))
        .map(|h| h.trim().to_string());

    match embedded_hash {
        None => vec![Diagnostic::warn(
            manifest_file, 0, 0,
            format!(
                "Generated file '{}' has no @aria-manifest-hash header — run aria-build generate to regenerate",
                generated_file.display()
            ),
        )],
        Some(hash) if hash != current_hash => vec![Diagnostic::error(
            manifest_file, 0, 0,
            format!(
                "Generated file '{}' is stale (hash mismatch: embedded={}, current={}) — run aria-build generate",
                generated_file.display(), hash, current_hash
            ),
        )],
        Some(_) => vec![],
    }
}
