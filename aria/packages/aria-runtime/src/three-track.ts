/**
 * ThreeTrack<T, P, E> — Three-way discriminated union for partial success.
 * Source: doc 12 (error-propagation.md), doc 03 (composition-patterns.md)
 *
 * Used by PARALLEL_JOIN compositions where some branches may succeed
 * and some may fail, and the caller must decide how to proceed.
 */

/** All required branches succeeded — full success */
export interface ThreeTrackSuccess<T> {
  readonly _tag: 'SUCCESS';
  readonly value: T;
}

/** Minimum required results met, but not all branches succeeded */
export interface ThreeTrackPartialSuccess<P> {
  readonly _tag: 'PARTIAL_SUCCESS';
  readonly value: P;
  /** Number of branches that succeeded */
  readonly succeeded_count: number;
  /** Number of branches that failed */
  readonly failed_count: number;
}

/** Too few branches succeeded — not enough to meet minimum_required_results */
export interface ThreeTrackFailure<E> {
  readonly _tag: 'FAILURE';
  readonly error: E;
}

/** Three-track discriminated union for PARALLEL_JOIN compositions */
export type ThreeTrack<T, P, E> =
  | ThreeTrackSuccess<T>
  | ThreeTrackPartialSuccess<P>
  | ThreeTrackFailure<E>;

/** Type guard: narrows to full success */
export function isThreeTrackSuccess<T, P, E>(
  result: ThreeTrack<T, P, E>
): result is ThreeTrackSuccess<T> {
  return result._tag === 'SUCCESS';
}

/** Type guard: narrows to partial success */
export function isThreeTrackPartialSuccess<T, P, E>(
  result: ThreeTrack<T, P, E>
): result is ThreeTrackPartialSuccess<P> {
  return result._tag === 'PARTIAL_SUCCESS';
}

/** Type guard: narrows to failure */
export function isThreeTrackFailure<T, P, E>(
  result: ThreeTrack<T, P, E>
): result is ThreeTrackFailure<E> {
  return result._tag === 'FAILURE';
}
