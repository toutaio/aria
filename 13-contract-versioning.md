# Contract Versioning and Migration
### Third Iteration — How ARU contracts evolve without breaking AI trust

---

## The Immutability Paradox

`02-atomic-responsibility-units.md` states:

> *"Once an ARU's contract is published, it is frozen."*

This is architecturally ideal but practically impossible. Business requirements change. Type system insights improve. Security vulnerabilities are discovered. Contracts will change. The question is not whether — it's how.

The versioning system must satisfy two competing requirements:
1. **AI trust**: an AI that loaded a contract last session must know if that contract is still valid
2. **Evolution**: the system must be able to change contracts without catastrophic disruption

---

## The Contract Lifecycle

Every ARU contract passes through a defined lifecycle. Transitions are explicit and one-directional (no going backward).

```
  DRAFT ──▶ CANDIDATE ──▶ STABLE ──▶ DEPRECATED ──▶ TOMBSTONED
    │            │            │             │               │
  (local)    (review)     (trusted)    (sunset clock    (deleted,
  (no cache)  (short       (cache        running)       migration
              cache)       forever)                      required)
```

### DRAFT
- Contract is being designed; may change freely
- AI must NOT cache this contract between sessions
- Only the creating agent/human works with this ARU
- No other ARU may declare a dependency on a DRAFT ARU

### CANDIDATE
- Contract is proposed and under review
- Short-lived cache permitted (session-scoped only)
- Other ARUs may use it, but with explicit `pinned_version` declaration
- Breaking changes still allowed, but trigger re-validation of all dependents

### STABLE
- Contract is frozen; breaking changes forbidden
- Cache indefinitely (or until a new major version is published)
- The default state for all production ARUs
- Any change that passes the Non-Breaking Change Test (see below) is allowed without lifecycle transition

### DEPRECATED
- A newer version exists or the ARU is being removed
- Cache is invalidated; AI must reload on next use
- Mandatory fields added to manifest:
  ```yaml
  deprecated_at: ISO8601Timestamp
  reason: "replaced by auth.token.validate.v2 — adds refresh token support"
  replacement: "auth.token.validate"  # new version id, or NONE
  sunset_at: ISO8601Timestamp         # hard deadline for consumer migration
  migration_aru: "auth.token.migrate.v1ToV2"  # optional bridge ARU
  ```
- Consumers receive build warnings; after `sunset_at` they receive build errors

### TOMBSTONED
- The ARU no longer exists in any form
- Its semantic address is permanently reserved (never reused — reuse causes AI confusion)
- The graph index retains the node as a tombstone for historical reference
- Any consumer still depending on a TOMBSTONED ARU is a hard build failure

---

## Semantic Versioning for ARUs

ARUs use a three-part version: `MAJOR.MINOR.PATCH`

| Change | Version | Breaking? | Lifecycle effect |
|---|---|---|---|
| Bug fix, same types | `PATCH` | No | Stays STABLE |
| Add optional output field | `MINOR` | No | Stays STABLE |
| New error variant in output union | `MINOR` | **Semi** — consumers must handle new case | Stays STABLE, consumers get build warning |
| Change input type | `MAJOR` | Yes | Triggers deprecation of old contract |
| Change output success type | `MAJOR` | Yes | Triggers deprecation |
| Remove field from output | `MAJOR` | Yes | Triggers deprecation |
| Change side effect declaration | `MAJOR` | Yes | Triggers deprecation |

### The Non-Breaking Change Test

A change is non-breaking if and only if:

```
1. All existing consumers compile without change
2. All existing consumer tests pass without change
3. The output type is a supertype of (or equal to) the previous output type
   (you may ADD information; you may not REMOVE or ALTER it)
4. No previously valid input is now rejected
```

---

## The Migration ARU

When a breaking change is unavoidable, a **Migration ARU** is created. This is a special TRANSFORM ARU that bridges the old contract to the new one:

```
Consumer (still using v1) ──▶ [MigrationARU: v1_output → v2_input] ──▶ New downstream (expecting v2)
```

Migration ARU naming convention:
```
[domain].[entity].migrate.[fromVersion]To[toVersion]
auth.token.migrate.v1Tov2
```

Migration ARU properties:
- Layer: same as the ARU being migrated
- Side effects: NONE (pure transformation only)
- Stability: STABLE (must be reliable during the migration window)
- Sunset: deprecated and tombstoned when all consumers have migrated

The migration ARU makes contract evolution a **graph operation**: insert the migration node between old producer and new consumer, verify type compatibility, done. No big-bang refactoring.

---

## Versioned Semantic Addresses

When a breaking change creates a new version, the new ARU gets a new semantic address:

```
Old: auth.token.validate          (v1, now DEPRECATED)
New: auth.token.validate          (v2, now STABLE — same address, new version)
```

The semantic address stays the same. The graph tracks both versions simultaneously during the migration window:

```
Graph during migration:
  auth.token.validate@1.x.x  ── DEPRECATED ──▶ [consumers pinned to v1]
  auth.token.validate@2.0.0  ── STABLE ──────▶ [consumers upgraded to v2]
  auth.token.migrate.v1Tov2  ── STABLE ──────▶ [bridge for slow migrators]
```

After `sunset_at`, `@1.x.x` is TOMBSTONED and the migration ARU is DEPRECATED.

---

## Session-Level Caching and Invalidation

Since current AI systems have no persistent cross-session memory, the framework redefines "caching" as an **injection model**:

### How "Caching" Actually Works in Practice

1. **The manifest bundle**: a pre-built artifact containing all STABLE and FROZEN contracts, serialized and indexed by semantic address
2. **Injected at session start**: when an AI session begins, the relevant manifest bundle is injected as context preamble (or fetched via tool)
3. **Version-pinned**: the bundle carries a `bundle_version` hash; any contract change increments the hash
4. **Invalidation**: if an AI holds a `bundle_version` that doesn't match the current graph index, it must reload the affected manifests before proceeding

```yaml
manifest_bundle:
  bundle_version: "sha256:a3f9..."
  generated_at: "2026-03-12T22:00:00Z"
  contracts:
    - id: "auth.token.validate"
      version: "2.1.0"
      stability: "STABLE"
      contract_hash: "sha256:b7c2..."
      # ... full contract at 'use' context level
```

### Invalidation Triggers

A manifest bundle is considered stale when:
- Any included ARU has changed its `MAJOR` or `MINOR` version
- Any included ARU has changed lifecycle state (e.g., STABLE → DEPRECATED)
- A new ARU has been added that is depended upon by an included ARU
- The graph index `bundle_version` hash differs from the cached version

When stale, the AI **must not use cached contract data**. It reloads from the graph index before proceeding.

---

## AI Behavior at Each Lifecycle State

| State | AI caches? | AI generates code against it? | AI warns consumer? |
|---|---|---|---|
| DRAFT | Never | Only if it is the creating agent | N/A |
| CANDIDATE | Session-only | Yes, with explicit version pin | No |
| STABLE | Via bundle | Yes | No |
| DEPRECATED | Never | Yes (old), with migration warning | Yes — "use replacement instead" |
| TOMBSTONED | Never | No — build failure | Hard error |

---

## The Bootstrapping Question

Who decides when a contract transitions from DRAFT to CANDIDATE to STABLE?

In the human-AI collaboration model (see `14-human-ai-collaboration.md`):
- **DRAFT → CANDIDATE**: the generating AI agent proposes; a Reviewer agent validates
- **CANDIDATE → STABLE**: a human approves (this is the human's primary role in L1–L4)
- **STABLE → DEPRECATED**: initiated by a human; executed by the graph system
- **DEPRECATED → TOMBSTONED**: automatic after `sunset_at` (no human needed)

The human's involvement is minimal and at high-leverage decision points — not in the implementation details.
