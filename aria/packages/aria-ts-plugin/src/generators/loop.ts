import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * LOOP: A →[cond]→ A
 * Bounded iteration. The condition ARU determines whether to continue.
 * max_iterations prevents infinite loops.
 */
export function generateLoop(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, condition_aru, max_iterations } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const maxIter = max_iterations ?? 100;
  const conditionComment = condition_aru
    ? `// Condition ARU: ${condition_aru}`
    : '// No condition_aru declared — provide one in the manifest';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

${conditionComment}
export type ${name}ConditionFn = (state: ${name}Output) => Promise<boolean>;

export type ${name}BodyFn = (
  state: ${name}Input | ${name}Output,
) => Promise<Result<${name}Output, ${name}Error>>;

/** Max iterations: ${maxIter} */
export type ${name}Fn = (
  initial: ${name}Input,
  condition: ${name}ConditionFn,
  body: ${name}BodyFn,
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}
