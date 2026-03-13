#!/usr/bin/env node
/**
 * Postinstall script — validates that the platform package installed correctly
 * and prints the resolved binary path for debugging.
 */

import { getBinPath } from './index.js';

try {
  const binPath = getBinPath();
  console.log(`[aria/build-bin] resolved aria-build → ${binPath}`);
} catch (err) {
  // Not a hard failure — CI may not have the platform package installed
  // (e.g., running on an unsupported platform or in a cross-compile environment)
  console.warn(`[aria/build-bin] warning: ${err.message}`);
}
