# ARIA — Atomic Responsibility Interface Architecture

> A framework for building software systems where every component is an unambiguous contract — designed for AI-assisted development at any scale.

---

## What is ARIA?

ARIA is a software architecture framework built around a central thesis:

> **If every component is an unambiguous contract with a precise responsibility, a known layer, a typed interface, and a predictable connection model — then an AI can work with maximum precision using minimum context.**

Traditional architecture was designed for human cognitive limits (modularity, readability). ARIA is designed for *AI cognitive limits*: finite context windows, sensitivity to ambiguity, and pattern-recognition strengths.

The result is a system where:
- Every unit of code has a **semantic address** (`domain.subdomain.verb.entity`)
- Every connection between units is **typed and declared** in a manifest file
- Every composition follows one of **14 named patterns** (PIPE, FORK, SAGA, CIRCUIT_BREAKER, ...)
- An AI agent working on any ARU needs to load *only that ARU's manifest* to understand its full contract

---

## The Five Pillars

| Pillar | Document | Description |
|---|---|---|
| 1. Layers | [doc 01](docs/01-abstraction-layers.md) | L0–L5 layer model. WHERE each piece lives. |
| 2. ARUs | [doc 02](docs/02-atomic-responsibility-units.md) | Atomic Responsibility Units. WHAT each piece is. |
| 3. Composition | [doc 03](docs/03-composition-patterns.md) | 14 composition patterns. HOW pieces connect. |
| 4. Context Manifests | [doc 04](docs/04-context-manifests.md) | `.manifest.yaml` files. HOW contracts are declared. |
| 5. Semantic Graph | [doc 05](docs/05-semantic-graph.md) | The dependency graph. HOW the system is validated. |

---

## Repository Layout

```
ai-architecture/
├── docs/                        # ARIA framework specification (24 documents)
│   ├── 00-overview.md           # Framework overview — start here
│   ├── 01–23-*.md               # Full specification docs
│   └── TASKS.md                 # Framework development backlog
│
├── aria/                        # Build toolchain implementation
│   ├── schema/                  # aria-manifest.schema.json (JSON Schema draft-07)
│   ├── crates/
│   │   ├── aria-core/           # Rust library: manifest types, checkers, Salsa DB
│   │   ├── aria-build/          # Rust CLI: aria-build check|impact|bundle|generate
│   │   └── aria-lsp/            # Rust LSP server: real-time IDE validation
│   ├── packages/
│   │   ├── aria-runtime/        # @aria/runtime — Result<T,E>, RailError, ThreeTrack
│   │   ├── aria-ts-plugin/      # TypeScript code generator (all 14 patterns)
│   │   ├── aria-vscode/         # VS Code extension wrapper
│   │   ├── aria-build-bin/      # @aria/build-bin — platform binary shim
│   │   └── aria-build-bin-*/    # Platform-specific binary packages
│   └── aria-build-prototype/    # Node.js prototype CLI (Phase 1)
│
└── openspec/                    # OpenSpec change management
```

---

## Quick Start

### 1. Install the CLI

```bash
npm install -g @aria/build-bin
# or
cargo install --path aria/crates/aria-build
```

### 2. Create your first manifest

```yaml
# src/auth/identity/auth.identity.authenticate.user.manifest.yaml

identity:
  address: auth.identity.authenticate.user
  layer: 1
  stability: stable

contract:
  input_schema:
    type: AuthRequest
  output_schema:
    type: AuthToken
  error_types:
    - AuthError.INVALID_CREDENTIALS
    - AuthError.ACCOUNT_LOCKED

composition:
  pattern: PIPE
  input_type: AuthRequest
  output_type: AuthToken
  error_types:
    - AuthError.INVALID_CREDENTIALS
    - AuthError.ACCOUNT_LOCKED

layer:
  declared: 1
  dependencies:
    - auth.token.generate.jwt
    - auth.credential.verify.password

budget:
  max_response_ms: 200
  max_memory_mb: 32

tests:
  unit_test_file: auth.identity.authenticate.user.test.ts
  coverage_threshold: 80
```

### 3. Validate all manifests

```bash
# Validate with full compliance (level 5 — all checks)
aria-build check ./src

# Validate with a specific compliance level
aria-build check ./src --compliance-level 2

# JSON output for CI
aria-build check ./src --format json
```

### 4. See the impact of a change

```bash
# Show all ARUs transitively depending on a given one
aria-build impact auth.identity.authenticate.user
```

### 5. Generate TypeScript wrappers

```bash
# Generate typed wrappers from manifests
aria-build generate ./src
```

---

## Compliance Levels

| Level | Checks Applied |
|---|---|
| `0` | JSON Schema validation only |
| `1` | + Naming conventions (verb vocabulary, address format, error type prefix) |
| `2` | + Semantic graph (cycle detection, cross-domain dependency rules, type compatibility) |
| `3` | + Composition codegen freshness (stale `*.generated.ts` detection via manifest hash) |
| `4` | + Bundle freshness (`aria-build bundle` output is up-to-date) |
| `5` | All checks (default) |

---

## AI Tool Skills

This repository ships skill files for popular AI coding assistants so they produce ARIA-compliant code automatically — no manual context priming needed.

| Tool | File | Auto-loaded? |
|------|------|--------------|
| Claude (Anthropic) | [`CLAUDE.md`](./CLAUDE.md) | ✅ Yes — Claude Code loads it automatically |
| GitHub Copilot | [`.github/copilot-instructions.md`](./.github/copilot-instructions.md) | ✅ Yes — Copilot loads it repo-wide |
| Cursor (legacy) | [`.cursorrules`](./.cursorrules) | ✅ Yes — loaded by older Cursor versions |
| Cursor (current) | [`.cursor/rules/aria.mdc`](./.cursor/rules/aria.mdc) | ✅ Yes — loaded by Cursor v0.43+ |
| Windsurf | [`.windsurfrules`](./.windsurfrules) | ✅ Yes — loaded automatically |
| Cline (VS Code) | [`.clinerules`](./.clinerules) | ✅ Yes — loaded by the Cline extension |
| Continue.dev | [`skills/continue-context.md`](./skills/continue-context.md) | 🔧 Manual — paste into `systemMessage` in `~/.continue/config.json` |

**Canonical primer**: [`skills/aria-primer.md`](./skills/aria-primer.md) — the single source of truth. All tool files embed content derived from it. Update the primer first, then regenerate tool files when the framework evolves.

---

## Manifest Sections

A full manifest can declare up to 14 sections. Required sections depend on the layer:

| Section | L0 | L1+ | L3+ |
|---|---|---|---|
| `identity` | ✓ | ✓ | ✓ |
| `contract` | ✓ | ✓ | ✓ |
| `composition` | ✓ | ✓ | ✓ |
| `dependencies` | — | ✓ | ✓ |
| `budget` | — | ✓ | ✓ |
| `tests` | — | ✓ | ✓ |
| `behavioral_contract` | — | — | ✓ |
| `health_contract` | — | — | ✓ |
| `diagnostic_contract` | — | — | ✓ |
| `layer` | opt | opt | opt |
| `stability` | opt | opt | opt |
| `migration` | opt | opt | opt |
| `observability` | opt | opt | opt |
| `documentation` | opt | opt | opt |

See [doc 20 — Unified Manifest Schema](docs/20-manifest-schema.md) for full field reference.

---

## Composition Patterns

ARIA defines 14 composition patterns covering all common distributed system concerns:

| Pattern | Use Case |
|---|---|
| `PIPE` | Sequential transform: `T → Result<U, E>` |
| `FORK` | Fan-out to named branches |
| `JOIN` | Fan-in from named branches |
| `PARALLEL_FORK` | Concurrent fan-out: `T → Array<Result<U, E>>` |
| `PARALLEL_JOIN` | Concurrent fan-in with partial success: `ThreeTrack<T, P, E>` |
| `SCATTER_GATHER` | Scatter inputs → gather results |
| `CIRCUIT_BREAKER` | Failure isolation with open/half-open/closed states |
| `SAGA` | Distributed transaction with compensation steps |
| `COMPENSATING_TRANSACTION` | Forward + compensation ARU pair |
| `STREAMING_PIPELINE` | `AsyncIterable<Chunk> → AsyncIterable<Result<U, E>>` |
| `CACHE_ASIDE` | Read-through cache with injected CacheStore |
| `BULKHEAD` | Concurrency isolation with pool injection |
| `PRIORITY_QUEUE` | Priority-envelope dispatch |
| `EVENT_SOURCING` | Command → events + aggregate projection |

See [doc 03 — Composition Patterns](docs/03-composition-patterns.md) for full details.

---

## Semantic Addressing

Every ARU has a hierarchical address that encodes its domain, subdomain, verb, and entity:

```
auth.identity.authenticate.user
│    │        │            └─ Entity: what it operates on
│    │        └─────────────── Verb: what it does (from vocabulary)
│    └──────────────────────── Subdomain
└───────────────────────────── Domain
```

- **L0** addresses are 2 segments: `domain.entity` (no verb — primitive types only)
- **L1+** addresses are 4 segments: `domain.subdomain.verb.entity`

The verb vocabulary is non-overlapping across layer types (query, command, event, streaming). See [doc 06 — Naming Conventions](docs/06-naming-conventions.md).

---

## TypeScript Runtime

The `@aria/runtime` package provides the core type primitives with zero external dependencies:

```typescript
import { Result, success, failure, isSuccess } from '@aria/runtime';
import { RailError, wrapWithProvenance } from '@aria/runtime';
import { ThreeTrack } from '@aria/runtime';
import { createTraceContext } from '@aria/runtime';

// Railway-oriented result type
const result: Result<AuthToken, AuthError> = success(token);

// Error with provenance tracking
const err: RailError<AuthError> = wrapWithProvenance(
  AuthError.INVALID,
  'auth.identity.authenticate.user',
  ctx
);

// Three-track for PARALLEL_JOIN
const joined: ThreeTrack<Output, Partial<Output>, Error> = { _tag: 'PARTIAL_SUCCESS', ... };

// Trace context propagated through composition chains
const ctx = createTraceContext({ originARU: 'auth.identity.authenticate.user' });
```

---

## VS Code Integration

Install the `aria-vscode` extension to get real-time validation as you edit `.manifest.yaml` files:

- **Diagnostics** — schema, naming, and graph violations highlighted inline
- **Completions** — verb vocabulary suggestions when editing `identity.address`
- **Hover** — field documentation from the manifest schema

The extension uses `aria-lsp`, a [tower-lsp](https://github.com/ebkalderon/tower-lsp) server backed by the same Salsa incremental computation database as the CLI. Changes to one manifest only revalidate the affected ARUs.

---

## Documentation

| # | Document | Summary |
|---|---|---|
| [00](docs/00-overview.md) | Overview | Framework thesis, the five pillars, design philosophy |
| [01](docs/01-abstraction-layers.md) | Abstraction Layers | L0–L5 layer definitions and contracts |
| [02](docs/02-atomic-responsibility-units.md) | Atomic Responsibility Units | ARU anatomy and structural spec |
| [03](docs/03-composition-patterns.md) | Composition Patterns | All 14 patterns with type signatures |
| [04](docs/04-context-manifests.md) | Context Manifests | Manifest format and purpose |
| [05](docs/05-semantic-graph.md) | Semantic Graph | Dependency graph structure and validation |
| [06](docs/06-naming-conventions.md) | Naming Conventions | Semantic addressing and verb vocabulary |
| [07](docs/07-consistency-amplification.md) | Consistency Amplification | How ARIA amplifies AI consistency |
| [08](docs/08-type-system.md) | Type System | Algebraic types and Result<T,E> |
| [09](docs/09-type-states.md) | Type States | State machine encoding in types |
| [10](docs/10-algebraic-types.md) | Algebraic Types | Sum and product types in ARIA |
| [11](docs/11-type-compatibility.md) | Type Compatibility | Rules for composition type checking |
| [12](docs/12-error-propagation.md) | Error Propagation | Railway-oriented error model |
| [13](docs/13-contract-versioning.md) | Contract Versioning | Versioning and migration protocol |
| [14](docs/14-human-ai-collaboration.md) | Human-AI Collaboration | Compliance levels and collaboration model |
| [15](docs/15-task-decomposition.md) | Task Decomposition | Grammar for decomposing work |
| [16](docs/16-ai-agent-roles.md) | AI Agent Roles | Specialist agent role definitions |
| [17](docs/17-aru-lifecycle.md) | ARU Lifecycle | Draft → stable → deprecated lifecycle |
| [18](docs/18-observability.md) | Observability | Tracing, metrics, and health contracts |
| [19](docs/19-multi-agent-infrastructure.md) | Multi-Agent Infrastructure | Orchestrating multiple AI agents |
| [20](docs/20-manifest-schema.md) | Manifest Schema | Authoritative schema reference (all 14 sections) |
| [21](docs/21-runtime-composition.md) | Runtime Composition | Runtime execution of composition patterns |
| [22](docs/22-domain-decomposition.md) | Domain Decomposition | Protocol for decomposing domains |
| [23](docs/23-test-infrastructure.md) | Test Infrastructure | Testing strategy and contracts |

---

## License

MIT
