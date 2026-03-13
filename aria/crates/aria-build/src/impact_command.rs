use std::path::Path;
use anyhow::Result;
use serde_json::{json, Value};
use aria_core::{Manifest, SemanticGraph};
use crate::cli::ImpactFormat;

/// Print all ARUs that transitively depend on `aru_id`, grouped by layer, in topological order.
pub fn run_impact(
    manifests: &[(std::path::PathBuf, Manifest)],
    aru_id: &str,
    format: &ImpactFormat,
) -> Result<()> {
    let graph = SemanticGraph::build(manifests);

    if graph.get_node(aru_id).is_none() {
        eprintln!("[ERROR] ARU '{}' not found in the project manifests", aru_id);
        std::process::exit(1);
    }

    let dependents = graph.transitive_dependents(aru_id);

    // Group by layer
    let mut by_layer: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for dep_id in &dependents {
        let layer = graph.get_node(dep_id)
            .map(|n| format!("{}", n.layer))
            .unwrap_or_else(|| "UNKNOWN".to_string());
        by_layer.entry(layer).or_default().push(dep_id.clone());
    }

    match format {
        ImpactFormat::Json => {
            let output = json!({
                "target": aru_id,
                "total_dependents": dependents.len(),
                "by_layer": by_layer,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        ImpactFormat::Table => {
            println!("Impact analysis for: {}", aru_id);
            println!("Total transitive dependents: {}", dependents.len());
            println!();
            for (layer, ids) in &by_layer {
                println!("  {} ({} ARUs):", layer, ids.len());
                for id in ids {
                    println!("    - {}", id);
                }
            }
        }
    }

    Ok(())
}
