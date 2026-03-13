# Chapter 2: Setup

By the end of this chapter, you'll have a working ARIA development environment and an empty project scaffold ready for Chapter 3. This should take about 10–15 minutes on a clean machine.

---

## Prerequisites

Before installing anything, verify that the following tools are present and at the required versions:

| Tool | Minimum Version | Where to Get It |
|------|----------------|-----------------|
| **Node.js** | ≥ 18.0.0 | [nodejs.org](https://nodejs.org) |
| **npm** | ≥ 9.0.0 | Comes with Node.js |
| **Git** | ≥ 2.30 | [git-scm.com](https://git-scm.com) |
| **aria-build CLI** | ≥ 0.1.0 | Installed in the next section |
| **A code editor** | — | VS Code recommended (setup instructions below) |

Run these commands to check your current versions:

```bash
node --version   # should print v18.x.x or higher
npm --version    # should print 9.x.x or higher
git --version    # should print 2.x.x or higher
```

If any of these commands fail or print a version below the minimum, see the [Troubleshooting](#troubleshooting) section at the end of this chapter.

---

## Installing the ARIA CLI

The `aria-build` CLI is the primary tool for working with ARIA manifests. It validates manifests against the schema, checks compliance rules, builds the semantic graph, and generates TypeScript wrappers for composition patterns.

> **Version note**: This tutorial was written and verified against `aria-build` **v0.1.x**. If you are using a newer version, all commands shown here should still work, but output formatting may differ slightly.

Install the CLI globally via npm:

```bash
npm install -g aria-build
```

Verify the installation succeeded:

```bash
aria-build --version
# Expected output: aria-build/0.1.x
```

Verify the key subcommands are available:

```bash
aria-build --help
# Should list: check, bundle, generate, impact
```

If `aria-build` is not found after installation, see [Troubleshooting](#troubleshooting).

---

## Editor Setup — Manifest Schema Validation

ARIA ships a JSON Schema file for manifests (`aria/schema/aria-manifest.schema.json`). Configuring your editor to use this schema gives you real-time validation as you write `.manifest.yaml` files — red squiggles for missing required fields, wrong types, invalid enum values, and layer violations.

### VS Code (Recommended)

1. Install the [YAML extension by Red Hat](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml) from the VS Code marketplace.

2. Create or open `.vscode/settings.json` in your project and add:

```json
{
  "yaml.schemas": {
    "./aria/schema/aria-manifest.schema.json": "**/*.manifest.yaml"
  }
}
```

3. Open any `.manifest.yaml` file. Red squiggles will appear for any schema violations, and you'll get autocompletion for field names and enum values.

> **Note**: The path `./aria/schema/aria-manifest.schema.json` is relative to your workspace root. If you copy the schema to a different location (as described in the project scaffold below), update this path accordingly.

### Other Editors

The schema file at `aria/schema/aria-manifest.schema.json` is a standard **JSON Schema Draft-07** document. It is supported by most editor YAML plugins:

- **IntelliJ / WebStorm**: Enable via *Languages & Frameworks → Schemas and DTDs → JSON Schema Mappings*
- **Neovim**: Use `yaml-language-server` with the `yaml.schemas` configuration (same format as VS Code)
- **Sublime Text**: Install the `LSP-yaml` package and configure schemas in LSP settings

---

## Creating Your Project

### Step 1: Create the project directory

```bash
mkdir url-shortener-aria
cd url-shortener-aria
git init
npm init -y
```

### Step 2: Create the ARIA directory structure

ARIA doesn't mandate a specific top-level layout, but co-locating ARUs by domain and subdomain keeps navigation intuitive. Create the directories for the URL shortener:

```bash
mkdir -p src/url/shortcode
mkdir -p src/url/link
mkdir -p src/url/store
mkdir -p src/url/pipeline
mkdir -p src/url/domain
```

### Step 3: Create the ARIA config file

The `aria.config.yaml` file tells the CLI where your source lives:

```bash
cat > aria.config.yaml << 'EOF'
version: "1"
src: "./src"
EOF
```

### Step 4: Copy the manifest schema for local validation

If you're working from the ARIA repository, copy the schema into your project so editor validation works without a network connection:

```bash
# If you cloned the ARIA repository
mkdir -p aria/schema
cp /path/to/aria-architecture/aria/schema/aria-manifest.schema.json aria/schema/
```

If you don't have the ARIA repository locally, the schema is also available via the `aria-build` CLI:

```bash
aria-build init-schema ./aria/schema/
```

### Step 5: Create the VS Code settings file

```bash
mkdir -p .vscode
cat > .vscode/settings.json << 'EOF'
{
  "yaml.schemas": {
    "./aria/schema/aria-manifest.schema.json": "**/*.manifest.yaml"
  }
}
EOF
```

### Full Project Structure

After completing the steps above, your project should look like this:

```
url-shortener-aria/
├── src/
│   └── url/
│       ├── shortcode/          ← L1 atoms live here
│       │   ├── validate.format.ts
│       │   ├── validate.format.manifest.yaml
│       │   ├── generate.hash.ts
│       │   └── generate.hash.manifest.yaml
│       ├── link/               ← L2 molecules live here
│       │   ├── create.fromOriginal.ts
│       │   └── create.fromOriginal.manifest.yaml
│       ├── store/              ← L3 organisms live here
│       │   ├── persist.link.ts
│       │   ├── persist.link.manifest.yaml
│       │   ├── resolve.shortCode.ts
│       │   └── resolve.shortCode.manifest.yaml
│       ├── pipeline/           ← L4 systems live here
│       │   ├── orchestrate.shorten.ts
│       │   └── orchestrate.shorten.manifest.yaml
│       └── domain/             ← L5 boundaries live here
│           ├── expose.api.ts
│           └── expose.api.manifest.yaml
├── aria/
│   └── schema/
│       └── aria-manifest.schema.json
├── aria.config.yaml
├── package.json
└── .vscode/
    └── settings.json
```

The files under `src/url/` don't exist yet — you'll create them in Chapter 3. The directory structure is what matters for now.

---

## Verify Your Setup

Run the ARIA compliance check on your (currently empty) `src` directory:

```bash
aria-build check ./src
```

Expected output for a fresh project with no manifest files yet:

```
✓ No manifests found in ./src — nothing to check.
```

This is correct. An empty project has nothing to validate. Once you start adding manifests in Chapter 3, this command will verify that they're well-formed and that layer rules are respected.

You can also confirm that `aria-build` can find your config:

```bash
aria-build check
# With no argument, reads src path from aria.config.yaml
# Same output: ✓ No manifests found in ./src — nothing to check.
```

If both commands succeed, your environment is ready.

---

## Troubleshooting

### Problem: `aria-build: command not found`

**Cause**: npm's global `bin` directory is not on your `PATH`.

**Resolution**:
```bash
npm config get prefix          # shows your npm prefix, e.g. /usr/local or /home/user/.npm-global
export PATH="$(npm config get prefix)/bin:$PATH"
```

To make this permanent, add the `export PATH` line to your `~/.bashrc` (bash) or `~/.zshrc` (zsh) and restart your terminal.

---

### Problem: `aria-build check` reports "unknown field" on a valid manifest

**Cause**: The `aria-manifest.schema.json` in your project was copied from a different version of ARIA than the CLI you have installed.

**Resolution**: Ensure the schema matches your installed CLI version:
```bash
aria-build --version        # check CLI version, e.g. aria-build/0.1.4
# Re-copy the schema from the ARIA repo at the matching git tag
git -C /path/to/aria-architecture checkout v0.1.4
cp /path/to/aria-architecture/aria/schema/aria-manifest.schema.json aria/schema/
```

---

### Problem: VS Code YAML extension shows no schema validation

**Cause**: The path in `.vscode/settings.json` is incorrect or VS Code hasn't reloaded after the settings change.

**Resolution**: The path must be relative to your **workspace root** (the folder you opened in VS Code, not the current file's directory). Use:
```json
{
  "yaml.schemas": {
    "./aria/schema/aria-manifest.schema.json": "**/*.manifest.yaml"
  }
}
```

Not an absolute path like `/home/user/projects/url-shortener-aria/aria/schema/...`. After saving `.vscode/settings.json`, reload VS Code with **Ctrl+Shift+P → Developer: Reload Window**.

---

### Problem: `node --version` prints v16 or lower

**Cause**: An older version of Node.js is installed, or the system default is pinned to an older version.

**Resolution**: Use [nvm](https://github.com/nvm-sh/nvm) (Node Version Manager) to install and switch to Node.js 18:

```bash
# Install nvm (if not already installed)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash

# Restart your terminal, then:
nvm install 18
nvm use 18
node --version   # should now print v18.x.x
```

To make Node 18 the default for all new terminals:
```bash
nvm alias default 18
```

---

### Problem: `npm init -y` fails with permission errors

**Cause**: The directory was created with elevated permissions, or npm's cache directory has a permissions issue.

**Resolution**:
```bash
# Fix directory ownership
sudo chown -R $(whoami) url-shortener-aria/

# Or, if the issue is the npm cache
npm cache clean --force
```

---

You're all set. In the next chapter, you'll write your first ARUs — the L0 types and L1 atoms that form the foundation of the URL shortener.

---
**[← Concepts](01-concepts.md)** | **[Back to index](00-introduction.md)** | **[Next: Starter Project →](03-project-starter.md)**
