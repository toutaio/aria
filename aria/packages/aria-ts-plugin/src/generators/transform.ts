import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * TRANSFORM: A → A'
 * Shape change within the same semantic domain.
 * Total transforms (no failure path) set error_types to [].
 */
export function generateTransform(doc: ManifestDoc): string {
  const { input_type, output_type, error_types } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const isTotal = !error_types || error_types.length === 0;
  const errUnion = isTotal ? 'never' : (error_types?.join(' | ') ?? 'never');
  const returnType = isTotal
    ? `${name}Output`
    : `Result<${name}Output, ${name}Error>`;
  const returnImport = isTotal ? '' : `import type { Result } from '@aria/runtime';\n\n`;
  const errorLine = isTotal ? '' : `export type ${name}Error = ${errUnion};\n\n`;
  return `${returnImport}export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
${errorLine}export type ${name}Fn = (
  input: ${name}Input,
) => Promise<${returnType}>;
`;
}
