/**
 * A rule app's decision table, read from the JSON LexLean exports for the
 * model: `lexlean/semantic-snapshot/2`, identified by its `semantic_id`.
 * Nothing here reads `.lex.tex`; the snapshot is LexLean's own output, and the
 * table is only what its definitions say, term by term.
 *
 * A rule app's module (the `rule-app` starter) defines `title`, `fields`,
 * `answers`, `otherwise`, `rules`, `guarantees` and `examples`. This module
 * turns them into the numbers the verified rule judge reads (`words`) and each
 * guarantee as data, which is what Lean's kernel checks in this process
 * (kernel-prove.mjs) and what the rule app's record carries (rule-app.mjs).
 *
 * Every refusal is a `PrismCodeError` with a `rule-table.*` code.
 */
import { PrismCodeError } from '../errors.mjs';

export const SNAPSHOT_SCHEMA = 'lexlean/semantic-snapshot/2';
/** The largest number the rule judge reads: its words are 32 bits. */
export const WORD_MAX = 0xffffffff;
/** The operators, by the number the rule judge reads. */
export const OPS = Object.freeze(['<', '<=', '>', '>=', '=', '!=']);
const NAMES = ['title', 'fields', 'answers', 'otherwise', 'rules', 'guarantees', 'examples'];
const DIGEST = /^[0-9a-f]{64}$/;

/** Throw the refusal `code` (`rule-table.*`), with what was wrong. */
const fail = (code, message) => {
  throw new PrismCodeError(code, message);
};

/** The snapshot, checked for the envelope this module relies on. Throws `rule-table.snapshot`. */
export function readSnapshot(input) {
  let snapshot = input;
  if (input instanceof Uint8Array || typeof input === 'string') {
    try {
      snapshot = JSON.parse(typeof input === 'string' ? input : new TextDecoder('utf-8', { fatal: true }).decode(input));
    } catch {
      fail('rule-table.snapshot', 'the snapshot is not JSON');
    }
  }
  if (snapshot?.spec !== SNAPSHOT_SCHEMA) fail('rule-table.snapshot', `the snapshot is not a ${SNAPSHOT_SCHEMA}`);
  if (!DIGEST.test(snapshot.semantic_id ?? '') || !DIGEST.test(snapshot.source_id ?? '') || !DIGEST.test(snapshot.compiler_semantics_id ?? '')) fail('rule-table.snapshot', 'the snapshot does not name its semantic, source and compiler-semantics IDs');
  if (!Array.isArray(snapshot.modules)) fail('rule-table.snapshot', 'the snapshot has no modules');
  return snapshot;
}

/** A term as a value: naturals as numbers, strings, lists, and records as objects. Anything else is refused. */
function valueOf(term, where) {
  switch (term?.kind) {
    case 'nat': {
      const n = Number(term.value);
      if (!Number.isSafeInteger(n)) fail('rule-table.term', `${where} holds a number past ${Number.MAX_SAFE_INTEGER}`);
      return n;
    }
    case 'string':
      return term.value;
    case 'nil':
      return [];
    case 'cons': {
      const out = [];
      let at = term;
      while (at?.kind === 'cons') {
        out.push(valueOf(at.head, `${where}[${out.length}]`));
        at = at.tail;
      }
      if (at?.kind !== 'nil') fail('rule-table.term', `${where} is not a list literal`);
      return out;
    }
    case 'record':
      return Object.fromEntries(term.fields.map((a) => [a.field, valueOf(a.value, `${where}.${a.field}`)]));
    default:
      return fail('rule-table.term', `${where} is a ${term?.kind ?? 'missing'} term; a rule app's definitions are literals`);
  }
}

/** The definitions of one module of the snapshot, by name. */
function definitionsOf(module) {
  const out = new Map();
  for (const d of module.semantic?.declarations ?? []) if (d.kind === 'definition' && (d.parameters ?? []).length === 0) out.set(d.name, d);
  return out;
}

/** The module that defines a rule app: the one with every definition the table needs. Throws `rule-table.module`. */
function ruleAppModule(snapshot) {
  const found = snapshot.modules.filter((m) => NAMES.every((n) => definitionsOf(m).has(n)));
  if (found.length !== 1) fail('rule-table.module', found.length === 0 ? `no module defines ${NAMES.join(', ')}: this is not a rule app` : 'more than one module defines a rule app');
  return found[0];
}

const whole = (n, what) => (Number.isInteger(n) && n >= 0 && n <= WORD_MAX ? n : fail('rule-table.range', `${what} is outside 0..${WORD_MAX}, the numbers the rule judge reads`));

/**
 * The table, read and checked: every index names something, every value lies in
 * its field, and every number fits the rule judge. Returns
 * `{ semanticId, module, title, fields, answers, otherwise, rules, guarantees, examples }`.
 */
export function ruleTable(input) {
  const snapshot = readSnapshot(input);
  const module = ruleAppModule(snapshot);
  const defs = definitionsOf(module);
  const v = Object.fromEntries(NAMES.map((n) => [n, valueOf(defs.get(n).body, n)]));
  if (typeof v.title !== 'string' || !v.title) fail('rule-table.shape', 'title is not a word');
  if (!Array.isArray(v.fields) || v.fields.length === 0) fail('rule-table.shape', 'fields is not a list of at least one Field');
  const fields = v.fields.map((f, i) => {
    if (typeof f?.name !== 'string' || !Array.isArray(f.values)) fail('rule-table.shape', `fields[${i}] is not a Field`);
    const least = whole(f.least, `the least ${f.name}`);
    const greatest = whole(f.greatest, `the greatest ${f.name}`);
    if (greatest < least) fail('rule-table.range', `${f.name}'s greatest is below its least`);
    if (f.values.length > 0 && (least !== 0 || greatest !== f.values.length - 1)) fail('rule-table.range', `${f.name} names ${f.values.length} words, so it runs from 0 to ${f.values.length - 1}`);
    return f.values.length ? { name: f.name, kind: 'choice', values: f.values, least, greatest } : { name: f.name, kind: 'number', least, greatest };
  });
  if (new Set(fields.map((f) => f.name)).size !== fields.length) fail('rule-table.shape', 'two fields share a name');
  if (!Array.isArray(v.answers) || v.answers.length === 0 || v.answers.some((a) => typeof a !== 'string' || !a)) fail('rule-table.shape', 'answers is not a list of words');
  const answer = (n, what) => (Number.isInteger(n) && n >= 0 && n < v.answers.length ? n : fail('rule-table.index', `${what} names answer ${n}, and there are ${v.answers.length}`));
  const condition = (c, what) => {
    const field = fields[c?.field];
    if (!field) fail('rule-table.index', `${what} names field ${c?.field}, and there are ${fields.length}`);
    if (!(c.op in OPS)) fail('rule-table.index', `${what} names operator ${c.op}; the operators are 0 to ${OPS.length - 1}`);
    return [c.field, c.op, whole(c.value, `${what}'s value`)];
  };
  const rules = v.rules.map((r, i) => ({ when: r.conditions.map((c, k) => condition(c, `rules[${i}].conditions[${k}]`)), answer: answer(r.answer, `rules[${i}]`) }));
  const guarantees = v.guarantees.map((g, i) => {
    if (g.always !== 0 && g.always !== 1) fail('rule-table.shape', `guarantees[${i}].always is ${g.always}; it is 0 (never) or 1 (always)`);
    // A guarantee about an answer no rule gives names the next number: the rule judge reads it as no answer.
    return { always: g.always === 1, when: g.conditions.map((c, k) => condition(c, `guarantees[${i}].conditions[${k}]`)), outcome: answer(g.answer, `guarantees[${i}]`) };
  });
  const examples = v.examples.map((e, i) => {
    if (typeof e?.request !== 'string' || typeof e.answer !== 'string') fail('rule-table.shape', `examples[${i}] is not an Example`);
    return { request: e.request, answer: e.answer };
  });
  return { semanticId: `sha256:${snapshot.semantic_id}`, module: module.name, title: v.title, fields, answers: v.answers, otherwise: answer(v.otherwise, 'otherwise'), rules, guarantees, examples };
}

/** The numbers the rule judge decides with, and each guarantee as data: the layout `ruleJudge_sound` speaks about. */
export function tableWords(table) {
  const bounds = table.fields.flatMap((f) => [f.least, f.greatest]);
  const words = [1, table.fields.length, ...bounds, table.otherwise, table.rules.length, ...table.rules.flatMap((r) => [r.when.length, r.answer, ...r.when.flat()])];
  return { words, guarantees: table.guarantees.map((g) => ({ always: g.always, when: g.when, outcome: g.outcome })) };
}

/** The request shape the text app reads, for request-text.mjs. */
export const tableShape = (table) => ({ fields: table.fields.map((f) => (f.kind === 'choice' ? { name: f.name, kind: 'choice', values: [...f.values] } : { name: f.name, kind: 'number', min: f.least, max: f.greatest })) });

// ---- The counterexample check, before Lean's kernel is asked. ----
// It reads the table as the rule judge does: the first rule whose conditions all hold answers, else `otherwise`.
// It is a fast check that is tested, not proved; what is proved is the kernel's answer.

const holds = ([field, op, value], request) => {
  const x = request[field];
  return op === 0 ? x < value : op === 1 ? x <= value : op === 2 ? x > value : op === 3 ? x >= value : op === 4 ? x === value : x !== value;
};

/** The answer index the table gives a request (one number per field), and the rule that gave it (null for otherwise). */
export function judgeTable(table, request) {
  for (const [i, r] of table.rules.entries()) if (r.when.every((c) => holds(c, request))) return { answer: r.answer, rule: i };
  return { answer: table.otherwise, rule: null };
}

/** The values worth trying for each field: its bounds, and every value a condition names, with its neighbours. */
function candidates(table) {
  const conds = [...table.rules, ...table.guarantees].flatMap((r) => r.when);
  return table.fields.map((f, i) => {
    const set = new Set([f.least, f.greatest]);
    for (const [field, , value] of conds) if (field === i) for (const v of [value - 1, value, value + 1]) set.add(v);
    return [...set].filter((v) => v >= f.least && v <= f.greatest).sort((a, b) => a - b);
  });
}

/**
 * A request that breaks a guarantee, among the values the conditions tell apart: `{ request, answer, rule }` with the
 * request as text, or `{ request: null, checked }` when none of `checked` requests breaks it. Every case is tried when
 * there are at most `limit` of them; past that the search stops there and says how many it tried.
 */
export function counterexample(table, guarantee, { limit = 200000 } = {}) {
  const lists = candidates(table);
  const text = (values) => values.map((v, i) => (table.fields[i].kind === 'choice' ? table.fields[i].values[v] : String(v))).join(' ');
  let checked = 0;
  const at = new Array(lists.length).fill(0);
  for (;;) {
    const request = at.map((k, i) => lists[i][k]);
    checked++;
    if (guarantee.when.every((c) => holds(c, request))) {
      const { answer, rule } = judgeTable(table, request);
      if ((answer === guarantee.outcome) !== guarantee.always) return { request: text(request), answer: table.answers[answer], rule };
    }
    if (checked >= limit) return { request: null, checked };
    let i = 0;
    while (i < at.length && ++at[i] === lists[i].length) at[i++] = 0;
    if (i === at.length) return { request: null, checked };
  }
}
