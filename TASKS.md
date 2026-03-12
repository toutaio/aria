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
| **3 — Claude's Recommendations** | Fixed layer rule contradiction. Created: `12-error-propagation.md` (railway model), `13-contract-versioning.md` (lifecycle + migration ARUs), `14-human-ai-collaboration.md` (ownership model + 5 compliance levels). Updated `00-overview.md` framing. |
| **4 — Continued Iteration** | Created `15-task-decomposition.md` (typed subtask grammar, decomposition DAG, parallelization, underspecified task detection). Extended `04-context-manifests.md`: added temporal/behavioral contracts section + 3-tier manifest derivation specification. |

---

## 🎯 Recommended Priority Order

Based on dependency graph of the tasks above:

```
1. error-propagation        ← most foundational missing piece; every composition depends on it
2. contract-versioning      ← makes caching/stability system actually work
3. human-ai-collab          ← reframes the whole framework more honestly; affects overview
4. compliance-levels        ← enables real-world adoption
5. task-decomposition       ← highest practical value for AI agents in the field
6. temporal-contracts       ← extends manifests, low disruption
7. manifest-derivation      ← fixes a silent failure mode
8. ai-agent-roles           ← new doc, builds on decomposition grammar
9. aru-lifecycle            ← builds on versioning + roles
10. observability           ← builds on ARU lifecycle + temporal contracts
11. capability-gap          ← extends semantic graph
12. context-caching         ← architectural decision, needs options analysis
13. patterns-async          ← extends composition patterns
14. parallel-agent-infra    ← most speculative, do last
15. logn-claim              ← editorial fix, low effort
```
