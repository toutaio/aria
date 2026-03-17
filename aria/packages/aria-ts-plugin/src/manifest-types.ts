/**
 * aria-ts-plugin: Minimal manifest type representation for code generation.
 * Only the fields needed to drive TypeScript code generation are included here.
 */

export type CompositionPattern =
  | 'PIPE'
  | 'FORK'
  | 'JOIN'
  | 'GATE'
  | 'ROUTE'
  | 'LOOP'
  | 'OBSERVE'
  | 'TRANSFORM'
  | 'VALIDATE'
  | 'CACHE'
  | 'STREAM'
  | 'PARALLEL_FORK'
  | 'PARALLEL_JOIN'
  | 'SCATTER_GATHER'
  | 'CIRCUIT_BREAKER'
  | 'SAGA'
  | 'COMPENSATING_TRANSACTION'
  | 'STREAMING_PIPELINE'
  | 'CACHE_ASIDE'
  | 'BULKHEAD'
  | 'PRIORITY_QUEUE'
  | 'EVENT_SOURCING';

export interface ManifestComposition {
  pattern: CompositionPattern;
  input_type: string;
  output_type: string;
  error_types?: string[];
  // GATE / ROUTE: predicate/condition ARU
  predicate_aru?: string;
  condition_aru?: string;
  // LOOP: iteration cap
  max_iterations?: number;
  // OBSERVE / TRANSFORM / VALIDATE / CACHE / STREAM: target ARU
  target_aru?: string;
  // CACHE: key derivation ARU
  key_aru?: string;
  // STREAM: element source and processor ARUs
  source_aru?: string;
  processor_aru?: string;
  backpressure?: string;
  // PARALLEL_FORK / PARALLEL_JOIN
  branches?: string[];
  minimum_required_results?: number;
  // CIRCUIT_BREAKER
  fallback_aru?: string;
  open_state_type?: string;
  // SAGA
  steps?: Array<{ aru: string; compensation?: string }>;
  // COMPENSATING_TRANSACTION
  forward_aru?: string;
  compensation_aru?: string;
  // CACHE_ASIDE
  cache_store_type?: string;
  cache_key_type?: string;
  // BULKHEAD
  pool_name?: string;
  capacity?: number;
  queue_overflow_type?: string;
  // PRIORITY_QUEUE
  priority_type?: string;
  // STREAMING_PIPELINE
  chunk_type?: string;
  // EVENT_SOURCING
  event_type?: string;
  aggregate_type?: string;
}

export interface ManifestIdentity {
  address: string;
  layer: number;
}

export interface ManifestDoc {
  identity: ManifestIdentity;
  composition: ManifestComposition;
  /** SHA-256 of sorted-key JSON serialization of the composition section */
  compositionHash: string;
}
