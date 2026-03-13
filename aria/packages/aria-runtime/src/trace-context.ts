/**
 * TraceContext — Cross-cutting tracing context for ARU composition chains.
 * Source: doc 12 (error-propagation.md), doc 14 (human-ai-collaboration.md)
 *
 * Created once at the entry point of a composition and threaded through all
 * ARU calls without modification. Enables end-to-end tracing in logs and
 * observability tooling.
 */

/** Trace context propagated through all ARU calls in a composition chain */
export interface TraceContext {
  /** UUID-v4 correlation ID for end-to-end tracing. Immutable once created. */
  readonly correlationId: string;
  /** Semantic address of the ARU that initiated this trace */
  readonly originARU: string;
  /** ISO8601 timestamp when this trace was created */
  readonly createdAt: string;
}

/** Options for createTraceContext */
export interface TraceContextOptions {
  /** Semantic address of the initiating ARU */
  originARU: string;
  /** Optional correlation ID. If not provided, a UUID-v4 is generated. */
  correlationId?: string;
}

/**
 * Create a new TraceContext.
 * Generates a UUID-v4 correlationId if one is not provided.
 *
 * @param options - Trace context creation options
 */
export function createTraceContext(options: TraceContextOptions): TraceContext {
  return {
    correlationId: options.correlationId ?? generateUUID(),
    originARU: options.originARU,
    createdAt: new Date().toISOString(),
  };
}

/**
 * Generate a UUID-v4 string.
 * Uses crypto.randomUUID() when available (Node.js 14.17+, modern browsers).
 * Falls back to a manual implementation for older environments.
 */
function generateUUID(): string {
  if (typeof crypto !== 'undefined' && crypto.randomUUID) {
    return crypto.randomUUID();
  }
  // Fallback implementation
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
    const r = (Math.random() * 16) | 0;
    const v = c === 'x' ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}
