mod cli;
mod loader;
mod check_command;
mod impact_command;
mod bundle_command;
mod generate_command;

use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use check_command::{CheckConfig, run_check, format_diagnostics};
use loader::{discover_manifests, load_manifests};
use aria_core::checker::Severity;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { dir, compliance_level, format } => {
            run_check_command(&dir, compliance_level, format);
        }
        Commands::Impact { aru_id, dir, format } => {
            let paths = discover_manifests(&dir).unwrap_or_default();
            let (manifests, _errors) = load_manifests(&paths);
            if let Err(e) = impact_command::run_impact(&manifests, &aru_id, &format) {
                eprintln!("[ERROR] {}", e);
                std::process::exit(1);
            }
        }
        Commands::Bundle { dir, domain } => {
            let paths = discover_manifests(&dir).unwrap_or_default();
            let (manifests, _errors) = load_manifests(&paths);
            if let Err(e) = bundle_command::run_bundle(&manifests, &dir, domain.as_deref()) {
                eprintln!("[ERROR] {}", e);
                std::process::exit(1);
            }
        }
        Commands::Generate { dir } => {
            let paths = discover_manifests(&dir).unwrap_or_default();
            // Only pass paths for manifests with composition: sections
            let (manifests, _) = load_manifests(&paths);
            let composition_paths: Vec<_> = manifests
                .iter()
                .filter(|(_, m)| m.manifest.composition.is_some())
                .map(|(p, _)| p.clone())
                .collect();
            if let Err(e) = generate_command::run_generate(&composition_paths, &dir) {
                eprintln!("[ERROR] {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_check_command(dir: &std::path::Path, compliance_level: u8, format: OutputFormat) {
    let paths = match discover_manifests(dir) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[ERROR] Failed to discover manifests: {}", e);
            std::process::exit(1);
        }
    };

    if paths.is_empty() {
        if format == OutputFormat::Json {
            println!(r#"{{"manifests_checked":0,"errors":0,"warnings":0,"diagnostics":[]}}"#);
        } else {
            println!("[WARN] No *.manifest.yaml files found in {}", dir.display());
        }
        std::process::exit(0);
    }

    let (manifests, parse_errors) = load_manifests(&paths);
    let config = CheckConfig { compliance_level, format: format.clone() };
    let (diagnostics, exit_code) = run_check(&manifests, &parse_errors, &config);

    // Bundle staleness check at level 4+
    if compliance_level >= 4 {
        let stale = bundle_command::check_bundle_staleness(&manifests, dir);
        let output = format_diagnostics(&stale, 0, compliance_level, &format);
        if !output.is_empty() && format == OutputFormat::Text {
            eprint!("{}", output);
        }
    }

    let output = format_diagnostics(&diagnostics, paths.len(), compliance_level, &format);

    if format == OutputFormat::Json {
        println!("{}", output);
    } else {
        if !output.is_empty() {
            eprint!("{}", output);
        }

        let error_count = diagnostics.iter().filter(|d| matches!(d.severity, Severity::Error)).count();
        let warn_count = diagnostics.iter().filter(|d| matches!(d.severity, Severity::Warn)).count();

        if error_count == 0 && warn_count == 0 {
            println!("✓ {} manifest(s) valid (compliance level {})", paths.len(), compliance_level);
        } else {
            eprintln!("✗ {} error(s), {} warning(s) in {} manifest(s)", error_count, warn_count, paths.len());
        }
    }

    std::process::exit(exit_code);
}
