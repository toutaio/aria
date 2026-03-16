import { readFile } from 'fs/promises';
import { existsSync } from 'fs';
import { join, resolve } from 'path';
import { parse as parseYAML } from 'yaml';

/**
 * Load .aria/config.yaml from the project root if it exists.
 * @param {string} projectRoot
 * @returns {Promise<{compliance_level?: number}>}
 */
export async function loadConfig(projectRoot) {
  const configPath = join(resolve(projectRoot), '.aria', 'config.yaml');
  if (!existsSync(configPath)) {
    return {};
  }
  try {
    const text = await readFile(configPath, 'utf8');
    const config = parseYAML(text, { strict: true });
    return config || {};
  } catch {
    return {};
  }
}

/**
 * Compliance levels — which checkers are active at each level.
 * Level N enables all checkers at N and below.
 *
 * Level 0: JSON Schema validation only — structural conformance
 * Level 1: + manifest presence (every source file has a .manifest.yaml)
 * Level 2: + naming enforcement (semantic addresses, verb vocabulary)
 * Level 3: + layer dependency rules (no upward imports, no cycles)
 * Level 4: + bundle freshness checks
 * Level 5: all checks (default)
 *
 * NOTE: Levels 0–1 are fully implemented. Levels 2–5 are stubs — the
 * constants exist but the corresponding checkers are not yet wired up.
 */
export const COMPLIANCE_LEVELS = {
  SCHEMA_VALIDATION: 0,
  MANIFEST_PRESENCE: 1,   // previously NAMING_ENFORCEMENT at level 1
  NAMING_ENFORCEMENT: 2,  // previously TYPE_GRAPH at level 2
  LAYER_RULES: 3,         // previously COMPOSITION_CODEGEN at level 3
  BUNDLE_FRESHNESS: 4,
  ALL: 5,
};
