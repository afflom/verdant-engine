import fs from 'node:fs';
const [wasm,mesh,framesArg,out]=process.argv.slice(2);
const {instance:{exports:h}}=await WebAssembly.instantiate(fs.readFileSync(wasm));
if (!h.verdant_wasm_init(h.verdant_wasm_default_seed())) throw Error('init');
const data=fs.readFileSync(mesh);
h.verdant_wasm_io_resize(data.length);
new Uint8Array(h.memory.buffer,h.verdant_wasm_io_ptr(),data.length).set(data);
const triangles=h.verdant_wasm_glb_import_io();
if(!triangles)throw Error('mesh');
const file=fs.openSync(out,'w');
const count=Number(framesArg);
for(let i=0;i<count;i++){
 const angle=.25+.55*Math.sin(2*Math.PI*i/count);
 const target=[0,1.2,0], camera=[7*Math.sin(angle),3,7*Math.cos(angle)];
 const length=h.verdant_wasm_render_tensor_rgba(640,320,...camera,...target);
 if(length!==640*320*4)throw Error('frame');
 fs.writeSync(file,new Uint8Array(h.memory.buffer,h.verdant_wasm_fb_ptr(),length));
}
fs.closeSync(file);console.log({triangles,frames:count});
