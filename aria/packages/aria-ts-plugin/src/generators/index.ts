/**
 * Generator registry — dispatches to the correct pattern generator.
 */

import type { ManifestDoc } from '../manifest-types.js';
import { generateFileHeader } from '../file-header.js';
import { generatePipe } from './pipe.js';
import { generateFork } from './fork.js';
import { generateJoin } from './join.js';
import { generateGate } from './gate.js';
import { generateRoute } from './route.js';
import { generateLoop } from './loop.js';
import { generateObserve } from './observe.js';
import { generateTransform } from './transform.js';
import { generateValidate } from './validate.js';
import { generateCache } from './cache.js';
import { generateStream } from './stream.js';
import { generateParallelFork } from './parallel-fork.js';
import { generateParallelJoin } from './parallel-join.js';
import { generateScatterGather } from './scatter-gather.js';
import { generateCircuitBreaker } from './circuit-breaker.js';
import { generateSaga } from './saga.js';
import { generateCompensatingTransaction } from './compensating-transaction.js';
import { generateStreamingPipeline } from './streaming-pipeline.js';
import { generateCacheAside } from './cache-aside.js';
import { generateBulkhead } from './bulkhead.js';
import { generatePriorityQueue } from './priority-queue.js';
import { generateEventSourcing } from './event-sourcing.js';

export function generateWrapper(doc: ManifestDoc): string {
  const header = generateFileHeader(doc.identity.address, doc.compositionHash);
  let body: string;

  switch (doc.composition.pattern) {
    case 'PIPE':                      body = generatePipe(doc); break;
    case 'FORK':                      body = generateFork(doc); break;
    case 'JOIN':                      body = generateJoin(doc); break;
    case 'GATE':                      body = generateGate(doc); break;
    case 'ROUTE':                     body = generateRoute(doc); break;
    case 'LOOP':                      body = generateLoop(doc); break;
    case 'OBSERVE':                   body = generateObserve(doc); break;
    case 'TRANSFORM':                 body = generateTransform(doc); break;
    case 'VALIDATE':                  body = generateValidate(doc); break;
    case 'CACHE':                     body = generateCache(doc); break;
    case 'STREAM':                    body = generateStream(doc); break;
    case 'PARALLEL_FORK':             body = generateParallelFork(doc); break;
    case 'PARALLEL_JOIN':             body = generateParallelJoin(doc); break;
    case 'SCATTER_GATHER':            body = generateScatterGather(doc); break;
    case 'CIRCUIT_BREAKER':           body = generateCircuitBreaker(doc); break;
    case 'SAGA':                      body = generateSaga(doc); break;
    case 'COMPENSATING_TRANSACTION':  body = generateCompensatingTransaction(doc); break;
    case 'STREAMING_PIPELINE':        body = generateStreamingPipeline(doc); break;
    case 'CACHE_ASIDE':               body = generateCacheAside(doc); break;
    case 'BULKHEAD':                  body = generateBulkhead(doc); break;
    case 'PRIORITY_QUEUE':            body = generatePriorityQueue(doc); break;
    case 'EVENT_SOURCING':            body = generateEventSourcing(doc); break;
    default: {
      const exhaustive: never = doc.composition.pattern;
      throw new Error(`Unknown composition pattern: ${exhaustive}`);
    }
  }

  return header + '\n' + body;
}
