# ARIA — Atomic Responsibility Interface Architecture
### A theoretical framework for AI-optimized software construction

---

## Core Insight

AI code helpers operate under hard constraints: finite context windows, sensitivity to ambiguity, and high performance on pattern recognition. Traditional software architecture was designed for human cognitive limits — modularity, readability, naming conventions. ARIA is designed for *AI cognitive limits*, which are fundamentally different.

The central thesis:

> **If every component of a codebase is an unambiguous contract with a precise responsibility, a known layer, a typed interface, and a predictable connection model — then an AI can work with maximum precision using minimum context.**

---

## The Problem Space

| Human Cognitive Limit | Human Architecture Solution | AI Cognitive Limit | ARIA Solution |
|---|---|---|---|
| Short-term memory (~7 items) | Functions, modules | Context window (~N tokens) | Context Manifests |
| Abstraction overload | Design patterns | Ambiguity → hallucination | Contract-First Design |
| Onboarding time | Documentation | No persistent memory | Semantic Addressing |
| Parallel reasoning | Layered architecture | Attention dilution | Isolation Layers |
| Long files hard to read | File splitting | Noise consumes context | Progressive Disclosure |

---

## The Five Pillars

```
┌─────────────────────────────────────────────────────────────────┐
│                         ARIA FRAMEWORK                          │
├─────────────┬──────────────┬────────────┬──────────┬───────────┤
│  Abstraction │   Atomic     │ Composition│ Context  │ Semantic  │
│    Layers   │Responsibility│  Patterns  │Manifests │  Graph    │
│   (WHERE)   │   Units      │   (HOW)    │  (COST)  │  (MAP)    │
│             │   (WHAT)     │            │          │           │
└─────────────┴──────────────┴────────────┴──────────┴───────────┘
```

1. **Abstraction Layers** — WHERE in the system each piece lives
2. **Atomic Responsibility Units (ARU)** — WHAT each piece is and does
3. **Composition Patterns** — HOW pieces connect to each other
4. **Context Manifests** — COST (in tokens) to understand/modify a piece
5. **Semantic Graph** — MAP of the entire codebase navigable by AI

---

## Prior Art & Influences

ARIA synthesizes ideas from several established fields. These are not reinventions — they are adaptations of proven concepts to the specific constraints of AI-assisted development.

| Concept | Origin | How ARIA Adapts It |
|---|---|---|
| Atom / Molecule / Organism layer names | **Atomic Design** — Brad Frost (2013) | Applied to software modules instead of UI components; extended with Primitive, System, and Domain layers |
| Strict layer dependency rules | **Clean Architecture** — Robert C. Martin (2017) | Dependency direction enforced mechanically via manifest validation rather than convention |
| Bounded contexts, ubiquitous language | **Domain-Driven Design** — Eric Evans (2003) | L5 Domain maps directly to a DDD bounded context |
| Hexagonal ports-and-adapters | **Hexagonal Architecture** — Alistair Cockburn (2005) | Infrastructure adapters live at L1/L2 so L3 organisms stay free of I/O concerns |
| Composition pattern vocabulary | **Enterprise Integration Patterns** — Hohpe & Woolf (2003) | Named patterns (PIPE, SAGA, CIRCUIT_BREAKER…) make inter-ARU topology machine-readable |
| Railway-oriented error model | **Railway-Oriented Programming** — Scott Wlaschin (fsharpforfun.com) | Formalized as the error-propagation semantics for all 22 composition patterns (see `12-error-propagation.md`) |

### Further Reading

- Brad Frost — *Atomic Design* (atomicdesign.bradfrost.com, 2016)
- Robert C. Martin — *Clean Architecture* (Prentice Hall, 2017)
- Eric Evans — *Domain-Driven Design* (Addison-Wesley, 2003)
- Alistair Cockburn — *Hexagonal Architecture* (alistair.cockburn.us/hexagonal-architecture, 2005)
- Gregor Hohpe & Bobby Woolf — *Enterprise Integration Patterns* (Addison-Wesley, 2003)
- Scott Wlaschin — *Railway Oriented Programming* (fsharpforfun.com/posts/recipe-part2)

---

## Design Philosophy

- **Humans define meaning, AI fills implementation** — humans own L0 (type vocabulary) and L5 (domain boundaries); AI operates primarily in L1–L4. See `14-human-ai-collaboration.md`.
- **Contracts over comments** — typed, machine-verifiable contracts replace prose documentation.
- **Predictability over cleverness** — consistent patterns allow AI to infer structure without reading it.
- **Minimum viable context** — any task should require reading the smallest possible subgraph of the codebase.
- **Ambiguity is a defect** — any interface that could be misunderstood by AI is architecturally broken.
- **Adoption is a gradient** — ARIA has five compliance levels; each level delivers independent efficiency gains. Full adoption is never required on day one.

---

## Documents in this Theory

| File | Contents |
|---|---|
| `01-abstraction-layers.md` | The L0–L5 hierarchical model |
| `02-atomic-responsibility-units.md` | ARU specification and structure |
| `03-composition-patterns.md` | The 22 composition patterns (10 core + 4 async/distributed + 8 extended) |
| `04-context-manifests.md` | Manifest structure and context budgets |
| `05-semantic-graph.md` | Codebase-as-DAG and context loading |
| `06-naming-conventions.md` | Semantic addressing system |
| `07-consistency-amplification.md` | How pattern resonance reduces AI context needs |
| `08-type-system.md` | Branded types, the L0 type registry, expressiveness rules |
| `09-type-states.md` | Type state machines — encoding data lifecycle in the type |
| `10-algebraic-types.md` | Sum types, product types, the universal contract grammar |
| `11-type-compatibility.md` | Rules for connecting ARUs; type checking the graph |
| `12-error-propagation.md` | Railway-oriented error model; success/failure rails for all 22 patterns |
| `13-contract-versioning.md` | Contract lifecycle, migration ARUs, session-level caching |
| `14-human-ai-collaboration.md` | Layer ownership map, ARIA compliance levels, adoption path |
| `15-task-decomposition.md` | Subtask grammar, decomposition DAG, parallelization, underspecified detection |
| `16-ai-agent-roles.md` | Navigator, Generator, Reviewer, Refactorer, Orchestrator — roles and protocols |
| `17-aru-lifecycle.md` | Six lifecycle phases, bootstrapping protocol, ARU density reference |
| `18-observability.md` | Trace propagation, health contracts, diagnostic surface |
| `19-multi-agent-infrastructure.md` | Task queue, shared registry, agent isolation, conflict resolution |
| `20-manifest-schema.md` | Unified authoritative manifest schema (all fields from docs 04, 09, 18) |
| `21-runtime-composition.md` | Runtime substrate for railway execution and TraceContext injection |
| `22-domain-decomposition.md` | Domain identification principles and boundary rules |
| `23-test-infrastructure.md` | Scenario→test mapping, test generation protocol, coverage verification |
