// LexLean's kernel declarations (`lexlean/kernel-declarations/2`, LexLean LN-15): the project's types, eliminators,
// projections, definitions, and theorem statements, and the proofs LexLean produced over them, as lean4export lines
// that continue the base library. LexLean decides what each construct means; this file only finds a proof of each
// statement LexLean did not prove itself, and hands everything to the kernel.
//
//   const p = planDeclarations(meta, suffix, library)   // library = loadLibrary(base + suffix)
//   const j = judgeDeclarations(meta, suffix, base, check)   // check(text) -> { exit, out }: the kernel over base
import { E, emitter, loadLibrary } from "./emit.mjs";
import { COVERED } from "./plan.mjs";

export const SPEC = "lexlean/kernel-declarations/2";

/**
 * Judge LexLean's declarations of a project, and plan the proofs LexLean did not give. `check(text)` is Lean's kernel
 * with the base library resident, answering `{ exit, out }`.
 *
 * The suffix is judged whole, so one proof the kernel refuses would refuse every theorem of the project. A proof
 * LexLean produced is therefore set aside when the kernel refuses it: its one `thm` line is left out, the theorem is
 * planned like any other (found here, or reported not proved), and the rest is judged again. Only produced theorem
 * proofs are ever set aside. A refused type, definition or statement still refuses the project, as before.
 *
 * Returns `{ plan, suffix, setAside, declarations }`: the plan over the suffix kept, that suffix, each theorem set
 * aside with the kernel's reason, and the kernel's answer on the suffix kept.
 */
export function judgeDeclarations(meta, suffix, base, check) {
  const setAside = new Map();
  let lines = suffix.split("\n");
  let answer = suffix.trim() ? check(suffix) : { exit: 0, out: [] };
  while (answer.exit !== 0) {
    const refused = /declaration '([^']+)'/.exec(answer.out.join("\n"))?.[1];
    const th = refused && meta.theorems.find((t) => t.produced && !setAside.has(t.theorem) && (t.produced === refused || t.theorem === refused));
    if (!th) break; // not a proof LexLean produced: the project is refused, as it always was
    const ix = loadLibrary(base + lines.join("\n")).nameIx.get(th.produced);
    const at = ix === undefined ? -1 : lines.findIndex((l) => l.includes('"thm"') && JSON.parse(l).thm?.name === ix);
    if (at < 0) break;
    lines = lines.filter((_, i) => i !== at);
    setAside.set(th.theorem, `the kernel refused LexLean's proof (${answer.out.slice(-1)[0] ?? "no reason given"})`);
    answer = check(lines.join("\n"));
  }
  const kept = lines.join("\n");
  const planned = setAside.size ? { ...meta, theorems: meta.theorems.map((t) => (setAside.has(t.theorem) ? { ...t, produced: null } : t)) } : meta;
  return { plan: planDeclarations(planned, kept, loadLibrary(base + kept)), suffix: kept, setAside, declarations: answer };
}

/**
 * The plan of a project given as LexLean's declarations. `meta` is `declarations.json`, `suffix` the lines of
 * `declarations.ndjson`, and `library` the base library and that suffix, loaded together.
 */
export function planDeclarations(meta, suffix, library) {
  if (meta.spec !== SPEC) throw new Error(`declarations: not a ${SPEC}: ${meta.spec}`);
  if (!meta.library) throw new Error("declarations: written without a library, so their numbering is not the base library's");
  const em = emitter(library), mark = em.checkpoint();
  const theorems = meta.theorems.map((th) => {
    const kind = th.proof?.kind ?? null;
    return {
      name: th.theorem, proof: kind, declared: [...th.axioms].sort(), produced: th.produced ?? null,
      status: th.produced ? null
        : !th.statement ? `not proved: its statement was not declared (${th.refused})`
        : COVERED[kind] || em.STRUCTURED.has(kind) ? null : `not proved: the proof kind ${kind} is not covered here yet`,
      proofJson: th.proof,
    };
  });
  // A proof LexLean gives by its shape is built from that shape; the others are found. `apply` names the theorems a
  // proof rests on, which are written before it.
  const byName = new Map(theorems.map((t) => [t.name, t]));
  const proofOf = (th) => em.STRUCTURED.has(th.proof) ? em.structured(th.name, th.proofJson) : em.theorem(script(th));
  for (const th of theorems) {
    if (th.status || th.produced) continue;
    try { th.deps = proofOf(th).deps ?? []; } catch (e) { th.status = `not proved: no proof found (${e.message})`; }
    em.rollback(mark);
  }
  for (let changed = true; changed; ) {
    changed = false;
    for (const th of theorems) {
      const dep = th.status || th.produced ? null : th.deps.find((d) => !byName.get(d) || byName.get(d).status);
      if (dep) { th.status = `not proved: it applies ${dep}, which is not proved here`; changed = true; }
    }
  }
  return {
    theorems, refused: meta.refused, bytes: suffix.length,
    /** The suffix, then a found proof of each named theorem, its dependencies first. */
    file(names) {
      const order = [], seen = new Set();
      const visit = (n) => {
        if (seen.has(n)) return;
        seen.add(n);
        const th = byName.get(n);
        if (th.produced) return; // already in the suffix
        for (const d of th.deps ?? []) visit(d);
        order.push(th);
      };
      for (const n of names) visit(n);
      // A found proof applies a theorem LexLean produced by its own name: `X : LexStmt.X := LexLeanKernel.X`, which
      // the kernel accepts only if LexLean's proof is of exactly the declared statement.
      if (order.length) for (const th of theorems) if (th.produced) em.write({ name: th.name, type: E.const("LexStmt." + th.name, []), value: E.const(th.produced, []) });
      for (const th of order) em.write(proofOf(th));
      const proofs = em.since(mark), out = suffix + (proofs.length ? proofs.join("\n") + "\n" : "");
      em.rollback(mark);
      return out;
    },
  };
}

const script = (th) => ({ name: th.name, ns: th.name.slice(0, th.name.lastIndexOf(".")), script: [{ kind: COVERED[th.proof] }], binders: [], raw: "", text: "" });
