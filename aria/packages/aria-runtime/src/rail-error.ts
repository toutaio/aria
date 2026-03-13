/**
 * RailError<E> — Error type with provenance tracking.
 * Source: doc 12 (error-propagation.md)
 *
 * Wraps a domain error with the origin ARU's semantic address and the trace context,
 * enabling full provenance tracking through a railway composition chain.
 */

import type { TraceContext } from './trace-context.js';

/** An error enriched with ARU origin and trace context for provenance tracking */
export interface RailError<E> {
  /** The underlying domain error (e.g., AuthError.EXPIRED) */
  readonly error: E;
  /** Semantic address of the ARU that produced this error */
  readonly origin_aru: string;
  /** The trace context at the time of failure */
  readonly ctx: TraceContext;
  /** ISO8601 timestamp of when the error occurred */
  readonly occurred_at: string;
}

/**
 * Wrap a domain error with provenance information.
 *
 * @param error - The domain-specific error value
 * @param origin_aru - Semantic address of the ARU that produced this error
 * @param ctx - The trace context at the time of failure
 */
export function wrapWithProvenance<E>(
  error: E,
  origin_aru: string,
  ctx: TraceContext,
): RailError<E> {
  return {
    error,
    origin_aru,
    ctx,
    occurred_at: new Date().toISOString(),
  };
}
