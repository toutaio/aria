# Atomic Responsibility Units (ARU)
### Pillar 2 of ARIA — WHAT each piece is and does

---

## Definition

An **Atomic Responsibility Unit (ARU)** is the fundamental building block of ARIA. Every piece of code in the system is an ARU. There is no code that exists outside an ARU.

> An ARU has **one reason to exist**, **one typed input**, **one typed output**, **one layer**, and **one manifest**.

The "lego block" metaphor is precise: every ARU has the same structural skeleton — only the semantics differ. This uniformity is what allows AI to work with any ARU using the same reasoning process.

---

## ARU Anatomy

```
┌──────────────────────────────────────────────────────┐
│                        ARU                           │
│                                                      │
│  ┌─────────┐    ┌─────────────────┐    ┌─────────┐   │
│  │  INPUT  │───▶│  RESPONSIBILITY │───▶│ OUTPUT  │   │
│  │ (typed) │    │   (single)      │    │ (typed) │   │
│  └─────────┘    └─────────────────┘    └─────────┘   │
│                         │                            │
│                   ┌─────▼──────┐                     │
│                   │  MANIFEST  │                     │
│                   └────────────┘                     │
└──────────────────────────────────────────────────────┘
```

---

## ARU Structural Specification

Every ARU consists of exactly these components:

### 1. Identity
```
id:      [domain].[subdomain].[verb].[entity]    # semantic address
layer:   L0 | L1 | L2 | L3 | L4 | L5
version: semver
```

### 2. Contract
```
input:          TypedSchema | NONE
output:         TypedSchema | ErrorUnion
preconditions:  [ logical assertions on input ]
postconditions: [ logical assertions on output ]
invariants:     [ conditions that must hold before and after ]
```

### 3. Effect Declaration
```
side_effects:   NONE | [ READ_DB, WRITE_DB, EMIT_EVENT, CALL_EXTERNAL, ... ]
idempotent:     true | false
deterministic:  true | false
```

### 4. Dependencies
```
depends_on: [ ARU_id, ... ]   # only from same layer or below
```

### 5. Manifest (see Context Manifests doc)
```
context_budget: { use, modify, extend }
test_contract:  [ scenario descriptions ]
```

---

## The Single Responsibility Enforcement

A well-formed ARU passes the **One Question Test**:

> *Can this ARU be described in a single sentence of the form:*
> **"Given [input], it [single verb] and returns [output]"**
> *without the word "and" appearing in the verb phrase?*

**PASS:** `"Given a raw password string, it hashes it and returns a Hash"` ✓
**FAIL:** `"Given user data, it validates and hashes and stores and returns a User"` ✗ → split into 4 ARUs

---

## ARU Types by Layer

| Layer | ARU Type | Allowed Verbs | Dependencies |
|---|---|---|---|
| L0 | Primitive | define, represent | None (self-contained) |
| L1 | Atom | transform, validate, generate, compute | L0 only |
| L2 | Molecule | compose, assemble, build, prepare | L0–L1 (prefer L1) |
| L3 | Organism | execute, apply, process, enforce | L0–L2 (prefer L2) |
| L4 | System | orchestrate, coordinate, pipeline | L0–L3 (prefer L3) |
| L5 | Domain | expose, integrate, guard, translate | L0–L4 (prefer L4) |

The verb constraint is intentional: it encodes the *kind* of work each layer does, preventing organisms from "validating" (L1 work) or atoms from "orchestrating" (L4 work).

---

## ARU Immutability Contract

Once an ARU's **contract** is published (input type, output type, postconditions), it is **frozen**. It may never change in a breaking way without a version increment.

This means:
- AI can **cache** its understanding of an ARU contract
- AI can **trust** that a contract it read last session is still valid
- Breaking changes are surfaced explicitly, not discovered by reading code

This is the equivalent of interface stability in traditional software — except it is enforced structurally, not culturally.

---

## ARU vs. Traditional Code Unit

| Aspect | Traditional Function/Class | ARU |
|---|---|---|
| Responsibility | Often multiple | Exactly one |
| Interface | Often implicit | Always explicit and typed |
| Layer | Informal or undefined | Formally declared |
| Side effects | Implicit | Explicitly declared |
| Documentation | Prose comments | Machine-readable manifest |
| Dependencies | Inferred from imports | Declared in manifest |
| AI context cost | Unknown | Known (context budget) |

---

## Composing ARUs

ARUs are designed to be composed. The **Composition Patterns** (see `03-composition-patterns.md`) define the legal ways two ARUs can be connected. Key rule:

> **ARUs communicate only through their typed contracts.**
> An ARU never reaches into another ARU's internals.

This is not just encapsulation — it is the mechanism that allows AI to swap, replace, or generate ARUs without reading their implementations.

---

## The Test Contract

Every ARU includes behavioral specifications in its manifest, expressed as scenarios (not implementation tests):

```
test_contract:
  - scenario: "valid input produces expected output shape"
  - scenario: "invalid input produces typed error, not exception"
  - scenario: "side effects are exactly those declared, no more"
  - scenario: "postconditions hold for all valid inputs"
```

These scenarios are the behavioral contract made executable. An AI generating an ARU implementation writes code until all scenarios pass — the scenarios are the definition of "done."

---

## Influences

The *atomic* metaphor — composing large systems from small, single-responsibility units — is shared with **Atomic Design** (Brad Frost, 2013) and the **Single Responsibility Principle** from *Clean Architecture* (Robert C. Martin, 2017). The key ARIA departure is that "atomic" is formalized as a machine-verifiable manifest contract rather than a design guideline, making it enforceable by tooling rather than code review.
