/**
 * Load and parse a .manifest.yaml file into a ManifestDoc for code generation.
 */

import { readFileSync } from 'node:fs';
import { load } from 'js-yaml';
import type { ManifestDoc, ManifestComposition, ManifestIdentity } from './manifest-types.js';
import { canonicalHash } from './canonical-hash.js';

interface RawManifest {
  identity?: {
    address?: string;
    layer?: number;
  };
  composition?: ManifestComposition;
}

export function loadManifest(filePath: string): ManifestDoc {
  const raw = readFileSync(filePath, 'utf8');
  const parsed = load(raw) as RawManifest;

  if (!parsed?.identity?.address) {
    throw new Error(`${filePath}: missing identity.address`);
  }
  if (!parsed?.composition?.pattern) {
    throw new Error(`${filePath}: missing composition.pattern`);
  }

  const identity: ManifestIdentity = {
    address: parsed.identity.address,
    layer: parsed.identity.layer ?? 0,
  };

  const composition = parsed.composition as ManifestComposition;
  const compositionHash = canonicalHash(composition);

  return { identity, composition, compositionHash };
}
