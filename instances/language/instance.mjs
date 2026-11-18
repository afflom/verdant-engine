import {openModels} from './vendor/site/models/models.mjs';
import {guestDecider} from './vendor/site/r4/decide.mjs';
import {loadArchive} from './vendor/prism-code/src/index.mjs';
import {GUEST, openGuest} from './vendor/site/senses/guest.mjs';

const local = path => new URL(path, import.meta.url);
async function read(path) {
  const response = await fetch(local(path));
  if (!response.ok) throw Error('Instance input unavailable: ' + path);
  return response;
}

export async function openInstance() {
  const lock = await (await read('./vendor/site/models/models.lock.json')).json();
  const holo = await loadArchive(new Uint8Array(await (await read('./vendor/verdant-language.holo')).arrayBuffer()));
  if (!holo.identities.verified) throw Error('Invalid language holo');
  const decide = guestDecider(holo.dispatch, {facet: null});
  const model = lock.models.find(m => m.id === 'smollm2-135m-instruct');
  const grant = {models: []};
  const route = await openModels({lock, decide, grant});
  const senses = await openGuest(await (await read('./vendor/site/senses/' + GUEST.path)).arrayBuffer(), GUEST.sha256);
  const {instance: {exports: engine}} = await WebAssembly.instantiate(await (await read('./vendor/engine/site/verdant-uorc.wasm')).arrayBuffer());
  if (!engine.verdant_wasm_init(engine.verdant_wasm_default_seed())) throw Error('Verdant initialization failed');
  function observe() {
    const length = engine.verdant_wasm_story_status_aux();
    if (!length) throw Error('Missing story observation');
    const raw = new TextDecoder().decode(new Uint8Array(engine.memory.buffer, engine.verdant_wasm_aux_ptr(), length));
    const [state, chapter, supply, objective, ...dialogue] = raw.split('|');
    return {state, chapter, supply, objective, dialogue: dialogue.join('|'), player: [engine.verdant_wasm_player_x(),engine.verdant_wasm_player_y(),engine.verdant_wasm_player_z()]};
  }
  return {
    observe,
    interactAt: (x,y,z) => engine.verdant_wasm_story_interact_at(x,y,z),
    async enable() {
      grant.models = [model.id];
      await route.prepare(model.id);
    },
    disable() { grant.models = []; },
    async ask(prompt, content = '') {
      if (!prompt.trim()) throw Error('Enter a question');
      const snapshot = observe();
      const graph = `@prefix sosa: <http://www.w3.org/ns/sosa/> .
<urn:verdant:turn> a sosa:Observation ; sosa:hasFeatureOfInterest <urn:verdant:player> ; sosa:madeBySensor <urn:verdant:text-input> ; sosa:hasSimpleResult ${JSON.stringify(prompt)} .`;
      const sensoryCheck = await senses('check\n' + graph);
      if (!sensoryCheck.startsWith('conforms')) throw Error(sensoryCheck);
      const system = 'Answer briefly using the player observations below. If the observations do not answer the question, say so.\n' +
        'Current objective: ' + snapshot.objective + '\nMost recent dialogue: ' + snapshot.dialogue +
        '\nChapter: ' + snapshot.chapter + (content ? '\nObserved text: ' + content : '');
      const answer = await route.generate(model.id, prompt, {system});
      return {prompt, answer, content, snapshot, sensoryCheck, model: model.source};
    }
  };
}
