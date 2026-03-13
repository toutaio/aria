/**
 * Canonical SHA-256 hash of a manifest's composition section.
 * Must match the Rust implementation in aria-core/src/canonical_hash.rs:
 *   - Sort keys recursively
 *   - Serialize to JSON with no extra whitespace
 *   - SHA-256 hex digest
 */

import { createHash } from 'node:crypto';

/** Recursively sort an object's keys for deterministic JSON serialization */
function sortKeys(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map(sortKeys);
  }
  if (value !== null && typeof value === 'object') {
    const obj = value as Record<string, unknown>;
    return Object.keys(obj)
      .sort()
      .reduce<Record<string, unknown>>((acc, k) => {
        acc[k] = sortKeys(obj[k]);
        return acc;
      }, {});
  }
  return value;
}

/** Compute the canonical SHA-256 hash of a manifest composition section */
export function canonicalHash(composition: unknown): string {
  const sorted = sortKeys(composition);
  const json = JSON.stringify(sorted);
  return createHash('sha256').update(json, 'utf8').digest('hex');
}
