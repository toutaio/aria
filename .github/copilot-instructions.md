<!-- ARIA Skills version: 1.0 -->
# ARIA Framework — GitHub Copilot Instructions

This repository uses **ARIA (Atomic Responsibility Interface Architecture)**. Apply these rules to all code completions, suggestions, and reviews.

> Full spec: `docs/` · Schema: `aria/schema/aria-manifest.schema.json` · Primer: `skills/aria-primer.md`

---

## Layers — assign one to every ARU

| Layer | Name      | Responsibility                       | Deps   |
|-------|-----------|--------------------------------------|--------|
| L0    | Primitive | Types, constants, pure functions     | None   |
| L1    | Atom      | Single-responsibility operations     | L0     |
| L2    | Molecule  | Composes atoms, one sub-purpose      | L0–L1  |
| L3    | Organism  | Business rules and decisions         | L0–L2  |
| L4    | System    | Orchestration only                   | L0–L3  |
| L5    | Domain    | Bounded context / integration surface| L0–L4  |

Rules: L4 = no business logic. L3 = no infrastructure calls without L1/L2 adapters. L0 = no side effects.

---

## Naming — semantic address

```
[domain].[subdomain].[verb].[entity]
```

Verb is **layer-locked**:
- L1: `validate` `transform` `compute` `generate` `encode` `decode` `hash` `compare`
- L2: `create` `build` `prepare` `assemble` `resolve`
- L3: `execute` `apply` `process` `enforce` `emit` `authorize` `persist`
- L4: `orchestrate` `coordinate` `pipeline` `route`
- L5: `expose` `integrate` `guard` `translate`

Examples:
```
auth.token.validate.signature        → L1
auth.session.create.fromToken        → L2
user.registration.execute.workflow   → L3
billing.payment.orchestrate.pipeline → L4
```

File names follow the address: `auth.token.validate.signature.manifest.yaml` / `.ts` / `.test.ts`

---

## Manifest — required for every ARU

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

---

## Composition Patterns — declare every connection

| # | Pattern         | Shape                     | When to use                                     |
|---|-----------------|---------------------------|-------------------------------------------------|
| 1 | PIPE            | `A → B`                   | Output of A feeds B                             |
| 2 | FORK            | `A → [B, C]`              | Same value to multiple ARUs                     |
| 3 | JOIN            | `[A, B] → C`              | Merge multiple outputs into one typed struct    |
| 4 | GATE            | `A → B \| ∅`              | Conditional pass-or-discard                     |
| 5 | ROUTE           | `A → B \| C`              | All branches declared, exactly one fires        |
| 6 | LOOP            | `A →[cond]→ A`            | Bounded iteration with max count declared       |
| 7 | OBSERVE         | `A → (A, Event)`          | Side-channel event, main flow unchanged         |
| 8 | TRANSFORM       | `A → A'`                  | Same domain, different shape                    |
| 9 | VALIDATE        | `A → A' \| Error`         | Typed contract enforcement                      |
|10 | CACHE           | `A → A` (memo)            | Transparent memoization                         |
|11 | STREAM          | `A → B*`                  | Element-by-element sequence processing          |
|12 | SAGA            | `[A→B→C] + compensations` | Distributed tx with typed rollback              |
|13 | CIRCUIT_BREAKER | `A → B (stateful)`        | Failure-threshold circuit opening               |
|14 | PARALLEL_JOIN   | `[A,B,C] → D (timeout)`   | Concurrent fan-out with timeout budget          |

---

## CLI

```bash
aria-build check ./src                             # validate manifests
aria-build check ./src --compliance-level 3        # enforce layer rules
aria-build impact auth.identity.authenticate.user  # change impact
aria-build bundle ./src                            # build semantic graph
aria-build generate ./src                          # generate TS wrappers
```

---

## Rules

- **Always** create a manifest alongside every new implementation file
- **Always** use `domain.subdomain.verb.entity` naming
- **Always** declare connections in `connections:` block with a named pattern
- **Never** skip layers upward (L1 cannot depend on L3+)
- **Never** put business logic in L4 Systems
- **Never** use generic names: `utils`, `helper`, `manager`, `service` without semantic address
- **Never** leave `side_effects` unset when the ARU writes to DB, emits events, or calls external APIs
- Run `aria-build check ./src` before committing
