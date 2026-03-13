use std::fs;
use std::path::Path;
use anyhow::Result;
use serde_json::{json, Value};
use sha2::{Sha256, Digest};
use hex;
use aria_core::{Manifest, manifest::Stability};

/// Build the manifest bundle: collect STABLE manifests, emit Level 1 signatures
/// to `.aria/manifest-bundle.json` with a SHA-256 `bundle_version`.
pub fn run_bundle(
    manifests: &[(std::path::PathBuf, Manifest)],
    project_root: &Path,
    domain_filter: Option<&str>,
) -> Result<()> {
    let stable: Vec<_> = manifests
        .iter()
        .filter(|(_, m)| {
            // Filter by stability
            if m.manifest.stability != Stability::Stable && m.manifest.stability != Stability::Frozen {
                return false;
            }
            // Filter by domain if specified
            if let Some(domain) = domain_filter {
                if !m.manifest.id.starts_with(&format!("{}.", domain)) {
                    return false;
                }
            }
            true
        })
        .collect();

    // Build Level 1 signatures
    let signatures: Vec<Value> = stable.iter().map(|(path, m)| {
        let manifest_hash = hex::encode(aria_core::canonical_hash(&m.manifest));
        json!({
            "id": m.manifest.id,
            "version": m.manifest.version,
            "schema_version": m.manifest.schema_version,
            "layer": format!("{}", m.manifest.layer.declared),
            "stability": format!("{:?}", m.manifest.stability),
            "lifecycle_phase": format!("{:?}", m.manifest.lifecycle.phase),
            "manifest_hash": manifest_hash,
            "file": path.display().to_string(),
        })
    }).collect();

    // Compute bundle_version as SHA-256 of all manifest hashes
    let mut hasher = Sha256::new();
    for sig in &signatures {
        if let Some(h) = sig["manifest_hash"].as_str() {
            hasher.update(h.as_bytes());
        }
    }
    let bundle_version = format!("sha256:{}", hex::encode(hasher.finalize()));

    let bundle = json!({
        "bundle_version": bundle_version,
        "manifest_count": signatures.len(),
        "domain_filter": domain_filter,
        "signatures": signatures,
    });

    let output_dir = project_root.join(".aria");
    fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("manifest-bundle.json");
    fs::write(&output_path, serde_json::to_string_pretty(&bundle)?)?;

    println!("Bundle written to {}", output_path.display());
    println!("  {} STABLE manifests", signatures.len());
    println!("  bundle_version: {}", bundle_version);

    Ok(())
}

/// Check if the manifest bundle is stale.
/// Returns a warning diagnostic if the stored bundle_version doesn't match recomputed.
pub fn check_bundle_staleness(
    manifests: &[(std::path::PathBuf, Manifest)],
    project_root: &Path,
) -> Vec<aria_core::Diagnostic> {
    use aria_core::Diagnostic;

    let bundle_path = project_root.join(".aria").join("manifest-bundle.json");
    if !bundle_path.exists() {
        return vec![];  // No bundle yet — not stale, just absent
    }

    let bundle_text = match fs::read_to_string(&bundle_path) {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    let bundle: Value = match serde_json::from_str(&bundle_text) {
        Ok(v) => v,
        Err(_) => return vec![],
    };
    let stored_version = bundle["bundle_version"].as_str().unwrap_or("").to_string();

    // Recompute bundle version from current STABLE manifests
    let stable: Vec<_> = manifests
        .iter()
        .filter(|(_, m)| m.manifest.stability == Stability::Stable || m.manifest.stability == Stability::Frozen)
        .collect();

    let mut hasher = Sha256::new();
    for (_, m) in &stable {
        let h = hex::encode(aria_core::canonical_hash(&m.manifest));
        hasher.update(h.as_bytes());
    }
    let current_version = format!("sha256:{}", hex::encode(hasher.finalize()));

    if stored_version != current_version {
        vec![Diagnostic::warn(
            &bundle_path, 0, 0,
            format!("Manifest bundle is stale (run aria-build bundle to regenerate). Stored: {}, Current: {}", stored_version, current_version),
        )]
    } else {
        vec![]
    }
}
