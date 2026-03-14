# Domain Decomposition Protocol
### Sixth Iteration — How to identify and bound L5 domains

---

## Influence: Domain-Driven Design

The concepts of **bounded context**, **ubiquitous language**, **context mapping**, and **anti-corruption layer** in this document originate from **Domain-Driven Design** (Eric Evans, Addison-Wesley, 2003) and its community extensions in *Implementing Domain-Driven Design* (Vaughn Vernon, 2013). ARIA's L5 Domain maps one-to-one to a DDD bounded context. The key adaptation: ARIA makes domain boundaries machine-readable (declared in manifests and enforced by `aria-build check`) rather than implicit agreements between teams.

---

## The Foundational Assumption Problem

Every ARIA document from 01 onward assumes that L5 domain boundaries exist and are correctly drawn. The layer ownership model says "humans define L5 domain boundaries." The semantic graph assumes bounded contexts. The semantic address format begins with `[domain]`.

But **how do you determine what the domains are?**

Domain decomposition is the highest-leverage, most error-prone decision in distributed system design. Wrong domain boundaries cascade into every downstream decision: naming, graph topology, cross-domain dependencies, ownership, and deployment. This document provides the protocol.

---

## What a Domain Is in ARIA

An ARIA domain (L5) is a **bounded context**: a region of the codebase where a consistent vocabulary, data model, and set of responsibilities is maintained.

Formal definition:
```
A domain D is a bounded context iff:
  1. D has a ubiquitous language — a set of terms with precise, agreed meanings within D
  2. D owns its type vocabulary — the L0 types within D are not shared with other domains
     (except through a declared Shared Kernel)
  3. D has a single clear ownership team or role
  4. D can be deployed, tested, and reasoned about independently
```

---

## Phase 1: Domain Identification

### The Linguistic Method

The most reliable technique for domain identification is **linguistic analysis of the problem space**:

```
Step 1: Collect domain language
  → Interview domain experts (or analyze requirement documents)
  → Extract nouns, verbs, and the relationships between them
  → Note where the same word means different things in different contexts — these are domain boundaries

Step 2: Find the natural vocabulary clusters
  → Group terms that are always used together
  → Terms that rarely appear together belong to different domains

Step 3: Identify where meaning changes at the boundary
  → "Account" in billing means an invoiceable entity
  → "Account" in auth means credentials and identity
  → Same word, different models → different domains
```

### The Event Storming Trigger List

Events are reliable domain indicators. Collect all domain events (things that happen), then cluster by:
- Who cares about this event?
- What state changes does this event drive?
- What vocabulary is used to describe this event?

Events that use the same vocabulary and drive the same state belong in the same domain.

### Red Flags That Indicate Poor Domain Boundaries

| Signal | What It Means | Remedy |
|---|---|---|
| A type is shared directly between domains | The boundary is wrong or a Shared Kernel is needed | Explicitly model the Shared Kernel or redraw the boundary |
| An ARU reaches across domain boundaries to L2/L3 | The dependency is illegally direct | Route through L5 EXPOSE/INTEGRATE ARU |
| Two domains always change together | They may be one domain split incorrectly | Merge, or introduce an explicit integration event |
| One team owns multiple disconnected domains | The ownership model is unclear | Assign clear ownership before refining boundaries |
| The same concept has different names in two domains | Natural — do NOT unify the names | Use an anti-corruption layer when the domains communicate |

---

## Phase 2: Boundary Rules

### What May Cross a Domain Boundary

Only typed contracts may cross a domain boundary. Direct type sharing is forbidden.

```
ALLOWED at domain boundary:
  ✓ L5 EXPOSE ARU — makes a service or event stream available to consumers
  ✓ L5 INTEGRATE ARU — consumes an external or inter-domain contract
  ✓ L5 TRANSLATE ARU — converts between domain models (anti-corruption layer)
  ✓ Event schemas (typed, versioned, published to a shared event bus)

FORBIDDEN at domain boundary:
  ✗ Sharing L0 types directly between domains (except Shared Kernel)
  ✗ L3 Organism in Domain A calling L2 Molecule in Domain B directly
  ✗ Sharing database tables across domains
  ✗ Sharing mutable state across domain boundaries
```

### Cross-Domain Dependency Resolution

When Domain A needs something from Domain B, the dependency resolves at **L5**, not at the
layer where the need arises:

```
Need: auth.session.execute.loginFlow (L3 in Auth domain) needs User profile data

WRONG:
  auth.session.execute.loginFlow  ──────▶  user.profile.load.byId  (direct L3→L2 cross-domain)

RIGHT:
  auth.session.execute.loginFlow
         │
         ▼
  auth.domain.integrate.userProfile    ← L5 INTEGRATE in Auth domain
         │
         ▼ (crosses boundary here — both sides are L5)
  user.domain.expose.profileQuery      ← L5 EXPOSE in User domain
         │
         ▼
  user.profile.load.byId              ← L2 Molecule in User domain
```

The graph edge between domains always connects two L5 ARUs. Layer skipping across domain
boundaries is not allowed (unlike within a domain where skipping is permitted with justification).

### The Shared Kernel

When two domains genuinely share a concept (not just use the same word), a **Shared Kernel**
is the formal mechanism:

```yaml
# Shared Kernel manifest
shared_kernel:
  id: "shared.identity"
  owned_by: [auth, user, notification]   # ALL owning domains must approve changes
  types:
    - UserId                             # L0 type shared across all three domains
    - EmailAddress                       # L0 type shared across all three domains
  change_protocol: "all_owners_approve"  # any change requires all three domains to approve
```

The Shared Kernel is small by definition. If more than 20% of a domain's L0 types are in the
Shared Kernel, the domain boundary is probably wrong.

---

## Phase 3: Anti-Corruption Layer Specification

When Domain A calls Domain B, it must never allow Domain B's model to corrupt Domain A's
ubiquitous language. The Anti-Corruption Layer (ACL) is mandatory at every inter-domain call.

### ACL Structure

An ACL always consists of exactly two L5 ARUs:

```
On the consuming side (Domain A):
  auth.domain.integrate.userProfile
    pattern: PIPE
    chain:
      - auth.domain.transform.fromUserModel    ← translates User model → Auth model
      - auth.domain.validate.profileResponse   ← validates the translated data
```

```
On the publishing side (Domain B):
  user.domain.expose.profileQuery
    pattern: PIPE
    chain:
      - user.profile.load.byId
      - user.domain.transform.toPublicProfile  ← strips internal fields before publishing
```

The consuming domain's INTEGRATE ARU is responsible for translation. The publishing domain's
EXPOSE ARU is responsible for sanitization. Both sides protect their own model.

### ACL Failure Semantics

ACL failures use the standard railway model. A failed TRANSLATE operation means the
inter-domain contract is broken — this is a critical failure that should never be silently
swallowed:

```
ACL failure escalation:
  TRANSLATE failure → ACL pipeline error handler
  → Logs with full provenance (domain boundary crossing is always logged)
  → Returns IntegrationError<E> to the consuming domain
  → Never exposes the publishing domain's internal error types
```

---

## Phase 4: Domain Graph Rules

The domain-level graph is a DAG — just like the ARU-level graph, but at L5:

```
Allowed:
  Domain A ──depends on──▶ Domain B   (via L5 EXPOSE/INTEGRATE)

Forbidden:
  Domain A ──depends on──▶ Domain A   (self-loop — should be subdomain, not separate domain)
  Domain A ──depends on──▶ Domain B
  Domain B ──depends on──▶ Domain A   (circular dependency — redraw boundaries)
```

### Detecting Domain Cycles

A domain cycle means the two domains should either be:
1. **Merged** — they are too coupled to be separate domains
2. **Decoupled via events** — replace synchronous cross-domain calls with domain events

```
BEFORE (cycle):
  Auth ──▶ User ──▶ Auth     ← cycle

AFTER (event-driven decoupling):
  Auth ──publishes──▶ AuthEvent.UserAuthenticated
  User ──subscribes──▶ AuthEvent.UserAuthenticated
                      (updates user.lastLogin without calling Auth directly)
```

---

## Phase 5: Domain Manifest

Every L5 Domain has a **Domain Manifest** — distinct from individual ARU manifests:

```yaml
domain_manifest:
  id: "auth"
  version: "3.0.0"

  identity:
    purpose: "Manages authentication, session lifecycle, and credential validation"
    team: "platform-security"
    ubiquitous_language:
      - term: "Token"
        definition: "A signed, time-limited credential proving identity"
      - term: "Session"
        definition: "An authenticated context with a bounded lifetime"

  boundary:
    exposes:                           # what this domain publishes
      - aru: "auth.domain.expose.sessionValidation"
        consumers: [user, billing, notification]
    integrates:                        # what this domain consumes from others
      - aru: "auth.domain.integrate.userProfile"
        provider: user

  shared_kernel_participation:
    - kernel: "shared.identity"
      owned_types: [UserId]            # types this domain owns in the kernel

  l0_registry:                         # L0 types owned exclusively by this domain
    - TokenString
    - ValidatedToken
    - HashedPassword
    - SessionId
    - AuthError

  compliance_level: 4                  # ARIA compliance level (see doc 14)
```

---

## Heuristics Reference

| Decision | Heuristic | Warning Sign |
|---|---|---|
| Split into separate domains? | Can you explain it to someone without mentioning the other? | If no → maybe one domain |
| Merge into one domain? | Do they always change together for the same reason? | If yes → probably one domain |
| Add to Shared Kernel? | Do 3+ domains need the exact same semantics? | If few domains need it → ACL instead |
| Add anti-corruption layer? | Is Domain B's model different from Domain A's model? | If same model → maybe Shared Kernel |
| Introduce domain event? | Is there a cycle? Is the dependency synchronous but shouldn't be? | If yes → domain event |

---

## Domain Decomposition as a Human Activity

Domain decomposition is defined as a **human activity** at L5. AI agents do not determine domain
boundaries — they navigate, validate, and implement within declared boundaries.

The Orchestrator may flag potential domain issues (e.g., detecting a cross-domain direct dependency
pattern in a proposed decomposition), but boundary decisions are escalated to humans:

```
Orchestrator detects: L3 ARU in domain A proposes direct dependency on L2 ARU in domain B
→ TASK_ESCALATED: "Cross-domain L3→L2 dependency detected. Human must decide:
    Option 1: Introduce L5 EXPOSE/INTEGRATE pair
    Option 2: Merge subdomain B into domain A
    Option 3: Promote the dependency to a Shared Kernel type"
```
