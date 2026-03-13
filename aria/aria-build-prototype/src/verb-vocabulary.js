/**
 * Verb vocabulary table for all 6 layers (L0–L5).
 * Each verb maps to exactly one layer. Verbs must not overlap between layers.
 * Source: 06-naming-conventions.md
 */

export const VERB_VOCABULARY = {
  L0: new Set([]),  // L0 uses [domain].[entity] — no verb segment
  L1: new Set([
    'validate', 'transform', 'compute', 'generate',
    'encode', 'decode', 'hash', 'compare'
  ]),
  L2: new Set([
    'create', 'build', 'prepare', 'assemble', 'resolve'
  ]),
  L3: new Set([
    'execute', 'apply', 'process', 'enforce', 'emit',
    'authorize', 'persist'
  ]),
  L4: new Set([
    'orchestrate', 'coordinate', 'pipeline', 'route'
  ]),
  L5: new Set([
    'expose', 'integrate', 'guard', 'translate'
  ])
};

/**
 * Returns the layer that owns a given verb, or null if the verb is unknown.
 * @param {string} verb
 * @returns {'L1'|'L2'|'L3'|'L4'|'L5'|null}
 */
export function inferLayerFromVerb(verb) {
  for (const [layer, verbs] of Object.entries(VERB_VOCABULARY)) {
    if (layer === 'L0') continue;
    if (verbs.has(verb)) return layer;
  }
  return null;
}

/**
 * Returns whether a verb is valid for a given layer.
 * @param {string} verb
 * @param {string} layer
 * @returns {boolean}
 */
export function isVerbValidForLayer(verb, layer) {
  return VERB_VOCABULARY[layer]?.has(verb) ?? false;
}
