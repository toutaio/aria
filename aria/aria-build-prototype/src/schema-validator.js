import Ajv from 'ajv';
import addFormats from 'ajv-formats';
import { readFile } from 'fs/promises';
import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __dirname = dirname(fileURLToPath(import.meta.url));

let _schema = null;
let _validator = null;

async function getValidator() {
  if (_validator) return _validator;

  const schemaPath = join(__dirname, '../../schema/aria-manifest.schema.json');
  const schemaText = await readFile(schemaPath, 'utf8');
  _schema = JSON.parse(schemaText);

  const ajv = new Ajv({
    allErrors: true,
    strict: true,
    strictAdditionalProperties: true,
    strictRequired: false,  // 'then' clauses reference properties defined in parent schema
  });
  addFormats(ajv);

  _validator = ajv.compile(_schema);
  return _validator;
}

/**
 * Validate a parsed manifest object against the JSON Schema.
 *
 * @param {string} filePath - Path to the manifest file (for error messages)
 * @param {object} content  - Parsed YAML content
 * @returns {Promise<{valid: boolean, errors: Array<{file: string, line: number|null, message: string}>}>}
 */
export async function validateSchema(filePath, content) {
  const validate = await getValidator();
  const valid = validate(content);

  if (valid) {
    return { valid: true, errors: [] };
  }

  const errors = (validate.errors || []).map((err) => {
    const path = err.instancePath || err.schemaPath || '';
    const field = path.replace(/^\/manifest\//, '').replace(/\//g, '.');
    const message = err.message || 'unknown schema error';
    return {
      file: filePath,
      line: null, // line numbers resolved in the checker layer
      message: `[ERROR] ${filePath}: schema violation at ${field || '(root)'} — ${message}`,
      field,
      rawError: err,
    };
  });

  return { valid: false, errors };
}
