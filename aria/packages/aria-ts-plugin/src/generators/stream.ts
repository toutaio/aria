import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * STREAM: A → B*
 * Processes a lazy/infinite sequence element-by-element.
 * Backpressure strategies: DROP | BUFFER(n) | ERROR
 */
export function generateStream(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, source_aru, processor_aru, backpressure } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const bpStrategy = backpressure ?? 'BUFFER(100)';
  const sourceComment = source_aru
    ? `// Source ARU: ${source_aru}`
    : '// No source_aru declared — provide one in the manifest';
  const processorComment = processor_aru
    ? `// Processor ARU: ${processor_aru}`
    : '// No processor_aru declared — provide one in the manifest';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

${sourceComment}
export type ${name}SourceFn = () => AsyncIterable<${name}Input>;

${processorComment}
export type ${name}ProcessorFn = (
  element: ${name}Input,
) => Promise<Result<${name}Output, ${name}Error>>;

/** Backpressure strategy: ${bpStrategy} */
export type ${name}Fn = (
  source: ${name}SourceFn,
  processor: ${name}ProcessorFn,
) => AsyncIterable<Result<${name}Output, ${name}Error>>;
`;
}
