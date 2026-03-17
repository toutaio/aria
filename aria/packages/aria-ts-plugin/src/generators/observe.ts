import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * OBSERVE: A → (A, Event)
 * Side-channel event emission. Main flow is unchanged.
 * Observer failures are isolated — they never propagate to the main flow.
 */
export function generateObserve(doc: ManifestDoc): string {
  const { input_type, output_type, error_types } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const eventType = output_type ?? 'unknown';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Event = ${eventType};
export type ${name}Error = ${errUnion};

/** Observer receives a copy of the input. Failures are swallowed — never propagate. */
export type ${name}ObserverFn = (
  input: ${name}Input,
) => Promise<void>;

/** Main flow passes through unchanged; side-channel event is emitted. */
export type ${name}Fn = (
  input: ${name}Input,
  observer: ${name}ObserverFn,
) => Promise<Result<${name}Input, ${name}Error>>;
`;
}
