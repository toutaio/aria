import { generateSyntheticManifest } from './helpers/generate-manifest.js';
import { writeFile, mkdir, rm } from 'fs/promises';
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { tmpdir } from 'os';

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROTO_DIR = join(__dirname, '..');
const CLI = join(PROTO_DIR, 'src/cli.js');

describe('aria-build check benchmark (Phase 1 acceptance criterion)', () => {
  let tmpDir;

  beforeAll(async () => {
    tmpDir = join(tmpdir(), `aria-bench-${Date.now()}`);
    await mkdir(tmpDir, { recursive: true });

    // Generate 50 synthetic valid manifests
    for (let i = 0; i < 50; i++) {
      const manifest = generateSyntheticManifest(i);
      const yaml = manifestToYaml(manifest);
      await writeFile(join(tmpDir, `${manifest.manifest.id.replace(/\./g, '-')}.manifest.yaml`), yaml);
    }
  });

  afterAll(async () => {
    await rm(tmpDir, { recursive: true, force: true });
  });

  test('50 manifests validate in under 2 seconds', () => {
    const start = Date.now();
    try {
      execSync(`node ${CLI} check ${tmpDir}`, { encoding: 'utf8', cwd: PROTO_DIR });
    } catch (err) {
      // If there are validation errors, let the timing assertion still run
      if (err.status > 1) throw err;
    }
    const elapsed = Date.now() - start;
    expect(elapsed).toBeLessThan(2000);
  });
});

function manifestToYaml(obj) {
  // Minimal YAML serializer for test fixtures
  const m = obj.manifest;
  return `manifest:
  id: "${m.id}"
  version: "1.0.0"
  schema_version: "1.0"
  identity:
    purpose: "${m.identity.purpose}"
    domain: "${m.identity.domain}"
    subdomain: "${m.identity.subdomain}"
    verb: "${m.identity.verb}"
    entity: "${m.identity.entity}"
  layer:
    declared: ${m.layer.declared}
    inferred: ${m.layer.inferred}
  contract:
    input:
      type: "InputType${m.identity.entity}"
    output:
      success: "SuccessType${m.identity.entity}"
      failure: "DomainError.FAILED"
    side_effects: NONE
    idempotent: true
    deterministic: true
  dependencies:
    - id: "domain.entity"
      layer: L0
      stability: STABLE
  context_budget:
    to_use: 100
    to_modify: 300
    to_extend: 500
    to_replace: 700
  test_contract:
    scenarios:
      - scenario: "Happy path passes"
    coverage_required: true
  stability: STABLE
  lifecycle:
    phase: STABLE
    stable_since: "2024-01-01T00:00:00Z"
  manifest_provenance:
    derived_by: "STATIC_ANALYSIS"
    reviewed_by: "REVIEWER_AGENT"
    approved_at: "2024-01-01T00:00:00Z"
`;
}
