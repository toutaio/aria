import { describe, it, expect } from 'vitest';
import { createTraceContext } from '../src/trace-context.js';

const UUID_REGEX = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

describe('TraceContext', () => {
  it('generates a UUID-v4 correlationId if not provided', () => {
    const ctx = createTraceContext({ originARU: 'auth.identity.authenticate.user' });
    expect(UUID_REGEX.test(ctx.correlationId)).toBe(true);
  });

  it('uses provided correlationId', () => {
    const id = 'abc123de-1234-4abc-8abc-abc123456789';
    const ctx = createTraceContext({ originARU: 'auth.identity', correlationId: id });
    expect(ctx.correlationId).toBe(id);
  });

  it('sets originARU correctly', () => {
    const ctx = createTraceContext({ originARU: 'payment.billing.process.invoice' });
    expect(ctx.originARU).toBe('payment.billing.process.invoice');
  });

  it('sets createdAt as a valid ISO8601 string', () => {
    const ctx = createTraceContext({ originARU: 'x' });
    expect(() => new Date(ctx.createdAt).toISOString()).not.toThrow();
  });
});
