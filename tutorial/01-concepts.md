# Chapter 1: Core Concepts

This chapter covers everything you need to understand ARIA before writing a single line of code. If you're the kind of person who prefers to learn by doing, feel free to skim this chapter now and come back to it as a reference — each section is self-contained. But if you read one chapter end-to-end before touching the project, make it this one. The concepts here explain not just *what* ARIA is, but *why* it makes codebases easier to work with for both humans and AI agents.

---

## Abstraction Layers (L0–L5)

Every ARU lives on exactly one layer. Layers are numbered L0 through L5, and they form a strict dependency hierarchy: a layer may only depend on layers *below* it. Never upward. Never skipping.

| Layer | Name | Responsibility | Allowed Dependencies |
|-------|------|----------------|----------------------|
| L0 | Primitive | Types, constants, pure functions | None |
| L1 | Atom | Single-responsibility operations | L0 |
| L2 | Molecule | Composes atoms, one sub-purpose | L0–L1 |
| L3 | Organism | Business rules and domain decisions | L0–L2 |
| L4 | System | Orchestration only — wires organisms | L0–L3 |
| L5 | Domain | Bounded context / integration surface | L0–L4 |

### URL Shortener Layer Map

Here's how the URL shortener project maps onto these layers. You'll build each of these in Chapters 3–5:

```
L0  url.types          → ShortCode, OriginalUrl, ShortenedLink
L1  url.shortcode      → validate.format, generate.hash
L2  url.link           → create.fromOriginal
L3  url.store          → persist.link, resolve.shortCode
L4  url.pipeline       → orchestrate.shorten
L5  url.domain         → expose.api
```

### Two Hard Rules

These are non-negotiable and enforced by `aria-build check`:

**L4 contains NO business logic.** L4 systems only route, sequence, and propagate errors between L3 organisms. If you find yourself writing an `if (user.isPremium)` check in an L4 file, that check belongs in an L3 organism instead.

**L0 has NO side effects. Ever.** L0 primitives are pure: given the same input, they always produce the same output and never touch a database, file system, network, or event bus. This is what makes them safe to use as a shared foundation across the entire stack.

---

## Atomic Responsibility Units (ARUs)

An ARU is the fundamental building block of an ARIA codebase. It is the smallest deployable unit of logic, and it has exactly four things:

- **One typed input**
- **One typed output** (with explicit success and failure types)
- **One layer** (L0–L5)
- **One manifest** (a co-located `.manifest.yaml` file)

### The One Question Test

Before deciding whether something deserves to be a single ARU, apply this test:

> "Given [input], it [single verb] and returns [output]."

If the verb phrase requires the word "and", you have two ARUs, not one. Split it.

**Good example:** `url.shortcode.validate.format`
> "Given a string, it *validates* the format and returns `ValidatedShortCode | FormatError`."

One verb (`validates`), no "and". This is an ARU.

**Bad example:** an `url.shortcode.validate.formatAndNormalize`
> "Given a string, it *validates* the format *and normalizes* the casing and returns `NormalizedShortCode | FormatError`."

Two verbs. Split into `url.shortcode.validate.format` (L1) and `url.shortcode.transform.normalize` (L1). Compose them at L2.

### What Is NOT an ARU

Consider a classic `UrlService` class:

```typescript
// ❌ NOT an ARU — this is an anti-pattern in ARIA
class UrlService {
  shorten(originalUrl: string): ShortenedLink { ... }
  resolve(shortCode: string): OriginalUrl { ... }
  getStats(shortCode: string): ClickStats { ... }
  deleteLink(shortCode: string): void { ... }
}
```

What's wrong here?

1. **Four different responsibilities** — shortening, resolving, analytics, and deletion. Four reasons to change.
2. **No typed contract** — the class boundary is the only interface, and it leaks everything.
3. **Impossible to compose predictably** — you can't pipe the output of `shorten` into another ARU because there's no declared output type.
4. **No manifest** — there's no machine-readable description of what this does, so AI agents can't reason about it without reading the full implementation.

The ARIA decomposition of this class into ARUs:

```typescript
// ✅ ARIA decomposition
url.link.create.fromOriginal     // L2 — shorten
url.store.resolve.shortCode      // L3 — resolve
url.analytics.compute.stats      // L1 — get stats
url.store.persist.link           // L3 — handles persistence (including delete)
```

Each ARU has one job, one typed contract, and a manifest. They compose through declared patterns.

---

## Naming Conventions

Every ARU has a **semantic address** — a dot-separated name that encodes its position in the architecture:

```
[domain].[subdomain].[verb].[entity]
```

All four segments are required for L1 and above. L0 primitives use only `[domain].[entity]` — they don't have a verb because they don't perform operations.

All segments use **kebab-case** (lowercase, hyphens for multi-word segments).

### Step-by-Step Worked Example

Let's derive the name `url.shortcode.validate.format` from scratch:

1. **What domain does this belong to?** → `url` (it's part of the URL shortener)
2. **What subdomain — what specific aspect of that domain?** → `shortcode` (it works on short codes specifically, not links or the full URL)
3. **What layer is it?** → L1 (it's a single atomic operation — no composition needed)
4. **What verb is canonical for L1?** → `validate` (it checks a contract; see the verb table below)
5. **What entity is being acted on?** → `format` (it validates the *format* of the short code)

**Result:** `url.shortcode.validate.format`

This name tells you everything: where it lives in the domain, what layer it's on, what it does, and what it does it *to* — before you've read a single line of implementation.

### Layer-Locked Verb Vocabulary

Verbs are not chosen freely. Each layer has a canonical set of verbs, and using the wrong verb for a layer is a naming violation caught by `aria-build check`:

| Layer | Canonical Verbs |
|-------|-----------------|
| L0 | *(no verb — `domain.entity` only)* |
| L1 | `validate` `transform` `compute` `generate` `encode` `decode` `hash` `compare` |
| L2 | `create` `build` `prepare` `assemble` `resolve` |
| L3 | `execute` `apply` `process` `enforce` `emit` `authorize` `persist` |
| L4 | `orchestrate` `coordinate` `pipeline` `route` |
| L5 | `expose` `integrate` `guard` `translate` |

The verb vocabulary isn't arbitrary. It encodes the *nature* of the operation at each layer:
- L1 verbs describe pure transformations or checks — no composition.
- L2 verbs describe assembly — bringing L1 atoms together.
- L3 verbs describe decisions and side effects — the layer where business logic lives.
- L4 verbs describe routing and sequencing — no logic, only wiring.
- L5 verbs describe boundaries — how the domain is exposed to or guarded from the outside world.

### File Naming Convention

Files follow the same semantic address. For `url.shortcode.validate.format`:

```
url.shortcode.validate.format.ts             ← implementation
url.shortcode.validate.format.manifest.yaml  ← manifest (required)
url.shortcode.validate.format.test.ts        ← tests
```

These three files always live together in the same directory. In the URL shortener project, they live under `src/url/shortcode/`.

---

## Context Manifests

Every ARU **must** have a co-located `.manifest.yaml` file. This is not optional. `aria-build check` will report a violation for any implementation file that lacks a manifest.

Here is a complete, annotated manifest for `url.shortcode.validate.format`:

```yaml
manifest:
  id: "url.shortcode.validate.format"        # Semantic address — unique across codebase
  version: "1.0.0"                           # Semver
  layer: L1                                  # Must match verb vocabulary

  identity:
    purpose: "validates that a short code matches allowed character format"
    domain: "url"
    subdomain: "shortcode"
    verb: "validate"
    entity: "format"

  contract:
    input:
      type: "string"
      constraints:
        - "non-empty"
        - "1–10 characters"
        - "alphanumeric only"
    output:
      success: "ValidatedShortCode"
      failure: "FormatError { code: TOO_LONG | INVALID_CHARS | EMPTY }"
    side_effects: NONE          # ← CRITICAL: must be set. Options: NONE | READ | WRITE | EVENT
    idempotent: true
    deterministic: true

  dependencies: []              # L1 atoms have no deps beyond L0 types

  context_budget:               # How many tokens an AI needs to work with this ARU
    to_use: 80                  # Just calling it as a dependency
    to_modify: 200              # Changing existing behavior
    to_extend: 350              # Adding new output cases
    to_replace: 150             # Full rewrite

  test_contract:
    - scenario: "valid 6-char alphanumeric returns ValidatedShortCode"
    - scenario: "11-char string returns FormatError TOO_LONG"
    - scenario: "string with spaces returns FormatError INVALID_CHARS"
    - scenario: "empty string returns FormatError EMPTY"

  stability: EXPERIMENTAL       # EXPERIMENTAL | STABLE | FROZEN

  connections:
    - pattern: VALIDATE         # One of 14 named patterns
      target: "url.link.create.fromOriginal"
```

### Critical Fields Explained

#### `side_effects`

This field **must** be declared on every ARU. It tells callers — and AI agents — what kind of external interactions this ARU has:

| Value | Meaning |
|-------|---------|
| `NONE` | The ARU is pure: same input always produces same output, no external state touched |
| `READ` | The ARU reads from a database, file system, or external service |
| `WRITE` | The ARU writes to a database, file system, cache, or any persistent store |
| `EVENT` | The ARU emits events or calls external APIs |

Why does this matter? Because it determines whether an operation is safe to **retry** (NONE and READ are always safe), **parallelize** (NONE is always safe, others require care), or **rollback** (WRITE and EVENT require compensation logic). When an AI agent is composing a pipeline or diagnosing a failure, `side_effects` is one of the first fields it reads.

Omitting `side_effects` is a defect. `aria-build check` will flag it at compliance level 1.

#### `context_budget`

The `context_budget` field is an estimate of how many **tokens** (LLM context window units) are needed to work with this ARU at different levels of engagement:

| Key | Meaning |
|-----|---------|
| `to_use` | Read the manifest contract to call this as a dependency. Just the input/output types. |
| `to_modify` | Load the implementation to change existing behavior. |
| `to_extend` | Load the implementation plus all callers to add new output cases or expand the contract. |
| `to_replace` | Full rewrite from scratch — implementation, tests, callers. |

These values are used by the `aria-build bundle` semantic graph to construct AI agent context windows that are *just big enough* for the task at hand. A small `to_use` budget (like 80 tokens for `url.shortcode.validate.format`) means an L2 molecule that depends on it can include it in context at almost no cost. A large `to_modify` budget signals to an AI agent that it needs to load substantial context before making changes.

#### Typed Failure Outputs

Note the `failure:` field:

```yaml
failure: "FormatError { code: TOO_LONG | INVALID_CHARS | EMPTY }"
```

This is not just documentation. The union type `TOO_LONG | INVALID_CHARS | EMPTY` is a typed contract that callers must handle. When an L2 molecule composes `url.shortcode.validate.format` with another ARU, it must declare how it routes each failure case — typically using a `ROUTE` or `GATE` pattern.

The full ARIA type system, including how failure types propagate through composition chains, is described in `docs/08-type-system.md` and `docs/12-error-propagation.md`.

---

## Composition Patterns

Every connection between two ARUs **must** be declared using one of 14 named patterns in the `connections:` block of the source ARU's manifest. An undeclared connection is a defect — it makes the semantic graph incomplete and breaks AI context loading.

The 14 patterns cover every meaningful way two units of logic can relate to each other:

| # | Pattern | Shape | Use When |
|---|---------|-------|----------|
| 1 | PIPE | `A → B` | Output of A feeds directly into B |
| 2 | FORK | `A → [B, C]` | Same value goes to multiple ARUs independently |
| 3 | JOIN | `[A, B] → C` | Multiple outputs merged into one typed struct |
| 4 | GATE | `A → B \| ∅` | Conditional — pass or discard |
| 5 | ROUTE | `A → B \| C` | Conditional branch — all paths must be handled |
| 6 | LOOP | `A →[cond]→ A` | Bounded iteration — must declare termination + max |
| 7 | OBSERVE | `A → (A, Event)` | Side-channel event without mutating main flow |
| 8 | TRANSFORM | `A → A'` | Shape change, same semantic domain |
| 9 | VALIDATE | `A → A' \| Error` | Contract enforcement with typed error |
| 10 | CACHE | `A → A` (memo) | Memoize expensive pure computation |
| 11 | STREAM | `A → B*` | Process lazy/infinite sequence element by element |
| 12 | SAGA | `[A→B→C] + compensations` | Distributed transaction with typed rollback |
| 13 | CIRCUIT_BREAKER | `A → B (stateful)` | Stateful failure detection — opens at failure threshold |
| 14 | PARALLEL_JOIN | `[A,B,C] → D (timeout)` | Fan-out with coordinated collection and timeout budget |

### Diagrams for the 5 Most Common Patterns

The following five patterns appear most frequently in real ARIA codebases. Three of them are demonstrated directly in this tutorial (marked ✅). The other two are reference material for more advanced use cases.

---

#### PIPE — ✅ Used in Chapter 3 (Starter Project)

The simplest pattern. The output of one ARU becomes the input of another.

```
url.shortcode.generate.hash
        │
        ▼  (PIPE)
url.link.create.fromOriginal
```

`generate.hash` produces a `ShortCode`. That `ShortCode` is piped directly into `create.fromOriginal`, which uses it (along with the original URL) to assemble a `ShortenedLink`.

---

#### VALIDATE — ✅ Used in Chapter 3 (Starter Project)

A specialization of PIPE where the output can be either a success type or a typed error. Callers must handle both branches.

```
input string
    │
    ▼  (VALIDATE)
url.shortcode.validate.format
    │                    │
    ▼ success            ▼ failure
ValidatedShortCode    FormatError
```

The two output branches must both be handled by the caller — either propagating the error or routing to a fallback. VALIDATE is how ARIA encodes the "parse, don't validate" pattern at the architecture level.

---

#### FORK — ✅ Used in Chapter 4 (Advanced Project)

The same value fans out to multiple ARUs that process it independently. All branches receive the same input. None depend on each other's output.

```
url.pipeline.orchestrate.shorten
        │
        ▼  (FORK)
   ┌────┴────┐
   ▼         ▼
response  url.analytics.emit.clickEvent
```

After the shorten pipeline completes, the result is forked: one branch returns the response to the caller, the other sends an analytics event. The analytics branch is fire-and-forget — a `side_effects: EVENT` ARU that doesn't block the response.

---

#### CIRCUIT_BREAKER — ✅ Used in Chapter 4 (Advanced Project)

A stateful pattern that wraps an ARU and tracks its failure rate. When failures exceed a threshold, the circuit "opens" and subsequent calls fast-fail without hitting the underlying ARU.

```
request
   │
   ▼  (CIRCUIT_BREAKER)
url.store.persist.link
   │ [CLOSED: normal]  [OPEN: fast-fail]
   ▼
persisted
```

Used in Chapter 4 to protect the persistence layer from cascading failures when the database is under stress. The circuit breaker itself is declared as a wrapper in the manifest — not a new ARU, but a pattern applied to an existing one.

---

#### SAGA — ✅ Used in Chapter 5 (AI Collaboration Project)

For multi-step operations where partial failure must be compensated. Each step declares a compensation action that undoes its effect if a later step fails.

```
url.import.execute.batch
   ├─ step 1: validate URLs    [compensate: discard]
   ├─ step 2: persist.link     [compensate: delete]
   └─ step 3: emit.importEvent [compensate: emit.rollbackEvent]
```

If step 3 fails, the SAGA compensates by running step 2's compensation (deleting the persisted link) and step 1's compensation (discarding the validated URLs). The full compensation chain is declared in the manifest, so AI agents can reason about rollback strategies without reading the implementation.

---

### Patterns Not Demonstrated in This Tutorial

The following patterns — JOIN, LOOP, OBSERVE, TRANSFORM, CACHE, STREAM, PARALLEL_JOIN — are part of the ARIA specification and follow the same rules as the patterns above. They are not used in the URL shortener project, but their full specifications are available in `docs/05-composition-patterns.md`. Each pattern has a generated TypeScript wrapper available via `aria-build generate`.

---

## ARU Lifecycle

Every ARU has a `stability` field that tracks its maturity:

### `EXPERIMENTAL`

The ARU is newly created. Its contract may change without notice. Callers should be aware that they may need to adapt. **Start all new ARUs here.** This is not a warning — it's the correct starting state.

### `STABLE`

The ARU's contract has been validated in production. Changes require a semver version bump and a deprecation notice on any removed or changed contract fields. Promote an ARU from EXPERIMENTAL to STABLE when:

- It has been in use for at least one release cycle
- All `test_contract` scenarios pass
- The contract has not changed during that period

### `FROZEN`

The ARU's contract will never change. It may only be **replaced**, never modified. This state exists for foundational ARUs that are used by a large number of callers — changing them would require coordinating migrations across many dependents. Promote from STABLE to FROZEN when:

- The ARU is a foundational type or utility
- More than 10 other ARUs depend on it
- The contract is genuinely permanent

> **⚠️ Warning: Do not freeze prematurely.** Once an ARU is marked `FROZEN`, *any* behavioral change — even a bug fix — requires creating a new ARU with a new semantic address and migrating all callers to it. This is intentional: it makes the cost of changing a foundational contract explicit. Only freeze ARUs whose contracts are truly permanent.

### Stability Progression Example

```
v1.0.0 → stability: EXPERIMENTAL   (just created)
v1.3.0 → stability: STABLE         (battle-tested, contract solid)
v2.0.0 → stability: FROZEN         (foundational type, 15 dependents)
```

If `v2.0.0` were `url.shortcode.validate.format`, and a new business requirement demanded that short codes allow underscores, the correct approach is **not** to modify the frozen ARU. Instead, create `url.shortcode.validate.extended-format` (EXPERIMENTAL), update callers incrementally, and eventually deprecate the frozen one.

---

**[← Introduction](00-introduction.md)** | **[Back to index](00-introduction.md)** | **[Next: Setup →](02-setup.md)**
