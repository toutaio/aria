#!/usr/bin/env node
/**
 * Shim: delegates to the platform-specific aria-build binary.
 * This file is what gets run when `aria-build` is invoked via npx or as a local bin.
 */

import { getBinPath } from './index.js';
import { execFileSync } from 'child_process';

const binPath = getBinPath();
execFileSync(binPath, process.argv.slice(2), { stdio: 'inherit' });
