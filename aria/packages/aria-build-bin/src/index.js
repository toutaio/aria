/**
 * @aria/build-bin — platform binary resolver.
 *
 * Resolves the correct platform-specific aria-build and aria-lsp binary paths
 * by looking up the installed optional platform package.
 *
 * Platform packages follow the pattern:
 *   aria-build-bin-{os}-{arch}
 *
 * Supported targets:
 *   linux-x64, linux-arm64, darwin-x64, darwin-arm64, win32-x64
 */

import { createRequire } from 'module';
import { platform, arch } from 'os';

const require = createRequire(import.meta.url);

/** Map Node.js os/arch to platform package suffix */
function platformSuffix() {
  const os = platform();  // 'linux' | 'darwin' | 'win32'
  const cpu = arch();     // 'x64' | 'arm64' | etc.

  const supported = [
    'linux-x64',
    'linux-arm64',
    'darwin-x64',
    'darwin-arm64',
    'win32-x64',
  ];

  const key = `${os}-${cpu}`;
  if (!supported.includes(key)) {
    throw new Error(
      `@aria/build-bin: unsupported platform ${key}. ` +
      `Supported: ${supported.join(', ')}`
    );
  }
  return key;
}

/** Resolve the absolute path to the aria-build binary */
export function getBinPath() {
  const suffix = platformSuffix();
  const pkgName = `aria-build-bin-${suffix}`;
  try {
    const pkgJson = require(`${pkgName}/package.json`);
    const binName = pkgJson.name.includes('win32') ? 'aria-build.exe' : 'aria-build';
    const pkgDir = require.resolve(`${pkgName}/package.json`).replace(/package\.json$/, '');
    return `${pkgDir}bin/${binName}`;
  } catch {
    throw new Error(
      `@aria/build-bin: platform package ${pkgName} not installed. ` +
      `Run \`npm install\` to install optional dependencies.`
    );
  }
}

/** Resolve the absolute path to the aria-lsp binary */
export function getLspPath() {
  // aria-lsp ships in the same binary package as aria-build
  return getBinPath().replace(/aria-build(\.exe)?$/, 'aria-lsp$1');
}
