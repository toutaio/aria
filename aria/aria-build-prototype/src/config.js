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
 * Level 0: manifest schema validation only
 * Level 1: + naming enforcement (semantic addresses, verb vocabulary)
 * Level 2: + type graph validation (composition type compatibility)
 * Level 3: + composition code generation checks
 * Level 4: + bundle freshness checks
 * Level 5: all checks
 */
export const COMPLIANCE_LEVELS = {
  SCHEMA_VALIDATION: 0,
  NAMING_ENFORCEMENT: 1,
  TYPE_GRAPH: 2,
  COMPOSITION_CODEGEN: 3,
  BUNDLE_FRESHNESS: 4,
  ALL: 5,
};
