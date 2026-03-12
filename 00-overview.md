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
| `03-composition-patterns.md` | The 10 connection patterns |
| `04-context-manifests.md` | Manifest structure and context budgets |
| `05-semantic-graph.md` | Codebase-as-DAG and context loading |
| `06-naming-conventions.md` | Semantic addressing system |
| `07-consistency-amplification.md` | How pattern resonance reduces AI context needs |
| `08-type-system.md` | Branded types, the L0 type registry, expressiveness rules |
| `09-type-states.md` | Type state machines — encoding data lifecycle in the type |
| `10-algebraic-types.md` | Sum types, product types, the universal contract grammar |
| `11-type-compatibility.md` | Rules for connecting ARUs; type checking the graph |
| `12-error-propagation.md` | Railway-oriented error model; success/failure rails |
| `13-contract-versioning.md` | Contract lifecycle, migration ARUs, session-level caching |
| `14-human-ai-collaboration.md` | Layer ownership map, ARIA compliance levels, adoption path |
