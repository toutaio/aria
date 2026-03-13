import { describe, it, expect } from 'vitest';
import {
  success,
  failure,
  isSuccess,
  isFailure,
  type Result,
} from '../src/result.js';

describe('Result<T, E>', () => {
  it('success() creates a Success value', () => {
    const r = success(42);
    expect(r._tag).toBe('Success');
    expect(isSuccess(r)).toBe(true);
    expect(isFailure(r)).toBe(false);
    if (isSuccess(r)) {
      expect(r.value).toBe(42);
    }
  });

  it('failure() creates a Failure value', () => {
    const r = failure(new Error('oops'));
    expect(r._tag).toBe('Failure');
    expect(isFailure(r)).toBe(true);
    expect(isSuccess(r)).toBe(false);
    if (isFailure(r)) {
      expect(r.error.message).toBe('oops');
    }
  });

  it('type-checks with string error type', () => {
    const r: Result<number, string> = failure('not found');
    expect(isFailure(r)).toBe(true);
  });
});
