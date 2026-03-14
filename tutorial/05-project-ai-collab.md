# Chapter 5: Project 3 — L5 Boundaries & Human-AI Collaboration

## 1. L5 — The Domain Boundary

So far, the URL shortener has been built as a self-contained set of ARUs from L0 through L4. But the system doesn't exist in a vacuum. Something has to expose it to the outside world — an HTTP server, a message consumer, a webhook handler. That something is **L5**.

Here's the distinction:

- **L4 orchestrates *within* a domain** — it wires together L3 organisms to execute a workflow
- **L5 defines the *boundary* of a domain** — it decides what the domain exposes to the outside world and how it integrates with other domains

Think of L5 as the loading dock of a warehouse. The warehouse does its own internal work (L0–L4). The loading dock is where trucks from the outside world arrive and pick up goods. It translates between the external world's language (HTTP requests, webhook payloads, CLI flags) and the internal domain's language (typed ARU contracts).

```
Inside the domain (L0–L4):
  url.pipeline.orchestrate.shorten    ← L4, internal
  url.store.persist.link              ← L3, internal
  url.link.create.fromOriginal        ← L2, internal

Domain surface (L5):
  url.domain.expose.api               ← L5, the public HTTP contract
  url.domain.integrate.webhooks       ← L5, external system integration
  url.domain.guard.rateLimit          ← L5, entry-point protection
```

The L5 verb vocabulary reflects this boundary role: `expose`, `integrate`, `guard`, `translate`.

### `src/url/domain/expose.api.manifest.yaml`

```yaml
manifest:
  id: "url.domain.expose.api"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L5
    inferred: L5
  identity:
    purpose: "exposes the URL shortener domain as an HTTP API"
    domain: "url"
    subdomain: "domain"
    verb: "expose"
    entity: "api"
  contract:
    input:
      type: "HttpRequest"
    output:
      success: "HttpResponse"
      failure: "HttpError.BAD_REQUEST | HttpError.NOT_FOUND | HttpError.RATE_LIMITED | HttpError.INTERNAL"
    side_effects: EXTERNAL
    idempotent: false
    deterministic: false
  dependencies:
    - id: "url.pipeline.orchestrate.shorten"
      layer: L4
      stability: EXPERIMENTAL
    - id: "url.store.execute.shortCode"
      layer: L3
      stability: EXPERIMENTAL
  context_budget:
    to_use: 150
    to_modify: 500
    to_extend: 700
    to_replace: 400
  test_contract:
    scenarios:
      - scenario: "POST /shorten with valid URL returns 200 with shortCode"
      - scenario: "GET /:shortCode with known code returns 302 redirect"
      - scenario: "GET /:shortCode with unknown code returns 404"
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections:
    - pattern: ROUTE
      target: "url.pipeline.orchestrate.shorten"
    - pattern: ROUTE
      target: "url.store.execute.shortCode"
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

The `ROUTE` pattern means: given the incoming request, exactly one branch fires — either the shorten pipeline (`POST /shorten`) or the resolve lookup (`GET /:shortCode`). All declared branches must be handled; you cannot have an implicit catch-all at the L5 boundary.

---

## 2. The Semantic Graph

When you run `aria-build bundle ./src`, the CLI reads every `.manifest.yaml` in the source tree and assembles them into a single semantic graph snapshot — a JSON document where every node is an ARU and every edge is a declared pattern instance.

```bash
aria-build bundle ./src
```

The output file (typically `aria-bundle.json`) contains:

```json
{
  "nodes": [
    { "id": "url.types", "layer": "L0", "stability": "EXPERIMENTAL" },
    { "id": "url.shortcode.validate.format", "layer": "L1", "stability": "EXPERIMENTAL" },
    { "id": "url.shortcode.generate.hash", "layer": "L1", "stability": "EXPERIMENTAL" },
    { "id": "url.link.create.fromOriginal", "layer": "L2", "stability": "EXPERIMENTAL" },
    { "id": "url.store.persist.link", "layer": "L3", "stability": "EXPERIMENTAL" },
    { "id": "url.store.execute.shortCode", "layer": "L3", "stability": "EXPERIMENTAL" },
    { "id": "url.analytics.emit.clickEvent", "layer": "L3", "stability": "EXPERIMENTAL" },
    { "id": "url.audit.emit.shortenEvent", "layer": "L3", "stability": "EXPERIMENTAL" },
    { "id": "url.pipeline.orchestrate.shorten", "layer": "L4", "stability": "EXPERIMENTAL" },
    { "id": "url.domain.expose.api", "layer": "L5", "stability": "EXPERIMENTAL" }
  ],
  "edges": [
    { "from": "url.shortcode.validate.format", "to": "url.link.create.fromOriginal", "pattern": "VALIDATE" },
    { "from": "url.shortcode.generate.hash", "to": "url.link.create.fromOriginal", "pattern": "PIPE" },
    { "from": "url.link.create.fromOriginal", "to": "url.store.persist.link", "pattern": "PIPE" },
    { "from": "url.pipeline.orchestrate.shorten", "to": "url.analytics.emit.clickEvent", "pattern": "FORK" },
    { "from": "url.pipeline.orchestrate.shorten", "to": "url.store.persist.link", "pattern": "CIRCUIT_BREAKER" },
    { "from": "url.pipeline.orchestrate.shorten", "to": "url.audit.emit.shortenEvent", "pattern": "OBSERVE" },
    { "from": "url.domain.expose.api", "to": "url.pipeline.orchestrate.shorten", "pattern": "ROUTE" },
    { "from": "url.domain.expose.api", "to": "url.store.execute.shortCode", "pattern": "ROUTE" }
  ]
}
```

This graph is the complete map of the domain. Every node is an ARU with a known layer, stability, and context budget. Every edge is a declared, named relationship — not an inferred one discovered by tracing imports at runtime.

The graph enables three things the source code alone cannot give you:

1. **Change impact analysis** — before touching any ARU, ask the graph what depends on it
2. **Precise context loading** — load only the manifests relevant to a given task, not the entire codebase
3. **Compliance checking** — verify layer rules, naming conventions, and pattern consistency across the whole system in a single pass

---

## 3. Impact Analysis with `aria-build impact`

Before making any change to an existing ARU, run the impact command. It tells you exactly which other ARUs are affected, through which patterns, and how many context tokens you'd need to safely load for an AI-assisted modification.

```bash
aria-build impact url.store.persist.link
```

Output:

```
Impact analysis for: url.store.persist.link

Direct dependents (1):
  → url.pipeline.orchestrate.shorten  [L4] via CIRCUIT_BREAKER + PIPE

Transitive dependents (1):
  → url.domain.expose.api             [L5] via ROUTE

Affected context budget: 1,850 tokens
Suggested context window for AI agent:
  - url.store.persist.link.manifest.yaml           (to_modify: 350)
  - url.pipeline.orchestrate.shorten.manifest.yaml  (to_use: 120)
  - url.domain.expose.api.manifest.yaml             (to_use: 150)
  Total: 620 tokens minimum / 1,850 tokens full context
```

This output tells you three things:

1. **Changing `persist.link` directly affects the L4 orchestrator** — the orchestrator depends on it via both PIPE and CIRCUIT_BREAKER
2. **Transitively, the L5 API boundary is affected** — because the orchestrator is what the API boundary routes to
3. **The minimum context to load for an AI agent is 620 tokens** — just the three manifests, not the full implementation files

The `to_use` budget means "how many tokens an AI agent needs to *read and understand* this ARU". The `to_modify` budget means "how many tokens it needs if it's going to *change* this ARU". The distinction is important for context window management at scale.

---

## 4. SAGA Pattern — URL Bulk Import

A new feature request: **bulk URL import**. Users can submit a list of URLs to shorten in one request. The requirement comes with a constraint: if any step in the batch fails after earlier steps have succeeded, the entire operation must roll back.

This is a classic distributed transaction scenario. ARIA handles it with the **SAGA** pattern.

```
url.import.execute.batch
   │
   ├─ Step 1: url.import.validate.urls
   │          [compensate: discard validated list — no-op]
   │
   ├─ Step 2: url.store.persist.link (×N)
   │          [compensate: url.store.delete.link (×N) — undo writes]
   │
   └─ Step 3: url.import.emit.completedEvent
              [compensate: url.import.emit.rolledBackEvent]
```

Each step has a declared compensation. If Step 3 fails, the SAGA framework runs the compensations for Step 2 and Step 1 in reverse order. The result is either a fully committed import or a fully rolled-back one — no partial state.

The compensation for Step 2 (`url.store.delete.link`) deletes each link that was persisted in that step. The compensation for Step 3 (`url.import.emit.rolledBackEvent`) emits an event telling downstream systems that the import was cancelled. These compensations are ARUs with their own manifests — they're not anonymous lambdas.

### `src/url/import/execute.batch.manifest.yaml`

```yaml
manifest:
  id: "url.import.execute.batch"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L3
    inferred: L3
  identity:
    purpose: "imports a batch of URLs with full rollback on partial failure"
    domain: "url"
    subdomain: "import"
    verb: "execute"
    entity: "batch"
  contract:
    input:
      type: "OriginalUrl[]"
      constraints:
        - "array of 1–100 URLs"
    output:
      success: "ImportResult { imported: number, shortCodes: ShortCode[] }"
      failure: "ImportError.PARTIAL_FAILURE | ImportError.VALIDATION_FAILED"
    side_effects: WRITE
    idempotent: false
    deterministic: false
  dependencies:
    - id: "url.import.validate.urls"
      layer: L1
      stability: EXPERIMENTAL
    - id: "url.store.persist.link"
      layer: L3
      stability: EXPERIMENTAL
    - id: "url.import.emit.completedEvent"
      layer: L3
      stability: EXPERIMENTAL
  behavioral_contract:
    max_retries: 0
    retry_strategy: none
    timeout: 10000ms
  health_contract:
    sla_latency_p99: 5000ms
    sla_availability: 99.0%
  diagnostic_surface:
    failure_indicators:
      - symptom: "ImportError.PARTIAL_FAILURE returned"
        check: "one or more persist steps failed mid-batch; check rollback completion"
      - symptom: "ImportError.VALIDATION_FAILED returned"
        check: "input URLs failed format validation before any writes occurred"
  context_budget:
    to_use: 150
    to_modify: 600
    to_extend: 800
    to_replace: 500
  test_contract:
    scenarios:
      - scenario: "batch of valid URLs returns ImportResult with correct count"
      - scenario: "partial persist failure triggers rollback and returns ImportError PARTIAL_FAILURE"
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections:
    - pattern: SAGA
      steps:
        - target: "url.import.validate.urls"
          compensate: null
        - target: "url.store.persist.link"
          compensate: "url.store.delete.link"
        - target: "url.import.emit.completedEvent"
          compensate: "url.import.emit.rolledBackEvent"
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

The SAGA declaration in the `connections` block explicitly names every step and every compensation. An AI agent or a human reviewer can read this manifest and understand the full transaction topology without reading a single line of implementation code.

---

## 5. PARALLEL_JOIN Pattern — Bulk Validation

Validating a list of 100 URLs sequentially would be slow. The **PARALLEL_JOIN** pattern validates them all concurrently, with a timeout budget that prevents a single slow validation from blocking the entire batch.

```
url.import.validate.urls
         │
         ▼  (PARALLEL_JOIN, timeout: 5000ms)
   ┌─────┼─────┐
   ▼     ▼     ▼
 url1   url2   url3  (validate each concurrently)
   └─────┼─────┘
         ▼
  ValidationReport { valid: string[], invalid: string[], timedOut: string[] }
  [on timeout: partial result with timed-out items marked as TIMEOUT]
```

The `config.onTimeout: "partial"` setting means: if the timeout fires before all validators complete, return what you have so far, with unfinished items marked in `timedOut`. The alternative — `fail-fast` — would return an error immediately when any validator hits the timeout.

### `src/url/import/validate.urls.manifest.yaml`

```yaml
manifest:
  id: "url.import.validate.urls"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L1
    inferred: L1
  identity:
    purpose: "validates a batch of URLs concurrently within a timeout budget"
    domain: "url"
    subdomain: "import"
    verb: "validate"
    entity: "urls"
  contract:
    input:
      type: "string[]"
      constraints:
        - "array of 1–100 URL strings"
    output:
      success: "ValidationReport { valid: string[], invalid: string[], timedOut: string[] }"
      failure: "ValidationError.EMPTY_BATCH"
    side_effects: NONE
    idempotent: true
    deterministic: false
  dependencies:
    - id: "url.shortcode.validate.format"
      layer: L1
      stability: EXPERIMENTAL
  context_budget:
    to_use: 90
    to_modify: 300
    to_extend: 450
    to_replace: 250
  test_contract:
    scenarios:
      - scenario: "all valid URLs returns ValidationReport with empty invalid and timedOut"
      - scenario: "mix of valid and invalid URLs returns correct counts in each category"
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections:
    - pattern: PARALLEL_JOIN
      target: "url.shortcode.validate.format"
      config:
        timeoutMs: 5000
        onTimeout: "partial"
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

`deterministic: false` here is notable — the function is pure in the sense that it has no side effects, but timing introduces non-determinism. The same input list may produce different `timedOut` entries depending on system load. The manifest captures this nuance.

---

## 6. Testing Across All Layers

With the full L0–L5 stack now defined, this section shows how to test each layer type and run the complete suite.

### What to Test at Each Layer

| Layer | Test type | Rationale |
|-------|-----------|-----------|
| L0 | None (compiler) | TypeScript enforces types at compile time |
| L1 | Pure unit | No deps, no mocks needed — just call the function |
| L2 | Unit + store reset | Pure but transitively imports store module |
| L3 | Unit with `beforeEach(clearStore)` | Stateful — store must be reset between tests |
| L4 | Unit (mock FORK targets) + integration | Mock fire-and-forget forks; PIPE chain runs real |
| L5 | Integration | Tests the full HTTP request → response cycle |

### L5 Integration Test — `src/url/domain/expose.api.test.ts`

The L5 boundary test drives the HTTP layer end-to-end. At this stage the HTTP server is represented by a handler function (not a running server), but the shape is the same:

```typescript
import { describe, it, expect, beforeEach, vi } from "vitest";
import { clearStore } from "../store/in-memory.store";
import { orchestrateShorten } from "../pipeline/orchestrate.shorten";
import { executeShortCode } from "../store/execute.shortCode";
import * as analytics from "../analytics/emit.clickEvent";
import type { OriginalUrl } from "../types";

// Simulated L5 handler — in production this would be an Express/Hono/Fastify route
async function handleShorten(body: { url: string }) {
  if (!body.url) return { status: 400, body: { error: "url required" } };
  const result = await orchestrateShorten(body.url as OriginalUrl);
  if (!result.ok) {
    const status = result.error.code === "DUPLICATE" ? 409 : 500;
    return { status, body: { error: result.error.code } };
  }
  return { status: 200, body: { shortCode: result.value.shortCode } };
}

async function handleResolve(shortCode: string) {
  const result = await executeShortCode(shortCode as any);
  if (!result.ok) return { status: 404, body: { error: "NOT_FOUND" } };
  return { status: 302, headers: { location: result.value.originalUrl } };
}

describe("url.domain.expose.api (integration)", () => {
  beforeEach(() => {
    clearStore();
    vi.spyOn(analytics, "emitClickEvent").mockResolvedValue(undefined);
  });

  it("POST /shorten with valid URL returns 200 with shortCode", async () => {
    const res = await handleShorten({ url: "https://example.com" });
    expect(res.status).toBe(200);
    expect((res.body as any).shortCode).toHaveLength(6);
  });

  it("GET /:shortCode with known code returns 302 redirect", async () => {
    const shortenRes = await handleShorten({ url: "https://example.com" });
    const shortCode = (shortenRes.body as any).shortCode;
    const res = await handleResolve(shortCode);
    expect(res.status).toBe(302);
    expect((res.headers as any).location).toBe("https://example.com");
  });

  it("GET /:shortCode with unknown code returns 404", async () => {
    const res = await handleResolve("xxxxxx");
    expect(res.status).toBe(404);
  });
});
```

This test exercises the *full stack* from L5 down through L4, L2, L1, and L3 — with only the analytics FORK mocked. If any ARU is misconfigured, misnamed, or has a broken contract, this test will catch it.

### Running the Full Suite

```bash
npm test
```

Expected output at the end of Chapter 5:

```
✓ src/url/shortcode/validate.format.test.ts       (4 tests)
✓ src/url/shortcode/generate.hash.test.ts         (3 tests)
✓ src/url/link/create.fromOriginal.test.ts        (3 tests)
✓ src/url/store/persist.link.test.ts              (2 tests)
✓ src/url/store/execute.shortCode.test.ts         (2 tests)
✓ src/url/analytics/emit.clickEvent.test.ts       (2 tests)
✓ src/url/pipeline/orchestrate.shorten.test.ts    (3 tests)
✓ src/url/domain/expose.api.test.ts               (3 tests)

Test Files  8 passed (8)
     Tests  22 passed (22)
```

Run unit tests only (faster during development):

```bash
npx vitest run --exclude "**/*.integration.test.ts" --exclude "**/expose.api.test.ts"
```

---

## 7. Human-AI Collaboration Workflow

This is where the whole system pays off. Everything built so far — the layer boundaries, the manifests, the semantic graph, the impact analysis — is designed to make AI-assisted development *safer and more accurate*.

Let's walk through a concrete scenario.

### Scenario: Add rate limiting to the URL shortener

A developer wants to add rate limiting to prevent users from creating more than 100 short URLs per minute. They want help from an AI agent.

---

**Without ARIA:**

The AI agent either receives the entire codebase (thousands of tokens, most of it irrelevant) or receives only the file the developer thinks matters (missing context from dependent systems). In the first case, the agent hallucinates changes to unrelated code. In the second, it makes incomplete changes that break at runtime.

---

**With ARIA:**

**Step 1**: Run impact analysis before giving the task to the agent.

```bash
aria-build impact url.pipeline.orchestrate.shorten
```

Output:

```
Impact analysis for: url.pipeline.orchestrate.shorten

Direct dependents (1):
  → url.domain.expose.api  [L5] via ROUTE

Suggested context window for AI agent (to_modify scope):
  - url.pipeline.orchestrate.shorten.manifest.yaml   (400 tokens)
  - url.domain.expose.api.manifest.yaml              (150 tokens)
  - url.store.persist.link.manifest.yaml             (100 tokens)
  Total: 650 tokens
```

**Step 2**: Load only those three manifests — plus their implementation files if the agent needs to write code — into the agent's context window.

This is 650 tokens of manifests instead of however many thousands of tokens the full codebase contains. The agent gets precisely the context it needs to make a correct, scoped change.

**Step 3**: Give the agent a scoped task with explicit constraints.

```
You are working on the ARIA `url.pipeline.orchestrate.shorten` ARU (L4).

Your task: add rate limiting by wrapping `url.store.persist.link` with a
CIRCUIT_BREAKER pattern and introducing a new L3 ARU `url.ratelimit.enforce.perUser`
that checks a sliding-window counter before the persist step.

Loaded context:
- url.pipeline.orchestrate.shorten.manifest.yaml  (you may modify this)
- url.store.persist.link.manifest.yaml            (you may read, do not modify)
- url.domain.expose.api.manifest.yaml             (dependent — do not modify)

Rules:
- Do NOT add business logic (counter checks, limit comparisons) to L4
- Any new ARU must have a co-located .manifest.yaml
- Declare all new connections in the manifest connections: block
- Set side_effects: READ on any ARU that reads the rate limit counter
- The CIRCUIT_BREAKER config must declare failureThreshold, recoveryTimeout, successThreshold
```

**Step 4**: The agent makes the change, adds a new `url.ratelimit.enforce.perUser` L3 ARU with a manifest, and updates the L4 orchestrator's `connections` block.

**Step 5**: Run the check.

```bash
aria-build check ./src
```

If it passes, the change is compliant. If not, the output tells you exactly which rule was violated — wrong verb for layer, missing side_effects declaration, unknown connection target — and the agent can self-correct with a follow-up prompt.

---

> **The key insight**: ARIA makes AI agents more accurate by giving them *exactly the right amount of context* — not too much (which causes hallucinations and irrelevant changes), not too little (which causes incomplete changes). The semantic graph is the AI's map. The manifests are the AI's vocabulary. The `aria-build check` is the AI's compiler.

This is a fundamentally different relationship with AI tooling. Instead of "here is the whole codebase, good luck", it becomes "here is the precise subgraph of the system you need to understand to complete this task, and here are the rules you must follow". The system becomes navigable, and navigability is what makes AI assistance reliable at scale.

---

## 8. Further Reading

The concepts introduced in this chapter are covered in depth in the documentation:

- [`docs/14-human-ai-collaboration.md`](../docs/14-human-ai-collaboration.md) — the full human-AI collaboration model, including multi-turn agent workflows and context budget management
- [`docs/19-multi-agent-infrastructure.md`](../docs/19-multi-agent-infrastructure.md) — multi-agent orchestration: how multiple specialized agents work together using the semantic graph as coordination substrate
- [`docs/05-semantic-graph.md`](../docs/05-semantic-graph.md) — semantic graph specification: the schema, bundle format, and graph query API
- [`docs/15-task-decomposition.md`](../docs/15-task-decomposition.md) — how ARIA enables automatic task decomposition for AI agents based on the dependency graph
- [`docs/16-ai-agent-roles.md`](../docs/16-ai-agent-roles.md) — the taxonomy of AI agent roles in an ARIA codebase: reader, modifier, reviewer, planner

---

**[← Advanced Project](04-project-advanced.md)** | **[Back to index](00-introduction.md)** | **[Next: Conclusion →](06-conclusion.md)**
