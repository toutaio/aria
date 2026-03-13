import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** PIPE: T → Result<U, E> */
export function generatePipe(doc: ManifestDoc): string {
  const { input_type, output_type, error_types } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

export type ${name}Fn = (
  input: ${name}Input,
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}
