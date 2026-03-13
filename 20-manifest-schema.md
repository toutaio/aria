# Unified Manifest Schema
### Sixth Iteration — Single authoritative definition of the ARU manifest

---

## Why a Unified Schema

The ARU manifest is the central artifact of ARIA. It is the source from which:
- AI agents load context (`04-context-manifests.md`)
- The Semantic Graph is built (`05-semantic-graph.md`)
- Type compatibility is checked (`11-type-compatibility.md`)
- Error propagation is configured (`12-error-propagation.md`)
- Contract versions are tracked (`13-contract-versioning.md`)
- Health contracts are evaluated (`18-observability.md`)
- Diagnostic surfaces are populated (`18-observability.md`)
- Type state machines are referenced (`09-type-states.md`)
- Lifecycle phase transitions are gated (`17-aru-lifecycle.md`)

Before this document, manifest fields were scattered across multiple documents with no single canonical schema. This is the authoritative definition. All other documents reference this one.

---

## Schema Derivation Tiers

Every field in the schema has a **derivation tier** — how it gets its value:

| Tier | Source | Requires Human? |
|---|---|---|
| **Tier 1** | Auto-derived from static analysis (types, dependencies, layer) | No |
| **Tier 2** | AI-proposed, human-approved (purpose, test scenarios, stability) | Approval only |
| **Tier 3** | Human-declared (domain boundaries, sunset dates, known failure patterns) | Yes |

Fields marked `[T1]`, `[T2]`, `[T3]` throughout this document.

---

## Complete Manifest Schema

```yaml
# ============================================================
# ARIA ARU MANIFEST — Authoritative Schema (doc 20)
# ============================================================

manifest:

  # ----------------------------------------------------------
  # SECTION 1: Identity
  # ----------------------------------------------------------
  id: "[domain].[subdomain].[verb].[entity]"    # [T1] semantic address — unique, immutable
  version: "MAJOR.MINOR.PATCH"                  # [T1/T3] semver; T3 for breaking changes
  schema_version: "1.0"                         # [T1] which version of this manifest schema

  identity:
    purpose: "one sentence, active voice"        # [T2] what this ARU does (not how)
    domain: "domain_name"                        # [T1] top-level bounded context
    subdomain: "subdomain_name"                  # [T1] functional area within domain
    verb: "verb_class"                           # [T1] must be in layer's verb vocabulary (doc 06)
    entity: "entity_name"                        # [T1] the data subject

  # ----------------------------------------------------------
  # SECTION 2: Layer
  # ----------------------------------------------------------
  layer:
    declared: L0 | L1 | L2 | L3 | L4 | L5      # [T2] approved by human at Tier 2
    inferred: L0 | L1 | L2 | L3 | L4 | L5       # [T1] derived from verb vocabulary
    # Build failure if declared != inferred

  # ----------------------------------------------------------
  # SECTION 3: Contract
  # ----------------------------------------------------------
  contract:
    input:
      type: "TypeName"                           # [T1] branded type from L0 registry
      constraints:                               # [T2] additional semantic constraints
        - "constraint description"
    output:
      success: "TypeName"                        # [T1] success-path type
      failure: "ErrorType { code: A | B | C }"  # [T1/T2] T1 for structural; T2 for semantics
    side_effects: NONE | READ | WRITE | EXTERNAL # [T2] most conservative wins
    idempotent: true | false                     # [T2]
    deterministic: true | false                  # [T1] false if uses time, random, external I/O

  # ----------------------------------------------------------
  # SECTION 4: Type State (optional — only for stateful ARUs)
  # ----------------------------------------------------------
  type_state:                                    # [T2] omit for stateless ARUs
    input_state: "StateName"                     # the type state this ARU expects
    output_state: "StateName"                    # the type state this ARU produces
    machine_ref: "domain.TypeStateMachine"       # reference to the L0 state machine definition
    # See: 09-type-states.md

  # ----------------------------------------------------------
  # SECTION 5: Dependencies
  # ----------------------------------------------------------
  dependencies:                                  # [T1] derived from import analysis
    - id: "domain.sub.verb.entity"
      layer: L0 | L1 | L2 | L3 | L4 | L5
      version_pin: "MAJOR.MINOR.PATCH"           # [T2] required for CANDIDATE dependencies
      stability: EXPERIMENTAL | STABLE | FROZEN  # [T1] from dependency's manifest

  # ----------------------------------------------------------
  # SECTION 6: Composition (only for composed ARUs)
  # ----------------------------------------------------------
  composition:                                   # [T2] required if this ARU is a composition
    pattern: PIPE | FORK | JOIN | GATE | ROUTE | LOOP | OBSERVE |
             TRANSFORM | VALIDATE | CACHE | STREAM | SAGA | CIRCUIT_BREAKER | PARALLEL_JOIN
    chain: []                                    # [T2] for PIPE: ordered ARU id list
    error_handler: "domain.sub.handleError"      # [T2] required for PIPE, FORK, JOIN, STREAM
    # Pattern-specific fields:
    # FORK:
    #   branches: []
    # JOIN:
    #   merge_type: "ProductTypeName"            # always a product type — no union merges
    # GATE:
    #   predicate_aru: "domain.sub.validate.pred"
    # ROUTE:
    #   predicate_aru: "domain.sub.validate.pred"
    #   branches:
    #     true_branch: "domain.sub.verb.entity"
    #     false_branch: "domain.sub.verb.entity"
    # LOOP:
    #   condition_aru: "domain.sub.compute.condition"
    #   max_iterations: N
    # CACHE:
    #   key_aru: "domain.sub.compute.cacheKey"
    #   ttl_seconds: N
    #   invalidation: "domain.sub.emit.invalidate"
    #   read_failure_policy: FALLTHROUGH | FAIL
    # STREAM:
    #   source_aru: "domain.sub.verb.source"
    #   processor_aru: "domain.sub.verb.element"
    #   backpressure: DROP | BUFFER(n) | ERROR
    #   element_failure_policy: DROP | DEAD_LETTER | FAIL_STREAM
    #   dead_letter_aru: "domain.sub.handle.deadLetter"
    # SAGA:
    #   steps:
    #     - aru: "..."
    #       compensating_aru: "..."
    # CIRCUIT_BREAKER:
    #   target_aru: "domain.sub.verb.entity"
    #   failure_threshold: N
    #   evaluation_window_ms: N
    #   half_open_probe_interval_ms: N
    # PARALLEL_JOIN:
    #   branches: []
    #   minimum_required_results: N
    #   timeout_ms: N
    #   partial_success_handler: "domain.sub.handle.partial"

  # ----------------------------------------------------------
  # SECTION 7: Saga Participation (only for SAGA step ARUs)
  # ----------------------------------------------------------
  saga_participant:                              # [T2] required if this ARU participates in a SAGA
    compensating_aru: "domain.sub.compensate.entity"
    idempotency_key_field: "fieldName"

  # ----------------------------------------------------------
  # SECTION 8: Context Budget
  # ----------------------------------------------------------
  context_budget:
    to_use: N       # [T1] tokens to call this ARU correctly (signature + types + errors)
    to_modify: N    # [T1] tokens to change internal behavior
    to_extend: N    # [T1] tokens to add new functionality
    to_replace: N   # [T1] tokens to re-implement from scratch

  # ----------------------------------------------------------
  # SECTION 9: Test Contract
  # ----------------------------------------------------------
  test_contract:                                 # [T2] AI-proposed, human-approved
    - scenario: "description of behavior"        # each becomes a test case (see doc 23)
    coverage_required: true                      # [T3] default true; false only with justification
    mutation_testing: true | false               # [T3] default false; recommended for L3+

  # ----------------------------------------------------------
  # SECTION 10: Behavioral Contract (performance + ordering)
  # ----------------------------------------------------------
  behavioral_contract:                           # [T2] AI-proposed from measurements or defaults
    max_latency_p99: "Nms"
    max_latency_p999: "Nms"
    max_calls_per_second: N
    max_calls_per_user_per_second: N
    idempotent_within: "Ns | not_applicable"
    timeout: "Nms"
    must_be_called_after: []                     # semantic ordering constraints
    must_be_called_before: []
    max_retries: N
    retry_strategy: "exponential_backoff | linear | none"

  # ----------------------------------------------------------
  # SECTION 11: Stability and Lifecycle
  # ----------------------------------------------------------
  stability: EXPERIMENTAL | STABLE | FROZEN     # [T2] approved by human
  lifecycle:
    phase: SPECIFIED | DRAFT | CANDIDATE | STABLE | DEPRECATED | TOMBSTONED  # [T1]
    candidate_since: ISO8601Timestamp            # [T1] set when Reviewer approves
    stable_since: ISO8601Timestamp               # [T1] set when human approves
    deprecated_since: ISO8601Timestamp           # [T3] set by human
    sunset_at: ISO8601Timestamp                  # [T3] required when DEPRECATED
    migration_aru: "domain.sub.migrate.entity"   # [T3] required when DEPRECATED
    tombstoned_at: ISO8601Timestamp              # [T1] set by system

  # ----------------------------------------------------------
  # SECTION 12: Health Contract
  # ----------------------------------------------------------
  health_contract:                               # [T2] required for L3+; optional for L1–L2
    sla_latency_p99: "Nms"                       # SLA target (may differ from behavioral_contract)
    sla_availability: "N.NN%"
    health_check_aru: "domain.sub.check.health"  # [T2] optional; auto-inferred if absent
    degraded_threshold:                          # when to transition HEALTHY → DEGRADED
      error_rate_percent: N
      latency_p99_multiplier: N                  # e.g. 2.0 = degraded at 2× SLA latency
    circuit_open_threshold:                      # when to transition DEGRADED → CIRCUIT_OPEN
      error_rate_percent: N
      consecutive_failures: N

  # ----------------------------------------------------------
  # SECTION 13: Diagnostic Surface
  # ----------------------------------------------------------
  diagnostic_surface:                            # [T2/T3] mixed derivation
    failure_indicators:                          # [T2] AI-proposed from contract analysis
      - symptom: "description of observable failure signal"
        check: "what to look at"
    known_failure_patterns:                      # [T3] human-approved; grows over time
      - pattern: "pattern name"
        description: "what it looks like"
        resolution_steps:
          - "step 1"
          - "step 2"
        minimum_context_level: 1 | 2 | 3 | 4
    escalation_path:                             # [T1] derived from composition dependency graph
      - layer: L1
        handler: "domain.sub.handle.atomError"
      - layer: L3
        handler: "domain.sub.handle.orgError"

  # ----------------------------------------------------------
  # SECTION 14: Manifest Provenance
  # ----------------------------------------------------------
  manifest_provenance:
    derived_by: "STATIC_ANALYSIS | AI_PROPOSED | HUMAN_DECLARED"  # [T1] overall derivation source
    reviewed_by: "REVIEWER_AGENT | HUMAN"                         # [T1] who approved this manifest
    approved_at: ISO8601Timestamp                                  # [T1]
    bundle_version: "sha256:..."                                   # [T1] hash for cache invalidation
```

---

## Field Mandatoriness by Layer

Not every field is required for every ARU. Required fields scale with layer complexity:

| Section | L0 | L1 | L2 | L3 | L4 | L5 |
|---|---|---|---|---|---|---|
| Identity | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Layer | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Contract | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Type State | — | if applicable | if applicable | if applicable | — | — |
| Dependencies | — | ✓ | ✓ | ✓ | ✓ | ✓ |
| Composition | — | — | if composite | ✓ | ✓ | ✓ |
| Saga Participation | — | — | — | if applicable | — | — |
| Context Budget | — | ✓ | ✓ | ✓ | ✓ | ✓ |
| Test Contract | — | ✓ | ✓ | ✓ | ✓ | ✓ |
| Behavioral Contract | — | optional | optional | ✓ | ✓ | ✓ |
| Stability + Lifecycle | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Health Contract | — | — | — | ✓ | ✓ | ✓ |
| Diagnostic Surface | — | — | — | ✓ | ✓ | ✓ |
| Manifest Provenance | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

## Manifest Evolution Protocol

The manifest schema itself has a version (`schema_version`). When the schema evolves:

### Backward-Compatible Changes (MINOR version bump)
- Adding optional fields
- Adding new enum values to existing fields
- Expanding a field's allowed derivation tier

These changes do NOT require existing manifests to be updated. Old manifests remain valid.

### Breaking Changes (MAJOR version bump)
- Renaming required fields
- Changing field types
- Removing fields
- Changing mandatoriness (optional → required)

Breaking manifest schema changes require:
1. A **schema migration ARU** — a TRANSFORM that upgrades old manifest format to new format
2. A build-time compatibility flag day announcement
3. A deprecation period of at least one release cycle

### Manifest Validation

The manifest schema is expressed as a JSON Schema artifact (`aria-manifest.schema.json`).
Every manifest is validated against this schema at build time. Invalid manifests are build failures.

---

## Known Failure Pattern Write-Back Protocol

The `diagnostic_surface.known_failure_patterns` field grows over time as new failure modes are
discovered. The write-back protocol ensures this knowledge is captured:

```
1. AI Diagnostic Agent identifies a novel failure pattern during investigation
2. Agent proposes pattern addition in the format above (Tier 2 proposal)
3. Human reviews and approves the proposed pattern (Tier 3 elevation)
4. Approved pattern is committed to the manifest file by the Reviewer agent
5. Manifest bundle is re-built (new bundle_version hash)
6. All sessions started after bundle rebuild receive the new pattern
```

No agent may write to `known_failure_patterns` without human approval.
Unapproved patterns may be stored in a `proposed_failure_patterns` scratch field (not validated).

---

## Relationship to Other Documents

| Document | What It Uses From This Schema |
|---|---|
| `04-context-manifests.md` | Sections 1–10 (manifest structure, context budgets, behavioral contracts) |
| `09-type-states.md` | Section 4 (type_state reference) |
| `11-type-compatibility.md` | Section 3 (contract types) and Section 5 (composition) |
| `12-error-propagation.md` | Section 6 (composition.error_handler, pattern-specific fields) |
| `13-contract-versioning.md` | Section 11 (lifecycle fields) |
| `17-aru-lifecycle.md` | Section 11 (lifecycle.phase transitions) |
| `18-observability.md` | Sections 12–13 (health_contract, diagnostic_surface) |
| `19-multi-agent-infrastructure.md` | Section 14 (bundle_version for cache invalidation) |

This document supersedes any conflicting manifest structure defined in the above documents.
When in doubt, this schema is authoritative.
