/**
 * @aria/runtime — Public API
 *
 * Zero external dependencies. Ships as both CJS and ESM.
 */

// Result<T, E> — railway discriminated union
export type { Result, Success, Failure } from './result.js';
export { isSuccess, isFailure, success, failure } from './result.js';

// RailError<E> — error with provenance
export type { RailError } from './rail-error.js';
export { wrapWithProvenance } from './rail-error.js';

// ThreeTrack<T, P, E> — three-way discriminated union for PARALLEL_JOIN
export type { ThreeTrack, ThreeTrackSuccess, ThreeTrackPartialSuccess, ThreeTrackFailure } from './three-track.js';
export { isThreeTrackSuccess, isThreeTrackPartialSuccess, isThreeTrackFailure } from './three-track.js';

// TraceContext — cross-cutting trace propagation
export type { TraceContext, TraceContextOptions } from './trace-context.js';
export { createTraceContext } from './trace-context.js';
