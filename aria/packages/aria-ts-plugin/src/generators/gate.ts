import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * GATE: T → T | ∅
 * If the predicate ARU returns true the value passes through; otherwise it is discarded.
 * A false predicate is NOT a failure — it is a drop.
 */
export function generateGate(doc: ManifestDoc): string {
  const { input_type, error_types, predicate_aru } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const predicateComment = predicate_aru
    ? `// Predicate ARU: ${predicate_aru}`
    : '// No predicate_aru declared — provide one in the manifest';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Error = ${errUnion};

${predicateComment}
export type ${name}PredicateFn = (input: ${name}Input) => Promise<boolean>;

/** Returns the input unchanged when the predicate is true; null when it is false (drop). */
export type ${name}Fn = (
  input: ${name}Input,
  predicate: ${name}PredicateFn,
) => Promise<Result<${name}Input | null, ${name}Error>>;
`;
}
