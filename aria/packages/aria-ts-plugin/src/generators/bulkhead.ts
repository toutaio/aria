import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** BULKHEAD: T → Result<U, E | BulkheadRejected> with pool injection */
export function generateBulkhead(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, pool_name, capacity, queue_overflow_type } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const overflowT = queue_overflow_type ?? `${name}BulkheadRejected`;
  const capacityComment = capacity !== undefined ? `\n/** Pool capacity: ${capacity} */` : '';
  const poolNameComment = pool_name ? `\n/** Pool name: ${pool_name} */` : '';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};
${capacityComment}${poolNameComment}
export interface ${name}BulkheadRejected {
  readonly _tag: 'BulkheadRejected';
  readonly pool: string;
  readonly capacity: number;
}

export type ${name}Fn = (
  input: ${name}Input,
) => Promise<Result<${name}Output, ${name}Error | ${overflowT}>>;
`;
}
