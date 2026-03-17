/**
 * Utility: convert a dot-delimited semantic address to a PascalCase type name.
 * Handles kebab-case segments within each dot-delimited part.
 * e.g. "auth.identity.authenticate.user"       → "AuthIdentityAuthenticateUser"
 * e.g. "search.index.execute.bulk-index"        → "SearchIndexExecuteBulkIndex"
 * e.g. "notification.dispatch.execute.parallel-fork" → "NotificationDispatchExecuteParallelFork"
 */
export function addressToTypeName(address: string): string {
  return address
    .split('.')
    .map(segment =>
      segment
        .split('-')
        .map(part => part.charAt(0).toUpperCase() + part.slice(1))
        .join('')
    )
    .join('');
}

/**
 * Derive the output filename for a manifest.
 * e.g. "auth.identity.authenticate.user" → "auth.identity.authenticate.user.generated.ts"
 */
export function outputFilename(address: string): string {
  return `${address}.generated.ts`;
}
