# Context Manifests
### Pillar 4 of ARIA — COST of understanding and modifying each piece

---

## The Context Budget Problem

Every AI code helper operates within a context window. Reading irrelevant code is not free — it displaces relevant code, dilutes attention, and increases the probability of errors. In large codebases, unmanaged context is the primary cause of AI mistakes.

ARIA treats **context as a scarce resource** that must be explicitly managed. The Context Manifest is the mechanism for this management.

> A Context Manifest answers one question for the AI: *"What is the minimum I need to read to accomplish this task?"*

---

## The Manifest Structure

Every ARU has a manifest. The manifest is machine-readable and co-located with the ARU. The **complete authoritative schema** with all fields, derivation tiers, and mandatoriness rules is in `20-manifest-schema.md`. This section shows a representative example for context loading purposes.

```yaml
manifest:
  id: "auth.token.validate"
  version: "2.1.0"
  layer: L1
  
  identity:
    purpose: "validates JWT token signature and expiry"
    domain: "auth"
    subdomain: "token"
    verb: "validate"
    entity: "token"

  contract:
    input:
      type: "TokenString"
      constraints:
        - "non-empty string"
        - "matches JWT format regex"
    output:
      success: "ValidatedToken"
      failure: "AuthError { code: EXPIRED | INVALID_SIGNATURE | MALFORMED }"
    side_effects: NONE
    idempotent: true
    deterministic: true

  dependencies:
    - id: "crypto.jwt.decode"
      layer: L1
    - id: "time.now"
      layer: L1

  context_budget:
    to_use: 120         # tokens needed to USE this ARU (call it correctly)
    to_modify: 340      # tokens needed to SAFELY MODIFY behavior
    to_extend: 580      # tokens needed to ADD new functionality
    to_replace: 200     # tokens needed to REPLACE with equivalent ARU

  test_contract:
    - scenario: "valid unexpired token returns ValidatedToken with correct claims"
    - scenario: "expired token returns AuthError with code EXPIRED"
    - scenario: "tampered signature returns AuthError with code INVALID_SIGNATURE"
    - scenario: "non-JWT string returns AuthError with code MALFORMED"

  stability: STABLE     # EXPERIMENTAL | STABLE | FROZEN

  behavioral_contract:
    max_latency_p99: "15ms"
    max_latency_p999: "50ms"
    max_calls_per_second: 10000      # system-wide rate limit
    max_calls_per_user_per_second: 20
    idempotent_within: "not_applicable"
    timeout: "100ms"                 # caller should abandon after this
    must_be_called_after: []         # ordering constraints
    must_be_called_before: []
    max_retries: 3
    retry_strategy: "exponential_backoff"
```

---

## Context Budget Levels

The four context budget levels correspond to distinct AI task types:

### `to_use` (smallest)
What an AI needs when it wants to *call* this ARU from another ARU.
Includes: function signature, input/output types, error types, usage example.
AI loading this: "I'm building something that needs this capability."

### `to_modify` (medium)
What an AI needs to change the internal behavior of this ARU.
Includes: `to_use` + implementation logic + dependency contracts + test scenarios.
AI loading this: "I need to change how this works."

### `to_extend` (larger)
What an AI needs to add new functionality to this ARU or create a sibling.
Includes: `to_modify` + related ARUs in the same subdomain + composition patterns in use.
AI loading this: "I need to add a new capability nearby."

### `to_replace` (specialized)
What an AI needs to implement an equivalent replacement.
Includes: contract + test scenarios + interface only (no implementation).
AI loading this: "I need to rewrite this from scratch."

---

## Progressive Disclosure Protocol

The manifest enables **progressive disclosure** — AI loads information in layers, stopping when it has enough:

```
Level 1:  SIGNATURE     ~50 tokens   → name, layer, purpose (always loaded)
Level 2:  CONTRACT     ~200 tokens   → types, errors, effects
Level 3:  BEHAVIOR     ~500 tokens   → logic summary, test scenarios
Level 4:  IMPLEMENTATION  full       → actual code
```

AI starts at Level 1 and descends only when the task requires it.

| Task | Context Level Needed |
|---|---|
| Understand system topology | Level 1 only |
| Call an ARU correctly | Level 2 |
| Debug unexpected behavior | Level 3 |
| Refactor internals | Level 4 |
| Write a new ARU in the same domain | Level 2 of neighbors + Level 4 of one example |

---

## Stability Markers

Every ARU declares its stability state:

| State | Meaning | AI Behavior |
|---|---|---|
| `EXPERIMENTAL` | Contract may change | Load full context; do not cache contract |
| `STABLE` | Contract is reliable | Cache contract; load implementation as needed |
| `FROZEN` | Contract will never change | Cache forever; implementation is read-only |

L0 Primitives are always `FROZEN`. L1 Atoms should reach `STABLE` quickly. L3+ Organisms are typically `STABLE` or `EXPERIMENTAL` during active development.

---

## The Manifest as AI Work Order

When an AI is assigned a task, the first step is **manifest resolution**: gather the manifests of all ARUs relevant to the task and compute the total context budget.

```
Task: "Add rate limiting to auth.token.validate"

Manifest Resolution:
  Primary ARU:    auth.token.validate         → to_extend:  580 tokens
  Related ARU:    auth.ratelimit.check        → to_use:     120 tokens
  Pattern needed: GATE (conditional pass)     → pattern:     80 tokens
  ──────────────────────────────────────────────────────────────────
  Total budget:   ~780 tokens (before implementation)
```

This tells the AI exactly what to load, in what order, and when to stop reading.

---

## Manifest Aggregation: System-Level Budget

At Layer 4 (System), an AI orchestrating a pipeline can compute the **total context budget** for understanding the entire flow:

```
PaymentProcessingPipeline context budget:
  Manifest summaries of all 12 ARUs:   12 × 50  =  600 tokens   (signatures)
  Contracts of directly used ARUs:      5 × 200  = 1000 tokens   (L3 organisms)
  Composition pattern declarations:     8 × 80   =  640 tokens   (connections)
  ─────────────────────────────────────────────────────────────────────────────
  Total to orchestrate correctly:       ~2240 tokens
```

Without manifests, an AI would read all implementation code — easily 50,000+ tokens for the same system.

## Temporal and Behavioral Contracts

The type system captures structural invariants — what data looks like. Temporal contracts capture **behavioral invariants** — how an ARU behaves over time, under load, and in distributed contexts.

These are critical for AI generating code that interacts with rate-limited APIs, payment systems, event-driven pipelines, or any ARU with real-world performance requirements.

### Behavioral Contract Fields

```yaml
behavioral_contract:
  # Latency (measured at the ARU boundary, not the network boundary)
  max_latency_p50:   "5ms"      # median latency target
  max_latency_p99:   "15ms"     # tail latency target (99th percentile)
  max_latency_p999:  "50ms"     # extreme tail (999th percentile)
  timeout:           "100ms"    # caller must abandon and fail-fast after this

  # Rate limits (AI generating retry/loop logic MUST respect these)
  max_calls_per_second:          10000   # system-wide
  max_calls_per_user_per_second: 20      # per-identity limit (if applicable)
  max_concurrent_calls:          500     # parallelism ceiling

  # Retry semantics (for LOOP/RECOVER pattern generation)
  max_retries:       3
  retry_strategy:    "exponential_backoff"   # none | fixed | exponential_backoff | jitter
  retryable_errors:  ["TIMEOUT", "PROVIDER_UNAVAILABLE"]  # only these are safe to retry
  non_retryable_errors: ["INVALID_SIGNATURE", "EXPIRED"]  # retrying is wrong

  # Idempotency (critical for payment/mutation ARUs)
  idempotent_within: "24h"      # safe to call again with same input within this window
                                 # "not_applicable" for pure functions
                                 # "never" for non-idempotent side-effecting ARUs

  # Ordering constraints (for AI generating compositions)
  must_be_called_after:  []     # list of ARU ids that must have been called first
  must_be_called_before: []     # list of ARU ids that must be called after

  # Circuit breaker hint (see 03-composition-patterns.md §CIRCUIT_BREAKER)
  circuit_breaker:
    failure_threshold_percent: 50
    evaluation_window: "10s"
    open_duration: "30s"
```

### Why Behavioral Contracts Matter for AI

Without these fields, an AI generating retry logic has no information about:
- Which errors are safe to retry (retrying `INVALID_SIGNATURE` is meaningless)
- How long to wait between retries
- How many retries are safe before giving up
- Whether calling the same ARU twice produces duplicate side effects

An AI generating a payment flow without `idempotent_within: "24h"` may generate code that double-charges customers on retry. An AI generating a rate-limited API client without `max_calls_per_user_per_second: 20` will generate code that hammers the provider and triggers bans.

**Behavioral contracts are not optional.** For any ARU with `side_effects != NONE` or `deterministic: false`, the `behavioral_contract` section is required or the manifest is incomplete.

### Behavioral Contract Derivation

| Field | How Derived |
|---|---|
| `max_latency_*` | Measured by performance test suite; auto-updated on CI |
| `timeout` | Set by human (business SLA decision) |
| `max_calls_per_second` | Declared by human (from provider docs or load testing) |
| `idempotent_within` | Declared by human (business rule for at-least-once delivery) |
| `retryable_errors` | Derived from error type semantics + human review |
| `must_be_called_after/before` | Declared by human (ordering is a business rule) |

---

## Manifest Derivation Specification

The claim that "manifests are inferred from typed code" requires precision. Different fields have different derivation sources. Using a stale or incorrect manifest is worse than having no manifest — it actively misleads the AI.

### Field Derivation Table

| Field | Source | Method | Staleness Risk |
|---|---|---|---|
| `id` | File path | Deterministic from directory structure | None — changes break builds |
| `version` | Version control | Extracted from git tag / package version | None — tied to release process |
| `layer` | Directory structure | Inferred from path segment `l0/`–`l5/` | None |
| `contract.input` | Type signature | Static analysis of function signature | Low — type errors catch drift |
| `contract.output` | Type signature | Static analysis | Low |
| `contract.side_effects` | Effect system annotations | Extracted from effect type declarations | Low if effect system enforced |
| `contract.idempotent` | Test coverage | Derived from idempotency test presence | Medium |
| `contract.deterministic` | Side effects + randomness analysis | Static analysis of dependencies | Medium |
| `dependencies` | Import statements | Extracted from import graph | Low |
| `context_budget.*` | Token count of rendered levels | Measured on each build | Low — automated |
| `test_contract` | Test file analysis | AI proposes from test names + assertions | **High** — tests drift from intent |
| `identity.purpose` | **Cannot be derived** | Human writes; AI may draft | **High** — easily goes stale |
| `stability` | **Cannot be derived** | Human declares | Medium |
| `behavioral_contract.max_latency_*` | Performance test results | Measured on CI | Low — automated |
| `behavioral_contract.timeout` | **Cannot be derived** | Human declares | Low — rarely changes |
| `behavioral_contract.idempotent_within` | **Cannot be derived** | Human declares (business rule) | Low — rarely changes |
| `behavioral_contract.retryable_errors` | Error type semantics + human | Semi-derived + human review | Medium |

### Three Derivation Tiers

**Tier 1 — Auto-Derived (build-time, zero human effort):**
`id`, `version`, `layer`, `contract.input`, `contract.output`, `dependencies`, `context_budget.*`, `max_latency_*`

**Tier 2 — AI-Proposed, Human-Approved (low human effort):**
`identity.purpose`, `test_contract`, `retryable_errors`, `contract.deterministic`

The AI drafts these from naming conventions, type signatures, and test patterns. A human reviews and approves. Approval is a 30-second review, not a writing task. Disagreement triggers a flag in the build — not a failure, but a tracked annotation debt.

**Tier 3 — Human-Declared (non-derivable):**
`stability`, `timeout`, `idempotent_within`, `must_be_called_after/before`, `behavioral_contract.max_calls_per_*`

These encode business decisions, SLA commitments, and operational constraints that have no code-derivable equivalent.

### Manifest Staleness Detection

A manifest is flagged as potentially stale when:
- The ARU's implementation changed since `manifest_generated_at`
- The input or output type changed (even non-breaking)
- A dependency was added or removed
- A new error type was added to the output union

On flag: the affected fields are re-derived (Tier 1), re-proposed (Tier 2), or a human notification is issued (Tier 3). The manifest never silently drifts — stale fields are visible in the build output.

---

## Manifest Generation (Summary)

Manifests are a **hybrid artifact** — partly generated, partly proposed by AI, partly declared by humans. The split is defined by the derivation table above.

The manifest is co-located with the ARU and versioned alongside it. It is never a separate documentation system — it lives or dies with its ARU.

1. **Tier 1 fields are auto-generated** on every build from static analysis and test metrics
2. **Tier 2 fields are AI-proposed** when the ARU is first created; humans approve during code review
3. **Tier 3 fields are human-declared** once and rarely change; tracked as annotation debt if missing
4. **Staleness detection** runs on every build; stale fields are flagged, not silently accepted
5. **Context budgets are measured** by rendering each progressive disclosure level and counting tokens against the target AI model
