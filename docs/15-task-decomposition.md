# Task Decomposition Grammar
### Third Iteration — Structured work orders for AI agents

---

## The Problem with Free-Form Tasks

When an AI receives a high-level task like *"add OAuth support to the auth domain"*, it faces an unbounded problem. Without a decomposition grammar, it:

- Has no principled stopping point at each step
- Cannot coordinate with other agents safely
- Has no way to signal *what kind* of work each step is
- Cannot detect when a task is too broad to execute

The Task Decomposition Grammar solves this by converting a natural-language goal into a **typed, layered, dependency-ordered work tree** before a single line of code is written.

> The decomposition IS the design. Implementation is the mechanical execution of the decomposition.

---

## The Decomposition Hierarchy

```
Task
 └── Subtask[]
       ├── type: SubtaskType
       ├── target: ARU_id | TypeRegistryEntry | GraphEdge
       ├── layer: L0–L5
       ├── depends_on: Subtask.id[]
       └── assignable_to: AgentRole
```

A **Task** is the high-level goal. A **Subtask** is the smallest unit of work that can be assigned to a single agent, affects a single target, and produces a verifiable output. A Subtask maps 1:1 to exactly one of the operations in the Subtask Type Taxonomy.

---

## Subtask Type Taxonomy

Every subtask has a declared type. Types are organized by the layer they primarily affect:

### L0 Operations

| Type | Meaning | Output |
|---|---|---|
| `TYPE_ADDITION` | Add a new branded type or primitive to the L0 registry | New type definition |
| `TYPE_STATE_ADDITION` | Add a new state to an existing type state machine | Updated state machine |
| `TYPE_STATE_TRANSITION` | Add a new legal transition between existing states | Updated transition table |
| `TYPE_CONSTRAINT_ADD` | Add a new constraint/guarantee to an existing type | Updated type definition |

### L1–L4 ARU Operations

| Type | Meaning | Output |
|---|---|---|
| `ARU_CREATION` | Create a new ARU at a declared layer | New ARU + manifest |
| `ARU_MODIFICATION` | Non-breaking change to an existing ARU | Updated ARU (PATCH/MINOR version) |
| `ARU_VERSION` | Breaking change — creates a new version of an ARU | New versioned ARU + migration ARU |
| `ARU_DEPRECATION` | Initiate the deprecation lifecycle of an ARU | Updated manifest (STABLE→DEPRECATED) |

### Graph Operations

| Type | Meaning | Output |
|---|---|---|
| `GRAPH_EDGE_ADD` | Declare a new composition between two existing ARUs | New edge in semantic graph |
| `GRAPH_EDGE_REMOVE` | Remove a composition (with impact verification) | Removed edge + verified no breakage |
| `GRAPH_EDGE_RETYPE` | Change the composition pattern of an existing edge | Updated edge type |
| `GRAPH_REWIRE` | Remove an ARU from a chain and reconnect neighbors | Updated subgraph |

### Cross-Cutting Operations

| Type | Meaning | Output |
|---|---|---|
| `MANIFEST_UPDATE` | Update non-contract manifest fields (stability, temporal contracts) | Updated manifest |
| `MIGRATION_CREATION` | Create a migration ARU bridging two contract versions | New migration ARU |
| `TEST_ADDITION` | Add test scenarios for an ARU's test contract | Updated test contract |
| `REFERENCE_SET_UPDATE` | Add/update an example in the reference set | Updated reference example |

---

## Decomposition Rules

### Rule 1: L0 subtasks always come first

Type additions and state machine changes must be complete before any ARU can be created that uses the new types.

```
depends_on chain: TYPE_ADDITION → ARU_CREATION (L1) → ARU_CREATION (L2) → GRAPH_EDGE_ADD
```

### Rule 2: Layer ordering within ARU creation

An ARU at layer N can only be created after all its L0–(N-1) dependencies exist:

```
VALID:   TYPE_ADDITION → ARU_CREATION(L1) → ARU_CREATION(L2)
INVALID: ARU_CREATION(L2) before ARU_CREATION(L1) it depends on
```

### Rule 3: Graph edges come after both endpoint ARUs exist

```
VALID:   ARU_CREATION(A) + ARU_CREATION(B) → GRAPH_EDGE_ADD(A→B)
INVALID: GRAPH_EDGE_ADD before either endpoint is created
```

### Rule 4: MIGRATION_CREATION requires both versions to exist

```
VALID:   ARU_VERSION(v2) created → MIGRATION_CREATION(v1→v2)
INVALID: MIGRATION_CREATION before v2 exists
```

### Rule 5: Parallelizable subtasks are explicitly tagged

Any subtask with no `depends_on` entries, or whose dependencies are all resolved, is **parallelizable**. Multiple AI agents can execute parallelizable subtasks simultaneously without coordination.

---

## The Task Decomposition Object

```yaml
task:
  id: "feat/oauth-support"
  description: "Add OAuth 2.0 login support to the auth domain"
  requested_by: "human"
  target_layer_range: [L0, L3]
  estimated_subtasks: 9

  subtasks:

    - id: "t01"
      type: TYPE_ADDITION
      description: "Add OAuthProvider, OAuthCallbackCode, OAuthAccessToken to L0 registry"
      layer: L0
      targets: ["auth.OAuthProvider", "auth.OAuthCallbackCode", "auth.OAuthAccessToken"]
      depends_on: []
      assignable_to: GENERATOR
      parallelizable: true   # no dependencies

    - id: "t02"
      type: TYPE_STATE_ADDITION
      description: "Add OAuthPending and OAuthVerified states to auth.Session state machine"
      layer: L0
      targets: ["auth.Session.stateMachine"]
      depends_on: ["t01"]
      assignable_to: GENERATOR
      parallelizable: false

    - id: "t03"
      type: ARU_CREATION
      description: "Create auth.oauth.validate.callbackCode — validates OAuth callback code format"
      layer: L1
      targets: ["auth.oauth.validate.callbackCode"]
      input_type: "auth.OAuthCallbackCode"
      output_type: "auth.ValidatedOAuthCode | auth.OAuthError.INVALID_CODE"
      depends_on: ["t01"]
      assignable_to: GENERATOR
      parallelizable: true   # only depends on t01 which completes before this starts

    - id: "t04"
      type: ARU_CREATION
      description: "Create auth.oauth.exchange.token — exchanges code for access token via provider"
      layer: L1
      targets: ["auth.oauth.exchange.token"]
      input_type: "auth.ValidatedOAuthCode × auth.OAuthProvider"
      output_type: "auth.OAuthAccessToken | auth.OAuthError.EXCHANGE_FAILED"
      side_effects: ["CALL_EXTERNAL"]
      depends_on: ["t03"]
      assignable_to: GENERATOR
      parallelizable: false

    - id: "t05"
      type: ARU_CREATION
      description: "Create auth.oauth.resolve.userIdentity — resolves user from access token"
      layer: L2
      targets: ["auth.oauth.resolve.userIdentity"]
      input_type: "auth.OAuthAccessToken × auth.OAuthProvider"
      output_type: "auth.OAuthUserIdentity | auth.OAuthError.IDENTITY_FETCH_FAILED"
      depends_on: ["t04"]
      assignable_to: GENERATOR
      parallelizable: false

    - id: "t06"
      type: ARU_CREATION
      description: "Create user.oauth.create.orLink — creates user or links OAuth to existing account"
      layer: L2
      targets: ["user.oauth.create.orLink"]
      input_type: "auth.OAuthUserIdentity"
      output_type: "user.UserDomainObject | user.UserError.CONFLICT"
      depends_on: ["t05"]
      assignable_to: GENERATOR
      parallelizable: false

    - id: "t07"
      type: ARU_CREATION
      description: "Create auth.oauth.execute.loginFlow — full OAuth login organism"
      layer: L3
      targets: ["auth.oauth.execute.loginFlow"]
      composition:
        pattern: PIPE
        chain: ["auth.oauth.validate.callbackCode", "auth.oauth.exchange.token",
                "auth.oauth.resolve.userIdentity", "user.oauth.create.orLink",
                "auth.session.create.fromOAuthUser"]
        error_handler: "auth.oauth.handleError.loginFlow"
      depends_on: ["t02", "t03", "t04", "t05", "t06"]
      assignable_to: GENERATOR
      parallelizable: false

    - id: "t08"
      type: ARU_CREATION
      description: "Create auth.oauth.handleError.loginFlow — error handler for OAuth flow"
      layer: L3
      targets: ["auth.oauth.handleError.loginFlow"]
      input_type: "RailError<auth.OAuthError.* | user.UserError.CONFLICT>"
      depends_on: ["t01", "t02"]
      assignable_to: GENERATOR
      parallelizable: true   # can be built in parallel with t03–t06

    - id: "t09"
      type: GRAPH_EDGE_ADD
      description: "Wire OAuth login flow into the auth.domain.expose.api surface"
      layer: L5
      targets: ["auth.domain → auth.oauth.execute.loginFlow"]
      pattern: PIPE
      depends_on: ["t07", "t08"]
      assignable_to: HUMAN    # L5 decisions are human-owned
      parallelizable: false
```

---

## The Dependency Graph of the Example

```
t01 (types) ──────────────────────────────────────────────────┐
     │                                                        │
     ├──▶ t02 (state machine) ──────────────────────────┐    │
     │                                                   │    │
     └──▶ t03 (L1 validate) ──▶ t04 (L1 exchange) ──▶  │    │
                                       │                 │    │
                                       ▼                 ▼    ▼
                                  t05 (L2 resolve) ──▶ t07 (L3 organism) ──▶ t09 (L5 wire)
                                       │                 ▲
                                       ▼                 │
                                  t06 (L2 create) ───────┘
                                  
     t08 (error handler) ──── parallel to t03-t06 ──────────▶ t07
```

**Parallelizable groups:**
- Group A: `t01` alone (no dependencies)
- Group B: `t02`, `t03`, `t08` simultaneously (after t01)
- Group C: `t04` (after t03)
- Group D: `t05`, `t06` (after t04)
- Group E: `t07` (after all of D + t02 + t08)
- Group F: `t09` (after t07) — human step

---

## Task Completeness Validation

Before any subtask is executed, the full decomposition is validated:

1. **Coverage check** — does the decomposition account for all new types, ARUs, and graph edges implied by the description?
2. **Dependency acyclicity** — is the subtask DAG acyclic?
3. **Layer ordering** — do all ARU_CREATION subtasks have their L0 dependencies declared?
4. **Type compatibility** — for each GRAPH_EDGE_ADD, are the input/output types compatible?
5. **Human touchpoints** — are all L0 and L5 subtasks flagged `assignable_to: HUMAN`?

A decomposition that fails any of these checks is returned to the Orchestrator agent with a structured error — it is not executed.

---

## Underspecified Task Detection

When a task is too broad to decompose unambiguously, the Orchestrator emits a `TASK_UNDERSPECIFIED` signal:

```yaml
task_validation_error:
  type: TASK_UNDERSPECIFIED
  description: "Add authentication to the app"
  reason: "No domain specified. 'Authentication' spans auth, user, session, billing domains."
  clarification_needed:
    - "Which domain is primary target?"
    - "Is this a new auth system or extension of existing?"
    - "Which auth methods are in scope (password, OAuth, MFA)?"
  suggested_scope_reduction:
    - "Add password-based login to auth domain" ← specific enough
    - "Add OAuth Google provider to existing auth domain" ← specific enough
```

The signal surfaces to the human task-assigner. Execution is blocked until the task is scoped.
