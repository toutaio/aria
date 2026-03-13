# Test Infrastructure
### Sixth Iteration — From scenario declarations to executable verification

---

## The Test Contract as Definition of Done

Every ARU manifest includes a `test_contract` section (`20-manifest-schema.md §Section 9`):

```yaml
test_contract:
  - scenario: "valid unexpired token returns ValidatedToken with correct claims"
  - scenario: "expired token returns AuthError with code EXPIRED"
  - scenario: "tampered signature returns AuthError with code INVALID_SIGNATURE"
```

These are not documentation. They are the **definition of done** for every ARU. An ARU is not
CANDIDATE until all declared scenarios have corresponding executable tests that pass.

But no prior document specifies: what is a "corresponding test"? Who creates it? How does
the Reviewer verify coverage? This document answers those questions.

---

## The Scenario-to-Test Mapping

### Scenario Grammar

Every test scenario follows the **Given-When-Then** structure, even when written as a sentence:

```
"valid unexpired token returns ValidatedToken with correct claims"

Implicit structure:
  Given: a token that is structurally valid and not expired
  When:  auth.token.validate.signature(token) is called
  Then:  the result is { success: true, value: ValidatedToken } with matching claims
```

The Generator agent expands each scenario into this three-part structure when generating tests.
If a scenario cannot be decomposed into G/W/T, it is **underspecified** and must be clarified
before the ARU proceeds to DRAFT.

### Scenario Types

| Scenario Type | Trigger | Expected Output |
|---|---|---|
| **Happy path** | Valid, typical input | Success-track output |
| **Error path** | Each declared error variant | Specific `ErrorType.CODE` |
| **Boundary case** | Edge of type constraints | Success or specific error |
| **Invariant** | Post-condition that must always hold | Structural assertion |
| **Type state** | State machine transition | Output type in correct state |

Every ARU's test contract must include at minimum:
- One happy path scenario
- One scenario per declared error variant
- One boundary case per constraint declared in `contract.input.constraints`

---

## Test Generation Protocol

### Who Generates Tests

The **Generator agent** generates initial test stubs as part of the `ARU_CREATION` subtask.
Test generation is **not a separate subtask** — it is part of creating the ARU.

The Generator's output for an ARU_CREATION subtask is always three files:
1. ARU implementation file
2. ARU manifest file
3. ARU test file (generated from test_contract scenarios)

```
Orchestrator: ARU_CREATION subtask → Generator
Generator produces:
  auth/token/validate/signature.ts           ← implementation
  auth/token/validate/signature.manifest.yaml ← manifest
  auth/token/validate/signature.test.ts      ← generated tests
```

### Test File Structure

Each scenario maps to one test case. The Generator follows a deterministic structure:

```typescript
// GENERATED from test_contract — validate with: aria test verify auth.token.validate.signature
// Modify scenarios in the manifest, then regenerate. Do not manually restructure.

import { validateSignature } from "./signature";
import { buildValidToken, buildExpiredToken, buildTamperedToken } from "@aria/test-fixtures";

describe("auth.token.validate.signature", () => {

  // Scenario: "valid unexpired token returns ValidatedToken with correct claims"
  it("returns ValidatedToken with correct claims for a valid token", async () => {
    const token = buildValidToken({ sub: "user-123", exp: future() });
    const result = await validateSignature(token);
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.value.claims.sub).toBe("user-123");
    }
  });

  // Scenario: "expired token returns AuthError with code EXPIRED"
  it("returns AuthError.EXPIRED for an expired token", async () => {
    const token = buildExpiredToken();
    const result = await validateSignature(token);
    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.code).toBe("EXPIRED");
    }
  });

  // Scenario: "tampered signature returns AuthError with code INVALID_SIGNATURE"
  it("returns AuthError.INVALID_SIGNATURE for a tampered token", async () => {
    const token = buildTamperedToken();
    const result = await validateSignature(token);
    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.code).toBe("INVALID_SIGNATURE");
    }
  });

});
```

### Test Fixture Protocol

Test fixtures are **typed builders**, not raw mocks. Every L0 type has a corresponding
`build{TypeName}(overrides?)` fixture function:

```typescript
// Generated from L0 type registry — one fixture per type
buildValidToken(overrides?: Partial<TokenClaims>): TokenString
buildExpiredToken(): TokenString
buildTamperedToken(): TokenString
buildValidatedToken(overrides?: Partial<ValidatedToken>): ValidatedToken
```

Fixtures are generated automatically when new L0 types are added to the registry. The Generator
uses fixtures in test code — never raw literals or `as any` casts.

---

## Reviewer Verification Mechanism

### How the Reviewer Checks Coverage

The Reviewer does not use LLM judgment to verify test coverage. Coverage verification is
**mechanical** — a structured comparison between manifest scenarios and test file annotations:

```
Reviewer verification algorithm:
  1. Read test_contract from manifest → list of scenario strings S
  2. Read test file → extract all "// Scenario: ..." comments → list T
  3. For each s in S:
       if no t in T where normalize(s) == normalize(t):
         → COVERAGE_GAP: scenario "s" has no corresponding test
  4. If any coverage gaps: reject → return to Generator with list of missing scenarios
  5. If all covered: proceed to compile + run tests
```

The `normalize()` function strips punctuation, lowercases, and trims whitespace.
This is not semantic matching — it is exact string matching after normalization.

This means: **the `// Scenario: ...` comment in the test file must match the manifest scenario
string exactly** (after normalization). The Generator is responsible for this. It is a build
constraint, not a style suggestion.

### Coverage Gate

The DRAFT → CANDIDATE transition is blocked if:
- Any declared scenario has no corresponding test
- Any test fails
- Type errors exist in the test file

A test file that compiles and has no scenario gaps but has all tests marked `it.skip` is
a build warning, not a build failure — but the Reviewer must explicitly note it.

---

## Composition Chain Tests

Per-ARU tests verify individual ARU behavior. They do not verify **railway semantics** —
the behavior of the PIPE chain as a whole.

Composition chains require **chain-level tests** in addition to per-ARU tests.

### Chain Test Structure

A chain test is generated alongside the composition wrapper (see `21-runtime-composition.md`):

```typescript
// GENERATED composition test — auth.session.execute.loginFlow
describe("auth.session.execute.loginFlow [COMPOSITION]", () => {

  // Scenario: "complete login flow succeeds with valid credentials"
  it("returns UserSession for valid token", async () => {
    const token = buildValidToken({ sub: "user-123" });
    const result = await loginFlow(token, mockTraceContext());
    expect(result.success).toBe(true);
  });

  // Scenario: "expired token short-circuits at validation step"
  it("short-circuits at step 1 and returns EXPIRED error for expired token", async () => {
    const token = buildExpiredToken();
    const result = await loginFlow(token, mockTraceContext());
    expect(result.success).toBe(false);
    if (!result.success) {
      // Verify short-circuit: error comes from step 1, not downstream
      expect(result.error.origin_aru).toBe("auth.token.validate.signature");
      expect(result.error.error.code).toBe("EXPIRED");
    }
  });

  // Scenario: "error handler receives RailError with correct provenance"
  it("delivers RailError with origin_aru and trace_id to error handler", async () => {
    const token = buildTamperedToken();
    const ctx = mockTraceContext({ correlationId: "test-cid-001" });
    const result = await loginFlow(token, ctx);
    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.trace_id).toBe("test-cid-001");
      expect(result.error.origin_aru).toBeDefined();
    }
  });

});
```

Chain test scenarios are declared in the **composition ARU's manifest** `test_contract` section.
They are generated by the same Generator agent at composition creation time.

---

## Mutation Testing

Mutation testing is the strongest available signal that tests are **genuinely complete** (not just coverage-complete).

### When Mutation Testing Is Required

```yaml
test_contract:
  mutation_testing: true    # required for L3+ ARUs; optional for L1–L2
```

The Reviewer checks this field for L3+ ARUs. If `mutation_testing: false` on an L3 ARU, the
Reviewer requests justification before approving.

### What Mutation Testing Verifies

A mutation test suite introduces small changes to the ARU implementation (mutations) and verifies
that at least one test fails for each mutation. Common mutation types:

| Mutation | Example | Test Should Catch |
|---|---|---|
| Boundary flip | `>` → `>=` | Boundary case scenario |
| Error code swap | `EXPIRED` → `INVALID_SIGNATURE` | Each error path scenario |
| Null insertion | `return value` → `return null` | Happy path scenario |
| Condition inversion | `if (valid)` → `if (!valid)` | Both paths scenario |

A "surviving mutation" (a change that no test catches) is a coverage gap. Surviving mutations
block the CANDIDATE transition with the same severity as a missing scenario.

### Mutation Testing in CI

The build pipeline runs mutation testing only on ARUs with `mutation_testing: true`. This is
expensive — it should not run on every file. L3+ ARUs encoding business rules are the target.

---

## Test Infrastructure Summary

| Concern | Who Handles It | How |
|---|---|---|
| Scenario declaration | Generator (Tier 2 proposal) + human approval | manifest `test_contract` |
| Test stub generation | Generator agent | From scenario strings at creation time |
| Fixture generation | Build tooling | From L0 type registry |
| Coverage verification | Reviewer agent | Mechanical scenario↔comment matching |
| Test execution | CI pipeline | Standard language test runner |
| Mutation testing | CI pipeline (L3+ only) | Language-specific mutation tool |
| Chain-level test generation | Build tooling (with composition wrapper) | From composition manifest |
| Known failure pattern update | Reviewer agent (after human approval) | Write-back to manifest |

---

## Test File Naming and Location

Test files follow the semantic address naming convention:

```
src/
  auth/
    token/
      validate/
        signature.ts                    ← ARU implementation
        signature.manifest.yaml         ← ARU manifest
        signature.test.ts               ← ARU unit tests (generated)
    session/
      execute/
        loginFlow.ts                    ← composition ARU (generated wrapper)
        loginFlow.manifest.yaml
        loginFlow.test.ts               ← per-ARU tests
        loginFlow.composition.test.ts   ← chain-level composition tests (generated)
```

The `.composition.test.ts` suffix distinguishes chain tests from per-ARU tests.
Both run in the same test suite but are generated by different processes.
