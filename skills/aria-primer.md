<!-- ARIA Skills version: 1.0 -->
# ARIA Framework — AI Tool Primer

> This file is the **canonical source of truth** for all AI tool skill files in this repository.
> Tool-specific files (CLAUDE.md, .cursorrules, etc.) embed this content directly.
> Edit here first, then regenerate tool files.
>
> Reference docs: `docs/` directory (24 specification documents)
> Schema: `aria/schema/aria-manifest.schema.json`

---

## What Is ARIA?

**ARIA (Atomic Responsibility Interface Architecture)** is a framework for building software systems optimised for AI-assisted development. Its core thesis:

> If every component has an unambiguous contract, a precise responsibility, a known layer, a typed interface, and a predictable connection model — an AI can work with maximum precision using minimum context.

Every piece of code is an **ARU (Atomic Responsibility Unit)** described by a **manifest file**. ARUs connect through one of **14 named composition patterns**. The whole system forms a **Semantic Graph** navigable without reading implementation code.

---

## Pillar 1 — Abstraction Layers (WHERE)

A layer may only depend on layers **below** it. No skipping upward, no cycles.

| Layer | Name      | Responsibility                          | Allowed Dependencies |
|-------|-----------|-----------------------------------------|----------------------|
| L0    | Primitive | Types, constants, pure functions        | None                 |
| L1    | Atom      | Single-responsibility operations        | L0                   |
| L2    | Molecule  | Composes atoms for one sub-purpose      | L0–L1                |
| L3    | Organism  | Business rules and domain decisions     | L0–L2                |
| L4    | System    | Orchestration — wires organisms         | L0–L3                |
| L5    | Domain    | Bounded context boundaries              | L0–L4                |

**Key rules:**
- L0: No side effects. Ever. Pure vocabulary.
- L1: Stateless preferred. One thing, one reason.
- L2: Has exactly one named purpose derivable from its inputs/outputs.
- L3: Where "why" lives — policies, workflows, business logic.
- L4: No business logic allowed here — only routing and sequencing.
- L5: Anti-corruption layers, external integration surfaces.
- Same-layer coupling is only allowed through explicitly declared interfaces.

---

## Pillar 2 — Atomic Responsibility Units (WHAT)

An **ARU** has: one reason to exist · one typed input · one typed output · one layer · one manifest.

**One Question Test** — a valid ARU can be described as:
> "Given [input], it [single verb] and returns [output]" — with no "and" in the verb phrase.

### ARU Type by Layer

| Layer | Type      | Canonical Verbs                                        |
|-------|-----------|--------------------------------------------------------|
| L0    | Primitive | (no verb — just `domain.entity`)                       |
| L1    | Atom      | validate, transform, compute, generate, encode, decode, hash, compare |
| L2    | Molecule  | create, build, prepare, assemble, resolve              |
| L3    | Organism  | execute, apply, process, enforce, emit, authorize, persist |
| L4    | System    | orchestrate, coordinate, pipeline, route               |
| L5    | Domain    | expose, integrate, guard, translate                    |

---

## Pillar 3 — Composition Patterns (HOW)

Every connection between ARUs **must** be declared as one of these 14 patterns. Undeclared connections are architectural defects.

| # | Pattern         | Shape                     | Description                                                        |
|---|-----------------|---------------------------|--------------------------------------------------------------------|
| 1 | PIPE            | `A → B`                   | Linear transformation; output of A is input of B                  |
| 2 | FORK            | `A → [B, C]`              | Fan-out; same value passed to multiple ARUs independently          |
| 3 | JOIN            | `[A, B] → C`              | Fan-in; multiple outputs merged into one typed input               |
| 4 | GATE            | `A → B \| ∅`              | Conditional pass; flows to B only if predicate is true             |
| 5 | ROUTE           | `A → B \| C`              | Conditional branch; exactly one branch fires, all paths handled    |
| 6 | LOOP            | `A →[cond]→ A`            | Bounded iteration; must declare termination condition + max bound  |
| 7 | OBSERVE         | `A → (A, Event)`          | Non-mutating side channel; main flow unchanged, event emitted      |
| 8 | TRANSFORM       | `A → A'`                  | Shape change within same semantic domain; no new information       |
| 9 | VALIDATE        | `A → A' \| Error`         | Contract enforcement; typed output or typed error                  |
|10 | CACHE           | `A → A` (memo)            | Transparent memoization; identical inputs return stored output     |
|11 | STREAM          | `A → B*`                  | Lazy/infinite sequence; B processes each element as it arrives     |
|12 | SAGA            | `[A→B→C] + [C⁻¹→B⁻¹→A⁻¹]`| Distributed transaction with typed compensating actions            |
|13 | CIRCUIT_BREAKER | `A → B (stateful)`        | Stateful failure detection; opens circuit at failure threshold     |
|14 | PARALLEL_JOIN   | `[A,B,C] → D (timeout)`   | Fan-out with coordinated collection within a time budget           |

---

## Pillar 4 — Context Manifests (COST)

Every ARU has a `.manifest.yaml` co-located with its implementation. Manifests make contracts machine-readable and enable AI to work with minimum context.

### Manifest File Naming
```
<domain>.<subdomain>.<verb>.<entity>.manifest.yaml
```
Example: `auth.token.validate.signature.manifest.yaml`

### Key Manifest Fields
```yaml
manifest:
  id: "auth.token.validate.signature"   # semantic address
  version: "1.0.0"                      # semver
  layer: L1                             # L0–L5

  identity:
    purpose: "one sentence describing exactly what this ARU does"
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
    side_effects: NONE          # or READ | WRITE | EXTERNAL
    idempotent: true
    deterministic: true

  dependencies:
    - id: "crypto.jwt.decode"
      layer: L1

  context_budget:
    to_use: 120       # tokens to call this ARU correctly
    to_modify: 340    # tokens to safely change its behavior
    to_extend: 580    # tokens to add new functionality nearby
    to_replace: 200   # tokens to rewrite an equivalent

  test_contract:
    - scenario: "valid unexpired token returns ValidatedToken"
    - scenario: "expired token returns AuthError EXPIRED"
    - scenario: "tampered signature returns AuthError INVALID_SIGNATURE"

  stability: STABLE   # EXPERIMENTAL | STABLE | FROZEN

  connections:        # declared composition pattern edges
    - pattern: PIPE
      target: "auth.session.create.fromToken"
```

### Context Budget Levels
- **to_use**: Read only the signature, types, errors — enough to call it
- **to_modify**: + implementation logic + dependency contracts + test scenarios
- **to_extend**: + related ARUs in same subdomain + composition patterns in use
- **to_replace**: contract + test scenarios only (no implementation)

---

## Pillar 5 — Semantic Graph (MAP)

The Semantic Graph is the full dependency map of all ARUs and their connections. It is built automatically from manifest files by `aria-build`.

- Every ARU is a **node** (identified by its semantic address)
- Every composition pattern instance is a **typed edge**
- The graph enables: impact analysis, context loading, compliance checking, visualization

---

## Naming Conventions

### Semantic Address Format
```
[domain].[subdomain].[verb].[entity]
```
All four segments required for L1+. L0 uses `[domain].[entity]` only.

| Segment    | Purpose                        | Examples                            |
|------------|--------------------------------|-------------------------------------|
| `domain`   | Bounded context                | `auth`, `user`, `billing`           |
| `subdomain`| Functional area within domain  | `token`, `session`, `password`      |
| `verb`     | Operation class (layer-locked) | `validate`, `create`, `execute`     |
| `entity`   | The data subject               | `token`, `email`, `invoice`         |

### Example Addresses by Layer
```
auth.token.validate.signature        → L1 Atom
auth.session.create.fromToken        → L2 Molecule
user.registration.execute.workflow   → L3 Organism
billing.payment.orchestrate.pipeline → L4 System
notification.domain.expose.api       → L5 Domain
```

### File Naming Rules
- Manifest: `<address>.manifest.yaml`
- Implementation (TS): `<address>.ts`
- Tests: `<address>.test.ts`
- `domain`, `subdomain`, `verb` in **kebab-case**; `entity` in **camelCase** for multi-word qualifiers (`fromToken`, `shortCode`)

---

## CLI Commands

```bash
# Validate manifests and check ARIA compliance
aria-build check ./src
aria-build check ./src --compliance-level 2   # levels 1–5
aria-build check ./src --format json          # CI output

# Analyse impact of changing an ARU
aria-build impact auth.identity.authenticate.user

# Bundle all manifests into a semantic graph snapshot
aria-build bundle ./src

# Generate TypeScript wrappers from manifests (all 14 patterns)
aria-build generate ./src
```

### Compliance Levels
| Level | What Is Checked |
|-------|-----------------|
| 0 | JSON Schema validation only |
| 1 | + Manifest presence (every ARU has a manifest) |
| 2 | + Naming convention compliance |
| 3 | + Layer dependency rule enforcement |
| 4 | + Bundle freshness |
| 5 | All checks (default) |

---

## Do ✅ / Don't ❌

| ✅ Do | ❌ Don't |
|-------|---------|
| Assign every new file an ARIA layer (L0–L5) | Create code without a manifest |
| Name ARUs using `domain.subdomain.verb.entity` | Use generic names like `utils`, `helpers`, `manager` |
| Declare every inter-ARU connection as a named pattern | Leave connections implicit or undocumented |
| Keep L3 Organisms free of infrastructure concerns | Put database calls in L3 without an L1/L2 adapter |
| Keep L4 Systems free of business logic | Add conditional business rules to an L4 orchestrator |
| Use the verb vocabulary for each layer | Use `execute` at L1 or `validate` at L3 |
| Set `side_effects: NONE` for pure computations | Forget to declare side effects (they become invisible) |
| Write one scenario per `test_contract` entry | Leave `test_contract` empty |
| Run `aria-build check` before committing | Commit manifests without validation |
| Declare `stability: EXPERIMENTAL` for new ARUs | Ship FROZEN stability on first version |

---

## Quick Reference Card

```
Layer?    → Pick L0–L5 based on what the code IS (type/operation/logic/orchestration/boundary)
Name?     → domain.subdomain.VERB.entity  (verb must match layer's verb vocabulary)
File?     → same-name.manifest.yaml co-located with implementation
Connect?  → choose one of 14 patterns; declare in manifest `connections:` block
Validate? → aria-build check ./src
Impact?   → aria-build impact <address>
```

---

*Canonical docs: `docs/00-overview.md` through `docs/23-*.md`*
*Schema: `aria/schema/aria-manifest.schema.json`*
