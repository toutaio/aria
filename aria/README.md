# ARIA Build Toolchain

The ARIA build toolchain implements build-time validation, code generation, and IDE support for the [ARIA architecture framework](https://github.com/your-org/ai-architecture).

## What's included

| Component | Description |
|---|---|
| `aria-build` | Rust CLI: manifest validation, code generation, bundle building |
| `aria-core` | Rust library: manifest parsing, semantic graph, validation rules |
| `aria-lsp` | Rust LSP server: real-time IDE validation |
| `@aria/runtime` | TypeScript library: `Result<T,E>`, `RailError`, `ThreeTrack`, `TraceContext` |
| `aria-ts-plugin` | TypeScript code generator: composition wrappers, test fixtures |
| `@aria/build-bin` | npm shim: installs the `aria-build` binary without requiring Rust |

## Quickstart

### Install via npm (no Rust required)

```bash
npm install --save-dev @aria/build-bin
# or
pnpm add -D @aria/build-bin
```

The `@aria/build-bin` package resolves and installs the correct pre-built binary for your platform automatically.

### Install via cargo

```bash
cargo install aria-build
```

### Run manifest validation

```bash
# Validate all *.manifest.yaml files in the current project
aria-build check

# With a specific compliance level (0–5, default 5)
aria-build check --compliance-level 3

# Machine-readable output for CI
aria-build check --format json
```

### Generate composition wrappers

```bash
# Generate TypeScript wrapper files for all manifests with composition: sections
aria-build generate
```

Generated files are named `*.generated.ts` and committed to version control by default, enabling code review and diffing of architectural changes.

### Integration with tsc

Add `aria-build generate` before your TypeScript compile step:

```json
{
  "scripts": {
    "prebuild": "aria-build check && aria-build generate",
    "build": "tsc"
  }
}
```

### Integration with cargo

Add `aria-build check` as a build script dependency:

```toml
# build.rs
fn main() {
    println!("cargo:rerun-if-changed=*.manifest.yaml");
    // Run aria-build check in the build script
}
```

## Configuration

Create `.aria/config.yaml` in your project root to set defaults:

```yaml
compliance_level: 3   # 0–5 (default: 5)
```

## Generated files policy

ARIA generates `*.generated.ts` files alongside your composition manifests. These are committed to version control by default because:

1. **Code review**: Generated wrappers are architectural contracts. Reviewing them catches misconfigurations before merge.
2. **Diffing**: Changes to manifest `composition:` sections produce visible diffs in generated files.
3. **Freshness checking**: `aria-build check` embeds a manifest hash in each generated file and reports staleness.

**To opt out** (e.g., in large monorepos where generated files create noise), add to your project's `.gitignore`:

```
*.generated.ts
```

When using gitignore opt-out, run `aria-build generate` as part of your CI pipeline before `tsc`.

## Compliance levels

| Level | Checks enabled |
|---|---|
| 0 | Manifest schema validation only |
| 1 | + Naming enforcement (semantic addresses, verb vocabulary) |
| 2 | + Type graph validation (composition type compatibility) |
| 3 | + Composition code generation checks |
| 4 | + Bundle freshness checks |
| 5 | All checks (default) |

## Offline / airgapped environments

Set `ARIA_BUILD_BIN_PATH` to the absolute path of the `aria-build` binary:

```bash
export ARIA_BUILD_BIN_PATH=/opt/aria/bin/aria-build
```

When set, `@aria/build-bin` skips all network-based binary resolution.

## Repository structure

```
aria/
├── Cargo.toml                  ← Rust workspace
├── package.json                ← pnpm workspace
├── crates/
│   ├── aria-core/              ← library: manifest parsing, graph, validation
│   ├── aria-build/             ← binary: CLI wrapping aria-core
│   └── aria-lsp/               ← binary: LSP server linking aria-core
└── packages/
    ├── aria-runtime/           ← @aria/runtime (zero-dependency TypeScript)
    ├── aria-ts-plugin/         ← TypeScript code generator
    └── aria-build-bin/         ← @aria/build-bin platform binary shim
```

## License

MIT
