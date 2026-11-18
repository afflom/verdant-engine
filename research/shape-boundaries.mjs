// Exercise the shipped engine through its public Wasm ABI. The GLB contains two
// translated instances of one mesh so each node may declare its physical role.
import assert from 'node:assert/strict';
import fs from 'node:fs';
const moduleBytes = fs.readFileSync(process.argv[2] ?? 'site/verdant-uorc.wasm');
const {instance:{exports:h}} = await WebAssembly.instantiate(moduleBytes);
assert.equal(h.verdant_wasm_init(h.verdant_wasm_default_seed()),1);
function wall() {
  const pos = Buffer.from(new Float32Array([-4,-4,0,4,-4,0,4,4,0,-4,-4,0,4,4,0,-4,4,0]).buffer);
  const doc={asset:{version:'2.0'},scene:0,scenes:[{nodes:[0,1]}],nodes:[
    {mesh:0,translation:[0,30,2]},
    {mesh:0,translation:[20,30,2],extras:{role:'FluidVolume'}}],
    meshes:[{primitives:[{attributes:{POSITION:0}}]}],accessors:[{bufferView:0,componentType:5126,count:6,type:'VEC3',min:[-4,-4,0],max:[4,4,0]}],
    buffers:[{byteLength:pos.length}],bufferViews:[{buffer:0,byteLength:pos.length}]};
  const json=Buffer.from(JSON.stringify(doc));
  const padded=Buffer.alloc(Math.ceil(json.length/4)*4,32);json.copy(padded);
  const words=(...fields)=>Buffer.from(new Uint32Array(fields).buffer);
  return Buffer.concat([words(0x46546c67,2,28+padded.length+pos.length,padded.length,0x4e4f534a),padded,words(pos.length,0x004e4942),pos]);
}
function camera(x){
  assert.equal(h.verdant_wasm_camera_set_anchor(1,x,30,0),1);
  assert.equal(h.verdant_wasm_camera_set_pose_intent(0,0,6000,1000),1);
  return h.verdant_wasm_camera_eye_z();
}
const glb=wall();
h.verdant_wasm_io_resize(glb.length);
new Uint8Array(h.memory.buffer,h.verdant_wasm_io_ptr(),glb.length).set(glb);
assert.equal(h.verdant_wasm_glb_import_io(),4);
const solid=camera(0), fluid=camera(20), gap=camera(10);
assert.ok(solid>0 && solid<2,`solid wall at z=2, eye reached ${solid}`);
assert.equal(fluid,6);
assert.equal(gap,6);
assert.equal(h.verdant_wasm_set_lod_rank(1),1);
assert.equal(camera(0),solid);
h.verdant_wasm_clear_imported_glb();
assert.equal(camera(0),6);
console.log(JSON.stringify({solid,fluid,gap,afterClear:camera(0)}));
