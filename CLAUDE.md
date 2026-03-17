<!-- ARIA Skills version: 1.0 -->
# ARIA Framework — Claude Project Instructions

You are working in an **ARIA (Atomic Responsibility Interface Architecture)** repository. Every piece of code is an ARU described by a manifest. Follow these rules for all code generation, review, and analysis tasks.

> Full specification: `docs/` (24 documents) · Schema: `aria/schema/aria-manifest.schema.json` · Canonical primer: `skills/aria-primer.md`

---

## 1. Abstraction Layers — WHERE code lives

Assign every new ARU to exactly one layer. A layer may only depend on layers below it — no upward dependencies, no cycles.

| Layer | Name      | Responsibility                          | Allowed Deps |
|-------|-----------|-----------------------------------------|--------------|
| L0    | Primitive | Types, constants, pure functions        | None         |
| L1    | Atom      | Single-responsibility operations        | L0           |
| L2    | Molecule  | Composes atoms for one sub-purpose      | L0–L1        |
| L3    | Organism  | Business rules and domain decisions     | L0–L2        |
| L4    | System    | Orchestration only — wires organisms    | L0–L3        |
| L5    | Domain    | Bounded context boundaries              | L0–L4        |

- **L0**: No side effects. Ever.
- **L3**: Where business logic lives. No infrastructure calls without L1/L2 adapters.
- **L4**: No business logic. Only routing, sequencing, error propagation.
- Same-layer coupling only through explicitly declared interfaces.

---

## 2. Atomic Responsibility Units — WHAT each piece is

Every ARU has: one reason to exist · one typed input · one typed output · one layer · one manifest.

**One Question Test**: "Given [input], it [single verb] and returns [output]" — no "and" in the verb phrase.

### Verb vocabulary (layer-locked — use the right verb for the right layer)

| Layer | Canonical Verbs |
|-------|-----------------|
| L0    | (no verb — `domain.entity` only) |
| L1    | validate, transform, compute, generate, encode, decode, hash, compare |
| L2    | create, build, prepare, assemble, resolve |
| L3    | execute, apply, process, enforce, emit, authorize, persist |
| L4    | orchestrate, coordinate, pipeline, route |
| L5    | expose, integrate, guard, translate |

---

## 3. Naming Conventions — Semantic Address

```
[domain].[subdomain].[verb].[entity]
```

All four segments required for L1+. L0 uses `[domain].[entity]`. All segments in **kebab-case**.

| Segment    | Examples                            |
|------------|-------------------------------------|
| `domain`   | `auth`, `user`, `billing`           |
| `subdomain`| `token`, `session`, `password`      |
| `verb`     | `validate`, `create`, `execute`     |
| `entity`   | `token`, `email`, `invoice`         |

**Examples:**
```
auth.token.validate.signature        → L1 Atom
auth.session.create.fromToken        → L2 Molecule
user.registration.execute.workflow   → L3 Organism
billing.payment.orchestrate.pipeline → L4 System
notification.domain.expose.api       → L5 Domain
```

**File naming:**
- Manifest: `auth.token.validate.signature.manifest.yaml`
- Implementation: `auth.token.validate.signature.ts`
- Tests: `auth.token.validate.signature.test.ts`

---

## 4. Context Manifests — every ARU needs one

Co-locate a `.manifest.yaml` with every ARU. Use this structure:

```yaml
manifest:
  id: "auth.token.validate.signature"
  version: "1.0.0"
  layer: L1

  identity:
    purpose: "validates JWT token signature and expiry"
    domain: "auth"
    subdomain: "token"
    verb: "validate"
    entity: "signature"

  contract:
    input:
      type: "TokenString"
      constraints:
        - "non-empty string"
        - "matches JWT format"
    output:
      success: "ValidatedToken"
      failure: "AuthError { code: EXPIRED | INVALID_SIGNATURE | MALFORMED }"
    side_effects: NONE
    idempotent: true
    deterministic: true

  dependencies:
    - id: "crypto.jwt.decode"
      layer: L1

  context_budget:
    to_use: 120
    to_modify: 340
    to_extend: 580
    to_replace: 200

  test_contract:
    - scenario: "valid unexpired token returns ValidatedToken"
    - scenario: "expired token returns AuthError EXPIRED"
    - scenario: "tampered signature returns AuthError INVALID_SIGNATURE"

  stability: EXPERIMENTAL   # EXPERIMENTAL | STABLE | FROZEN

  connections:
    - pattern: PIPE
      target: "auth.session.create.fromToken"
```

---

## 5. Composition Patterns — HOW ARUs connect

Every connection between ARUs **must** be declared as one of these 14 patterns. Undeclared connections are defects.

| # | Pattern         | Shape                     | Use when…                                              |
|---|-----------------|---------------------------|--------------------------------------------------------|
| 1 | PIPE            | `A → B`                   | Output of A feeds directly into B                      |
| 2 | FORK            | `A → [B, C]`              | Same value goes to multiple ARUs independently         |
| 3 | JOIN            | `[A, B] → C`              | Multiple outputs merged into one typed struct          |
| 4 | GATE            | `A → B \| ∅`              | Conditional — pass or discard                          |
| 5 | ROUTE           | `A → B \| C`              | Conditional branch — all paths must be handled         |
| 6 | LOOP            | `A →[cond]→ A`            | Bounded iteration — must declare termination + max     |
| 7 | OBSERVE         | `A → (A, Event)`          | Side-channel event without mutating main flow          |
| 8 | TRANSFORM       | `A → A'`                  | Shape change, same semantic domain                     |
| 9 | VALIDATE        | `A → A' \| Error`         | Contract enforcement with typed error                  |
|10 | CACHE           | `A → A` (memo)            | Memoize expensive pure computation                     |
|11 | STREAM          | `A → B*`                  | Process lazy/infinite sequence element by element      |
|12 | SAGA            | `[A→B→C] + compensations` | Distributed transaction with typed rollback steps      |
|13 | CIRCUIT_BREAKER | `A → B (stateful)`        | Stateful failure detection — opens at failure threshold|
|14 | PARALLEL_JOIN   | `[A,B,C] → D (timeout)`   | Fan-out with coordinated collection and timeout budget |

---

## 6. Semantic Graph — the MAP

`aria-build bundle ./src` assembles all manifests into a semantic graph. Every node is an ARU; every edge is a typed pattern instance. The graph enables impact analysis and AI context loading.

---

## 7. CLI Commands

```bash
aria-build check ./src                        # validate manifests + compliance
aria-build check ./src --compliance-level 2   # levels 0–5
aria-build check ./src --format json          # CI-friendly JSON output
aria-build impact auth.identity.authenticate.user  # impact analysis for a change
aria-build bundle ./src                       # build semantic graph snapshot
aria-build generate ./src                     # generate TS wrappers (all 14 patterns)
```

| Level | Checks |
|-------|--------|
| 1 | Manifest presence |
| 2 | + Naming conventions |
| 3 | + Layer dependency rules |
| 4 | + Bundle freshness |
| 5 | All checks (default) |

---

## 8. Do ✅ / Don't ❌

| ✅ Do | ❌ Don't |
|-------|---------|
| Assign every new ARU a layer (L0–L5) | Create code without a manifest |
| Name ARUs as `domain.subdomain.verb.entity` | Use names like `utils`, `helpers`, `manager` |
| Declare every inter-ARU connection as a named pattern | Leave connections implicit |
| Keep L3 free of infrastructure concerns | Call DB/HTTP directly from L3 |
| Keep L4 free of business logic | Add `if (user.isPremium)` checks in L4 |
| Use the correct verb vocabulary for each layer | Use `execute` at L1 or `validate` at L3 |
| Declare all side effects in the manifest | Emit events without declaring them |
| Run `aria-build check` before committing | Commit without validation |
| Start new ARUs at `stability: EXPERIMENTAL` | Ship `stability: FROZEN` on first version |

---

## 9. Quick Reference

```
Layer?    → L0 type / L1 operation / L2 composition / L3 logic / L4 orchestration / L5 boundary
Name?     → domain.subdomain.VERB.entity  (verb must match layer's vocabulary)
Manifest? → <address>.manifest.yaml co-located with implementation
Connect?  → choose one of 14 patterns; declare in manifest `connections:` block
Check?    → aria-build check ./src
Impact?   → aria-build impact <address>
```
