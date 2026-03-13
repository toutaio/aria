# The Semantic Graph
### Pillar 5 of ARIA — MAP of the entire codebase navigable by AI

---

## The Codebase as a Directed Acyclic Graph

In ARIA, the entire codebase is formally a **Directed Acyclic Graph (DAG)**:

- **Nodes** = ARUs (each uniquely identified by semantic address)
- **Edges** = Composition Pattern instances (typed relationships)
- **Levels** = Abstraction Layers (L0–L5, with edges only going upward)

This is not a metaphor — the graph is a first-class artifact, generated from the manifest declarations and used by AI as the primary navigation tool.

```
L5  [AuthDomain] ─────────────────────────────────────────────────
         │ EXPOSE
L4  [UserOnboardingSystem] ──────────────────────────────────────
         │ PIPE             │ PIPE
L3  [RegisterUser]     [VerifyEmail] ───────────────────────────
         │ PIPE              │ PIPE
L2  [CreateVerifiedUser]  [SendVerification] ──────────────────
      │    │                  │
     PIPE  PIPE              PIPE
      │    │                  │
L1 [Hash] [Validate]     [BuildEmail] ───────────────────────
      │       │                │
L0 [Hash_T] [Email_T]   [Template_T] ──────────────────────
```

---

## Graph Properties

### Acyclicity (No Circular Dependencies)
The DAG constraint is absolute. Cycles indicate architectural violations — two ARUs that depend on each other belong in a higher-layer ARU that orchestrates both.

A cycle detector runs at build time. Any cycle is a **hard build failure**.

### Monotonic Layering
All edges point from higher layers to lower layers. An edge from L2 to L3 is a build failure (inversion). An edge skipping layers (L3 to L0) is also a build failure (skip).

### Edge Type Completeness
Every edge must declare its Composition Pattern. An untyped edge (plain import without a declared pattern) is a build failure.

---

## Graph Queries Available to AI

The Semantic Graph supports a set of pre-defined queries that AI uses for navigation. These are computed at build time and served as index artifacts.

### Q1: Minimum Subgraph for Task
*"What is the smallest set of ARUs I need to read to implement X?"*

```
query: minimum_subgraph(target_aru, task_type)
input: target ARU id + task type (use | modify | extend | replace)
output: ordered list of ARU ids with their required context level

example:
  minimum_subgraph("auth.token.validate", MODIFY)
  → [
      { aru: "auth.token.validate",   level: IMPLEMENTATION },
      { aru: "crypto.jwt.decode",     level: CONTRACT },
      { aru: "time.now",              level: CONTRACT },
      { aru: "auth.types.TokenString", level: CONTRACT }
    ]
  total_budget: 1240 tokens
```

### Q2: Impact Radius
*"If I change X, what else might be affected?"*

```
query: impact_radius(target_aru)
input: ARU id
output: all ARUs that transitively depend on target (upstream)

example:
  impact_radius("auth.types.TokenString")
  → 7 ARUs across L1–L4 that use this type
  → contract_change_risk: HIGH (frozen type being changed)
```

### Q3: Capability Search
*"Is there already an ARU that does X?"*

```
query: capability_search(semantic_description)
input: natural language description of desired behavior
output: ranked list of ARU ids with similarity scores

example:
  capability_search("validate email format")
  → [
      { aru: "user.email.validate", score: 0.97 },
      { aru: "notification.recipient.validate", score: 0.61 }
    ]
```

This prevents AI from creating duplicate ARUs.

### Capability Gap Protocol

When `capability_search` returns no confident match, the Navigator does not silently proceed. It runs a structured gap assessment:

```
capability_gap_assessment:
  query: "validate OAuth callback code format"
  best_match_score: 0.41   # below confidence threshold of 0.70

  gap_analysis:
    step_1_domain_check:
      question: "Does a domain exist that should own this capability?"
      result: "YES — auth domain owns OAuth concerns"
      action: "Create new ARU in auth domain"

    step_2_cross_domain_check:
      question: "Does this require capabilities from multiple domains?"
      result: "NO — purely auth.oauth concern"
      
    step_3_external_dep_check:
      question: "Does this require wrapping an external library/API?"
      result: "NO — format validation is pure logic"
      
    step_4_scope_check:
      question: "Is the capability description specific enough to create one ARU?"
      result: "YES"

  resolution: CREATE_NEW_ARU
  proposed_address: "auth.oauth.validate.callbackCode"
  proposed_layer: L1
  proposed_subtask_type: ARU_CREATION
```

Gap assessment outcomes:

| Outcome | Condition | Action |
|---|---|---|
| `CREATE_NEW_ARU` | Domain exists, scope is clear | Orchestrator adds ARU_CREATION subtask |
| `CREATE_INTEGRATION_ARU` | Requires external dep | Refactorer wraps in anti-corruption ARU at L5 |
| `SPLIT_EXISTING_ARU` | Match score 0.5–0.7 (partial match) | Refactorer extracts the needed sub-capability |
| `TASK_UNDERSPECIFIED` | Score < 0.4 or ambiguous domain | Orchestrator emits signal, blocks execution, returns to human |
| `CREATE_DOMAIN` | No domain exists for this capability | Human touchpoint required before proceeding |


*"How should I connect A and B?"*

```
query: composition_suggest(aru_a, aru_b)
input: two ARU ids
output: compatible patterns based on their input/output types

example:
  composition_suggest("auth.token.validate", "auth.session.create")
  → PIPE (ValidatedToken output matches session.create input)
  → or ROUTE (if conditional session creation is intended)
```

### Q5: Layer Neighbor Discovery
*"What other ARUs exist at the same layer in this domain?"*

```
query: neighbors(aru_id, layer, domain)
output: all ARUs with same layer and domain prefix
use: finding patterns to follow, ensuring naming consistency
```

---

## The Graph Index

The Semantic Graph is stored as a serialized index artifact, generated at build time. Its structure:

```json
{
  "nodes": {
    "auth.token.validate": {
      "layer": 1,
      "manifest_hash": "sha256:...",
      "context_budget": { "use": 120, "modify": 340, "extend": 580 },
      "stability": "STABLE"
    }
  },
  "edges": [
    {
      "from": "auth.token.validate",
      "to": "crypto.jwt.decode",
      "pattern": "PIPE",
      "type_compatibility": "verified"
    }
  ],
  "layer_surfaces": {
    "1": ["auth.token.validate", "crypto.jwt.decode", ...],
    "2": ["auth.session.create", ...]
  }
}
```

This index is the **first artifact loaded** by any AI agent beginning work on the codebase. It is the map. Everything else is territory.

---

## AI Navigation Protocol

When an AI agent is initialized for a task:

```
Step 1:  Load graph index              (~500 tokens, one-time)
Step 2:  Resolve task to target ARU    (semantic search or explicit id)
Step 3:  Query minimum_subgraph        (get ordered read list + budgets)
Step 4:  Query impact_radius           (understand blast radius of changes)
Step 5:  Load ARU manifests in order   (progressive disclosure)
Step 6:  Load implementations          (only as far as task requires)
Step 7:  Execute task                  (write, modify, or analyze)
Step 8:  Verify impact_radius ARUs     (confirm no unintended breakage)
```

This protocol means AI **never reads code it doesn't need** and **always understands the impact of what it changes.**

---

## Graph Evolution

The graph is a living artifact. When an ARU is added, modified, or removed:

1. Graph index is regenerated
2. Impact radius is computed for the change
3. ARUs in the impact radius have their manifests validated
4. Type compatibility of all affected edges is re-verified
5. Context budgets of upstream ARUs are recalculated

This creates an **automatic ripple detection system**: changes that break the graph are caught before AI is asked to work with broken contracts.

---

## The Graph as Architecture Documentation

For the rare case where a human needs to understand the system, the Semantic Graph can be rendered visually at any layer of abstraction:

- Layer 5 view: 5–10 domain nodes
- Layer 4 view: system pipelines
- Layer 3 view: business logic topology
- Layer 2 view: molecule composition
- Layer 1 view: atom dependency map

Each view is a subgraph projection — the same data, different zoom level. The architecture IS the graph. There is no separate documentation to maintain.
