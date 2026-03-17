import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * ROUTE: T → BranchA | BranchB
 * Exactly one branch fires. All branches are declared; the condition ARU selects which.
 */
export function generateRoute(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, branches = [], condition_aru } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const conditionComment = condition_aru
    ? `// Condition ARU: ${condition_aru}`
    : '// No condition_aru declared — provide one in the manifest';
  const branchKeys = branches.length > 0
    ? branches.map(b => `  | '${b}'`).join('\n')
    : `  | string // no branches declared`;
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

export type ${name}Branch =
${branchKeys};

${conditionComment}
export type ${name}ConditionFn = (input: ${name}Input) => Promise<${name}Branch>;

export type ${name}BranchFn = (
  input: ${name}Input,
) => Promise<Result<${name}Output, ${name}Error>>;

export type ${name}Routes = Record<${name}Branch, ${name}BranchFn>;

export type ${name}Fn = (
  input: ${name}Input,
  condition: ${name}ConditionFn,
  routes: ${name}Routes,
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}
