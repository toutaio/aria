import type { ManifestDoc } from '../manifest-types.js';
import { addressToTypeName } from '../utils.js';

/** CACHE_ASIDE: T → Result<U, E> with CacheStore injection */
export function generateCacheAside(doc: ManifestDoc): string {
  const { input_type, output_type, error_types, cache_store_type, cache_key_type } = doc.composition;
  const name = addressToTypeName(doc.identity.address);
  const errUnion = error_types?.join(' | ') ?? 'never';
  const storeT = cache_store_type ?? `${name}CacheBackend`;
  const keyT = cache_key_type ?? 'string';
  return `import type { Result } from '@aria/runtime';

export type ${name}Input = ${input_type};
export type ${name}Output = ${output_type};
export type ${name}CacheKey = ${keyT};
export type ${name}Error = ${errUnion};

export interface ${name}CacheStore {
  get(key: ${name}CacheKey): Promise<${name}Output | undefined>;
  set(key: ${name}CacheKey, value: ${name}Output): Promise<void>;
  invalidate(key: ${name}CacheKey): Promise<void>;
}

export type ${name}Fn = (
  input: ${name}Input,
  cache: ${storeT} extends string ? ${name}CacheStore : ${storeT},
) => Promise<Result<${name}Output, ${name}Error>>;
`;
}
