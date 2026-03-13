/**
 * Result<T, E> — Railway-oriented discriminated union.
 * Source: doc 12 (error-propagation.md), doc 08 (type-system.md)
 */

/** Successful result */
export interface Success<T> {
  readonly _tag: 'Success';
  readonly value: T;
}

/** Failed result */
export interface Failure<E> {
  readonly _tag: 'Failure';
  readonly error: E;
}

/** Discriminated union of Success and Failure */
export type Result<T, E> = Success<T> | Failure<E>;

/** Type guard: narrows Result to Success<T> */
export function isSuccess<T, E>(result: Result<T, E>): result is Success<T> {
  return result._tag === 'Success';
}

/** Type guard: narrows Result to Failure<E> */
export function isFailure<T, E>(result: Result<T, E>): result is Failure<E> {
  return result._tag === 'Failure';
}

/** Construct a Success value */
export function success<T>(value: T): Success<T> {
  return { _tag: 'Success', value };
}

/** Construct a Failure value */
export function failure<E>(error: E): Failure<E> {
  return { _tag: 'Failure', error };
}
