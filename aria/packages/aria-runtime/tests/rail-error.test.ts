import { describe, it, expect } from 'vitest';
import { wrapWithProvenance } from '../src/rail-error.js';
import { createTraceContext } from '../src/trace-context.js';

describe('RailError<E>', () => {
  it('wraps error with provenance fields', () => {
    const ctx = createTraceContext({ originARU: 'auth.identity.authenticate.user' });
    const railErr = wrapWithProvenance('AUTH_EXPIRED', 'auth.identity.authenticate.user', ctx);

    expect(railErr.error).toBe('AUTH_EXPIRED');
    expect(railErr.origin_aru).toBe('auth.identity.authenticate.user');
    expect(railErr.ctx).toBe(ctx);
    expect(typeof railErr.occurred_at).toBe('string');
    expect(new Date(railErr.occurred_at).getFullYear()).toBeGreaterThan(2020);
  });
});
