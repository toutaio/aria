# Naming Conventions — Semantic Addressing
### Supporting System of ARIA — making structure inferrable without reading

---

## The Principle of Semantic Density

In ARIA, names are not labels — they are **compressed contracts**. A well-formed name allows an AI to infer an ARU's layer, domain, verb class, and entity type without loading its manifest.

> Every token spent on ambiguous naming is a token wasted. Every token saved through predictable naming is a token available for reasoning.

---

## The Semantic Address Format

```
[domain].[subdomain].[verb].[entity]
```

All four segments are required for L1+ ARUs. L0 Primitives use only `[domain].[entity]`.

| Segment | Purpose | Examples |
|---|---|---|
| `domain` | Bounded context | `auth`, `user`, `billing`, `notification` |
| `subdomain` | Functional area within domain | `token`, `session`, `password`, `profile` |
| `verb` | Operation class (see below) | `validate`, `create`, `transform`, `emit` |
| `entity` | The data subject | `token`, `email`, `invoice`, `address` |

**Examples:**
- `auth.token.validate.signature` — L1 Atom
- `auth.session.create.fromToken` — L2 Molecule
- `user.registration.execute.workflow` — L3 Organism
- `billing.payment.orchestrate.pipeline` — L4 System
- `notification.domain.expose.api` — L5 Domain

---

## Verb Vocabulary by Layer

Verbs are not free-form. Each layer has a defined verb set. Using a verb from the wrong layer signals an architectural violation.

### L0 Verbs (types and constants — no verb, just entity)
```
[domain].[entity]
Examples: auth.TokenString, user.EmailAddress, billing.Money
```

### L1 Verbs (atomic operations)
```
validate    — checks and passes or rejects
transform   — converts shape without adding information
compute     — performs calculation, returns result
generate    — produces new value (UUID, token, timestamp)
encode      — serializes to transmission format
decode      — deserializes from transmission format
hash        — one-way transformation
compare     — returns boolean or order
```

### L2 Verbs (composition)
```
create      — assembles a new domain entity
build       — constructs a data structure
prepare     — readies data for a downstream process
assemble    — combines multiple parts into one unit
resolve     — looks up or derives a value
```

### L3 Verbs (business logic)
```
execute     — runs a workflow or use case
apply       — enforces a policy or rule
process     — handles a domain event or command
enforce     — applies invariants and constraints
emit        — produces domain events
authorize   — makes access control decision
```

### L4 Verbs (orchestration)
```
orchestrate — coordinates multiple organisms
coordinate  — manages sequencing and timing
pipeline    — chains a series of steps
route       — dispatches to subsystems
```

### L5 Verbs (domain surface)
```
expose      — makes functionality available externally
integrate   — connects to external systems
guard       — enforces domain boundary rules
translate   — converts between domain models (anti-corruption)
```

---

## Inferring Layer from Name

Because verb vocabularies don't overlap between layers, an AI can determine an ARU's layer from its name alone:

```
auth.token.validate.signature  →  verb: "validate" → L1 Atom
user.profile.create.record     →  verb: "create"   → L2 Molecule  
payment.fraud.enforce.policy   →  verb: "enforce"  → L3 Organism
billing.checkout.orchestrate   →  verb: "orchestrate" → L4 System
```

This means AI can **build a mental model of the system structure** from a list of ARU names alone — before reading a single manifest.

---

## Naming Rules for Types (L0)

Types use PascalCase and encode their domain and constraints:

```
{Domain}{Qualifier}{Entity}

Examples:
  EmailAddress        (not just "Email" — ambiguous)
  HashedPassword      (not "Password" — ambiguous at what stage)
  ValidatedToken      (not "Token" — state is encoded in name)
  PositiveInteger     (not "int" — constraint is in the name)
  NonEmptyString      (constraint declared in name)
  ISO8601Timestamp    (format declared in name)
```

**Anti-patterns:**
```
❌  Data        (no domain, no entity)
❌  Result      (no information about what result)
❌  User        (is this raw input, validated, persisted, projected?)
✓   UserDomainObject
✓   UserCreateRequest
✓   UserPersistenceRecord
```

---

## Naming Rules for Errors

Errors are typed, namespaced, and encode the domain and failure mode:

```
{Domain}Error.{FailureMode}

Examples:
  AuthError.EXPIRED
  AuthError.INVALID_SIGNATURE
  UserError.NOT_FOUND
  BillingError.INSUFFICIENT_FUNDS
  ValidationError.FORMAT_MISMATCH
  ValidationError.CONSTRAINT_VIOLATED
```

Never use generic error types like `Error`, `Exception`, or `string`. AI cannot reason about what went wrong from a generic type.

---

## File and Directory Structure

The naming convention extends to the filesystem. Directory structure mirrors the semantic address:

```
src/
  auth/
    token/
      validate/
        signature.{ext}         ← auth.token.validate.signature (ARU)
        signature.manifest.yaml ← manifest
        signature.test.{ext}    ← test contract
      session/
        create/
          fromToken.{ext}       ← auth.session.create.fromToken (ARU)
  user/
    profile/
      create/
        record.{ext}            ← user.profile.create.record (ARU)
```

Given a semantic address, the file path is deterministic. Given a file path, the semantic address is deterministic. No lookup required.

---

## Naming Enforcement

Naming is not a convention — it is a constraint enforced at build time:

1. **Verb validation**: verb must exist in the layer's allowed verb set
2. **Layer consistency**: declared layer in manifest must match inferred layer from verb
3. **Address uniqueness**: no two ARUs may share an address
4. **Type name encoding**: type names must encode domain + state/constraint
5. **Error namespacing**: error types must be domain-prefixed

Violations are build failures, not warnings. Names are part of the architecture contract.
