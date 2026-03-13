/**
 * Utility: convert a dot-delimited semantic address to a PascalCase type name.
 * e.g. "auth.identity.authenticate.user" → "AuthIdentityAuthenticateUser"
 */
export function addressToTypeName(address: string): string {
  return address
    .split('.')
    .map(s => s.charAt(0).toUpperCase() + s.slice(1))
    .join('');
}

/**
 * Derive the output filename for a manifest.
 * e.g. "auth.identity.authenticate.user" → "auth.identity.authenticate.user.generated.ts"
 */
export function outputFilename(address: string): string {
  return `${address}.generated.ts`;
}
