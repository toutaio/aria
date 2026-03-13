use std::path::PathBuf;
use std::sync::Arc;
use crate::manifest::Manifest;
use crate::checker::Diagnostic;

/// The Salsa database for incremental manifest validation.
#[salsa::database(ManifestValidationStorage)]
pub struct AriaDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for AriaDatabase {}

impl AriaDatabase {
    pub fn new() -> Self {
        AriaDatabase {
            storage: salsa::Storage::default(),
        }
    }
}

impl Default for AriaDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Salsa-compatible diagnostic record (uses String for Eq compatibility).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticRecord {
    pub severity: String,
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub message: String,
}

impl DiagnosticRecord {
    pub fn to_diagnostic(&self) -> Diagnostic {
        use crate::checker::Severity;
        Diagnostic {
            severity: match self.severity.as_str() {
                "WARN" => Severity::Warn,
                _ => Severity::Error,
            },
            file: PathBuf::from(&self.file),
            line: self.line,
            col: self.col,
            message: self.message.clone(),
            range: None,
        }
    }
}

/// Wrapper that makes Manifest usable as Salsa query output by implementing Eq via YAML serialization.
#[derive(Debug, Clone)]
pub struct ManifestEq(pub Manifest);

impl PartialEq for ManifestEq {
    fn eq(&self, other: &Self) -> bool {
        serde_yaml::to_string(&self.0).ok() == serde_yaml::to_string(&other.0).ok()
    }
}

impl Eq for ManifestEq {}

/// Input query: raw YAML content for a manifest file.
#[salsa::query_group(ManifestValidationStorage)]
pub trait ManifestValidation: salsa::Database {
    /// Input: raw YAML content of a manifest file. Set by the build driver.
    #[salsa::input]
    fn manifest_file_content(&self, path: PathBuf) -> Arc<String>;

    /// Derived: parsed manifest (None if YAML is invalid).
    fn parsed_manifest(&self, path: PathBuf) -> Option<Arc<ManifestEq>>;

    /// Derived: JSON Schema validation diagnostics for a single manifest.
    fn schema_diagnostics(&self, path: PathBuf) -> Arc<Vec<DiagnosticRecord>>;

    /// Derived: naming checker diagnostics for a single manifest.
    fn naming_diagnostics(&self, path: PathBuf) -> Arc<Vec<DiagnosticRecord>>;
}

fn parsed_manifest(db: &dyn ManifestValidation, path: PathBuf) -> Option<Arc<ManifestEq>> {
    let content = db.manifest_file_content(path);
    Manifest::from_yaml(&content).ok().map(|m| Arc::new(ManifestEq(m)))
}

fn schema_diagnostics(db: &dyn ManifestValidation, path: PathBuf) -> Arc<Vec<DiagnosticRecord>> {
    use crate::checkers::schema::check_schema_str;

    let content = db.manifest_file_content(path.clone());
    let results = check_schema_str(&path, &content);
    Arc::new(records_from_diags(results))
}

fn naming_diagnostics(db: &dyn ManifestValidation, path: PathBuf) -> Arc<Vec<DiagnosticRecord>> {
    use crate::checkers::naming::check_naming_slice;

    let parsed = db.parsed_manifest(path.clone());
    let manifest = match parsed {
        Some(ref m) => &m.0,
        None => return Arc::new(vec![]),
    };

    let results = check_naming_slice(&[(&path, manifest)]);
    Arc::new(records_from_diags(results))
}

fn records_from_diags(diags: Vec<Diagnostic>) -> Vec<DiagnosticRecord> {
    diags
        .into_iter()
        .map(|d| DiagnosticRecord {
            severity: format!("{}", d.severity),
            file: d.file.display().to_string(),
            line: d.line,
            col: d.col,
            message: d.message,
        })
        .collect()
}
