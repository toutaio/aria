const DOMAINS = ['auth', 'user', 'billing', 'notification', 'payment'];
const L1_VERBS = ['validate', 'transform', 'compute', 'generate', 'encode', 'decode', 'hash', 'compare'];
const L1_ENTITIES = ['token', 'email', 'password', 'signature', 'hash', 'format', 'address', 'id'];

/**
 * Generate a synthetic valid L1 manifest for testing.
 * @param {number} index - Unique index to ensure address uniqueness
 * @returns {object} Manifest object
 */
export function generateSyntheticManifest(index) {
  const domain = DOMAINS[index % DOMAINS.length];
  const subdomain = `sub${Math.floor(index / DOMAINS.length)}`;
  const verb = L1_VERBS[index % L1_VERBS.length];
  const entity = L1_ENTITIES[Math.floor(index / L1_VERBS.length) % L1_ENTITIES.length] + index;

  return {
    manifest: {
      id: `${domain}.${subdomain}.${verb}.${entity}`,
      version: '1.0.0',
      schema_version: '1.0',
      identity: {
        purpose: `Synthetic test manifest number ${index}`,
        domain,
        subdomain,
        verb,
        entity,
      },
      layer: {
        declared: 'L1',
        inferred: 'L1',
      },
      contract: {
        input: { type: `InputType${entity}` },
        output: { success: `SuccessType${entity}`, failure: `DomainError.FAILED` },
        side_effects: 'NONE',
        idempotent: true,
        deterministic: true,
      },
      dependencies: [{ id: 'domain.entity', layer: 'L0', stability: 'STABLE' }],
      context_budget: { to_use: 100, to_modify: 300, to_extend: 500, to_replace: 700 },
      test_contract: {
        scenarios: [{ scenario: 'Happy path passes' }],
        coverage_required: true,
      },
      stability: 'STABLE',
      lifecycle: { phase: 'STABLE', stable_since: '2024-01-01T00:00:00Z' },
      manifest_provenance: {
        derived_by: 'STATIC_ANALYSIS',
        reviewed_by: 'REVIEWER_AGENT',
        approved_at: '2024-01-01T00:00:00Z',
      },
    },
  };
}
