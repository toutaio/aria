import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** EVENT_SOURCING: Command<T> → Result<Event<E>[], Error> with aggregate projection */
export function generateEventSourcing(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, event_type, aggregate_type } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const eventT = event_type ?? `${name}Event`;
  const aggregateT = aggregate_type ?? `${name}Aggregate`;
  return `import type { Result } from '@aria/runtime';

export type ${name}Command = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

export type ${name}Event = ${eventT};
export type ${name}Aggregate = ${aggregateT};

/** Apply a command to the current aggregate state, returning emitted events */
export type ${name}CommandHandlerFn = (
  command: ${name}Command,
  aggregate: ${name}Aggregate,
) => Promise<Result<Array<${name}Event>, ${name}Error>>;

/** Reduce an event into the aggregate state */
export type ${name}ProjectionFn = (
  aggregate: ${name}Aggregate,
  event: ${name}Event,
) => ${name}Aggregate;
`;
}
