import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** PARALLEL_JOIN: Array<T> → ThreeTrack<U, Partial<U>, E> */
export function generateParallelJoin(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, minimum_required_results } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const minResultsComment = minimum_required_results !== undefined
    ? `\n/** Minimum required successful results: ${minimum_required_results} */`
    : '';
  return `import type { ThreeTrack } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}PartialOutput = Partial<${output_type}>;
export type ${name}Error = ${errUnion};
${minResultsComment}
export type ${name}Fn = (
  inputs: ReadonlyArray<${name}Input>,
) => Promise<ThreeTrack<${name}Output, ${name}PartialOutput, ${name}Error>>;
`;
}
