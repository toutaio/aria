import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** COMPENSATING_TRANSACTION: forward + compensation ARU pair */
export function generateCompensatingTransaction(doc: ManifestDoc): string {
  const { input_type, output_type, error_types } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

export interface ${name}CompensationContext {
  originalInput: ${name}Input;
  partialOutput?: Partial<${name}Output>;
}

export type ${name}ForwardFn = (
  input: ${name}Input,
) => Promise<Result<${name}Output, ${name}Error>>;

export type ${name}CompensationFn = (
  ctx: ${name}CompensationContext,
) => Promise<void>;
`;
}
