use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;
use serde::Serialize;
use serde_yaml;
use aria_core::manifest::Manifest;

/// Recursively discover all *.manifest.yaml files in a directory.
pub fn discover_manifests(root: &Path) -> Result<Vec<PathBuf>> {
    let mut results = vec![];
    collect_manifests(root, &mut results)?;
    Ok(results)
}

fn collect_manifests(dir: &Path, results: &mut Vec<PathBuf>) -> Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if path.is_dir() {
            if name_str == "node_modules" || name_str == ".git" || name_str == "target" {
                continue;
            }
            collect_manifests(&path, results)?;
        } else if name_str.ends_with(".manifest.yaml") {
            results.push(path);
        }
    }

    Ok(())
}

/// Load and parse all manifest files, returning successfully parsed ones and any errors.
pub fn load_manifests(paths: &[PathBuf]) -> (Vec<(PathBuf, Manifest)>, Vec<(PathBuf, String)>) {
    use rayon::prelude::*;
    let results: Vec<_> = paths
        .par_iter()
        .map(|path| {
            let text = match fs::read_to_string(path) {
                Ok(t) => t,
                Err(e) => return Err((path.clone(), e.to_string())),
            };
            match Manifest::from_yaml(&text) {
                Ok(m) => Ok((path.clone(), m)),
                Err(e) => Err((path.clone(), e.to_string())),
            }
        })
        .collect();

    let mut manifests = vec![];
    let mut errors = vec![];
    for r in results {
        match r {
            Ok(pair) => manifests.push(pair),
            Err(pair) => errors.push(pair),
        }
    }
    (manifests, errors)
}
