import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/**
 * CACHE: A → A (memo)
 * Transparent memoisation. On hit, returns stored value. On miss, executes and stores.
 */
export function generateCache(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, key_aru } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const keyComment = key_aru
    ? `// Key derivation ARU: ${key_aru}`
    : '// No key_aru declared — provide one in the manifest';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}Error = ${errUnion};

${keyComment}
export type ${name}KeyFn = (input: ${name}Input) => string;

/** Cache store interface — inject the implementation */
export interface ${name}CacheStore {
  get(key: string): Promise<${name}Output | undefined>;
  set(key: string, value: ${name}Output): Promise<void>;
}

/** Returns cached value on hit; executes and caches on miss */
export type ${name}Fn = (
  input: ${name}Input,
  store: ${name}CacheStore,
  keyFn: ${name}KeyFn,
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}
