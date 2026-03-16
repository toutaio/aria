use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

/// Layer designation for an ARU. Source: 01-abstraction-layers.md
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layer {
    L0, L1, L2, L3, L4, L5,
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layer::L0 => write!(f, "L0"),
            Layer::L1 => write!(f, "L1"),
            Layer::L2 => write!(f, "L2"),
            Layer::L3 => write!(f, "L3"),
            Layer::L4 => write!(f, "L4"),
            Layer::L5 => write!(f, "L5"),
        }
    }
}

impl Layer {
    /// Returns the numeric value of the layer for comparisons.
    pub fn numeric(&self) -> u8 {
        match self {
            Layer::L0 => 0, Layer::L1 => 1, Layer::L2 => 2,
            Layer::L3 => 3, Layer::L4 => 4, Layer::L5 => 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Stability {
    Experimental,
    Stable,
    Frozen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LifecyclePhase {
    Specified,
    Draft,
    Candidate,
    Stable,
    Deprecated,
    Tombstoned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SideEffects {
    None,
    Read,
    Write,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompositionPattern {
    Pipe,
    Fork,
    Join,
    Gate,
    Route,
    Loop,
    Observe,
    Transform,
    Validate,
    Cache,
    Stream,
    Saga,
    CircuitBreaker,
    ParallelJoin,
}

/// Section 1: Identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub purpose: String,
    pub domain: String,
    pub subdomain: String,
    pub verb: String,
    pub entity: String,
}

/// Section 2: Layer
/// Supports both shorthand (`layer: "L1"`) and full object form (`layer: { declared, inferred }`).
/// When the shorthand is used, `inferred` is `None` until the layer-inference pass fills it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "LayerFieldRaw")]
pub struct LayerSection {
    pub declared: Layer,
    pub inferred: Option<Layer>,
}

/// Internal helper for `serde(untagged)` deserialization of the layer field.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum LayerFieldRaw {
    /// Shorthand: `layer: "L1"`
    Shorthand(Layer),
    /// Full form: `layer: { declared: "L1", inferred: "L1" }`
    Full { declared: Layer, inferred: Option<Layer> },
}

impl From<LayerFieldRaw> for LayerSection {
    fn from(raw: LayerFieldRaw) -> Self {
        match raw {
            LayerFieldRaw::Shorthand(layer) => LayerSection { declared: layer, inferred: None },
            LayerFieldRaw::Full { declared, inferred } => LayerSection { declared, inferred },
        }
    }
}

/// Section 3: Contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub input: ContractInput,
    pub output: ContractOutput,
    pub side_effects: SideEffects,
    pub idempotent: bool,
    pub deterministic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInput {
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractOutput {
    pub success: String,
    pub failure: String,
}

/// Section 4: Type State (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeState {
    pub input_state: String,
    pub output_state: String,
    pub machine_ref: String,
}

/// Section 5: Dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub id: String,
    pub layer: Layer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_pin: Option<String>,
    pub stability: Stability,
}

/// Section 6: Composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    pub pattern: CompositionPattern,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handler: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate_aru: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_aru: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iterations: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_aru: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_window_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub half_open_probe_interval_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_required_results: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<SagaStep>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaStep {
    pub aru: String,
    pub compensating_aru: String,
}

/// Section 7: Saga participation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaParticipant {
    pub compensating_aru: String,
    pub idempotency_key_field: String,
}

/// Section 8: Context budget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBudget {
    pub to_use: u32,
    pub to_modify: u32,
    pub to_extend: u32,
    pub to_replace: u32,
}

/// Section 9: Test contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestContract {
    pub scenarios: Vec<TestScenario>,
    #[serde(default = "default_true")]
    pub coverage_required: bool,
    #[serde(default)]
    pub mutation_testing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    pub scenario: String,
}

fn default_true() -> bool { true }

/// Section 10: Behavioral contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralContract {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency_p99: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency_p999: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_calls_per_second: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_calls_per_user_per_second: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<String>,
    #[serde(default)]
    pub must_be_called_after: Vec<String>,
    #[serde(default)]
    pub must_be_called_before: Vec<String>,
}

/// Section 11: Lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifecycle {
    pub phase: LifecyclePhase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stable_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunset_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_aru: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tombstoned_at: Option<String>,
}

/// Section 12: Health contract (required for L3+)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthContract {
    pub sla_latency_p99: String,
    pub sla_availability: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_aru: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degraded_threshold: Option<HealthThreshold>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_open_threshold: Option<CircuitOpenThreshold>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthThreshold {
    pub error_rate_percent: f64,
    pub latency_p99_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitOpenThreshold {
    pub error_rate_percent: f64,
    pub consecutive_failures: u32,
}

/// Section 13: Diagnostic surface (required for L3+)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSurface {
    #[serde(default)]
    pub failure_indicators: Vec<FailureIndicator>,
    #[serde(default)]
    pub known_failure_patterns: Vec<FailurePattern>,
    #[serde(default)]
    pub escalation_path: Vec<EscalationEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureIndicator {
    pub symptom: String,
    pub check: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePattern {
    pub pattern: String,
    pub description: String,
    pub resolution_steps: Vec<String>,
    pub minimum_context_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationEntry {
    pub layer: Layer,
    pub handler: String,
}

/// Section 14: Manifest provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestProvenance {
    pub derived_by: String,
    pub reviewed_by: String,
    pub approved_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_version: Option<String>,
}

/// The complete manifest body (all 14 sections)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestBody {
    pub id: String,
    pub version: String,
    pub schema_version: String,
    pub identity: Identity,
    pub layer: LayerSection,
    pub contract: Contract,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_state: Option<TypeState>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition: Option<Composition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saga_participant: Option<SagaParticipant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_budget: Option<ContextBudget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_contract: Option<TestContract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavioral_contract: Option<BehavioralContract>,
    pub stability: Stability,
    pub lifecycle: Lifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_contract: Option<HealthContract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_surface: Option<DiagnosticSurface>,
    pub manifest_provenance: ManifestProvenance,
}

/// Top-level wrapper matching the YAML structure `manifest: { ... }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub manifest: ManifestBody,
}

impl Manifest {
    /// Deserialize a manifest from YAML text.
    pub fn from_yaml(text: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(text)
    }

    /// Serialize the manifest to YAML text.
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}
