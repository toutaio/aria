# Type Compatibility
### Second Iteration — Rules for connecting ARUs across the Semantic Graph

---

## The Connection Problem

The Semantic Graph has typed edges. An edge between ARU A and ARU B is only valid if the types are compatible. "Compatible" has a precise, formal definition in ARIA — not a fuzzy, runtime-discovered one.

Type compatibility is checked at **graph build time**, before any AI generates implementation code. By the time an AI writes code, it is guaranteed that the composition it is implementing is type-correct.

---

## The Type Compatibility Hierarchy

ARIA uses **nominal subtyping with structural projection**:

- Two types are **identical** if they have the same name (same brand, same state stage)
- One type is a **subtype** of another if the subtype carries all the guarantees of the supertype plus additional ones
- Types are **compatible** if one is a subtype of (or identical to) the other

```
Type Compatibility: A ≤ B means "A is compatible with B as input"

ValidatedEmail ≤ RawEmailString      ✓  (validated implies raw format is correct)
RawEmailString ≤ ValidatedEmail      ✗  (raw does NOT imply validated)
HashedPassword ≤ RawPassword         ✗  (hashed cannot be used as raw)
```

---

## Compatibility Rules by Composition Pattern

### PIPE: `A → B`
```
Compatible iff:  A.output_type ≤ B.input_type

Example:
  A: (RawPassword) → HashedPassword
  B: (HashedPassword) → StorageRecord
  
  A.output = HashedPassword
  B.input  = HashedPassword
  HashedPassword ≤ HashedPassword  ✓  PIPE is valid
  
Incompatible example:
  A: (RawInput) → RawPassword
  B: (HashedPassword) → StorageRecord
  
  A.output = RawPassword
  B.input  = HashedPassword
  RawPassword ≤ HashedPassword  ✗  PIPE is invalid — missing hash step
```

### VALIDATE: `A → A | Error`
```
Compatible iff:  A.output_success_type ≤ downstream.input_type
                 downstream.input_type = A.input_type
                 (Validate does not change the type on success)
                 
Special case: Validate CAN narrow the type:
  Input:  NonEmptyString
  Output: ValidatedEmail | ValidationError
  
  ValidatedEmail is a NARROWER type than NonEmptyString — this is valid narrowing.
  The output type must be a subtype of the input type on success paths.
```

### FORK: `A → [B, C]`
```
Compatible iff:  A.output_type ≤ B.input_type
            AND  A.output_type ≤ C.input_type
            (same output must be compatible with ALL fork targets)
```

### JOIN: `[A, B] → C`
```
Compatible iff:  C.input_type is a product type containing A.output_type and B.output_type
                 (C must explicitly declare that it takes both)
                 
Example:
  A: () → UserId
  B: () → ValidatedProfileData
  C: (UserId × ValidatedProfileData) → UserProfile | UserError
  
  JOIN([A, B] → C) is valid because C.input = UserId × ValidatedProfileData
```

### ROUTE: `A → B | C`
```
Compatible iff:  A.output_type ≤ B.input_type
            AND  A.output_type ≤ C.input_type
            (predicate selects, not transforms — both branches must accept same type)
```

### GATE: `A → B | ∅`
```
Compatible iff:  A.output_type ≤ B.input_type
            AND  the ∅ (drop) case is explicitly handled by the caller
```

### TRANSFORM: `A → A'`
```
Compatible iff:  A.output_type and A'.input_type are semantically equivalent
                 but structurally different representations of the same domain entity
                 
Example:
  A: UserDomainObject → UserApiResponse
  
  UserDomainObject and UserApiResponse represent the same user in different shapes.
  This is valid TRANSFORM.
  
Invalid:
  A: UserDomainObject → InvoiceSummary
  
  These are different domain entities — this is not a transform, it's wrong.
```

### LOOP: `A →[condition]→ A`
```
Compatible iff:  A.output_type ≤ A.input_type
                 (loop feeds back into itself — must be type-stable)
            AND  termination_condition type is declared
            AND  max_iterations is declared in manifest
```

### CACHE: `CACHE(A)`
```
Compatible iff:  A is declared deterministic: true
            AND  A is declared side_effects: NONE
            AND  A.input_type implements CacheKeyable interface (hashable)
```

### OBSERVE: `A → (A, Event)`
```
Compatible iff:  A.output_type passes through unchanged
            AND  Event is a typed L0 event schema (not raw string/any)
            AND  event bus accepts Event.type
```

---

## Type Widening and Narrowing

### Narrowing (valid, desirable)
Moving from a less specific type to a more specific type through validation:
```
string → NonEmptyString → TrimmedString → EmailAddress → ValidatedEmail
         (each step adds guarantees)
```
AI should always prefer the most specific type available for any given operation.

### Widening (usually a defect)
Moving from a specific type to a less specific type:
```
ValidatedEmail → string    ← loses validation guarantee
HashedPassword → string    ← loses hash guarantee (security risk)
```

**Widening is only allowed at domain boundaries (L5)** when communicating with external systems that don't understand domain types. All widening must go through a declared TRANSFORM ARU — never implicitly.

---

## Type Compatibility Matrix

A quick reference for what is always compatible, sometimes compatible, and never compatible:

| From \ To | Primitive | Branded | TypeState (early) | TypeState (later) |
|---|---|---|---|---|
| Primitive | ✓ | ✗ | ✗ | ✗ |
| Branded | ✗ | ✓ if same brand | ✗ | ✗ |
| TypeState (early) | ✗ | ✗ | ✓ if same/earlier | ✗ |
| TypeState (later) | ✗ | ✗ | ✓ (narrows) | ✓ if same stage |

No row/column is all ✓ except identity (same type). Every connection across types must be an explicit ARU — no implicit coercion, no casting.

---

## Compatibility Errors and How AI Resolves Them

When the graph validator detects a type incompatibility, it produces a structured error that AI can act on:

```yaml
compatibility_error:
  edge: "A(auth.password.validate) → B(user.credentials.persist)"
  type_error: "TYPE_MISMATCH"
  
  A.output: "ValidatedRawPassword"
  B.input:  "HashedPassword"
  
  diagnosis: "A produces ValidatedRawPassword but B requires HashedPassword.
               A hash step is missing between validation and persistence."
  
  suggested_resolution:
    insert_between: "auth.password.hash.bcrypt"
    pattern: "PIPE"
    new_chain: "validate → PIPE → hash → PIPE → persist"
```

The structured error gives the AI a complete diagnosis and a suggested fix. The AI doesn't need to reason from scratch — it receives a type-checked work order.

---

## The Type Checker as AI Co-Pilot

The type compatibility system acts as a **real-time feedback loop** for AI code generation:

1. AI declares a composition in the manifest (before writing code)
2. Type checker validates all edges
3. Errors are returned as structured YAML (not raw text)
4. AI adjusts the composition and re-validates
5. Once the composition is type-valid, AI writes the implementation

This **declaration-first loop** means that by the time an AI writes implementation code, the architecture is already verified correct. Implementation becomes mechanical — fill in the declared contract.

The type system is not a constraint on AI creativity. It is the **substrate on which AI creativity operates** without risk of structural errors.
