import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** STREAMING_PIPELINE: AsyncIterable<Chunk> → AsyncIterable<U> */
export function generateStreamingPipeline(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, chunk_type } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const chunkT = chunk_type ?? input_type;
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Chunk = ${chunkT};
export type ${name}Error = ${errUnion};

export type ${name}Fn = (
  input: AsyncIterable<${name}Chunk>,
) => AsyncIterable<Result<${name}Output, ${name}Error>>;
`;
}
