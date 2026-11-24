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
