---
description: ARIA Framework rules for all AI completions in this repository
globs: ["**/*"]
alwaysApply: true
---

# ARIA Framework — Cursor IDE Rules

This repository uses **ARIA (Atomic Responsibility Interface Architecture)**. Apply these rules to all completions and suggestions.

> Primer: `skills/aria-primer.md` · Docs: `docs/` · Schema: `aria/schema/aria-manifest.schema.json`

## Layer Model

Every ARU belongs to exactly one layer. Higher layers may only depend on lower ones.

| Layer | Name      | Responsibility                       | Allowed Deps |
|-------|-----------|--------------------------------------|--------------|
| L0    | Primitive | Types, constants, pure functions     | None         |
| L1    | Atom      | Single-responsibility operations     | L0           |
| L2    | Molecule  | Composes atoms, one sub-purpose      | L0–L1        |
| L3    | Organism  | Business rules and decisions         | L0–L2        |
| L4    | System    | Orchestration only                   | L0–L3        |
| L5    | Domain    | Bounded context / integration surface| L0–L4        |

- L0: No side effects. Ever.
- L3: No direct infrastructure calls — use L1/L2 adapters.
- L4: No business logic — only routing and sequencing.

## Naming Convention

```
[domain].[subdomain].[verb].[entity]    (all kebab-case)
```

Verb vocabulary is **layer-locked**:
- **L1**: `validate` `transform` `compute` `generate` `encode` `decode` `hash` `compare`
- **L2**: `create` `build` `prepare` `assemble` `resolve`
- **L3**: `execute` `apply` `process` `enforce` `emit` `authorize` `persist`
- **L4**: `orchestrate` `coordinate` `pipeline` `route`
- **L5**: `expose` `integrate` `guard` `translate`

Examples:
```
auth.token.validate.signature        → L1 Atom
auth.session.create.fromToken        → L2 Molecule
user.registration.execute.workflow   → L3 Organism
billing.payment.orchestrate.pipeline → L4 System
notification.domain.expose.api       → L5 Domain
```

File naming: `<address>.manifest.yaml`, `<address>.ts`, `<address>.test.ts`

## Manifest (required for every ARU)

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
      constraints: ["non-empty string", "matches JWT format"]
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
  stability: EXPERIMENTAL
  connections:
    - pattern: PIPE
      target: "auth.session.create.fromToken"
```

## Composition Patterns

All 22 patterns — every inter-ARU connection must use one:

| # | Pattern                  | Shape                     | Use when                                        |
|---|--------------------------|---------------------------|-------------------------------------------------|
| 1 | PIPE                     | `A → B`                   | Output of A feeds directly into B               |
| 2 | FORK                     | `A → [B, C]`              | Same value to multiple ARUs independently       |
| 3 | JOIN                     | `[A, B] → C`              | Multiple outputs merged into typed struct       |
| 4 | GATE                     | `A → B \| ∅`              | Conditional pass-or-discard                     |
| 5 | ROUTE                    | `A → B \| C`              | All branches declared, exactly one fires        |
| 6 | LOOP                     | `A →[cond]→ A`            | Bounded iteration, must declare max count       |
| 7 | OBSERVE                  | `A → (A, Event)`          | Side-channel event, main flow unchanged         |
| 8 | TRANSFORM                | `A → A'`                  | Same domain, different shape                    |
| 9 | VALIDATE                 | `A → A' \| Error`         | Typed contract enforcement                      |
|10 | CACHE                    | `A → A` (memo)            | Transparent memoization                         |
|11 | STREAM                   | `A → B*`                  | Element-by-element sequence processing          |
|12 | SAGA                     | `[A→B→C] + compensations` | Distributed tx with typed rollback              |
|13 | CIRCUIT_BREAKER          | `A → B (stateful)`        | Failure-threshold circuit opening               |
|14 | PARALLEL_JOIN            | `[A,B,C] → D (timeout)`   | Concurrent fan-out with timeout budget          |
|15 | PARALLEL_FORK            | `A → [B, C] (parallel)`   | Parallel independent branches, all fire         |
|16 | SCATTER_GATHER           | `A[] → B[]`               | Scatter input elements, gather results          |
|17 | COMPENSATING_TRANSACTION | `A → B \| rollback`       | Forward with explicit typed compensation        |
|18 | STREAMING_PIPELINE       | `A → B*` (chunked)        | Chunk-by-chunk streaming transformation         |
|19 | CACHE_ASIDE              | `A → B` (load-through)    | Cache miss triggers load and store              |
|20 | BULKHEAD                 | `A → B` (pool-isolated)   | Resource pool isolation with overflow handling  |
|21 | PRIORITY_QUEUE           | `A → B` (prioritised)     | Priority-ordered delivery to target ARU         |
|22 | EVENT_SOURCING           | `A → B*` (events)         | Append events, project aggregate state          |

## CLI Commands

```bash
aria-build check ./src                             # validate all manifests
aria-build check ./src --compliance-level 3        # enforce layer dependency rules
aria-build impact auth.identity.authenticate.user  # change impact analysis
aria-build bundle ./src                            # build semantic graph snapshot
aria-build generate ./src                          # generate TypeScript wrappers
```

## Hard Rules

- Every new implementation file needs a co-located `.manifest.yaml`
- Always use `domain.subdomain.verb.entity` naming
- Always declare connections in `connections:` with a named pattern
- Never create `utils/`, `helpers/`, `managers/` without semantic addresses
- Never allow upward layer dependencies (L1 cannot import L3+)
- Never add business logic to L4 Systems
- Run `aria-build check ./src` before every commit

## Setup for Cursor

Copy this file to `.cursor/rules/aria.mdc` in your project root to activate these rules in Cursor IDE.
You can also use the legacy `.cursorrules` file at the project root — copy the content without the YAML frontmatter.
