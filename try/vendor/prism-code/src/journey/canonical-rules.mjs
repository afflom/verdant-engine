/**
 * The rules of the canonical build, one for one with `model/canonical-build`
 * (CanonicalBuild.lex.tex), where Lean proves what they guarantee:
 *
 * - `holoAdmitted`: a holo is admitted only when its `semantic_id` recomputes
 *   from its sources, under the pinned kernel, library and compiler.
 * - `termReusable`: a stored proof term stands in for producing one, never for
 *   judging it, and only under the same key, kernel, library, statement, axiom
 *   policy and bytes.
 * - `proved`: only a term the kernel accepted in this run, whose bytes are what
 *   they are named, for its own statement, within its exact axiom policy.
 * - `status`: proved, refused or pending, and nothing in between. Where a term
 *   came from (in-process, the user's Actions run, a store) is not an input,
 *   which is how the model shows a fallback produces terms and never verdicts.
 *
 * Addresses are compared as strings here and as numbers in the model; each
 * rule is equality on them, so the two agree (test/canonical-build-model.test.mjs).
 */

/** The statuses, by the model's numbers. */
export const STATUS = Object.freeze(['proved', 'refused', 'pending']);
/** The three axioms Lean's kernel admits by name; anything else counts as `observedOther`. */
export const LEAN_AXIOMS = Object.freeze(['Classical.choice', 'Quot.sound', 'propext']);

export const holoAdmitted = (s) => s.sealedId === s.recomputedId && s.kernelNamed === s.kernelPinned && s.libraryNamed === s.libraryPinned && s.compilerNamed === s.compilerPinned;

export const termReusable = (s) =>
  s.present === 1 &&
  s.storedKey === s.computedKey &&
  s.storedKernel === s.kernelPinned &&
  s.storedLibrary === s.libraryPinned &&
  s.storedStatement === s.statement &&
  s.storedPolicy === s.policy &&
  s.storedBytes === s.derivedBytes;

export const withinPolicy = (j) => (j.observedPropext === 0 || j.policyPropext === 1) && (j.observedChoice === 0 || j.policyChoice === 1) && (j.observedQuot === 0 || j.policyQuot === 1) && j.observedOther === 0;

export const proved = (j) => j.hasTerm === 1 && j.termNamed === j.termDerived && j.termStatement === j.statement && j.kernelRan === 1 && j.kernelAccepted === 1 && withinPolicy(j);

export const status = (j) => (proved(j) ? 0 : j.hasTerm === 1 && j.kernelRan === 1 ? 1 : 2);

/** A sorted, de-duplicated axiom list: the canonical form of a policy, so two equal policies have equal bytes. */
export const axiomPolicy = (names) => [...new Set(names)].sort();

/**
 * The facts `proved` decides on, from one kernel judgement. `term` is the
 * bytes' address as named and `derived` as hashed now; `axioms` is what the
 * kernel observed, `null` when it did not answer; `policy` the theorem's own.
 */
export function judgedFacts({ hasTerm, termNamed = '', termDerived = '', termStatement = '', statement, kernelRan, kernelAccepted, axioms, policy, source = 0 }) {
  const seen = new Set(axioms ?? []);
  const allowed = new Set(policy);
  return {
    hasTerm: hasTerm ? 1 : 0,
    termNamed,
    termDerived,
    termStatement,
    statement,
    kernelRan: kernelRan ? 1 : 0,
    kernelAccepted: kernelAccepted ? 1 : 0,
    observedPropext: seen.has('propext') ? 1 : 0,
    observedChoice: seen.has('Classical.choice') ? 1 : 0,
    observedQuot: seen.has('Quot.sound') ? 1 : 0,
    observedOther: [...seen].filter((a) => !LEAN_AXIOMS.includes(a)).length,
    policyPropext: allowed.has('propext') ? 1 : 0,
    policyChoice: allowed.has('Classical.choice') ? 1 : 0,
    policyQuot: allowed.has('Quot.sound') ? 1 : 0,
    source,
  };
}
