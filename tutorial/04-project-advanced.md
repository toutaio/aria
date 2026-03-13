# Chapter 4: Project 2 — Analytics & Orchestration (L4)

## 1. Intro — Why We Need L4

The URL shortener from Chapter 3 works, but new requirements have arrived:

1. **Track a click event** every time a short URL is resolved
2. **Add rate limiting** on the persist operation to protect the store from bursts
3. **Add audit logging** for all shorten operations, for compliance purposes

You might be tempted to add these directly to the existing L3 organisms. Let's think about why that doesn't work.

The shorten operation now needs to coordinate multiple L3 calls in sequence:
1. Create the link (L2)
2. Persist it (L3)
3. Emit an analytics event (L3)
4. Emit an audit log (L3)

If you stuff steps 3 and 4 into `persist.link`, that organism now has two responsibilities: storing data *and* firing events. It can no longer be described with a single verb phrase. It violates the Single Responsibility Principle.

**L4 exists precisely for this**: wiring multiple organisms together in a sequence, without containing any business logic of its own. L4 is the conductor. L3 is the orchestra. L4 tells each musician when to play — it does not decide what notes to play.

---

## 2. The L3 vs L4 Boundary

Before writing the L4 orchestrator, let's look at the most common mistake when working with this boundary.

### The anti-pattern — business logic in L4

```typescript
// ❌ WRONG — url.pipeline.orchestrate.shorten (L4)
export async function orchestrateShorten(originalUrl: OriginalUrl) {
  const link = createFromOriginal(originalUrl);
  if (!link.ok) return link;

  // ❌ Business logic in L4: "if the URL starts with http, it's low-risk"
  const riskLevel = originalUrl.startsWith("https") ? "low" : "high";
  if (riskLevel === "high") {
    await auditLog.warn(`High-risk URL shortened: ${originalUrl}`);
  }

  return persistLink(link.value);
}
```

What's wrong with this?

- **The `riskLevel` decision is business logic** — it belongs in an L3 ARU with its own manifest, its own test contract, and its own stability declaration
- **L4 is now untestable as pure orchestration** — you can't test "does it wire things correctly?" without also testing "does it compute risk correctly?"
- **Adding a new risk rule requires modifying L4** — which breaks its single responsibility of sequencing

### The correct pattern — logic in L3, wiring in L4

```typescript
// ✓ CORRECT — url.risk.assess.level (L3)
export function assessRiskLevel(url: OriginalUrl): RiskLevel {
  return url.startsWith("https") ? "low" : "high";
}

// ✓ CORRECT — url.pipeline.orchestrate.shorten (L4)
export async function orchestrateShorten(originalUrl: OriginalUrl) {
  const link = createFromOriginal(originalUrl);
  if (!link.ok) return link;

  const risk = assessRiskLevel(originalUrl);        // L3 call
  await emitAuditEvent({ link: link.value, risk }); // L3 call

  return persistLink(link.value);                   // L3 call
}
```

L4 only does sequencing. Every conditional, every decision, every calculation lives in an L3 ARU where it can be named, tested, and reasoned about independently.

The test for whether something belongs in L4: *"If I remove all the `if` statements, does any behaviour change?"* If yes, there is business logic in your L4 orchestrator.

---

## 3. L4 Orchestration — `url.pipeline.orchestrate.shorten`

Now let's build the correct orchestrator. It sequences three operations: create the link, persist it, and fire an analytics event as a side channel.

### `src/url/pipeline/orchestrate.shorten.ts`

```typescript
import { createFromOriginal } from "../link/create.fromOriginal";
import { persistLink } from "../store/persist.link";
import { emitClickEvent } from "../analytics/emit.clickEvent";
import type { OriginalUrl, ShortenedLink, FormatError, StoreError } from "../types";

type ShortenError = FormatError | StoreError;
type Result =
  | { ok: true; value: ShortenedLink }
  | { ok: false; error: ShortenError };

export async function orchestrateShorten(originalUrl: OriginalUrl): Promise<Result> {
  // Step 1: Create the link (L2)
  const link = createFromOriginal(originalUrl);
  if (!link.ok) return link;

  // Step 2: Persist (L3)
  const persisted = await persistLink(link.value);
  if (!persisted.ok) return persisted;

  // Step 3: Emit analytics event (L3) — FORK: main flow continues regardless
  emitClickEvent(persisted.value).catch(console.error);

  return persisted;
}
```

Step 3 is fire-and-forget. If analytics fails, the caller still gets a successful response. This is the **FORK** pattern — the same value is sent to two consumers, and the main path does not wait for the fork to complete.

### `src/url/pipeline/orchestrate.shorten.manifest.yaml`

```yaml
manifest:
  id: "url.pipeline.orchestrate.shorten"
  version: "1.0.0"
  layer: L4
  identity:
    purpose: "orchestrates the full URL shortening pipeline: create, persist, emit analytics"
    domain: "url"
    subdomain: "pipeline"
    verb: "orchestrate"
    entity: "shorten"
  contract:
    input:
      type: "OriginalUrl"
    output:
      success: "ShortenedLink"
      failure: "FormatError | StoreError"
    side_effects: EVENT          # via emitClickEvent fork
    idempotent: false
    deterministic: false
  dependencies:
    - id: "url.link.create.fromOriginal"
      layer: L2
    - id: "url.store.persist.link"
      layer: L3
    - id: "url.analytics.emit.clickEvent"
      layer: L3
  context_budget:
    to_use: 120
    to_modify: 400
    to_extend: 600
    to_replace: 350
  test_contract:
    - scenario: "valid URL returns ShortenedLink and fires analytics event"
    - scenario: "invalid URL format returns FormatError"
    - scenario: "store duplicate returns StoreError DUPLICATE"
    - scenario: "analytics failure does not affect main response"
  stability: EXPERIMENTAL
  connections:
    - pattern: PIPE
      target: "url.link.create.fromOriginal"
    - pattern: PIPE
      target: "url.store.persist.link"
    - pattern: FORK
      target: "url.analytics.emit.clickEvent"
```

The `connections` block declares three relationships: two sequential PIPEs and one FORK. The semantic graph will render these as distinct edge types, making it immediately visible that analytics is a non-blocking side channel.

---

## 4. FORK Pattern — Analytics

The FORK pattern sends the same value to multiple consumers simultaneously. In this case: the main response path and the analytics emitter.

```
url.pipeline.orchestrate.shorten
         │
         ▼  (FORK)
    ┌────┴──────────────────┐
    ▼                       ▼
response path     url.analytics.emit.clickEvent
(continues)       (fire-and-forget, async)
```

The fork consumer does not block the main path. Errors in the fork are handled independently — typically logged and discarded — so they cannot degrade the primary user experience.

### `src/url/analytics/emit.clickEvent.ts`

```typescript
import type { ShortenedLink } from "../types";

export interface ClickEvent {
  shortCode: string;
  timestamp: Date;
  type: "SHORTEN";
}

export async function emitClickEvent(link: ShortenedLink): Promise<void> {
  const event: ClickEvent = {
    shortCode: link.shortCode,
    timestamp: new Date(),
    type: "SHORTEN",
  };
  // In production: publish to event bus / analytics service
  console.log("[analytics]", JSON.stringify(event));
}
```

### `src/url/analytics/emit.clickEvent.manifest.yaml`

```yaml
manifest:
  id: "url.analytics.emit.clickEvent"
  version: "1.0.0"
  layer: L3
  identity:
    purpose: "emits a click event to the analytics stream when a URL is shortened"
    domain: "url"
    subdomain: "analytics"
    verb: "emit"
    entity: "clickEvent"
  contract:
    input:
      type: "ShortenedLink"
    output:
      success: "void"
      failure: "never (errors are swallowed — analytics must not break the main flow)"
    side_effects: EVENT
    idempotent: false
    deterministic: false
  dependencies:
    - id: "url.types"
      layer: L0
  context_budget:
    to_use: 80
    to_modify: 250
    to_extend: 350
    to_replace: 200
  stability: EXPERIMENTAL
  connections: []
```

`connections: []` here is correct — `emit.clickEvent` is a leaf node. The connection is declared on the *caller* (the orchestrator), not the callee.

---

## 5. CIRCUIT_BREAKER Pattern — Protecting the Store

A circuit breaker sits in front of a dependency that may become intermittently unavailable — a database, an external API, a downstream service. It tracks failures and, after a configurable threshold, stops sending requests ("opens the circuit") to give the dependency time to recover.

```
request
   │
   ▼  (CIRCUIT_BREAKER)
url.store.persist.link
   │
   ├── [CLOSED]    → normal operation, requests pass through
   ├── [OPEN]      → fast-fail, return StoreError STORE_UNAVAILABLE immediately
   └── [HALF-OPEN] → probe: if success → CLOSED again, if fail → OPEN
```

**When to use it**: any time you call infrastructure (database, cache, external API) that may become intermittently unavailable. Without a circuit breaker, a slow or failed dependency causes threads to pile up waiting for timeouts, which cascades into a full system failure. The circuit breaker prevents this by failing fast once the threshold is reached.

You declare the circuit breaker in the L4 manifest's `connections` block. Add this entry to `url.pipeline.orchestrate.shorten.manifest.yaml`:

```yaml
connections:
  - pattern: PIPE
    target: "url.link.create.fromOriginal"
  - pattern: PIPE
    target: "url.store.persist.link"
  - pattern: FORK
    target: "url.analytics.emit.clickEvent"
  - pattern: CIRCUIT_BREAKER
    target: "url.store.persist.link"
    config:
      failureThreshold: 5       # open after 5 consecutive failures
      recoveryTimeout: 30000    # attempt recovery after 30 seconds
      successThreshold: 2       # close again after 2 consecutive successes
```

> **Note**: The `CIRCUIT_BREAKER` is a manifest-level declaration of intent. The actual state machine is provided by the ARIA runtime or a library like [`opossum`](https://nodeshift.dev/opossum/). The manifest makes the intent visible in the semantic graph, enabling `aria-build impact` to flag "this operation is circuit-breaker-protected" when you're reasoning about retry strategies or deployment rollouts.

When the circuit is OPEN, `orchestrateShorten` will receive `StoreError { code: "STORE_UNAVAILABLE" }` immediately, without waiting for the database timeout. This means the API can return a `503` to the client in milliseconds instead of hanging for 30 seconds.

---

## 6. OBSERVE Pattern — Audit Logging

The OBSERVE pattern is a side-channel tap. It is similar to FORK, but with a stricter constraint: the observer **never changes the main flow's value or type**. The main flow passes through unchanged.

```
url.pipeline.orchestrate.shorten
         │
         ▼ (OBSERVE)
    ┌────┴──────────────────┐
    ▼                       ▼
main flow         url.audit.emit.shortenEvent
(unchanged)       (side-channel, does not affect main flow)
```

**FORK vs OBSERVE** — the key distinction:

| | FORK | OBSERVE |
|---|---|---|
| Main flow value changes? | No | No |
| Main flow type changes? | No | No |
| Caller waits for fork? | No | No |
| Semantic intent | "fan out to consumers" | "tap the stream without altering it" |
| Graph meaning | data distribution | observability / auditing |

Use FORK when you genuinely have multiple consumers of the same data. Use OBSERVE when you're instrumenting — adding logging, metrics, tracing — that should be invisible to the rest of the system.

### `src/url/audit/emit.shortenEvent.manifest.yaml`

```yaml
manifest:
  id: "url.audit.emit.shortenEvent"
  version: "1.0.0"
  layer: L3
  identity:
    purpose: "emits an audit log event whenever a URL is shortened"
    domain: "url"
    subdomain: "audit"
    verb: "emit"
    entity: "shortenEvent"
  contract:
    input:
      type: "ShortenedLink"
    output:
      success: "void"
      failure: "never"
    side_effects: EVENT
    idempotent: false
    deterministic: false
  dependencies:
    - id: "url.types"
      layer: L0
  context_budget:
    to_use: 70
    to_modify: 200
    to_extend: 300
    to_replace: 180
  stability: EXPERIMENTAL
  connections: []
```

Add the OBSERVE connection to the L4 orchestrator manifest:

```yaml
  - pattern: OBSERVE
    target: "url.audit.emit.shortenEvent"
```

The complete `connections` block for `url.pipeline.orchestrate.shorten` now reads:

```yaml
  connections:
    - pattern: PIPE
      target: "url.link.create.fromOriginal"
    - pattern: PIPE
      target: "url.store.persist.link"
    - pattern: FORK
      target: "url.analytics.emit.clickEvent"
    - pattern: CIRCUIT_BREAKER
      target: "url.store.persist.link"
      config:
        failureThreshold: 5
        recoveryTimeout: 30000
        successThreshold: 2
    - pattern: OBSERVE
      target: "url.audit.emit.shortenEvent"
```

The audit log observer and the analytics fork are both fire-and-forget. They add observability without touching the response contract.

---

## 7. Running `aria-build check`

With the new ARUs in place:

```bash
aria-build check ./src
```

Expected output:

```
✓ url.types                           L0  EXPERIMENTAL
✓ url.shortcode.validate.format       L1  EXPERIMENTAL
✓ url.shortcode.generate.hash         L1  EXPERIMENTAL
✓ url.link.create.fromOriginal        L2  EXPERIMENTAL
✓ url.store.persist.link              L3  EXPERIMENTAL
✓ url.store.resolve.shortCode         L3  EXPERIMENTAL
✓ url.analytics.emit.clickEvent       L3  EXPERIMENTAL
✓ url.audit.emit.shortenEvent         L3  EXPERIMENTAL
✓ url.pipeline.orchestrate.shorten    L4  EXPERIMENTAL

9 ARUs validated. 0 errors. 0 warnings.
Patterns validated: PIPE ×4, VALIDATE ×1, FORK ×1, CIRCUIT_BREAKER ×1, OBSERVE ×1
```

Eight distinct patterns have now appeared across this codebase. The check validates not just that each manifest is structurally valid, but that every `target` in every `connections` entry resolves to a known ARU ID — there are no dangling references.

If you rename an ARU — say, from `url.store.persist.link` to `url.store.save.link` — the check will immediately flag every manifest that references the old name. You get a refactor-safe dependency graph for free.

---

**[← Starter Project](03-project-starter.md)** | **[Back to index](00-introduction.md)** | **[Next: AI Collaboration →](05-project-ai-collab.md)**
