import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** PRIORITY_QUEUE: { priority: P, payload: T } → Result<U, E> */
export function generatePriorityQueue(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, priority_type } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const priorityT = priority_type ?? 'number';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Priority = ${priorityT};
export type ${name}Error = ${errUnion};

export interface ${name}PriorityEnvelope {
  priority: ${name}Priority;
  payload: ${name}Input;
}

export type ${name}Fn = (
  envelope: ${name}PriorityEnvelope,
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}
