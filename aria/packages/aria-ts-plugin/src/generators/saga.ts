import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** SAGA: Sequence of steps, each with optional compensation */
export function generateSaga(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, steps = [] } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const stepTypes = steps.map((s, i) => {
    const stepName = addressToTypeName(s.aru);
    const compLine = s.compensation ? `\n  compensate?: (input: ${stepName}Input) => Promise<void>;` : '';
    return `export interface ${name}Step${i + 1} {\n  execute: (input: ${stepName}Input) => Promise<Result<${stepName}Output, ${errUnion}>>;\n  aru: '${s.aru}';${compLine}\n}`;
  }).join('\n\n');
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

${stepTypes}

export type ${name}Fn = (
  input: ${name}Input,
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}

