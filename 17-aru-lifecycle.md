# ARU Lifecycle
### Fourth Iteration — How ARUs are born, mature, evolve, and retire

---

## The Complete Lifecycle

An ARU is not a static artifact. It passes through a defined lifecycle from conception to retirement. Each phase has a defined owner, a defined set of legal operations, and a transition gate.

```
                       CONCEPTION
                           │
                    ┌──────▼──────┐
                    │  SPECIFIED  │  ← Orchestrator produces subtask spec
                    └──────┬──────┘
                           │ Generator creates
                    ┌──────▼──────┐
                    │    DRAFT    │  ← Implementation exists; not validated
                    └──────┬──────┘
                           │ Reviewer approves
                    ┌──────▼──────┐
                    │  CANDIDATE  │  ← Contract validated; short-lived cache
                    └──────┬──────┘
                           │ Human approves
                    ┌──────▼──────┐
                    │   STABLE    │  ← Trusted; manifest bundle injected; long-lived
                    └──────┬──────┘
                           │ Breaking change needed
                    ┌──────▼──────┐
                    │ DEPRECATED  │  ← Sunset clock running; migration ARU exists
                    └──────┬──────┘
                           │ sunset_at reached
                    ┌──────▼──────┐
                    │ TOMBSTONED  │  ← Deleted; address reserved; no consumers allowed
                    └─────────────┘
```

---

## Phase 1: Specification

The lifecycle begins not with code, but with a **subtask spec** produced by the Orchestrator's task decomposition. An ARU that is not in a decomposition does not exist.

This prevents the most common form of architectural decay: ARUs created ad-hoc to solve immediate problems, with no declared layer, no typed contract, and no integration into the graph.

**What exists at this phase:**
- A `subtask` object of type `ARU_CREATION`
- Declared: id (semantic address), layer, input type, output type, composition pattern
- Not yet exists: implementation, manifest, files

**Owner:** Orchestrator (produces spec) + Human (approves L0 types if new)
**Transition gate:** Subtask completeness validation passes

---

## Phase 2: Draft

The Generator creates the ARU. The manifest is in `DRAFT` state.

**What exists at this phase:**
- Implementation file
- Manifest file (Tier 1 auto-derived fields populated; Tier 2 AI-proposed)
- Test file (generated from test_contract scenarios)
- Status: `DRAFT`

**Caching behavior:** Never cached. No other ARU may declare a dependency on a DRAFT ARU.

**Owner:** Generator
**Legal operations:**
- Implementation can be freely modified
- Manifest fields can be revised
- Tests can be added/changed

**Transition gate:** Generator submits to Reviewer

---

## Phase 3: Candidate

The Reviewer approves the DRAFT. The contract is now validated but not yet trusted.

**What exists at this phase:**
- All DRAFT artifacts + Reviewer sign-off in manifest
- Status: `CANDIDATE`
- `candidate_since: timestamp`

**Caching behavior:** Session-scoped cache only. The contract is considered reliable for the current session.

**Owner:** Reviewer (validates); Orchestrator (tracks status)
**Legal operations:**
- Generator may make Reviewer-requested fixes (returns to DRAFT temporarily)
- Other ARUs may depend on it, but must use `version_pin` in their manifest

**Transition gate:** Human approves (CANDIDATE → STABLE)

### The Human Approval Step

This is the primary human touchpoint in L1–L4 development. The human is NOT reviewing implementation — they are reviewing the **contract**: does this ARU do what the decomposition intended? Is the semantic address right? Is the stability declaration appropriate?

Human approval typically takes < 1 minute per ARU. It is a judgment call, not a code review.

---

## Phase 4: Stable

The ARU is trusted. Its contract is frozen. It is included in manifest bundles.

**What exists at this phase:**
- All CANDIDATE artifacts
- Status: `STABLE`
- `stable_since: timestamp`
- Included in the next manifest bundle build

**Caching behavior:** Included in manifest bundles; loaded at session start. Long-lived cache.

**Owner:** The contract is immutable. Implementation changes allowed only if they don't change the contract (PATCH/MINOR versions).

**Legal operations (non-breaking):**
- PATCH: bug fixes that don't change observable behavior
- MINOR: add optional output field, add new error variant (with consumer build warning)
- Manifest updates: stability notes, temporal contract measurements, test additions

**Illegal operations:**
- Change input type
- Change output success type
- Remove error variant
- Change side_effects declaration

**Transition gate:** A breaking change is needed → human initiates deprecation

---

## Phase 5: Deprecated

A new version or replacement exists. The old contract is on a sunset clock.

**What exists at this phase:**
- STABLE artifacts unchanged (the contract itself does not change — it just becomes "old")
- Manifest additions:
  ```yaml
  deprecated_at: "2026-06-01T00:00:00Z"
  reason: "Replaced by v2 which adds refresh token support"
  replacement: "auth.token.validate@2.0.0"
  sunset_at: "2026-12-01T00:00:00Z"
  migration_aru: "auth.token.migrate.v1Tov2"
  ```
- Status: `DEPRECATED`

**Caching behavior:** Not cached. Every session reloads and sees the deprecation warning.

**Owner:** Orchestrator tracks; Refactorer creates migration ARU; consumers must migrate.

**Consumer behavior at this phase:**
- Build: warning (not error)
- AI Generator: will not create new ARUs that depend on this; suggests replacement
- AI Reviewer: flags any ARU under review that depends on this as a required migration

**Transition gate:** `sunset_at` is reached (automated)

---

## Phase 6: Tombstoned

The ARU is deleted. Its semantic address is permanently reserved.

**What exists at this phase:**
- Tombstone record in graph index:
  ```yaml
  tombstone:
    id: "auth.token.validate@1.x.x"
    tombstoned_at: "2026-12-01T00:00:00Z"
    replaced_by: "auth.token.validate@2.0.0"
    reason: "Deprecated after v2 migration complete"
  ```
- No files. Implementation is deleted.

**Why reserve the address:** If the address were released, a future ARU could claim it. An AI that had cached the old contract would assume the new ARU has the same contract. This is a silent correctness failure. Reserved addresses prevent this class of bug.

**Consumer behavior:** Any ARU still declaring a dependency on a TOMBSTONED address is a **hard build failure**.

---

## The Bootstrapping Problem

The lifecycle above assumes a task decomposition exists. But who creates the first decomposition? Who writes the L0 type registry for a new domain?

### New Codebase Bootstrap Protocol

```
Phase A: Domain Mapping (Human-led, ~1–2 days)
  1. Human identifies bounded contexts (future L5 domains)
  2. Human names the key domain entities (future L0 types)
  3. Human sketches the major state transitions for each entity
  4. Output: L0 type registry draft + state machine skeletons

Phase B: L0 Population (Human + AI, ~1 day)
  1. Navigator searches for similar domains in reference set
  2. Generator proposes full type definitions from human sketches
  3. Human reviews and approves each type
  4. Output: complete L0 registry, all types STABLE

Phase C: L1 Atom Discovery (AI-primary, ~2–3 days)
  1. Orchestrator decomposes "populate L1 atom layer for [domain]"
  2. Navigator identifies what atoms are needed from the L0 types
  3. Generator creates atoms following reference set patterns
  4. Reviewer validates each
  5. Human approves CANDIDATE → STABLE batch
  6. Output: complete L1 atom surface

Phase D: Upward Layer Construction
  L2, L3, L4 follow same pattern, using lower layers as building blocks.
  Each layer is driven by task decomposition from the Orchestrator.

Phase E: L5 Domain Surface (Human-led)
  Human defines what the domain exposes externally.
  AI implements the surface ARUs.
```

**Total human time for a medium-sized new domain:** ~1 day of concentrated work, then sporadic approval tasks.

### Existing Codebase Migration Bootstrap

See `14-human-ai-collaboration.md` — the compliance levels define the migration path, starting with type branding (Level 1) and working up.

---

## Reference ARU Density

A healthy ARIA codebase maintains a predictable **ARU density** per layer:

| Layer | Typical ARU Count (per domain) | Signal if too low | Signal if too high |
|---|---|---|---|
| L0 | 10–30 types | Domain vocabulary is incomplete | Types are too granular; merge some |
| L1 | 20–60 atoms | Business logic is leaking into L2+ | Atoms are doing too much; split |
| L2 | 10–30 molecules | L3 is assembling too much | Molecules have multiple purposes |
| L3 | 5–15 organisms | System layer is doing business logic | Organisms have too many deps |
| L4 | 2–8 systems | Domain is too siloed | System is an orchestration monolith |
| L5 | 1–3 surfaces | Domain has no external contract | Domain has too many entry points |

These are ranges, not rules. But significant deviation from these ranges is a signal that the ARU boundary definitions need review. The Navigator can compute density metrics and surface them as part of graph health monitoring.
