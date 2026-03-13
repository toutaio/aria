#!/usr/bin/env node
/**
 * Shim: delegates to the platform-specific aria-lsp binary.
 */

import { getLspPath } from './index.js';
import { execFileSync } from 'child_process';

const binPath = getLspPath();
execFileSync(binPath, process.argv.slice(2), { stdio: 'inherit' });
