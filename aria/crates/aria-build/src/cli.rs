use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aria-build", version, about = "ARIA build tool — manifest validation, code generation, bundle building")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate all *.manifest.yaml files in the project
    Check {
        /// Project directory to search for manifests (default: current directory)
        #[arg(default_value = ".")]
        dir: PathBuf,

        /// Compliance level: 0=schema only, 1=+naming, 2=+type-graph, 3=+codegen, 4=+bundle, 5=all
        #[arg(short = 'l', long, default_value = "5", value_parser = clap::value_parser!(u8).range(0..=5))]
        compliance_level: u8,

        /// Output format: text (default) or json
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },

    /// Show all ARUs that transitively depend on the given ARU id
    Impact {
        /// The ARU semantic address to analyze (e.g., auth.token.validate.signature)
        aru_id: String,

        /// Project directory to search for manifests
        #[arg(default_value = ".")]
        dir: PathBuf,

        /// Output format: table (default) or json
        #[arg(long, default_value = "table")]
        format: ImpactFormat,
    },

    /// Build the manifest bundle (.aria/manifest-bundle.json)
    Bundle {
        /// Project directory
        #[arg(default_value = ".")]
        dir: PathBuf,

        /// Filter by domain name
        #[arg(long)]
        domain: Option<String>,
    },

    /// Generate composition wrapper code for all manifests with composition: sections
    Generate {
        /// Project directory
        #[arg(default_value = ".")]
        dir: PathBuf,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            other => Err(format!("Unknown format '{}': expected 'text' or 'json'", other)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImpactFormat {
    Table,
    Json,
}

impl std::str::FromStr for ImpactFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "table" => Ok(ImpactFormat::Table),
            "json" => Ok(ImpactFormat::Json),
            other => Err(format!("Unknown format '{}': expected 'table' or 'json'", other)),
        }
    }
}
