use std::path::Path;
use std::collections::{HashMap, HashSet};
use crate::manifest::{Manifest, Layer};
use crate::checker::{Diagnostic, CheckResult};
fn verb_to_layer(verb: &str) -> Option<Layer> {
    match verb {
        "validate" | "transform" | "compute" | "generate" |
        "encode" | "decode" | "hash" | "compare" => Some(Layer::L1),

        "create" | "build" | "prepare" | "assemble" | "resolve" => Some(Layer::L2),

        "execute" | "apply" | "process" | "enforce" |
        "emit" | "authorize" | "persist" => Some(Layer::L3),

        "orchestrate" | "coordinate" | "pipeline" | "route" => Some(Layer::L4),

        "expose" | "integrate" | "guard" | "translate" => Some(Layer::L5),

        _ => None,
    }
}

/// Parse a semantic address into its segments.
fn parse_address(id: &str) -> Option<AddressParts> {
    let parts: Vec<&str> = id.split('.').collect();
    match parts.len() {
        2 => Some(AddressParts { domain: parts[0], subdomain: None, verb: None, entity: parts[1], is_l0: true }),
        4 => Some(AddressParts {
            domain: parts[0],
            subdomain: Some(parts[1]),
            verb: Some(parts[2]),
            entity: parts[3],
            is_l0: false,
        }),
        _ => None,
    }
}

struct AddressParts<'a> {
    domain: &'a str,
    subdomain: Option<&'a str>,
    verb: Option<&'a str>,
    entity: &'a str,
    is_l0: bool,
}

/// Run all naming enforcement checks on a set of manifests.
pub fn check_naming(manifests: &[(std::path::PathBuf, crate::manifest::Manifest)]) -> CheckResult {
    let pairs: Vec<(&std::path::Path, &crate::manifest::Manifest)> = manifests
        .iter()
        .map(|(p, m)| (p.as_path(), m))
        .collect();
    check_naming_slice(&pairs)
}

pub fn check_naming_slice(manifests: &[(&Path, &Manifest)]) -> CheckResult {
    let mut diagnostics = vec![];
    let mut seen_ids: HashMap<String, &Path> = HashMap::new();

    for (file, manifest) in manifests {
        let m = &manifest.manifest;
        let id = &m.id;

        // Address uniqueness
        if let Some(first_file) = seen_ids.get(id.as_str()) {
            diagnostics.push(Diagnostic::error(
                *file, 0, 0,
                format!("Duplicate semantic address '{}' (first seen in {})", id, first_file.display()),
            ));
        } else {
            seen_ids.insert(id.clone(), file);
        }

        // Semantic address format
        match parse_address(id) {
            None => {
                diagnostics.push(Diagnostic::error(
                    *file, 0, 0,
                    format!("Invalid semantic address '{}' — expected [domain].[entity] (L0) or [domain].[subdomain].[verb].[entity] (L1+)", id),
                ));
                continue;
            }
            Some(parts) if parts.is_l0 => {
                // L0 address — only validate layer consistency
                if m.layer.declared != Layer::L0 {
                    diagnostics.push(Diagnostic::error(
                        *file, 0, 0,
                        format!("L0 address format '{}' requires layer.declared: L0, got {:?}", id, m.layer.declared),
                    ));
                }
            }
            Some(parts) => {
                let verb = parts.verb.unwrap();
                match verb_to_layer(verb) {
                    None => {
                        diagnostics.push(Diagnostic::error(
                            *file, 0, 0,
                            format!("Unknown verb '{}' in semantic address '{}' — not in any layer vocabulary", verb, id),
                        ));
                    }
                    Some(inferred_layer) => {
                        // Layer consistency check
                        if m.layer.declared != inferred_layer {
                            diagnostics.push(Diagnostic::error(
                                *file, 0, 0,
                                format!(
                                    "Layer mismatch: declared={} but verb '{}' implies {} for address '{}'",
                                    m.layer.declared, verb, inferred_layer, id
                                ),
                            ));
                        }

                        // Tier 1 drift: stored inferred != recomputed
                        if m.layer.inferred != inferred_layer {
                            diagnostics.push(Diagnostic::warn(
                                *file, 0, 0,
                                format!(
                                    "Inferred layer mismatch: stored inferred={}, recomputed={} for address '{}'",
                                    m.layer.inferred, inferred_layer, id
                                ),
                            ));
                        }
                    }
                }

                // Error type domain-prefix check
                let failure = &m.contract.output.failure;
                let has_domain_prefix = failure
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
                    && (failure.contains("Error.") || failure.contains("Error {"));

                if !has_domain_prefix {
                    diagnostics.push(Diagnostic::error(
                        *file, 0, 0,
                        format!("contract.output.failure '{}' does not match {{Domain}}Error.{{Code}} pattern", failure),
                    ));
                }
            }
        }
    }

    diagnostics
}

