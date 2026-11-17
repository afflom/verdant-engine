// Run from a Verdant Engine checkout: node research/observer-replay.mjs
// These are synthetic, controlled samples, not measurements from participants.
import assert from 'node:assert/strict';
import fs from 'node:fs';
const bytes=fs.readFileSync('site/verdant-uorc.wasm');
async function sample(dwellMillis) {
  const {instance}=await WebAssembly.instantiate(bytes);
  const h=instance.exports;
  assert.equal(h.verdant_wasm_init(h.verdant_wasm_default_seed()),1);
  const text=new TextEncoder().encode('A close mosaic portrait of the duck');
  h.verdant_wasm_io_resize(text.length);
  new Uint8Array(h.memory.buffer,h.verdant_wasm_io_ptr(),text.length).set(text);
  const length=h.verdant_wasm_route_prompt_io();
  assert.ok(length);
  const route=new TextDecoder().decode(new Uint8Array(h.memory.buffer,h.verdant_wasm_aux_ptr(),length));
  const slot=0;
  assert.equal(h.verdant_wasm_step_observer(slot,dwellMillis,0,0,0),1);
  return {dwellMillis,route,attended:h.verdant_wasm_observer_attended_slot(),subjectSalience:h.verdant_wasm_observer_salience_milli(slot),lod:h.verdant_wasm_observer_foveated_lod()};
}
const baseline=await sample(0), observed=await sample(840);
assert.deepEqual(await sample(840),observed);
assert.equal(observed.attended,0);
assert.ok(observed.subjectSalience>baseline.subjectSalience);
console.log(JSON.stringify({baseline,observed},null,2));
