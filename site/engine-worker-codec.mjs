export function readSolvedCamera(h){
return{
eye:[h.verdant_wasm_camera_eye_x(),h.verdant_wasm_camera_eye_y(),h.verdant_wasm_camera_eye_z()],
lookTarget:[h.verdant_wasm_camera_target_x(),h.verdant_wasm_camera_target_y(),h.verdant_wasm_camera_target_z()],
yawRad:(h.verdant_wasm_camera_yaw_milli()|0)/1000,
camMode:h.verdant_wasm_camera_mode()>>>0,
obstructed:h.verdant_wasm_camera_obstructed()===1,
clearanceMm:h.verdant_wasm_camera_clearance_milli()>>>0,
};
}
export function readUniverseCell(h){
return[Number(h.verdant_wasm_universe_cell_x()),Number(h.verdant_wasm_universe_cell_y()),Number(h.verdant_wasm_universe_cell_z())];
}
const mkV=(buf,v,i,p,n,c,q,uv=0)=>v&&i&&p&&n&&c&&q?{
pos:new Float32Array(buf,p,v*3),norm:new Float32Array(buf,n,v*3),col:new Float32Array(buf,c,v*3),
...(uv?{uv:new Float32Array(buf,uv,v*2)}:{uv:null}),idx:new Uint32Array(buf,q,i),
}:null;
export function readSplitMeshViews(h){
if(!h?.memory)return null;
const b=h.memory.buffer,sI=h.verdant_wasm_static_idx_count()>>>0,dI=h.verdant_wasm_dyn_idx_count()>>>0;
const sV=mkV(b,h.verdant_wasm_static_vert_count()>>>0,sI,h.verdant_wasm_scene_pos_ptr()>>>0,h.verdant_wasm_scene_norm_ptr()>>>0,h.verdant_wasm_scene_col_ptr()>>>0,h.verdant_wasm_scene_idx_ptr()>>>0,typeof h.verdant_wasm_scene_uv_ptr==="function"?h.verdant_wasm_scene_uv_ptr()>>>0:0);
if(sV)sV.epoch=h.verdant_wasm_static_epoch()>>>0;
const dV=mkV(b,h.verdant_wasm_dyn_vert_count()>>>0,dI,h.verdant_wasm_dyn_pos_ptr()>>>0,h.verdant_wasm_dyn_norm_ptr()>>>0,h.verdant_wasm_dyn_col_ptr()>>>0,h.verdant_wasm_dyn_idx_ptr()>>>0);
let iV=null,imI=0,ic=0;
if(typeof h.verdant_wasm_inst_count==="function"){
ic=h.verdant_wasm_inst_count()>>>0;imI=h.verdant_wasm_inst_mesh_idx_count()>>>0;
const dp=h.verdant_wasm_inst_data_ptr()>>>0;
const m=ic&&dp?mkV(b,h.verdant_wasm_inst_mesh_vert_count()>>>0,imI,h.verdant_wasm_inst_mesh_pos_ptr()>>>0,h.verdant_wasm_inst_mesh_norm_ptr()>>>0,h.verdant_wasm_inst_mesh_col_ptr()>>>0,h.verdant_wasm_inst_mesh_idx_ptr()>>>0):null;
if(m)iV={epoch:h.verdant_wasm_inst_epoch()>>>0,instCount:ic,pos:m.pos,norm:m.norm,col:m.col,idx:m.idx,data:new Float32Array(b,dp,ic*16)};
}
return{staticViews:sV,dynViews:dV,instViews:iV,tris:Math.floor(sI/3)+Math.floor(dI/3)+(iV?Math.floor(imI/3)*ic:0)};
}
let extP=null;
export const getExtendedWorkerModule=()=>(extP??=import("./engine-worker-extended.mjs"));
export async function handleExtendedWorkerMessage(msg,ctx){
return(await getExtendedWorkerModule()).handleExtendedWorkerMessage(msg,ctx);
}
export async function handleStreamOrGrantWorkerMessage(msg,ctx){
if(msg.kind==="stream-uorc"){
const{readIoBytes,stageIoBytes}=await getExtendedWorkerModule();
const targetRank=Math.max(0,Math.min(4,msg.targetRank|0))>>>0;
const uorcWasm=await ctx.getUorcWasm();
const streamBytes=msg.customStream instanceof Uint8Array&&msg.customStream.byteLength>0?msg.customStream:await ctx.getCanonicalUorcStream();
stageIoBytes(uorcWasm,streamBytes);
const ok=uorcWasm.verdant_wasm_uorc_decode_and_restore(targetRank)===1;
if(!ok){ctx.postMessage({kind:"uorc-streamed",id:msg.id,ok:false});return;}
const vdntBytes=readIoBytes(uorcWasm);
if(!ctx.getUorcSceneActive())ctx.syncCoreStateToUorc(uorcWasm);else ctx.syncOrgansToWasm(uorcWasm);
ctx.clearWalkJournal();ctx.setUorcSceneActive(true);ctx.setCustomGlbActive(false);
const sh=ctx.simHost();
const codedChecksum=Number(uorcWasm.verdant_wasm_coded_zero_decode_checksum(Math.min(3,targetRank)));
const lod=msg.lod||ctx.getCurrentLod();ctx.setCurrentLod(lod);
const cam=readSolvedCamera(sh);
const stats=ctx.drawScene(cam.eye,cam.lookTarget,lod,msg.simClockSec||0,false);
ctx.postMessage({kind:"uorc-streamed",id:msg.id,ok:true,lodRank:sh.verdant_wasm_active_lod_rank()>>>0,tris:stats.tris,streamBytes:streamBytes.byteLength,vdntBytes:vdntBytes.byteLength,codedChecksum});
return;
}
const uorcWasm=await ctx.getUorcWasm();
const idx=Math.max(0,Math.min(10,msg.organIdx>>>0));
const reqMode=msg.mode>>>0;
if(reqMode===0)ctx.organModes[idx]=0;else if(reqMode===2)ctx.organModes[idx]=2;else if(reqMode===1&&ctx.organModes[idx]!==2)ctx.organModes[idx]=1;
ctx.slmKappaCache.clear();
const allowed=ctx.syncOrgansToWasm(uorcWasm,idx);
ctx.postMessage({kind:"organ-grant-updated",id:msg.id,ok:true,organIdx:idx,mode:ctx.organModes[idx],allowed});
}
