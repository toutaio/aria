# Composition Patterns
### Pillar 3 of ARIA — HOW pieces connect to each other

---

## Overview

Composition Patterns are the **typed edges** between ARUs in the Semantic Graph. They define not just *that* two ARUs are connected, but *how* — the nature of the relationship, the data flow direction, and the behavioral contract of the connection itself.

Every connection between two ARUs must be declared as one of these patterns. Undeclared connections are architectural defects.

> Patterns are **first-class citizens** in ARIA. A pattern instance is itself an ARU at the composition layer.

---

## The 10 Composition Patterns

### 1. PIPE `A → B`
Linear transformation. Output of A is the input of B.

```
[A: X → Y] ──PIPE──▶ [B: Y → Z]
Result type: X → Z
```
- Most common pattern
- Chain length is unlimited but each link must be type-compatible
- AI reads a PIPE chain as a single narrative: "X becomes Z via steps"

---

### 2. FORK `A → [B, C, ...]`
Fan-out. Output of A is passed to multiple ARUs independently.

```
              ┌──▶ [B: Y → Z1]
[A: X → Y] ──┤
              └──▶ [C: Y → Z2]
```
- B and C receive the same value; neither knows about the other
- Results are independent (no shared state)
- Used for parallel processing, event broadcasting

---

### 3. JOIN `[A, B, ...] → C`
Fan-in. Multiple ARU outputs are combined into one input.

```
[A: X1 → Y1] ──┐
               MERGE ──▶ [C: (Y1, Y2) → Z]
[B: X2 → Y2] ──┘
```
- C's input type must match the merged output shape
- The merge type is declared (**tuple or struct — never union, never implicit**). A union merge would make C's logic dependent on which branch fired — that is a ROUTE, not a JOIN.
- Used for aggregation, combining results

---

### 4. GATE `A → B | ∅`
Conditional pass. Output of A flows to B only if a predicate is true; otherwise nothing.

```
[A: X → Y] ──[predicate(Y)]──▶ [B: Y → Z] | dropped
```
- The dropped path must be explicitly handled upstream
- Predicate is a declared L1 Atom, not inline logic
- Used for filtering pipelines, conditional execution

---

### 5. ROUTE `A → B | C`
Conditional branch. Output of A flows to exactly one of B or C based on a predicate.

```
                  ┌──[true]──▶ [B: Y → Z1]
[A: X → Y] ──────┤
                  └──[false]─▶ [C: Y → Z2]
```
- Unlike GATE, all paths must be handled (no drops)
- The routing predicate is a declared Atom
- Result type is `Z1 | Z2` (union)

---

### 6. LOOP `A →[condition]→ A`
Bounded iteration. Output of A feeds back into A until a condition is met.

```
[A: X → X] ──[while condition(X)]──▶ (loop)
                                 ──▶ [done: X → Y]
```
- **Must** declare a termination condition and maximum iteration bound
- Without a bound declaration, the ARU is malformed
- Used for retry logic, convergence algorithms, iterative refinement

---

### 7. OBSERVE `A → (A, Event)`
Non-mutating side channel. A processes its input normally AND emits an event, without affecting the main data flow.

```
[A: X → Y] ──────────────────────────────▶ Y (main flow)
             └──[event: EventSchema]──▶ EventBus
```
- The main output Y is unchanged by the observation
- Events are typed schemas, never raw strings
- Used for logging, auditing, telemetry, reactive triggers

---

### 8. TRANSFORM `A → A'`
Shape change within the same semantic domain. Input and output represent the same concept in different representations.

```
[A: User_DB_Record → User_Domain_Object]
[A: Celsius → Fahrenheit]
[A: JSON_Payload → TypedRequest]
```
- Semantically equivalent; structurally different
- Distinguished from PIPE because no *new* information is created
- Critical for anti-corruption layers at L5 Domain boundaries

---

### 9. VALIDATE `A → A' | Error`
Contract enforcement. A validates its input and either produces a refined output type or a typed error.

```
[A: X → X' | ValidationError]
```
- On success, output is **either identical to input or a narrowed subtype** — `ValidatedEmail` from `NonEmptyString` is valid narrowing
- Output is always `success_type | ErrorType` — never throws
- `X'` must be a subtype of `X` on success paths (narrowing is intentional and expected)
- Validators are L1 Atoms (the simplest possible ARUs)
- Can be composed in chains: `VALIDATE(format) → VALIDATE(range) → VALIDATE(business_rule)`

---

### 10. CACHE `A → A` *(with memoization)*
Transparent memoization layer. Identical inputs return stored outputs without re-executing A.

```
[cache_key(X)] ──[hit]──▶ stored_Y
[A: X → Y]    ──[miss]──▶ Y ──▶ store(X, Y)
```
- A's interface is unchanged (consumers are unaware of caching)
- Cache invalidation strategy is declared in the CACHE ARU manifest
- Only valid when A is declared `deterministic: true` and `side_effects: NONE`

---

## Distributed and Async Patterns

The 10 core patterns cover synchronous, in-process computation. Distributed and event-driven systems require four additional patterns. These are valid only at L3 (Organism) and above — atoms and molecules are always synchronous.

---

### 11. STREAM `A → B*`
Lazy or infinite sequence processing. A produces elements one at a time; B processes each as it arrives.

```
[A: Source → Element*] ──STREAM──▶ [B: Element → Result*]
```
- A and B are decoupled in time — B does not wait for A to finish
- **Backpressure** must be declared: what B does when it cannot keep up with A
  - `DROP`: discard excess elements (lossy)
  - `BUFFER(n)`: buffer up to n elements, then apply backpressure to A
  - `ERROR`: emit error when buffer is exceeded
- Maximum element count or termination condition must be declared (no unbounded streams without explicit declaration)
- Used for: log processing, event streams, file parsing, real-time pipelines

---

### 12. SAGA `[A → B → C] with [C⁻¹ → B⁻¹ → A⁻¹]`
Distributed transaction with typed compensation. Each step has a corresponding compensating action that is called on failure.

```
Forward:    [A] ──▶ [B] ──▶ [C]    (on success)
Compensate: [C⁻¹] ──▶ [B⁻¹] ──▶ [A⁻¹]  (on any failure — in reverse order)
```
- Each step ARU **must** have a declared `compensating_aru` in its manifest
- Compensation is always called in strict reverse order
- Compensation ARUs must be idempotent (they may be called more than once in retry scenarios)
- Used for: payment flows, multi-service writes, distributed state changes

```yaml
# In forward ARU manifest:
saga_participant:
  compensating_aru: "billing.charge.compensate.reverse"
  idempotency_key_field: "transactionId"
```

---

### 13. CIRCUIT_BREAKER `A → B (with state)`
Stateful failure detection. Unlike GATE (stateless predicate), the CIRCUIT_BREAKER accumulates failure history and opens the circuit when a threshold is exceeded.

```
State: CLOSED (normal) → OPEN (failing fast) → HALF_OPEN (probing) → CLOSED
```
- Wraps any ARU that calls an external system
- Failure threshold and evaluation window declared in `behavioral_contract`
- In OPEN state: returns `CircuitOpenError` immediately without calling the wrapped ARU
- In HALF_OPEN state: allows one probe call; success closes, failure re-opens
- Used for: external API calls, database calls, any I/O-bound ARU

The CIRCUIT_BREAKER is the composition system's enforcement of the `circuit_breaker` field in the ARU's `behavioral_contract`.

---

### 14. PARALLEL_JOIN `[A, B, C] → D (with timeout)`
Fan-out with coordinated collection and timeout. Unlike JOIN (which waits indefinitely), PARALLEL_JOIN collects results within a time budget.

```
              ┌──▶ [A: X → Y1]  ──(result or timeout)──┐
[Source] ─────┼──▶ [B: X → Y2]  ──(result or timeout)──┼──▶ [D: PartialResults → Z]
              └──▶ [C: X → Y3]  ──(result or timeout)──┘
```
- `timeout` is declared in the composition (not individual ARUs)
- `minimum_required_results` declares how many branches must succeed for D to proceed
- Branches that time out contribute `TimedOut` to the result union
- D's input type must handle `Y1 | TimedOut`, `Y2 | TimedOut`, etc.
- Used for: scatter-gather patterns, optional enrichment, non-critical data aggregation

---

## Updated Pattern Composition Matrix

| Composite | Built From | Common Use |
|---|---|---|
| Resilient call | `CIRCUIT_BREAKER → PIPE → LOOP(retry)` | External API call with retry and circuit breaking |
| Distributed transaction | `SAGA([PIPE chains])` | Multi-service writes with rollback |
| Audit pipeline | `PIPE → OBSERVE → PIPE` | Any mutation with audit log |
| Filtered broadcast | `FORK → [GATE, GATE, GATE]` | Event dispatch with routing |
| Safe transform | `VALIDATE → TRANSFORM → VALIDATE` | Input normalization |
| Cached computation | `VALIDATE → CACHE(PIPE)` | Expensive pure computations |
| Optional enrichment | `PARALLEL_JOIN(timeout=50ms, min=1)` | Enrich response with non-critical data |
| Event-driven pipeline | `STREAM → GATE → TRANSFORM → OBSERVE` | Real-time processing with filtering |

---

## Why Patterns Are Explicit

In traditional architecture, connections between components are implicit — you read the code to discover them. In ARIA, connections are **declared before implementation**.

This means:
- AI can understand system topology from manifests alone (no code reading)
- AI generating a new ARU knows exactly which patterns to use from the context
- Pattern violations are detectable statically (wrong type signatures, missing error handling)
- Refactoring is a graph operation: swap a node, keep the edges, verify type compatibility

The pattern declaration IS the design. Implementation is the execution of the design.

---

## Influences

The pattern vocabulary draws directly from **Enterprise Integration Patterns** (Gregor Hohpe & Bobby Woolf, Addison-Wesley, 2003), which catalogued messaging and integration topologies (pipes, filters, routers, aggregators). ARIA adapts their naming (PIPE, ROUTE, FORK/JOIN) and adds patterns for distributed systems (SAGA, CIRCUIT_BREAKER, PARALLEL_JOIN) drawn from reactive and microservices literature. The critical ARIA addition is that every pattern instance is a **typed, declared edge** in the semantic graph — not an implementation detail discovered by reading code.
