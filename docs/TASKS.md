# ARIA — Task Backlog
### Living document — updated across all iterations

This file tracks everything identified as needed: gaps to fill, fixes to make, new documents to write, assumptions to revisit. Updated as work progresses.

Status legend: `[ ]` pending · `[~]` in progress · `[x]` done

---

## 🔴 Critical Fixes

Issues that break existing theory — must be addressed before building on top.

| ID | Status | Title | Quick Note |
|---|---|---|---|
| `fix-layer-rule` | `[x]` | Layer dependency rule contradiction | docs 01 and 02 contradicted each other: "N→N-1 only" vs "L3 depends on L0–L2". Fixed: rule is now strictly directional (no inversions, no cycles) but not strictly adjacent. Skipping layers is allowed with documented justification. |
| `manifest-derivation` | `[x]` | Specify manifest derivation methodology | doc 04 says "inferred from typed code" — overoptimistic. Must specify: which fields auto-derive (types, effects, deps), which require annotation (purpose, stability, test scenarios), and what triggers manifest invalidation. Stale manifests silently break the entire progressive disclosure system. |
| `logn-claim` | `[x]` | Soften or justify the O(log N) context cost claim | doc 07 claims `context_cost = O(log N)` with no proof. Either provide a formal argument (entropy model, pattern convergence) or soften to "sublinear growth" with qualitative reasoning. Mathematical rigor matters for the framework's credibility. |

---

## 🟠 Foundational Gaps

Missing concepts that are required for the framework to be complete.

| ID | Status | Title | Quick Note |
|---|---|---|---|
| `error-propagation` | `[x]` | Railway-oriented error propagation model | Every PIPE chain has undefined failure behavior. Need formal propagation: success rail / failure rail, errors short-circuit and ride parallel rail to a declared handler. This determines what AI must generate for every PIPE composition — highest practical impact of all gaps. New doc: `12-error-propagation.md` |
| `contract-versioning` | `[x]` | Contract versioning and migration protocol | ARUs are "frozen" but changes are inevitable. Need full lifecycle: `DRAFT → CANDIDATE → STABLE → DEPRECATED → TOMBSTONED`. Key addition: **migration ARU** — a TRANSFORM that bridges old→new types, making evolution a graph operation not a flag day. Also: session-level caching invalidation rules. New doc: `13-contract-versioning.md` |
| `task-decomposition` | `[x]` | Task decomposition grammar | AI receives complex tasks like "add OAuth". Needs a structured work order grammar to decompose into typed subtasks (`TYPE_ADDITION`, `ARU_CREATION`, `ARU_MODIFICATION`, `GRAPH_REWIRE`) with dependency ordering and layer assignments. Enables safe parallel multi-agent execution. New doc: `14-task-decomposition.md` |
| `temporal-contracts` | `[x]` | Temporal/behavioral contracts in ARU manifest | Type system captures structure, not behavior. Need manifest fields: `max_latency_p99`, `max_calls_per_second`, `must_be_called_after` (ordering constraint), `idempotency_window`. Critical for AI generating code against rate-limited APIs, payment systems, event streams. Extend doc `04-context-manifests.md`. |
| `capability-gap` | `[x]` | Capability gap protocol | `capability_search` assumes a match exists. Need formal protocol when no ARU matches (score < threshold): check domain ownership → check cross-domain → check external dep → escalate `TASK_UNDERSPECIFIED` if too broad. Extend doc `05-semantic-graph.md`. |

---

## 🟡 Assumption Revisions

Existing claims that need to be revised based on Claude Code review.

| ID | Status | Title | Quick Note |
|---|---|---|---|
| `context-caching` | `[x]` | Redesign cross-session caching assumption | Framework assumes AI caches contract understanding across sessions — no current AI system does this. Pick one: (a) session-level caching only, (b) vector-store-backed semantic memory injected at session start, (c) pre-built manifest bundles served as context preamble. Must specify the mechanism, not just the intent. |
| `human-ai-collab` | `[x]` | Reframe L0/L5 as human-owned layers | L0 (type registry, state machines) and L5 (domain boundaries) implicitly require human judgment. Reframe as a human-AI collaboration model: humans define L0 vocabulary + L5 boundaries; AI operates primarily in L1–L4. More honest, more practical, better adoption story. Update `00-overview.md` and `01-abstraction-layers.md`. |
| `compliance-levels` | `[x]` | Define ARIA compliance levels for incremental adoption | All-or-nothing enforcement kills real-world adoption. Define levels: L0 (no compliance) → L1 (branded types) → L2 (+layer declarations) → L3 (+manifests) → L4 (+semantic graph) → L5 (full ARIA). Each level delivers measurable AI efficiency gain independently. New section in `00-overview.md` or new doc. |

---

## 🟢 New Documents to Write

New theory areas not yet covered.

| ID | Status | Title | Quick Note |
|---|---|---|---|
| `ai-agent-roles` | `[x]` | AI agent specialization and roles | Different tasks need different context strategies. Define roles: **Generator** (creates ARUs from decomposition), **Reviewer** (validates contracts and graph edges), **Refactorer** (restructures graph without breaking contracts), **Orchestrator** (task decomposition + agent assignment), **Navigator** (graph queries, capability search). Each role has its own context loading protocol and graph access pattern. |
| `aru-lifecycle` | `[x]` | Full ARU lifecycle | How ARUs are born (via decomposition grammar), mature (experimental→stable), version, get deprecated, and are retired. Includes the bootstrapping problem: who creates the first L0 type registry? Relationship to contract versioning. |
| `observability` | `[x]` | Observability as first-class system | OBSERVE pattern handles per-ARU telemetry but there's no system-level model. Need: correlation ID / trace propagation through PIPE chains, health contract definition per ARU type, anomaly surface spec (what context an AI diagnostic agent needs to investigate production issues). |
| `parallel-agent-infra` | `[x]` | Multi-agent parallel development infrastructure | Framework mentions parallel AI agents without specifying the infrastructure: orchestration layer, shared type registry with read/write protocol, graph merge/conflict resolution for simultaneous modifications, agent state isolation. Theory is 2+ years ahead of tooling — this doc should bridge theory and near-term implementation. |
| `patterns-async` | `[x]` | Extend composition patterns for async/distributed systems | Current 10 patterns cover synchronous in-process computation. Missing patterns for distributed systems: **STREAM** (lazy/infinite sequences, backpressure), **SAGA/COMPENSATE** (distributed transactions with typed rollback steps), **CIRCUIT BREAKER** (stateful failure detection, distinct from stateless GATE). Extend `03-composition-patterns.md`. |

---

## 📋 Iteration Log

| Iteration | What Was Done |
|---|---|
| **1 — High Level** | Created 7 core documents: overview, abstraction layers, ARUs, composition patterns, context manifests, semantic graph, naming conventions, consistency amplification |
| **2 — Type System** | Created 4 type system documents: type system overview, type states, algebraic types, type compatibility |
| **3 — Claude's Recommendations (Round 1)** | Fixed layer rule contradiction. Created: `12-error-propagation.md` (railway model), `13-contract-versioning.md` (lifecycle + migration ARUs), `14-human-ai-collaboration.md` (ownership model + 5 compliance levels). Updated `00-overview.md` framing. |
| **4 — Continued Iteration** | Created `15-task-decomposition.md` (typed subtask grammar, decomposition DAG, parallelization, underspecified task detection). Extended `04-context-manifests.md`: added temporal/behavioral contracts section + 3-tier manifest derivation specification. |
| **5 — Agent Infrastructure** | Created `16-ai-agent-roles.md`, `17-aru-lifecycle.md`, `18-observability.md`, `19-multi-agent-infrastructure.md`. Extended `03-composition-patterns.md` with 4 async patterns. Added capability gap protocol to `05-semantic-graph.md`. |
| **6 — Claude Review Round 2 Fixes** | Fixed 10 contradictions (C1–C10), resolved 3 missing pieces (M1–M3), created 4 new documents: `20-manifest-schema.md` (unified schema), `21-runtime-composition.md` (RCL + generated wrappers), `22-domain-decomposition.md` (L5 boundary protocol), `23-test-infrastructure.md` (scenario→test spec). All 31 todos done. |

---

## 🔴 Iteration 6 — Critical Fixes (Round 2 Review)

| ID | Status | Title |
|---|---|---|
| `c1-doc00-index` | `[x]` | Fix doc 00 stale index (pattern count, missing docs 14–19) |
| `c2-validate-semantics` | `[x]` | Fix VALIDATE "passes through unchanged" vs. type narrowing contradiction |
| `c3-join-union` | `[x]` | Fix JOIN union vs. product type contradiction between docs 03 and 11 |
| `c4-l4-ownership` | `[x]` | Fix L4 AI-autonomous graph edges contradicting human-approval in doc 14 |
| `c5-bootstrap-invariant` | `[x]` | Add bootstrap exception to "ARU not in decomposition does not exist" invariant |
| `c6-error-all-patterns` | `[x]` | Complete error propagation for all 14 patterns (9 previously missing) |
| `c7-route-predicate-failure` | `[x]` | Specify ROUTE predicate ARU failure semantics |
| `c8-persist-verb` | `[x]` | Add `persist` to L3 verb vocabulary in doc 06 |
| `c9-manifest-schema` | `[x]` | Create `20-manifest-schema.md` — unified authoritative manifest schema |
| `c10-approval-timing` | `[x]` | Align approval model: per-ARU streaming (not batch at end) |

## 🟠 Iteration 6 — Missing Pieces (Round 2 Review)

| ID | Status | Title |
|---|---|---|
| `m1-runtime-layer` | `[x]` | Create `21-runtime-composition.md` — runtime substrate for railway/trace execution |
| `m2-parallel-join-partial` | `[x]` | PARTIAL_SUCCESS third railway track for PARALLEL_JOIN |
| `m3-saga-compensation` | `[x]` | SAGA compensation failure-rail sub-protocol |

## 🟢 Iteration 6 — New Documents (Round 2 Review)

| ID | Status | Title |
|---|---|---|
| `new-domain-decomposition` | `[x]` | Create `22-domain-decomposition.md` — domain identification + boundary rules |
| `new-test-infrastructure` | `[x]` | Create `23-test-infrastructure.md` — scenario→test mapping, generation protocol |

---

## 🎯 Recommended Priority Order (Iteration 6)

```
1. c8-persist-verb              ← 1-line fix, unblocks naming validator
2. c1-doc00-index               ← structural fix, makes framework navigable
3. c2-validate-semantics        ← fixes type/behavior contradiction
4. c3-join-union                ← fixes composition/type contradiction
5. c4-l4-ownership              ← fixes governance contradiction
6. c5-bootstrap-invariant       ← fixes lifecycle contradiction
7. c10-approval-timing          ← fixes agent protocol contradiction
8. c7-route-predicate-failure   ← adds missing ROUTE failure rule
9. m2-parallel-join-partial     ← PARTIAL_SUCCESS track (prereq for c6)
10. m3-saga-compensation        ← SAGA failure protocol (prereq for c6)
11. c6-error-all-patterns       ← complete error propagation (depends on m2, m3)
12. c9-manifest-schema          ← new doc: unified manifest schema
13. m1-runtime-layer            ← new doc: runtime composition spec
14. new-domain-decomposition    ← new doc: domain boundary protocol
15. new-test-infrastructure     ← new doc: test infrastructure spec
```
