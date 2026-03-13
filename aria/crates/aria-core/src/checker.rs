use std::path::PathBuf;

/// Diagnostic severity. All checkers use these variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warn,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "ERROR"),
            Severity::Warn => write!(f, "WARN"),
        }
    }
}

/// A diagnostic produced by any aria-core checker.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub file: PathBuf,
    pub line: usize,
    pub col: usize,
    pub message: String,
    /// LSP-compatible range, if available from SpanMap.
    pub range: Option<LspRange>,
}

/// LSP-compatible source range (line/character, both 0-indexed).
#[derive(Debug, Clone)]
pub struct LspRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

impl Diagnostic {
    pub fn error(file: impl Into<PathBuf>, line: usize, col: usize, message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Error,
            file: file.into(),
            line,
            col,
            message: message.into(),
            range: None,
        }
    }

    pub fn warn(file: impl Into<PathBuf>, line: usize, col: usize, message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Warn,
            file: file.into(),
            line,
            col,
            message: message.into(),
            range: None,
        }
    }

    pub fn with_range(mut self, range: LspRange) -> Self {
        self.range = Some(range);
        self
    }

    pub fn format_cli(&self) -> String {
        let location = if self.line > 0 {
            format!("{}:{}", self.file.display(), self.line)
        } else {
            self.file.display().to_string()
        };
        format!("[{}] {} — {}", self.severity, location, self.message)
    }
}

/// The result of running a set of checkers.
pub type CheckResult = Vec<Diagnostic>;
