/**
 * aria-ts-plugin public API
 */
export { loadManifest } from './loader.js';
export { generateWrapper } from './generators/index.js';
export { canonicalHash } from './canonical-hash.js';
export { generateFileHeader } from './file-header.js';
export { addressToTypeName, outputFilename } from './utils.js';
export type { ManifestDoc, ManifestComposition, ManifestIdentity, CompositionPattern } from './manifest-types.js';
