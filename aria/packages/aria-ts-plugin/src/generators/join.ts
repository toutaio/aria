import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** JOIN: { branchA: A, branchB: B, ... } → Result<U, E> */
export function generateJoin(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, branches = [] } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const inputFields = branches.length > 0
    ? branches.map(b => `  ${b}: ${input_type};`).join('\n')
    : `  // no branches declared\n  [branch: string]: ${input_type};`;
  return `import type { Result } from '@aria/runtime';

export interface ${name}JoinInput {
${inputFields}
}

export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

export type ${name}Fn = (
  inputs: ${name}JoinInput,
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}
