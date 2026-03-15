# ARIA Tutorial

> **Every piece of code has a name, a contract, and a place. ARIA makes that true by design.**

## What Is ARIA?

Modern codebases grow fast — and as they grow, they become harder to navigate. Functions call other functions through implicit chains, classes accumulate responsibilities, and the only way to understand what a piece of code does is to read it in full. For a human developer, this is slow. For an AI agent trying to make a targeted change, it can be impossible without loading enormous amounts of context.

ARIA (Atomic Responsibility Interface Architecture) solves this coordination problem by making every piece of code an **Atomic Responsibility Unit (ARU)**. Each ARU has exactly one typed input, one typed output, one layer (L0–L5), and a co-located `.manifest.yaml` file that describes its purpose, contract, dependencies, and connections. You never have to guess what something does — the manifest tells you.

ARUs connect to one another through **14 named composition patterns** — things like `PIPE`, `FORK`, `VALIDATE`, and `SAGA`. Every connection between two ARUs must be declared in the manifest. This turns your entire codebase into a **semantic graph**: a machine-readable map that both humans and AI agents can traverse. A human can use it to understand what will break if they change a function. An AI agent can use it to load exactly the context it needs — no more, no less.

## Who This Tutorial Is For

This tutorial is for developers who want to understand ARIA from first principles and apply it to a real project. You should be comfortable with:

- Basic programming concepts (functions, types, modules)
- Reading and writing TypeScript syntax
- Reading and writing YAML

You do **not** need any prior experience with ARIA, architecture frameworks, or AI-assisted development workflows. We'll build everything up from scratch.

## What You'll Build

Throughout the tutorial, you'll build a **URL shortener** — a service that accepts a long URL, generates a short code, stores the mapping, and resolves short codes back to their original URLs.

This isn't a toy example chosen for simplicity. It's chosen because it exercises every layer of the ARIA stack:

- **L0**: types like `ShortCode`, `OriginalUrl`, and `ShortenedLink`
- **L1**: atoms like `url.shortcode.validate.format` and `url.shortcode.generate.hash`
- **L2**: molecules like `url.link.create.fromOriginal`
- **L3**: organisms like `url.store.persist.link` and `url.store.resolve.shortCode`
- **L4**: a system that orchestrates the full shorten pipeline
- **L5**: an API boundary that exposes the service to the outside world

The project grows across three chapters (Chapters 3, 4, and 5), each adding a new layer of capability. You can stop at any chapter and have a working, coherent project.

## Prerequisites

Before starting, make sure you have the following installed:

| Tool | Minimum Version | Where to get it |
|------|----------------|-----------------|
| Node.js | ≥ 18.0.0 | [nodejs.org](https://nodejs.org) |
| Git | ≥ 2.30 | [git-scm.com](https://git-scm.com) |
| TypeScript familiarity | — | [typescriptlang.org/docs](https://www.typescriptlang.org/docs/) |
| YAML familiarity | — | [yaml.org/spec](https://yaml.org/spec/1.2.2/) |

The `aria-build` CLI is installed as part of Chapter 2 (Setup).

## How To Use This Tutorial

The tutorial is structured as a sequence of chapters, but it's designed so you can stop or start anywhere:

- **Chapters 0–1** (Introduction + Concepts) cover the "why" and "what" of ARIA. If you prefer learning by doing, skim Chapter 1 and come back to it as a reference when questions arise.
- **Chapter 2** (Setup) gets your environment ready. Don't skip this.
- **Chapters 3–5** (Projects) are the hands-on part. Each one builds on the previous, but each also has a clear entry point described at the top.
- **Chapter 6** (Running the Application) starts the server, makes real HTTP requests, and explains what to replace to use a real database.
- **Chapter 7** (Conclusion) points you toward the full documentation and advanced topics.

If you're using this as a reference while working on an existing project, **Chapter 1** (Core Concepts) and the manifest examples in **Chapter 3** are the most useful standalone sections.

## Table of Contents

| # | File | Description |
|---|------|-------------|
| 0 | [00-introduction.md](00-introduction.md) | This file — orientation, structure, and prerequisites |
| 1 | [01-concepts.md](01-concepts.md) | Core concepts: layers, ARUs, naming, manifests, and composition patterns |
| 2 | [02-setup.md](02-setup.md) | Environment setup and project scaffold |
| 3 | [03-project-starter.md](03-project-starter.md) | Project 1 — URL Shortener core (L0–L3) |
| 4 | [04-project-advanced.md](04-project-advanced.md) | Project 2 — Analytics and orchestration (L4) |
| 5 | [05-project-ai-collab.md](05-project-ai-collab.md) | Project 3 — L5 boundaries and human-AI collaboration |
| 6 | [06-running.md](06-running.md) | Running the application |
| 7 | [07-conclusion.md](07-conclusion.md) | Conclusion and further reading |

---
**[Back to index](00-introduction.md)** | **[Next: Core Concepts →](01-concepts.md)**
