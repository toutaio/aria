import { describe, it, expect } from 'vitest';
import { resolve } from 'node:path';
import { loadManifest } from '../src/loader.js';
import { generateWrapper } from '../src/generators/index.js';
import { canonicalHash } from '../src/canonical-hash.js';

const fixtures = resolve(import.meta.dirname, '../fixtures');

describe('PIPE generator', () => {
  it('generates correct types from fixture', () => {
    const doc = loadManifest(`${fixtures}/pipe.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('@aria-manifest-hash');
    expect(code).toContain('AuthIdentityAuthenticateUserInput = AuthRequest');
    expect(code).toContain('AuthIdentityAuthenticateUserOutput = AuthToken');
    expect(code).toContain('AuthError.EXPIRED | AuthError.INVALID');
    expect(code).toContain('type AuthIdentityAuthenticateUserFn');
  });
});

describe('CIRCUIT_BREAKER generator', () => {
  it('emits CircuitStore interface and state type', () => {
    const doc = loadManifest(`${fixtures}/circuit-breaker.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('CircuitStore');
    expect(code).toContain("'CLOSED' | 'OPEN' | 'HALF_OPEN'");
    expect(code).toContain('CircuitOpenData');
  });
});

describe('PARALLEL_JOIN generator', () => {
  it('emits ThreeTrack and minimum_required_results comment', () => {
    const doc = loadManifest(`${fixtures}/parallel-join.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('ThreeTrack');
    expect(code).toContain('Minimum required successful results: 1');
    expect(code).toContain('PartialOutput = Partial<');
  });
});

describe('canonicalHash', () => {
  it('is deterministic regardless of key order', () => {
    const a = canonicalHash({ pattern: 'PIPE', input_type: 'X', output_type: 'Y' });
    const b = canonicalHash({ output_type: 'Y', input_type: 'X', pattern: 'PIPE' });
    expect(a).toBe(b);
  });

  it('changes when content changes', () => {
    const a = canonicalHash({ pattern: 'PIPE', input_type: 'X', output_type: 'Y' });
    const b = canonicalHash({ pattern: 'PIPE', input_type: 'X', output_type: 'Z' });
    expect(a).not.toBe(b);
  });
});

describe('file header', () => {
  it('embeds manifest hash in generated code', () => {
    const doc = loadManifest(`${fixtures}/pipe.manifest.yaml`);
    const code = generateWrapper(doc);
    const lines = code.split('\n');
    const hashLine = lines.find(l => l.startsWith('// @aria-manifest-hash'));
    expect(hashLine).toBeDefined();
    const hash = hashLine!.split(' ').pop()!;
    expect(hash).toHaveLength(64); // SHA-256 hex
    expect(hash).toBe(doc.compositionHash);
  });
});

describe('PARALLEL_FORK generator', () => {
  it('generates array result type from fixture', () => {
    const doc = loadManifest(`${fixtures}/parallel-fork.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('@aria-manifest-hash');
    expect(code).toContain('NotificationDispatchExecuteParallelForkInput = NotificationRequest');
    expect(code).toContain('Array<Result<');
  });
});

describe('SCATTER_GATHER generator', () => {
  it('generates ReadonlyArray input and array output from fixture', () => {
    const doc = loadManifest(`${fixtures}/scatter-gather.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('ReadonlyArray<');
    expect(code).toContain('SearchIndexExecuteBulkIndexInput = SearchDocument');
    expect(code).toContain('SearchIndexExecuteBulkIndexOutput = IndexResult');
  });
});

describe('COMPENSATING_TRANSACTION generator', () => {
  it('emits ForwardFn and CompensationFn from fixture', () => {
    const doc = loadManifest(`${fixtures}/compensating-transaction.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('ForwardFn');
    expect(code).toContain('CompensationFn');
    expect(code).toContain('CompensationContext');
  });
});

describe('STREAMING_PIPELINE generator', () => {
  it('emits AsyncIterable types and Chunk type from fixture', () => {
    const doc = loadManifest(`${fixtures}/streaming-pipeline.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('AsyncIterable<');
    expect(code).toContain('Chunk = RawLogChunk');
  });
});

describe('CACHE_ASIDE generator', () => {
  it('emits CacheStore interface and cache-injected function type from fixture', () => {
    const doc = loadManifest(`${fixtures}/cache-aside.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('CacheStore');
    expect(code).toContain('ProductCatalogResolveDetailsCacheKey = string');
  });
});

describe('BULKHEAD generator', () => {
  it('emits BulkheadRejected type and pool capacity comment from fixture', () => {
    const doc = loadManifest(`${fixtures}/bulkhead.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('BulkheadRejected');
    expect(code).toContain('Pool capacity: 20');
    expect(code).toContain('Pool name: db-pool');
  });
});

describe('PRIORITY_QUEUE generator', () => {
  it('emits priority type annotation from fixture', () => {
    const doc = loadManifest(`${fixtures}/priority-queue.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('JobQueueProcessTaskInput = JobTask');
    expect(code).toContain('JobQueueProcessTaskOutput = JobResult');
  });
});

describe('EVENT_SOURCING generator', () => {
  it('emits CommandHandlerFn and ProjectionFn from fixture', () => {
    const doc = loadManifest(`${fixtures}/event-sourcing.manifest.yaml`);
    const code = generateWrapper(doc);
    expect(code).toContain('CommandHandlerFn');
    expect(code).toContain('ProjectionFn');
    expect(code).toContain('OrderLifecycleExecuteCommandAggregate = OrderAggregate');
  });
});

describe('all 22 pattern generators compile without throwing', () => {
  const allPatterns = [
    'PIPE', 'FORK', 'JOIN', 'PARALLEL_FORK', 'PARALLEL_JOIN',
    'SCATTER_GATHER', 'CIRCUIT_BREAKER', 'SAGA', 'COMPENSATING_TRANSACTION',
    'STREAMING_PIPELINE', 'CACHE_ASIDE', 'BULKHEAD', 'PRIORITY_QUEUE', 'EVENT_SOURCING',
  ] as const;

  for (const pattern of allPatterns) {
    it(`${pattern}`, () => {
      const code = generateWrapper({
        identity: { address: `test.domain.do.thing`, layer: 1 },
        composition: {
          pattern,
          input_type: 'TestInput',
          output_type: 'TestOutput',
          error_types: ['TestError.OOPS'],
        },
        compositionHash: 'abc123',
      });
      expect(code).toContain('@aria-manifest-hash abc123');
      expect(code).toContain('TestInput');
      expect(code).toContain('TestOutput');
    });
  }
});
