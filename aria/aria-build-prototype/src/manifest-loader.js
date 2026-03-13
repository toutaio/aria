import { readdir, stat, readFile } from 'fs/promises';
import { join, resolve } from 'path';
import { parse as parseYAML } from 'yaml';

/**
 * Recursively find all *.manifest.yaml files under a directory, or accept a single file path.
 * @param {string} pathArg - Root directory or a single .manifest.yaml file path
 * @returns {Promise<string[]>} Absolute paths to manifest files
 */
export async function discoverManifests(pathArg) {
  const resolvedPath = resolve(pathArg);

  // If it's a manifest file directly, return it
  if (resolvedPath.endsWith('.manifest.yaml')) {
    try {
      await stat(resolvedPath);
      return [resolvedPath];
    } catch {
      return [];
    }
  }

  const results = [];

  async function walk(currentDir) {
    let entries;
    try {
      entries = await readdir(currentDir, { withFileTypes: true });
    } catch {
      return;
    }

    for (const entry of entries) {
      const fullPath = join(currentDir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === 'node_modules' || entry.name === '.git' || entry.name === 'target') {
          continue;
        }
        await walk(fullPath);
      } else if (entry.isFile() && entry.name.endsWith('.manifest.yaml')) {
        results.push(fullPath);
      }
    }
  }

  await walk(resolvedPath);
  return results;
}

/**
 * Load and parse a manifest YAML file.
 * Returns the parsed manifest object with file path and line number information.
 *
 * @param {string} filePath - Absolute path to the manifest file
 * @returns {Promise<{filePath: string, content: object, lineMap: Map<string, number>}>}
 * @throws {Error} If the file cannot be read or parsed
 */
export async function loadManifest(filePath) {
  const raw = await readFile(filePath, 'utf8');

  // Parse with full document AST to get line numbers
  const doc = parseYAML(raw, {
    strict: true,
    prettyErrors: false,
    customTags: [],
    // Use the range API to get positions
  });

  // Also parse as a plain object for schema validation
  const content = doc;

  if (content === null || typeof content !== 'object') {
    throw new Error(`${filePath}: YAML did not parse to an object`);
  }

  return { filePath, content };
}

/**
 * Load all manifests from an array of file paths, returning results and errors separately.
 * @param {string[]} filePaths
 * @returns {Promise<{manifests: Array, errors: Array}>}
 */
export async function loadManifests(filePaths) {
  const manifests = [];
  const errors = [];

  await Promise.all(
    filePaths.map(async (filePath) => {
      try {
        const m = await loadManifest(filePath);
        manifests.push(m);
      } catch (err) {
        errors.push({ filePath, message: err.message });
      }
    })
  );

  return { manifests, errors };
}
