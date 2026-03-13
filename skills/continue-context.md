<!-- ARIA Skills version: 1.0 -->
# ARIA Framework — Continue.dev System Prompt

## How to use this file

Paste the content below the horizontal rule into the `systemMessage` field of your Continue.dev `config.json`:

```json
{
  "models": [...],
  "systemMessage": "<paste content here>"
}
```

Your `config.json` is typically located at `~/.continue/config.json`.

The content below covers the complete ARIA framework primer so Continue.dev provides ARIA-compliant completions without additional prompting.

---

You are working in an **ARIA (Atomic Responsibility Interface Architecture)** repository. ARIA is a framework for AI-optimised software construction. Every piece of code is an ARU (Atomic Responsibility Unit) described by a manifest file. ARUs connect through 14 named composition patterns forming a Semantic Graph.

**Full docs**: `docs/` directory (24 specification documents)
**Schema**: `aria/schema/aria-manifest.schema.json`

## Layers — WHERE code lives

Assign every ARU to exactly one layer. Higher layers may only depend on lower ones.

| Layer | Name      | Responsibility                       | Allowed Deps |
|-------|-----------|--------------------------------------|--------------|
| L0    | Primitive | Types, constants, pure functions     | None         |
| L1    | Atom      | Single-responsibility operations     | L0           |
| L2    | Molecule  | Composes atoms, one sub-purpose      | L0–L1        |
| L3    | Organism  | Business rules and decisions         | L0–L2        |
| L4    | System    | Orchestration only                   | L0–L3        |
| L5    | Domain    | Bounded context / integration surface| L0–L4        |

Rules: L0 = no side effects; L3 = no direct infrastructure calls; L4 = no business logic.

## Naming — Semantic Address

```
[domain].[subdomain].[verb].[entity]   (all kebab-case)
```

Verb vocabulary is **layer-locked**:
- L1: `validate` `transform` `compute` `generate` `encode` `decode` `hash` `compare`
- L2: `create` `build` `prepare` `assemble` `resolve`
- L3: `execute` `apply` `process` `enforce` `emit` `authorize` `persist`
- L4: `orchestrate` `coordinate` `pipeline` `route`
- L5: `expose` `integrate` `guard` `translate`

Examples:
- `auth.token.validate.signature` → L1 Atom
- `auth.session.create.fromToken` → L2 Molecule
- `user.registration.execute.workflow` → L3 Organism
- `billing.payment.orchestrate.pipeline` → L4 System

File names mirror the address: `auth.token.validate.signature.manifest.yaml` / `.ts` / `.test.ts`

## Manifests — required for every ARU

Co-locate `<address>.manifest.yaml` with every implementation:

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

## Composition Patterns — declare every connection

Every inter-ARU connection must use one of these 14 patterns declared in `connections:`:

| # | Pattern         | Use when                                              |
|---|-----------------|-------------------------------------------------------|
| 1 | PIPE            | Output of A feeds directly into B                     |
| 2 | FORK            | Same value goes to multiple ARUs independently        |
| 3 | JOIN            | Multiple outputs merged into one typed struct         |
| 4 | GATE            | Conditional pass-or-discard                           |
| 5 | ROUTE           | Conditional branch — all paths declared               |
| 6 | LOOP            | Bounded iteration — must declare termination + max    |
| 7 | OBSERVE         | Side-channel event without mutating main flow         |
| 8 | TRANSFORM       | Shape change within same semantic domain              |
| 9 | VALIDATE        | Typed contract enforcement with error                 |
|10 | CACHE           | Transparent memoization                               |
|11 | STREAM          | Element-by-element sequence processing                |
|12 | SAGA            | Distributed tx with typed compensating actions        |
|13 | CIRCUIT_BREAKER | Stateful failure detection with threshold             |
|14 | PARALLEL_JOIN   | Concurrent fan-out with timeout budget                |

## CLI Commands

```bash
aria-build check ./src                             # validate manifests
aria-build check ./src --compliance-level 3        # enforce layer rules
aria-build impact auth.identity.authenticate.user  # impact analysis
aria-build bundle ./src                            # build semantic graph
aria-build generate ./src                          # generate TypeScript wrappers
```

## Rules

- Every implementation file needs a co-located `.manifest.yaml`
- Always use `domain.subdomain.verb.entity` naming (no `utils`, `helpers`, `manager`)
- Declare all inter-ARU connections with a named pattern in `connections:`
- No business logic in L4 Systems
- No infrastructure calls in L3 Organisms (use L1/L2 adapters)
- Run `aria-build check ./src` before committing
