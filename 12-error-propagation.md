# Error Propagation — Railway-Oriented Model
### Third Iteration — Formal failure semantics for all compositions

---

## The Gap

The composition patterns in `03-composition-patterns.md` define what happens when everything succeeds. They are silent on failure. In a `PIPE` chain of five ARUs, if ARU #3 returns an error, do ARUs #4 and #5 execute? Who catches the error? Where is it handled?

Without a formal answer, every AI generating a composition makes a different choice. The result is inconsistent error handling across the codebase — the exact kind of ambiguity ARIA exists to eliminate.

---

## The Railway Model

ARIA adopts **railway-oriented programming** as its formal error propagation model. The metaphor is precise:

```
                    SUCCESS RAIL
  ┌────┐   ┌────┐   ┌────┐   ┌────┐   ┌────────┐
  │ A  │──▶│ B  │──▶│ C  │──▶│ D  │──▶│ Result │
  └────┘   └────┘   └────┘   └────┘   └────────┘
     │         │        │        │
     │         │        │        │       FAILURE RAIL
     └─────────┴────────┴────────┴──▶ [ ErrorHandler ]
```

Every ARU in a PIPE chain has two output tracks:
- **Success rail**: the normal output flows to the next ARU
- **Failure rail**: any error bypasses all remaining ARUs and flows directly to a declared error handler

No ARU in the middle of a chain ever receives an error as input. Each ARU only ever sees valid, typed, success-path data.

---

## Formal Definition

### The Two-Track Type

Every ARU in a PIPE chain implicitly operates on a **two-track type**:

```
TwoTrack<T, E> =
  | { track: 'SUCCESS', value: T }
  | { track: 'FAILURE', error: E }
```

This is identical to `Result<T, E>` from `10-algebraic-types.md`. The railway model IS the `Result` type made architectural.

An ARU that is `validate` or `transform` switches between tracks:
- **Track switch (success → failure)**: ARU returns an Error — the value leaves the success rail
- **Track switch (failure → success)**: A `RECOVER` operation (see below) — the error is handled and flow resumes

### Implicit vs. Explicit Railway

ARUs don't need to know they're in a railway. Their contract is:

```
input:  T           (always a success-path value — never receives an error)
output: T' | Error  (may succeed or fail)
```

The composition system (not the ARU) is responsible for routing errors to the failure rail. The ARU is unaware of its position in the chain.

---

## Railway Composition Rules

### Rule 1: PIPE short-circuits on failure

```
A → B → C → D

If B returns Error:
  ✓ Error goes to failure rail immediately
  ✗ C and D are NOT called
  ✓ Failure rail delivers Error to the chain's ErrorHandler
```

### Rule 2: Every PIPE composition declares an ErrorHandler

No PIPE chain is architecturally complete without a declared error handler. The handler is an ARU:

```yaml
composition:
  pattern: PIPE
  chain: [auth.token.validate, auth.session.create, user.profile.load]
  error_handler: auth.pipeline.handleError   ← required, not optional
```

An undeclared error handler is a **build failure**. There is no such thing as "errors are the caller's problem" — the caller IS the composition, and the composition must declare its handler.

### Rule 3: Error types accumulate through the chain

The failure rail carries a union of all possible error types from all ARUs in the chain:

```
Chain: [A: X→Y | ErrA] → [B: Y→Z | ErrB] → [C: Z→W | ErrC]
Failure rail type: ErrA | ErrB | ErrC
ErrorHandler input: ErrA | ErrB | ErrC
```

The error handler must be exhaustive — it must handle all variants. Unhandled error variants are a build failure (same exhaustiveness rule as in `10-algebraic-types.md`).

### Rule 4: Errors carry provenance

Every error on the failure rail is wrapped with provenance metadata:

```
RailError<E> = {
  error:      E                    ← the original typed error
  origin_aru: ARU_id               ← which ARU produced it
  trace_id:   CorrelationId        ← from the observability system
  timestamp:  ISO8601Timestamp
}
```

The AI generating an error handler receives a `RailError<ErrA | ErrB | ErrC>` — it always knows where the error came from without inspecting the error payload.

---

## The RECOVER Operation

A `RECOVER` operation switches from the failure rail back to the success rail. It is a special ARU:

```
RECOVER: RailError<E> → T | FatalError
```

It takes an error and either:
- Produces a valid success-path value (error is handled, flow continues)
- Produces a `FatalError` (error cannot be recovered, propagates up)

```
Example: retry on timeout

A → B → C
         │ (C times out)
         ▼ failure rail
    RECOVER(auth.retry.onTimeout)
         │ (retries C, produces success value)
         ▼ success rail (re-enters at C)
```

### RECOVER placement rules

RECOVER can only be placed:
1. At the end of a chain (terminal handler)
2. Immediately after the ARU it is recovering from (inline recovery)

It cannot appear in the middle of a chain recovering from an upstream ARU — that would create invisible control flow that is impossible for AI to reason about.

---

## Pattern-Specific Railway Semantics

Each composition pattern has defined failure semantics:

### FORK on failure
```
If A fails:        failure rail fires before any fork target is called
If B fails:        failure rail fires; C may or may not have been called
                   (non-deterministic — FORK targets are independent)
```
FORK targets that have been called are NOT automatically rolled back. If rollback is needed, use SAGA (see `patterns-async`).

### JOIN on failure
```
If A fails:        failure rail fires; B is NOT called (short-circuit)
If B fails:        A has already completed; failure rail fires
                   A's result is discarded (no rollback unless SAGA)
```

### LOOP on failure
```
Failure inside loop body:   failure rail fires; loop exits immediately
                             (no retry — use RECOVER inside the loop body for retry logic)
```

### GATE on failure (input ARU fails)
```
If the GATE predicate ARU fails:   failure rail fires
If A fails upstream of GATE:       failure rail fires before GATE is evaluated
```

---

## Nested Pipelines and Error Propagation

When a composition is nested inside another (a PIPE chain inside an ORGANISM inside a SYSTEM), failure rails compose:

```
System Pipeline:
  [OrganismA] ──PIPE──▶ [OrganismB] ──PIPE──▶ [OrganismC]
       │                     │                     │
  error_handler:          error_handler:        error_handler:
  organism_A.handler      organism_B.handler    system_level.handler
```

Each layer handles the errors it can recover from locally. Errors that cannot be recovered are **escalated** up the railway to the next layer's handler.

The escalation path is:
```
Atom-level error → Molecule handler (if declared)
                 → Organism handler (if not handled at molecule)
                 → System handler  (if not handled at organism)
                 → Domain handler  (if not handled at system)
                 → UNHANDLED (hard build failure — every error must be handled somewhere)
```

---

## What AI Must Generate for Every PIPE Composition

With the railway model defined, the AI's code generation task for a PIPE composition is fully specified:

1. **Chain** — the sequence of ARU calls on the success rail
2. **Error union** — the union of all error types across the chain
3. **Error handler** — an ARU that exhaustively handles the error union
4. **RECOVER points** (if any) — inline recovery for specific, known-recoverable errors
5. **Provenance wrapping** — `RailError<E>` wrapping at each failure point

No free-form error handling. No try/catch scattered through the code. No silent swallowing. The railway model makes the shape of every composition's error handling as predictable as the shape of its success path.
