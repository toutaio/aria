import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** SCATTER_GATHER: Array<T> → Result<Array<U>, E> */
export function generateScatterGather(doc: ManifestDoc): string {
  const { input_type, output_type, error_types } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

export type ${name}Fn = (
  inputs: ReadonlyArray<${name}Input>,
) => Promise<Result<Array<${name}Output>, ${name}Error>>;
`;
}
