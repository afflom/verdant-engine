/**
 * What the journey still takes from the SDK: the image a published app's
 * lockfile pins, and the axioms Lean's audit may report. No step runs the
 * image here.
 */

/**
 * The SDK image a new app's lockfile pins when it is published to GitHub: the PrismPM 0.4.0 release, the image
 * UOR-Foundation/template pins, so an app built here and the same app built on GitHub Actions
 * are built by one toolchain. Measured on 2026-11-16: with the 0.3.0 candidate here and the
 * release there, one spec gave two holos of different bytes and the same seal.
 */
export const DEFAULT_SDK_IMAGE = 'ghcr.io/uor-foundation/prismpm-sdk@sha256:d6c263a733a6bc36e0458c3f5b9a1f5ea9c14dfbe8c257d3f650a960b5fe633c';

/** `PrismX.X.name violates its exact axiom policy: observed [A, B]` gives `{ name, axioms }`. */
export function axiomViolations(diagnostics) {
  return diagnostics
    .filter((d) => d.code === 'LLV7005')
    .map((d) => {
      const m = /`([^`]+)` violates its exact axiom policy: observed (\[[^\]]*\])/.exec(d.message);
      if (!m) return null;
      const observed = m[2].slice(1, -1).split(',').map((a) => a.trim()).filter(Boolean);
      return { name: m[1].split('.').pop(), axioms: m[2], forbidden: observed.filter((a) => !ALLOWED_AXIOMS.includes(a)) };
    })
    .filter(Boolean);
}

/**
 * The only axioms a proof may rest on: Lean's standard three. An observed set
 * becomes a declaration's exact policy only when it stays inside these;
 * `sorryAx` (a missing proof), `Lean.ofReduceBool` (native_decide, which
 * trusts compiled code) or any other axiom fails the proof instead.
 */
export const ALLOWED_AXIOMS = Object.freeze(['propext', 'Quot.sound', 'Classical.choice']);

/** Each fix whose observed axioms leave the allowed three, as `name rests on axiom`; empty when every fix may be recorded. */
export const forbiddenAxioms = (fixes) => fixes.flatMap((f) => f.forbidden.map((a) => `${f.name} rests on ${a}`));

/** The refusal for axioms outside the allowed three. */
export const forbiddenMessage = (lines) => `Lean observed an axiom outside ${ALLOWED_AXIOMS.join(', ')}: ${lines.join('; ')}. A proof that rests on sorryAx, Lean.ofReduceBool (native_decide) or another axiom is not a proof, so Prism refuses it rather than record it.`;
