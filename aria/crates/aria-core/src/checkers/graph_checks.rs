use std::path::Path;
use crate::manifest::{Manifest, CompositionPattern, Layer};
use crate::checker::{Diagnostic, CheckResult};
use crate::graph::SemanticGraph;

/// Check cycles and cross-domain dependencies (delegating to SemanticGraph).
pub fn check_cycles(graph: &SemanticGraph) -> CheckResult {
    graph.check_cycles()
}

pub fn check_cross_domain(graph: &SemanticGraph) -> CheckResult {
    graph.check_cross_domain_dependencies()
}

/// Check type compatibility for all composition pattern edges.
/// Implements rules from doc 11 (type-compatibility.md) and doc 12 (error-propagation.md).
pub fn check_type_compatibility(graph: &SemanticGraph) -> CheckResult {
    let mut diagnostics = vec![];

    for node in graph.nodes() {
        let Some(comp) = &node.manifest.composition else { continue };

        match comp.pattern {
            CompositionPattern::Pipe => {
                // PIPE: each step's output.success must be compatible with next step's input.type
                // For now, we flag missing chain definitions
                if comp.chain.as_ref().map(|c| c.is_empty()).unwrap_or(true) {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("PIPE composition '{}' has no chain defined", node.id),
                    ));
                }
                // error_handler is required for PIPE
                if comp.error_handler.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("PIPE composition '{}' missing required error_handler", node.id),
                    ));
                }
            }
            CompositionPattern::Fork => {
                if comp.branches.as_ref().map(|b| b.is_empty()).unwrap_or(true) {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("FORK composition '{}' has no branches defined", node.id),
                    ));
                }
                if comp.error_handler.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("FORK composition '{}' missing required error_handler", node.id),
                    ));
                }
            }
            CompositionPattern::Join => {
                if comp.branches.as_ref().map(|b| b.is_empty()).unwrap_or(true) {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("JOIN composition '{}' has no branches defined", node.id),
                    ));
                }
                if comp.merge_type.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("JOIN composition '{}' missing required merge_type (must be a product type)", node.id),
                    ));
                }
                if comp.error_handler.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("JOIN composition '{}' missing required error_handler", node.id),
                    ));
                }
            }
            CompositionPattern::Gate => {
                if comp.predicate_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("GATE composition '{}' missing required predicate_aru", node.id),
                    ));
                }
            }
            CompositionPattern::Route => {
                if comp.predicate_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("ROUTE composition '{}' missing required predicate_aru", node.id),
                    ));
                }
            }
            CompositionPattern::Loop => {
                if comp.condition_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("LOOP composition '{}' missing required condition_aru", node.id),
                    ));
                }
                if comp.max_iterations.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("LOOP composition '{}' missing required max_iterations", node.id),
                    ));
                }
            }
            CompositionPattern::Saga => {
                if comp.steps.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("SAGA composition '{}' has no steps defined", node.id),
                    ));
                }
            }
            CompositionPattern::CircuitBreaker => {
                if comp.target_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("CIRCUIT_BREAKER composition '{}' missing required target_aru", node.id),
                    ));
                }
                if comp.failure_threshold.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("CIRCUIT_BREAKER composition '{}' missing required failure_threshold", node.id),
                    ));
                }
                if comp.evaluation_window_ms.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("CIRCUIT_BREAKER composition '{}' missing required evaluation_window_ms", node.id),
                    ));
                }
            }
            CompositionPattern::ParallelJoin => {
                if comp.branches.as_ref().map(|b| b.is_empty()).unwrap_or(true) {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("PARALLEL_JOIN composition '{}' has no branches defined", node.id),
                    ));
                }
                if comp.minimum_required_results.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("PARALLEL_JOIN composition '{}' missing required minimum_required_results", node.id),
                    ));
                }
            }
            CompositionPattern::Stream => {
                if comp.error_handler.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("STREAM composition '{}' missing required error_handler", node.id),
                    ));
                }
            }
            CompositionPattern::ParallelFork => {
                if comp.branches.as_ref().map(|b| b.is_empty()).unwrap_or(true) {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("PARALLEL_FORK composition '{}' has no branches defined", node.id),
                    ));
                }
                if comp.error_handler.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("PARALLEL_FORK composition '{}' missing required error_handler", node.id),
                    ));
                }
            }
            CompositionPattern::ScatterGather => {
                if comp.worker_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("SCATTER_GATHER composition '{}' missing required worker_aru", node.id),
                    ));
                }
                if comp.aggregate_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("SCATTER_GATHER composition '{}' missing required aggregate_aru", node.id),
                    ));
                }
                if comp.error_handler.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("SCATTER_GATHER composition '{}' missing required error_handler", node.id),
                    ));
                }
            }
            CompositionPattern::CompensatingTransaction => {
                if comp.forward_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("COMPENSATING_TRANSACTION composition '{}' missing required forward_aru", node.id),
                    ));
                }
                if comp.compensation_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("COMPENSATING_TRANSACTION composition '{}' missing required compensation_aru", node.id),
                    ));
                }
            }
            CompositionPattern::StreamingPipeline => {
                if comp.source_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("STREAMING_PIPELINE composition '{}' missing required source_aru", node.id),
                    ));
                }
                if comp.processor_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("STREAMING_PIPELINE composition '{}' missing required processor_aru", node.id),
                    ));
                }
                if comp.backpressure.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("STREAMING_PIPELINE composition '{}' missing required backpressure declaration", node.id),
                    ));
                }
            }
            CompositionPattern::CacheAside => {
                if comp.target_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("CACHE_ASIDE composition '{}' missing required target_aru (the ARU being cached)", node.id),
                    ));
                }
                if comp.key_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("CACHE_ASIDE composition '{}' missing required key_aru", node.id),
                    ));
                }
            }
            CompositionPattern::Bulkhead => {
                if comp.capacity.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("BULKHEAD composition '{}' missing required capacity", node.id),
                    ));
                }
                if comp.target_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("BULKHEAD composition '{}' missing required target_aru", node.id),
                    ));
                }
            }
            CompositionPattern::PriorityQueue => {
                if comp.target_aru.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("PRIORITY_QUEUE composition '{}' missing required target_aru", node.id),
                    ));
                }
                if comp.priority_type.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("PRIORITY_QUEUE composition '{}' missing required priority_type", node.id),
                    ));
                }
            }
            CompositionPattern::EventSourcing => {
                if comp.event_type.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("EVENT_SOURCING composition '{}' missing required event_type", node.id),
                    ));
                }
                if comp.aggregate_type.is_none() {
                    diagnostics.push(Diagnostic::error(
                        &node.file, 0, 0,
                        format!("EVENT_SOURCING composition '{}' missing required aggregate_type", node.id),
                    ));
                }
            }
            // OBSERVE, TRANSFORM, VALIDATE, CACHE: flexible — no strict required fields beyond pattern
            _ => {}
        }
    }

    diagnostics
}
