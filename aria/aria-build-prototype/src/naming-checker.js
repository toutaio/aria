import { inferLayerFromVerb } from './verb-vocabulary.js';

/**
 * Parse a semantic address into its segments.
 * L0: [domain].[entity] (2 segments)
 * L1+: [domain].[subdomain].[verb].[entity] (4 segments)
 *
 * @param {string} address
 * @returns {{ domain: string, subdomain?: string, verb?: string, entity: string, segments: string[] } | null}
 */
export function parseSemanticAddress(address) {
  if (!address || typeof address !== 'string') return null;
  const segments = address.split('.');
  if (segments.length === 2) {
    return { domain: segments[0], entity: segments[1], segments, isL0: true };
  }
  if (segments.length === 4) {
    return {
      domain: segments[0],
      subdomain: segments[1],
      verb: segments[2],
      entity: segments[3],
      segments,
      isL0: false,
    };
  }
  return null;  // invalid address format
}

/**
 * Run all naming enforcement checks on a set of loaded manifests.
 * Returns an array of diagnostic objects (severity: 'ERROR' | 'WARN').
 *
 * @param {Array<{filePath: string, content: object}>} manifests
 * @returns {Array<{severity: string, file: string, message: string}>}
 */
export function checkNaming(manifests) {
  const diagnostics = [];
  const seenIds = new Map();  // id -> filePath

  for (const { filePath, content } of manifests) {
    const m = content?.manifest;
    if (!m) continue;

    const id = m.id;
    const declaredLayer = m.layer?.declared;

    // 3.9: Address uniqueness
    if (id) {
      if (seenIds.has(id)) {
        diagnostics.push({
          severity: 'ERROR',
          file: filePath,
          message: `[ERROR] ${filePath}: duplicate semantic address '${id}' (first seen in ${seenIds.get(id)})`,
        });
      } else {
        seenIds.set(id, filePath);
      }
    }

    // 3.6: Semantic address format
    if (id) {
      const parsed = parseSemanticAddress(id);
      if (!parsed) {
        diagnostics.push({
          severity: 'ERROR',
          file: filePath,
          message: `[ERROR] ${filePath}: invalid semantic address '${id}' — expected [domain].[entity] (L0) or [domain].[subdomain].[verb].[entity] (L1+)`,
        });
        continue;
      }

      // L0 manifests use [domain].[entity] — no verb validation needed
      if (parsed.isL0) {
        // Verify declared layer matches
        if (declaredLayer && declaredLayer !== 'L0') {
          diagnostics.push({
            severity: 'ERROR',
            file: filePath,
            message: `[ERROR] ${filePath}: L0 address format '${id}' requires layer.declared: L0, got ${declaredLayer}`,
          });
        }
        continue;
      }

      // 3.7 + 3.8: Verb lookup and layer consistency
      const verb = parsed.verb;
      const inferredLayer = inferLayerFromVerb(verb);

      if (!inferredLayer) {
        diagnostics.push({
          severity: 'ERROR',
          file: filePath,
          message: `[ERROR] ${filePath}: unknown verb '${verb}' in semantic address '${id}' — not in any layer's vocabulary`,
        });
        continue;
      }

      // 3.8: layer.declared must match inferred layer
      if (declaredLayer && declaredLayer !== inferredLayer) {
        diagnostics.push({
          severity: 'ERROR',
          file: filePath,
          message: `[ERROR] ${filePath}: layer mismatch — declared=${declaredLayer} but verb '${verb}' implies ${inferredLayer} for address '${id}'`,
        });
      }

      // 4.10: Check layer.inferred field matches what we compute
      const storedInferred = m.layer?.inferred;
      if (storedInferred && storedInferred !== inferredLayer) {
        diagnostics.push({
          severity: 'WARN',
          file: filePath,
          message: `[WARN] ${filePath}: inferred layer mismatch — stored inferred=${storedInferred}, recomputed=${inferredLayer} for address '${id}'`,
        });
      }
    }

    // 3.10: Error type domain-prefix check
    const failure = m.contract?.output?.failure;
    if (failure) {
      // Allow patterns like "AuthError.INVALID" or "AuthError { code: A | B | C }"
      const errorTypePattern = /^[A-Z][a-zA-Z]*Error[.\s{]/;
      if (!errorTypePattern.test(failure)) {
        diagnostics.push({
          severity: 'ERROR',
          file: filePath,
          message: `[ERROR] ${filePath}: contract.output.failure '${failure}' does not match {Domain}Error.{Code} pattern`,
        });
      }
    }
  }

  return diagnostics;
}
