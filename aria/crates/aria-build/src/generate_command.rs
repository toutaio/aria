use std::path::Path;
use anyhow::Result;

/// Generate TypeScript composition wrapper code by invoking aria-ts-plugin
/// via Node.js child_process.
///
/// All TypeScript codegen logic lives in aria-ts-plugin.
/// aria-build owns only discovery and subprocess invocation.
pub fn run_generate(
    manifest_paths: &[std::path::PathBuf],
    project_root: &Path,
) -> Result<()> {
    use std::process::Command;

    if manifest_paths.is_empty() {
        println!("No manifest files found to generate wrappers for.");
        return Ok(());
    }

    // Build the argument list for aria-ts-plugin
    let mut args: Vec<String> = vec![
        "--project-root".to_string(),
        project_root.display().to_string(),
    ];
    for path in manifest_paths {
        args.push(path.display().to_string());
    }

    // Try to find aria-ts-plugin via node_modules or the global PATH
    let plugin_candidates = [
        project_root.join("node_modules").join("aria-ts-plugin").join("bin").join("aria-ts-plugin.js"),
        project_root.join("node_modules").join(".bin").join("aria-ts-plugin"),
    ];

    let plugin_path = plugin_candidates.iter().find(|p| p.exists());

    let status = match plugin_path {
        Some(path) => {
            Command::new("node")
                .arg(path)
                .args(&args)
                .current_dir(project_root)
                .status()?
        }
        None => {
            // Fall back to running aria-ts-plugin from PATH
            Command::new("aria-ts-plugin")
                .args(&args)
                .current_dir(project_root)
                .status()
                .map_err(|e| anyhow::anyhow!(
                    "aria-ts-plugin not found. Install it with: npm install --save-dev aria-ts-plugin\n\
                    Original error: {}", e
                ))?
        }
    };

    if !status.success() {
        anyhow::bail!("aria-ts-plugin exited with status {}", status);
    }

    Ok(())
}
