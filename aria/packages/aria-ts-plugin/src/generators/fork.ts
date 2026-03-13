import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** FORK: T → { branchA: Result<U,E>, branchB: Result<U,E>, ... } */
export function generateFork(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, branches = [] } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const branchFields = branches.length > 0
    ? branches.map(b => `  ${b}: Result<${output_type}, ${errUnion}>;`).join('\n')
    : `  // no branches declared\n  [branch: string]: Result<${output_type}, ${errUnion}>;`;
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

export interface ${name}ForkResult {
${branchFields}
}

export type ${name}Fn = (
  input: ${name}Input,
) => Promise<${name}ForkResult>;
`;
}
