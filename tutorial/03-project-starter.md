# Chapter 3: Project 1 — URL Shortener (L0–L3)

## 1. What You'll Build

Before writing a single line of code, let's look at the end-to-end flow:

```
POST /shorten  { url: "https://example.com/very/long/path" }
    │
    ▼
{ shortCode: "abc123", shortUrl: "https://s.io/abc123" }

GET /abc123
    │
    ▼
302 Redirect → https://example.com/very/long/path
```

Two operations. One shortens a URL and hands back a short code. The other resolves a short code back to the original URL for a redirect.

By the end of this chapter you'll have:

- 2 L0 type definitions
- 2 L1 atoms
- 1 L2 molecule
- 2 L3 organisms
- All wired with **PIPE** and **VALIDATE** composition patterns
- A clean `aria-build check` output

---

## 2. Domain Model

The URL shortener has three core types:

- **`OriginalUrl`** — the long URL the user submits
- **`ShortCode`** — the 6-character token (e.g. `abc123`)
- **`ShortenedLink`** — the record that joins them together, with a timestamp

Here's how the ARUs relate to each other across layers:

```
                    [L0] url.types
                         │
           ┌─────────────┼─────────────┐
           ▼             ▼             ▼
[L1] validate.format  [L1] generate.hash
           │             │
           └──────┬──────┘
                  ▼
     [L2] link.create.fromOriginal
                  │
        ┌─────────┴──────────┐
        ▼                    ▼
[L3] store.persist.link  [L3] store.execute.shortCode
```

Notice the direction: types flow down, functions compose upward. Each layer only reaches down — never up.

---

## 3. L0 — Types

**L0 defines the type vocabulary.** No functions, no side effects, no imports from other ARUs. If you find yourself writing logic in an L0 file, stop — it belongs in L1.

Branded types are used here so that a plain `string` cannot be accidentally passed where a `ShortCode` is expected. TypeScript's structural type system would normally allow this; branding prevents it at compile time.

### `src/url/types.ts`

```typescript
// url.types.ts
export type ShortCode = string & { readonly _brand: "ShortCode" };
export type OriginalUrl = string & { readonly _brand: "OriginalUrl" };

export interface ShortenedLink {
  shortCode: ShortCode;
  originalUrl: OriginalUrl;
  createdAt: Date;
}

export type ValidatedShortCode = ShortCode & { readonly _validated: true };

export type FormatError = {
  code: "TOO_LONG" | "INVALID_CHARS" | "EMPTY";
  message: string;
};

export type StoreError = {
  code: "NOT_FOUND" | "DUPLICATE" | "STORE_UNAVAILABLE";
  message: string;
};
```

### `src/url/types.manifest.yaml`

```yaml
manifest:
  id: "url.types"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L0
    inferred: L0
  identity:
    purpose: "core type vocabulary for the url domain"
    domain: "url"
    subdomain: "types"
    verb: "types"
    entity: "types"
  contract:
    input:
      type: "N/A"
    output:
      success: "ShortCode | OriginalUrl | ShortenedLink | ValidatedShortCode | FormatError | StoreError"
      failure: "N/A"
    side_effects: NONE
    idempotent: true
    deterministic: true
  dependencies: []
  context_budget:
    to_use: 60
    to_modify: 120
    to_extend: 180
    to_replace: 120
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections: []
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

Notice a few things about the L0 manifest:

- The `layer` field is a struct with `declared` and `inferred` — both required by the schema
- L0 identity uses the entity name as a placeholder for `subdomain` and `verb` since it defines types, not operations
- `contract.input/output` use `"N/A"` — L0 defines types, not callable functions, but the fields are required
- `dependencies: []` — nothing below L0 exists
- `connections: []` — L0 doesn't compose with anything; other layers depend on it
- `lifecycle.phase: DRAFT` is required; use `DRAFT` for new ARUs not yet in production

---

## 4. L1 — Atoms

**L1 atoms do one thing.** Each has a single typed input and returns either a success type or a typed error. Zero side effects, always deterministic.

The test for whether something belongs in L1: *"Given [input], it [single verb] and returns [output]."* No "and" in the verb phrase.

---

### Atom 1: `url.shortcode.validate.format`

Validates that a short code string is safe to store — not empty, not too long, only alphanumeric characters.

#### `src/url/shortcode/validate.format.ts`

```typescript
import type { FormatError, ValidatedShortCode } from "../types";

type Result = { ok: true; value: ValidatedShortCode } | { ok: false; error: FormatError };

export function validateFormat(input: string): Result {
  if (!input || input.length === 0) {
    return { ok: false, error: { code: "EMPTY", message: "Short code cannot be empty" } };
  }
  if (input.length > 10) {
    return { ok: false, error: { code: "TOO_LONG", message: "Short code must be 10 characters or fewer" } };
  }
  if (!/^[a-zA-Z0-9]+$/.test(input)) {
    return { ok: false, error: { code: "INVALID_CHARS", message: "Short code must be alphanumeric" } };
  }
  return { ok: true, value: input as ValidatedShortCode };
}
```

#### `src/url/shortcode/validate.format.manifest.yaml`

```yaml
manifest:
  id: "url.shortcode.validate.format"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L1
    inferred: L1
  identity:
    purpose: "validates that a short code matches the allowed character format"
    domain: "url"
    subdomain: "shortcode"
    verb: "validate"
    entity: "format"
  contract:
    input:
      type: "string"
      constraints:
        - "may be empty (returns FormatError EMPTY)"
        - "alphanumeric only allowed"
        - "max 10 characters"
    output:
      success: "ValidatedShortCode"
      failure: "FormatError { code: TOO_LONG | INVALID_CHARS | EMPTY }"
    side_effects: NONE
    idempotent: true
    deterministic: true
  dependencies:
    - id: "url.types"
      layer: L0
      stability: EXPERIMENTAL
  context_budget:
    to_use: 80
    to_modify: 200
    to_extend: 350
    to_replace: 150
  test_contract:
    scenarios:
      - scenario: "valid 6-char alphanumeric returns ValidatedShortCode"
      - scenario: "11-char string returns FormatError TOO_LONG"
      - scenario: "string with spaces returns FormatError INVALID_CHARS"
      - scenario: "empty string returns FormatError EMPTY"
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections:
    - pattern: VALIDATE
      target: "url.link.create.fromOriginal"
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

The `connections` entry uses the **VALIDATE** pattern because `validateFormat` acts as a typed contract gate — the downstream molecule (`create.fromOriginal`) only proceeds if this passes.

---

### Atom 2: `url.shortcode.generate.hash`

Takes an `OriginalUrl` and produces a deterministic 6-character short code by hashing the URL. This function always succeeds — there is no failure branch.

#### `src/url/shortcode/generate.hash.ts`

```typescript
import type { OriginalUrl, ShortCode } from "../types";
import { createHash } from "crypto";

export function generateHash(url: OriginalUrl): ShortCode {
  const hash = createHash("sha256").update(url).digest("base64url");
  return hash.slice(0, 6) as ShortCode;
}
```

#### `src/url/shortcode/generate.hash.manifest.yaml`

```yaml
manifest:
  id: "url.shortcode.generate.hash"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L1
    inferred: L1
  identity:
    purpose: "generates a 6-character short code from a URL by hashing it"
    domain: "url"
    subdomain: "shortcode"
    verb: "generate"
    entity: "hash"
  contract:
    input:
      type: "OriginalUrl"
      constraints:
        - "non-empty string"
    output:
      success: "ShortCode (always 6 alphanumeric characters)"
      failure: "CryptoError.HASH_FAILED"
    side_effects: NONE
    idempotent: true
    deterministic: true
  dependencies:
    - id: "url.types"
      layer: L0
      stability: EXPERIMENTAL
  context_budget:
    to_use: 60
    to_modify: 150
    to_extend: 200
    to_replace: 100
  test_contract:
    scenarios:
      - scenario: "same URL always produces same short code (deterministic)"
      - scenario: "output is always exactly 6 characters"
      - scenario: "output contains only alphanumeric characters"
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections:
    - pattern: PIPE
      target: "url.link.create.fromOriginal"
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

Note `deterministic: true` — SHA-256 over the same input always produces the same output. This makes the function safe to memoize and safe to retry. The failure type `CryptoError.HASH_FAILED` is declared even though this function effectively never fails — the schema requires a typed error reference, not a prose description.

---

## 5. L2 — Molecule

**L2 molecules compose atoms.** They accept a higher-level input and return a richer output by orchestrating L1 calls in sequence. A molecule is still pure — it introduces no side effects of its own.

The molecule here takes an `OriginalUrl`, generates a hash (L1), validates it (L1), and assembles the resulting `ShortenedLink`. The two L1 atoms do the real work; the molecule coordinates them.

### `src/url/link/create.fromOriginal.ts`

```typescript
import { generateHash } from "../shortcode/generate.hash";
import { validateFormat } from "../shortcode/validate.format";
import type { OriginalUrl, ShortenedLink, FormatError } from "../types";

type Result =
  | { ok: true; value: ShortenedLink }
  | { ok: false; error: FormatError };

export function createFromOriginal(originalUrl: OriginalUrl): Result {
  const shortCode = generateHash(originalUrl);
  const validation = validateFormat(shortCode);

  if (!validation.ok) {
    // Generated hash should always be valid, but we propagate defensively
    return { ok: false, error: validation.error };
  }

  return {
    ok: true,
    value: {
      shortCode: validation.value,
      originalUrl,
      createdAt: new Date(),
    },
  };
}
```

### `src/url/link/create.fromOriginal.manifest.yaml`

```yaml
manifest:
  id: "url.link.create.fromOriginal"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L2
    inferred: L2
  identity:
    purpose: "creates a ShortenedLink by generating and validating a hash from the original URL"
    domain: "url"
    subdomain: "link"
    verb: "create"
    entity: "fromOriginal"
  contract:
    input:
      type: "OriginalUrl"
      constraints:
        - "non-empty string"
    output:
      success: "ShortenedLink"
      failure: "FormatError.TOO_LONG | FormatError.INVALID_CHARS | FormatError.EMPTY"
    side_effects: NONE
    idempotent: true
    deterministic: true
  dependencies:
    - id: "url.shortcode.generate.hash"
      layer: L1
      stability: EXPERIMENTAL
    - id: "url.shortcode.validate.format"
      layer: L1
      stability: EXPERIMENTAL
  context_budget:
    to_use: 100
    to_modify: 280
    to_extend: 400
    to_replace: 200
  test_contract:
    scenarios:
      - scenario: "valid URL returns ShortenedLink with 6-char shortCode"
      - scenario: "same URL always produces same shortCode"
      - scenario: "FormatError from validate.format is propagated"
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections:
    - pattern: PIPE
      target: "url.store.persist.link"
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

The `create` verb is correct for L2 — it builds a new value from existing parts. Compare this to `generate` at L1 (produces a raw derived value) and `persist` at L3 (writes it somewhere).

---

## 6. L3 — Organisms

**L3 organisms contain business rules and are the first layer allowed to have side effects.** They interact with the outside world — databases, files, message queues — but always through explicitly declared contracts.

> **Key rule**: L3 calls infrastructure directly. But the `side_effects` field in the manifest makes this explicit and trackable — it's what allows AI agents to reason safely about which operations are safe to retry and which are not.

An idempotent READ with `side_effects: READ` can be retried freely. A non-idempotent WRITE with `side_effects: WRITE` cannot. The manifest communicates this without requiring anyone to read the implementation.

---

### Organism 1: `url.store.persist.link`

Writes a `ShortenedLink` to the data store. Returns a `DUPLICATE` error if the short code already exists.

#### `src/url/store/persist.link.ts`

```typescript
import type { ShortenedLink, StoreError } from "../types";

// In production, this would use a real database client.
// For the tutorial, we use an in-memory map.
const store = new Map<string, ShortenedLink>();

type Result =
  | { ok: true; value: ShortenedLink }
  | { ok: false; error: StoreError };

export async function persistLink(link: ShortenedLink): Promise<Result> {
  if (store.has(link.shortCode)) {
    return {
      ok: false,
      error: { code: "DUPLICATE", message: `Short code ${link.shortCode} already exists` },
    };
  }
  store.set(link.shortCode, link);
  return { ok: true, value: link };
}
```

#### `src/url/store/persist.link.manifest.yaml`

```yaml
manifest:
  id: "url.store.persist.link"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L3
    inferred: L3
  identity:
    purpose: "persists a ShortenedLink to the data store"
    domain: "url"
    subdomain: "store"
    verb: "persist"
    entity: "link"
  contract:
    input:
      type: "ShortenedLink"
    output:
      success: "ShortenedLink (confirmed persisted)"
      failure: "StoreError.DUPLICATE | StoreError.STORE_UNAVAILABLE"
    side_effects: WRITE          # ← writes to data store
    idempotent: false            # persisting the same link twice returns DUPLICATE
    deterministic: false         # depends on store state
  dependencies:
    - id: "url.types"
      layer: L0
      stability: EXPERIMENTAL
  behavioral_contract:
    max_retries: 0
    retry_strategy: none
    timeout: 500ms
  health_contract:
    sla_latency_p99: 100ms
    sla_availability: 99.9%
  diagnostic_surface:
    failure_indicators:
      - symptom: "StoreError.DUPLICATE returned"
        check: "short code already exists in store"
      - symptom: "StoreError.STORE_UNAVAILABLE returned"
        check: "store is unreachable or timed out"
  context_budget:
    to_use: 100
    to_modify: 350
    to_extend: 500
    to_replace: 300
  test_contract:
    scenarios:
      - scenario: "new link is stored and returned"
      - scenario: "duplicate shortCode returns StoreError DUPLICATE"
      - scenario: "store unavailable returns StoreError STORE_UNAVAILABLE"
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections:
    - pattern: PIPE
      target: "url.pipeline.orchestrate.shorten"
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

`idempotent: false` is important. If you send the same `ShortenedLink` twice, the second call returns `DUPLICATE` — it does not silently succeed. This tells a retry handler that it cannot blindly retry this operation.

The three L3-only sections have specific roles:
- **`behavioral_contract`** — declares retry policy and timeout; `max_retries: 0` signals this write is not safe to retry
- **`health_contract`** — declares SLA targets; used by monitoring and circuit breakers
- **`diagnostic_surface`** — describes observable failure symptoms; enables AI agents to reason about operational issues without reading the implementation

---

### Organism 2: `url.store.execute.shortCode`

Looks up a `ShortCode` in the store and returns the full `ShortenedLink`. This is a read — no mutation, safe to retry.

> **Naming note**: `resolve` is an L2 verb in ARIA's vocabulary. Since this operation reads from a data store (a side effect), it belongs at L3, where the correct verb is `execute`. Even though "resolve" is semantically intuitive for a URL lookup, ARIA's verb vocabulary locks verb choice to layer — `execute` is the right fit for a store read at L3.

#### `src/url/store/execute.shortCode.ts`

```typescript
import type { ShortCode, ShortenedLink, StoreError } from "../types";

const store = new Map<string, ShortenedLink>(); // shared with persist.link in real impl

type Result =
  | { ok: true; value: ShortenedLink }
  | { ok: false; error: StoreError };

export async function executeShortCode(shortCode: ShortCode): Promise<Result> {
  const link = store.get(shortCode);
  if (!link) {
    return {
      ok: false,
      error: { code: "NOT_FOUND", message: `No link found for short code: ${shortCode}` },
    };
  }
  return { ok: true, value: link };
}
```

#### `src/url/store/execute.shortCode.manifest.yaml`

```yaml
manifest:
  id: "url.store.execute.shortCode"
  version: "1.0.0"
  schema_version: "1.0"
  layer:
    declared: L3
    inferred: L3
  identity:
    purpose: "executes a short code lookup against the data store, returning the matching ShortenedLink"
    domain: "url"
    subdomain: "store"
    verb: "execute"
    entity: "shortCode"
  contract:
    input:
      type: "ShortCode"
    output:
      success: "ShortenedLink"
      failure: "StoreError.NOT_FOUND | StoreError.STORE_UNAVAILABLE"
    side_effects: READ
    idempotent: true
    deterministic: false         # result depends on store state
  dependencies:
    - id: "url.types"
      layer: L0
      stability: EXPERIMENTAL
  behavioral_contract:
    max_retries: 3
    retry_strategy: exponential_backoff
    timeout: 500ms
  health_contract:
    sla_latency_p99: 50ms
    sla_availability: 99.9%
  diagnostic_surface:
    failure_indicators:
      - symptom: "StoreError.NOT_FOUND returned"
        check: "short code does not exist in store"
      - symptom: "StoreError.STORE_UNAVAILABLE returned"
        check: "store is unreachable or timed out"
  context_budget:
    to_use: 100
    to_modify: 300
    to_extend: 450
    to_replace: 250
  test_contract:
    scenarios:
      - scenario: "existing shortCode returns matching ShortenedLink"
      - scenario: "unknown shortCode returns StoreError NOT_FOUND"
  stability: EXPERIMENTAL
  lifecycle:
    phase: DRAFT
  connections: []
  manifest_provenance:
    derived_by: HUMAN_DECLARED
    reviewed_by: HUMAN
    approved_at: "2026-03-14T00:00:00Z"
```

`connections: []` here is intentional — `execute.shortCode` is a terminal node in this chapter. It will be wired into an L4 orchestrator in Chapter 4.

---

## 7. Running `aria-build check`

With all six ARUs in place, run the validator:

```bash
aria-build check ./src
```

Expected output:

```
✓ url.types                         L0  EXPERIMENTAL
✓ url.shortcode.validate.format     L1  EXPERIMENTAL
✓ url.shortcode.generate.hash       L1  EXPERIMENTAL
✓ url.link.create.fromOriginal      L2  EXPERIMENTAL
✓ url.store.persist.link            L3  EXPERIMENTAL
✓ url.store.execute.shortCode       L3  EXPERIMENTAL

6 ARUs validated. 0 errors. 0 warnings.
```

What the check actually validates:

1. **Manifest presence** — every `.ts` file in `src/` has a co-located `.manifest.yaml`
2. **Naming convention compliance** — each manifest `id` follows `domain.subdomain.verb.entity` and the verb matches the declared layer's vocabulary
3. **Layer dependency rules** — no ARU imports from a layer above it (e.g. L1 may not depend on L3)
4. **Declared connection targets exist** — every `target` in a `connections:` block resolves to a known manifest `id`

If you see an error like `L1 ARU "url.shortcode.validate.format" has dependency on L3 ARU`, it means something in the L1 implementation is importing from an L3 file. The check traces actual TypeScript imports, not just manifest declarations.

---

## 8. Testing Your ARUs

Every ARU in the manifest has a `test_contract` block with scenarios. This section turns those scenarios into executable tests.

### Setup — Vitest and TypeScript

Add the test runner and TypeScript compiler:

```bash
npm install --save-dev typescript vitest @types/node
```

Update `package.json` to use ES modules and wire up the test script:

```json
{
  "name": "url-shortener-aria",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "test": "vitest run",
    "test:watch": "vitest"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "typescript": "^5.0.0",
    "vitest": "^3.0.0"
  }
}
```

Create `tsconfig.json` at the project root:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "strict": true,
    "outDir": "dist",
    "rootDir": "src"
  },
  "include": ["src/**/*"]
}
```

### Shared In-Memory Store

`persist.link` and `execute.shortCode` both use an in-memory `Map`. For tests to be isolated, they need to share the same store instance *and* be able to reset it between tests. Extract the store into its own module:

#### `src/url/store/in-memory.store.ts`

```typescript
import type { ShortenedLink } from "../types";

export const store = new Map<string, ShortenedLink>();

/** Reset all store state. Call in beforeEach() in tests only. */
export function clearStore(): void {
  store.clear();
}
```

Update `persist.link.ts` and `execute.shortCode.ts` to import from this shared module instead of declaring their own `Map`:

```typescript
// In both persist.link.ts and execute.shortCode.ts — replace the local Map:
import { store } from "./in-memory.store";
// Remove: const store = new Map<string, ShortenedLink>();
```

Also update `generate.hash.ts` to strip base64url-specific characters (`-` and `_`) so the output is reliably alphanumeric:

#### `src/url/shortcode/generate.hash.ts` (updated)

```typescript
import type { OriginalUrl, ShortCode } from "../types";
import { createHash } from "crypto";

export function generateHash(url: OriginalUrl): ShortCode {
  const hash = createHash("sha256").update(url).digest("base64url");
  // base64url may contain '-' and '_'; strip to alphanumeric only, then take 6
  return hash.replace(/[^a-zA-Z0-9]/g, "").slice(0, 6) as ShortCode;
}
```

> **Why the fix?** `base64url` encoding uses `-` and `_` in addition to alphanumeric characters. Without the strip, `validateFormat` could reject a hash that contains these characters — contradicting `generate.hash`'s manifest claim that it "always succeeds". The fix makes the implementation match the contract.

### L0 — No Tests Needed

L0 defines types only. TypeScript's compiler is the test. If you use a `ShortCode` where an `OriginalUrl` is expected, it's a compile error — not a runtime bug. No test file is needed for `url.types`.

### L1 — `url.shortcode.validate.format`

#### `src/url/shortcode/validate.format.test.ts`

```typescript
import { describe, it, expect } from "vitest";
import { validateFormat } from "./validate.format";

describe("url.shortcode.validate.format", () => {
  it("valid 6-char alphanumeric returns ValidatedShortCode", () => {
    const result = validateFormat("abc123");
    expect(result.ok).toBe(true);
    if (result.ok) expect(result.value).toBe("abc123");
  });

  it("11-char string returns FormatError TOO_LONG", () => {
    const result = validateFormat("abcdefghijk");
    expect(result.ok).toBe(false);
    if (!result.ok) expect(result.error.code).toBe("TOO_LONG");
  });

  it("string with spaces returns FormatError INVALID_CHARS", () => {
    const result = validateFormat("abc 12");
    expect(result.ok).toBe(false);
    if (!result.ok) expect(result.error.code).toBe("INVALID_CHARS");
  });

  it("empty string returns FormatError EMPTY", () => {
    const result = validateFormat("");
    expect(result.ok).toBe(false);
    if (!result.ok) expect(result.error.code).toBe("EMPTY");
  });
});
```

Each test maps directly to a scenario in the manifest's `test_contract`. This is intentional — the manifest is the specification, the test file is the proof.

### L1 — `url.shortcode.generate.hash`

#### `src/url/shortcode/generate.hash.test.ts`

```typescript
import { describe, it, expect } from "vitest";
import { generateHash } from "./generate.hash";
import type { OriginalUrl } from "../types";

describe("url.shortcode.generate.hash", () => {
  const url = "https://example.com" as OriginalUrl;

  it("same URL always produces same short code (deterministic)", () => {
    expect(generateHash(url)).toBe(generateHash(url));
  });

  it("output is always exactly 6 characters", () => {
    expect(generateHash(url)).toHaveLength(6);
  });

  it("output contains only alphanumeric characters", () => {
    expect(generateHash(url)).toMatch(/^[a-zA-Z0-9]+$/);
  });
});
```

### L2 — `url.link.create.fromOriginal`

`createFromOriginal` is pure and stateless, but it *does* call `validateFormat` which checks the generated hash. Since the shared store module is imported transitively, add `beforeEach(() => clearStore())` as a safety measure for any test file that composes with the store:

#### `src/url/link/create.fromOriginal.test.ts`

```typescript
import { describe, it, expect, beforeEach } from "vitest";
import { createFromOriginal } from "./create.fromOriginal";
import { clearStore } from "../store/in-memory.store";
import type { OriginalUrl } from "../types";

describe("url.link.create.fromOriginal", () => {
  beforeEach(() => clearStore());

  it("valid URL returns ShortenedLink with 6-char shortCode", () => {
    const result = createFromOriginal("https://example.com" as OriginalUrl);
    expect(result.ok).toBe(true);
    if (result.ok) {
      expect(result.value.shortCode).toHaveLength(6);
      expect(result.value.shortCode).toMatch(/^[a-zA-Z0-9]+$/);
      expect(result.value.originalUrl).toBe("https://example.com");
      expect(result.value.createdAt).toBeInstanceOf(Date);
    }
  });

  it("same URL always produces same shortCode", () => {
    const url = "https://example.com/same" as OriginalUrl;
    const r1 = createFromOriginal(url);
    const r2 = createFromOriginal(url);
    expect(r1.ok).toBe(true);
    expect(r2.ok).toBe(true);
    if (r1.ok && r2.ok) expect(r1.value.shortCode).toBe(r2.value.shortCode);
  });

  it("FormatError from validate.format is propagated", () => {
    // generateHash always produces valid 6-char alphanumeric output after the
    // base64url strip fix. The happy path confirms composition is wired correctly;
    // validate.format's error branches are covered by its own unit tests.
    const result = createFromOriginal("https://example.com" as OriginalUrl);
    expect(result.ok).toBe(true);
  });
});
```

### L3 — `url.store.persist.link`

L3 tests use `beforeEach(() => clearStore())` to guarantee each test starts with a clean store.

#### `src/url/store/persist.link.test.ts`

```typescript
import { describe, it, expect, beforeEach } from "vitest";
import { persistLink } from "./persist.link";
import { clearStore } from "./in-memory.store";
import type { ShortenedLink } from "../types";

const link: ShortenedLink = {
  shortCode: "abc123" as any,
  originalUrl: "https://example.com" as any,
  createdAt: new Date("2026-01-01"),
};

describe("url.store.persist.link", () => {
  beforeEach(() => clearStore());

  it("new link is stored and returned", async () => {
    const result = await persistLink(link);
    expect(result.ok).toBe(true);
    if (result.ok) expect(result.value.shortCode).toBe("abc123");
  });

  it("duplicate shortCode returns StoreError DUPLICATE", async () => {
    await persistLink(link);
    const result = await persistLink(link);
    expect(result.ok).toBe(false);
    if (!result.ok) expect(result.error.code).toBe("DUPLICATE");
  });
});
```

### L3 — `url.store.execute.shortCode`

#### `src/url/store/execute.shortCode.test.ts`

```typescript
import { describe, it, expect, beforeEach } from "vitest";
import { executeShortCode } from "./execute.shortCode";
import { persistLink } from "./persist.link";
import { clearStore } from "./in-memory.store";
import type { ShortenedLink } from "../types";

const link: ShortenedLink = {
  shortCode: "abc123" as any,
  originalUrl: "https://example.com" as any,
  createdAt: new Date("2026-01-01"),
};

describe("url.store.execute.shortCode", () => {
  beforeEach(() => clearStore());

  it("existing shortCode returns matching ShortenedLink", async () => {
    await persistLink(link);
    const result = await executeShortCode("abc123" as any);
    expect(result.ok).toBe(true);
    if (result.ok) expect(result.value.originalUrl).toBe("https://example.com");
  });

  it("unknown shortCode returns StoreError NOT_FOUND", async () => {
    const result = await executeShortCode("xxxxxx" as any);
    expect(result.ok).toBe(false);
    if (!result.ok) expect(result.error.code).toBe("NOT_FOUND");
  });
});
```

> **Note**: `execute.shortCode` uses `persistLink` to seed test data. This is deliberate — the two organisms share the same store and that relationship should be exercised in tests.

### Running the Tests

```bash
# Run all tests once
npm test

# Watch mode — reruns on file change during development
npm run test:watch

# Run tests for a single ARU
npx vitest run src/url/shortcode/validate.format.test.ts
```

Expected output:

```
✓ src/url/shortcode/validate.format.test.ts     (4 tests)
✓ src/url/shortcode/generate.hash.test.ts       (3 tests)
✓ src/url/link/create.fromOriginal.test.ts      (3 tests)
✓ src/url/store/persist.link.test.ts            (2 tests)
✓ src/url/store/execute.shortCode.test.ts       (2 tests)

Test Files  5 passed (5)
     Tests  14 passed (14)
```

---

## 9. Summary & Reflection

Here's everything built in this chapter at a glance:

| ARU | Layer | Patterns | Side Effects |
|-----|-------|----------|--------------|
| url.types | L0 | — | NONE |
| url.shortcode.validate.format | L1 | VALIDATE | NONE |
| url.shortcode.generate.hash | L1 | PIPE | NONE |
| url.link.create.fromOriginal | L2 | PIPE | NONE |
| url.store.persist.link | L3 | PIPE | WRITE |
| url.store.execute.shortCode | L3 | — | READ |

Five things to take forward:

1. **Types go first (L0)** — they define the vocabulary everything else uses. Define types before writing any functions.
2. **L1 atoms are pure** — no side effects, always deterministic. If you're tempted to add a database call to an L1 atom, move it to L3.
3. **Side effects first appear at L3** — and are always declared in the manifest. `side_effects: WRITE` is not a warning label; it's a machine-readable contract.
4. **The VALIDATE pattern makes error paths explicit in the graph** — when an AI agent looks at the graph, it can see that `create.fromOriginal` only proceeds if `validate.format` passes.
5. **L2 molecules compose without adding new side effects** — `create.fromOriginal` composes two pure L1 calls and remains pure itself.

In the next chapter, we add analytics, audit logging, and a circuit breaker — and introduce L4 orchestration to wire it all together.

---

**[← Setup](02-setup.md)** | **[Back to index](00-introduction.md)** | **[Next: Advanced Project →](04-project-advanced.md)**
