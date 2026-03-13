#!/usr/bin/env node
/**
 * aria-ts-plugin CLI
 * Usage: aria-ts-plugin generate <manifest.yaml> [--out <dir>]
 */

import { writeFileSync, mkdirSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { loadManifest } from './loader.js';
import { generateWrapper } from './generators/index.js';
import { outputFilename } from './utils.js';

const args = process.argv.slice(2);

if (args[0] !== 'generate' || !args[1]) {
  console.error('Usage: aria-ts-plugin generate <manifest.yaml> [--out <dir>]');
  process.exit(1);
}

const manifestPath = resolve(args[1]);
const outFlagIdx = args.indexOf('--out');
const outDir = outFlagIdx !== -1 && args[outFlagIdx + 1]
  ? resolve(args[outFlagIdx + 1])
  : dirname(manifestPath);

try {
  const doc = loadManifest(manifestPath);
  const code = generateWrapper(doc);
  const outFile = join(outDir, outputFilename(doc.identity.address));
  mkdirSync(outDir, { recursive: true });
  writeFileSync(outFile, code, 'utf8');
  console.log(`✓ Generated ${outFile}`);
} catch (err) {
  console.error(`✗ ${(err as Error).message}`);
  process.exit(1);
}
