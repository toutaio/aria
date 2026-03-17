import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * VALIDATE: A → A' | Error
 * Contract enforcement with a typed narrow output type.
 * The output type is the validated (narrowed) form of the input.
 */
export function generateValidate(doc: ManifestDoc): string {
  const { input_type, output_type, error_types } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
/** Narrowed/validated output type */
export type ${name}Validated = ${output_type};
export type ${name}Error = ${errUnion};

export type ${name}Fn = (
  input: ${name}Input,
) => Promise<Result<${name}Validated, ${name}Error>>;
`;
}
