import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROTO_DIR = join(__dirname, '..');
const CLI = join(PROTO_DIR, 'src/cli.js');
const VALID_DIR = join(PROTO_DIR, 'fixtures/valid-project');
const INVALID_DIR = join(PROTO_DIR, 'fixtures/invalid-project');

function runCheck(dir, extraArgs = '') {
  try {
    const result = execSync(`node ${CLI} check ${dir} ${extraArgs}`, {
      encoding: 'utf8',
      cwd: PROTO_DIR,
    });
    return { exitCode: 0, stdout: result, stderr: '' };
  } catch (err) {
    return {
      exitCode: err.status ?? 1,
      stdout: err.stdout ?? '',
      stderr: err.stderr ?? '',
    };
  }
}

describe('aria-build check integration tests', () => {
  test('valid-project: exits 0 with no errors', () => {
    const { exitCode, stderr } = runCheck(VALID_DIR);
    expect(exitCode).toBe(0);
    expect(stderr).not.toMatch(/\[ERROR\]/);
  });

  test('valid-project: --format json exits 0 and reports 0 errors', () => {
    const { exitCode, stdout } = runCheck(VALID_DIR, '--format json');
    expect(exitCode).toBe(0);
    const result = JSON.parse(stdout);
    expect(result.errors).toBe(0);
    expect(result.manifests_checked).toBeGreaterThan(0);
  });

  test('invalid-project: l3-missing-health-contract fails', () => {
    const { exitCode, stderr } = runCheck(
      join(INVALID_DIR, 'l3-missing-health-contract.manifest.yaml'),
    );
    // Schema requires health_contract for L3 — should fail
    expect(exitCode).toBe(1);
  });

  test('invalid-project: unknown-field fails', () => {
    const { exitCode, stderr } = runCheck(
      join(INVALID_DIR, 'unknown-field.manifest.yaml'),
    );
    expect(exitCode).toBe(1);
  });

  test('compliance level 0: only schema validation runs', () => {
    const { exitCode } = runCheck(VALID_DIR, '--compliance-level 0');
    expect(exitCode).toBe(0);
  });

  test('compliance level 1: naming enforcement runs', () => {
    const { exitCode } = runCheck(VALID_DIR, '--compliance-level 1');
    expect(exitCode).toBe(0);
  });
});
