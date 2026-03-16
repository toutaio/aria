#!/usr/bin/env node
import { Command } from 'commander';
import chalk from 'chalk';
import { discoverManifests, loadManifests } from './manifest-loader.js';
import { validateSchema } from './schema-validator.js';
import { checkNaming } from './naming-checker.js';
import { loadConfig, COMPLIANCE_LEVELS } from './config.js';

const program = new Command();

program
  .name('aria-build')
  .description('ARIA build tool — manifest validation and naming enforcement (Phase 1 prototype)')
  .version('0.1.0');

program
  .command('check [dir]')
  .description('Validate all *.manifest.yaml files in the project')
  .option('-l, --compliance-level <n>', 'Compliance level (0–5, default: 5)', '5')
  .option('--format <fmt>', 'Output format: text or json (default: text)', 'text')
  .action(async (dir = '.', options) => {
    const projectRoot = dir;
    const config = await loadConfig(projectRoot);
    const complianceLevel = options.complianceLevel !== undefined
      ? parseInt(options.complianceLevel, 10)
      : (config.compliance_level ?? COMPLIANCE_LEVELS.ALL);

    if (isNaN(complianceLevel) || complianceLevel < 0 || complianceLevel > 5) {
      console.error(chalk.red('[ERROR] --compliance-level must be between 0 and 5'));
      process.exit(1);
    }

    const isJson = options.format === 'json';
    const allDiagnostics = [];

    // Discover manifests
    const filePaths = await discoverManifests(projectRoot);

    if (!isJson && filePaths.length === 0) {
      console.log(chalk.yellow('[WARN] No *.manifest.yaml files found in ' + projectRoot));
      process.exit(0);
    }

    // Load manifests
    const { manifests, errors: loadErrors } = await loadManifests(filePaths);

    for (const err of loadErrors) {
      allDiagnostics.push({
        severity: 'ERROR',
        file: err.filePath,
        line: null,
        message: `[ERROR] ${err.filePath}: failed to parse YAML — ${err.message}`,
      });
    }

    // Level 0: Schema validation
    if (complianceLevel >= COMPLIANCE_LEVELS.SCHEMA_VALIDATION) {
      for (const { filePath, content } of manifests) {
        const { errors } = await validateSchema(filePath, content);
        allDiagnostics.push(...errors.map(e => ({ severity: 'ERROR', file: e.file, line: e.line, message: e.message })));
      }
    }

    // Level 1: Manifest presence (stub — naming checker used as proxy until presence checker is wired)
    if (complianceLevel >= COMPLIANCE_LEVELS.MANIFEST_PRESENCE) {
      const namingDiags = checkNaming(manifests);
      allDiagnostics.push(...namingDiags);
    }

    const errorCount = allDiagnostics.filter(d => d.severity === 'ERROR').length;
    const warnCount = allDiagnostics.filter(d => d.severity === 'WARN').length;

    if (isJson) {
      console.log(JSON.stringify({
        manifests_checked: filePaths.length,
        compliance_level: complianceLevel,
        errors: errorCount,
        warnings: warnCount,
        diagnostics: allDiagnostics,
      }, null, 2));
    } else {
      for (const diag of allDiagnostics) {
        if (diag.severity === 'ERROR') {
          console.error(chalk.red(diag.message));
        } else {
          console.warn(chalk.yellow(diag.message));
        }
      }

      if (errorCount === 0 && warnCount === 0) {
        if (!isJson) {
          console.log(chalk.green(`✓ ${filePaths.length} manifest(s) valid (compliance level ${complianceLevel})`));
        }
      } else {
        if (!isJson) {
          console.error(chalk.red(`\n✗ ${errorCount} error(s), ${warnCount} warning(s) in ${filePaths.length} manifest(s)`));
        }
      }
    }

    process.exit(errorCount > 0 ? 1 : 0);
  });

program.parse();
