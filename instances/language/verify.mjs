import assert from 'node:assert/strict';
import {writeFile} from 'node:fs/promises';
import {chromium} from 'playwright';

const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  await page.goto('http://127.0.0.1:8751');
  const result = await page.evaluate(async () => {
    const {openInstance} = await import('./instance.mjs');
    const v = await openInstance();
    await v.enable();
    const turns = [];
    turns.push(await v.ask('Who should I speak with in the citadel?'));
    for (const boat of ['Silver Heron','Amber Finch']) {
      turns.push(await v.ask('What is the boat called?', 'The boat is called ' + boat + '.'));
    }
    if (!v.interactAt(0, 0, 0)) throw Error('Archivist interaction failed');
    turns.push(await v.ask('Who should I deliver the Keystone to?'));
    v.disable();
    let refused = '';
    try { await v.ask('Where am I?'); } catch (e) { refused = e.message; }
    return {turns, refused};
  });
  await writeFile('transcript.json', JSON.stringify(result,null,2)+'\n');
  console.log(JSON.stringify(result,null,2));
  for (const [i,pattern] of [/archivist/i,/silver heron/i,/amber finch/i,/engineer/i].entries()) {
    assert.match(result.turns[i].answer,pattern);
    assert.equal(result.turns[i].sensoryCheck,'conforms 4');
  }
  assert.notEqual(result.turns[0].snapshot.state,result.turns[3].snapshot.state);
  assert.match(result.refused,/ModelMayNotRun/);
} finally { await browser.close(); }
