import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * CIRCUIT_BREAKER: Wraps an ARU with open/half-open/closed state management.
 * Emits a CircuitStore contract interface alongside the function type.
 */
export function generateCircuitBreaker(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, open_state_type } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const openState = open_state_type ?? 'unknown';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

export type ${name}CircuitState = 'CLOSED' | 'OPEN' | 'HALF_OPEN';

/** Mutable circuit state store — must be injected by the caller */
export interface ${name}CircuitStore {
  getState(): ${name}CircuitState;
  recordSuccess(): void;
  recordFailure(): void;
  getOpenStateData(): ${openState};
}

export type ${name}Fn = (
  input: ${name}Input,
  store: ${name}CircuitStore,
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}
