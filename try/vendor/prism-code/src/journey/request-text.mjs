/**
 * The text request of a Prism text app: the fields' values in order, separated
 * by single spaces. A choice field is one of its words; a number field is ASCII
 * digits, no sign, no leading zero, at most 18 of them, inside its bounds. A
 * request that is not that is answered `invalid`.
 *
 * `shape` is `{ fields: [{ name, kind: 'choice', values } | { name, kind: 'number', min, max }] }`.
 */
export const INVALID = 'invalid';
export const MAX_DIGITS = 18;

/** A number as the app reads it, or null. */
export function parseNumber(text) {
  if (!/^(0|[1-9][0-9]*)$/.test(text) || text.length > MAX_DIGITS) return null;
  return BigInt(text);
}

/** The decoded request, field name to value (a choice word or a BigInt), or null when malformed or out of bounds. */
export function decodeRequest(shape, text) {
  const parts = text.split(' ');
  if (parts.length !== shape.fields.length) return null;
  const request = {};
  for (const [i, f] of shape.fields.entries()) {
    const part = parts[i];
    if (f.kind === 'choice') {
      if (!f.values.includes(part)) return null;
      request[f.name] = part;
    } else {
      const n = parseNumber(part);
      if (n === null) return null;
      if (f.min !== undefined && n < BigInt(f.min)) return null;
      if (f.max !== undefined && n > BigInt(f.max)) return null;
      request[f.name] = n;
    }
  }
  return request;
}

/** The byte bounds of a request and an answer. */
export function requestLimits(shape) {
  const requestMaximum = Math.min(512, Math.max(64, 24 * shape.fields.length + 16));
  return { requestMaximum, responseMaximum: 64, guestAllocationMaximum: requestMaximum * 2 };
}
