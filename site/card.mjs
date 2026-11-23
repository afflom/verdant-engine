const LODS=[
[0,"lod:0","LOD0","LOD0 Proxy",0.125,0.35,0,[0.09,0.08,0.12],[0.14,0.11,0.16]],
[1,"lod:1","LOD1","LOD1 Medium",0.25,0.72,0,[0.12,0.17,0.24],[0.18,0.24,0.31]],
[2,"lod:2","LOD2","LOD2 High",0.625,0.92,0,[0.16,0.27,0.34],[0.24,0.35,0.39]],
[3,"lod:3","LOD3","LOD3 Lossless",1,1,1,[0.18,0.3,0.38],[0.3,0.4,0.44]],
[4,"scene:lod:4","LOD4","LOD4 Ultra HD",2,2,1,[0.2,0.34,0.44],[0.34,0.44,0.48]],
].map(([rank,key,short,label,res,density,l,sky,fog])=>({rank,key,short,label,res,density,lossless:Boolean(l),sky,fog}));
const NODES=[
[0,"scene:telos","Master Overview & W5H Governor",[2.2,3.8,9.2],18.5,460,260,0,0],
[1,"scene:acoustics","Parabolic Stone Arch Span",[7.5,1.8,-4],13.5,640,320,0,0],
[2,"scene:character","22-Joint Explorer & NavMesh",[0,1.6,11.5],6.2,620,350,1,0],
[3,"scene:joints","Articulated XPBD Rig & ECS Bay",[-6.5,1.8,9],10,-540,320,0,0],
[4,"scene:ui","South Terrace & Tropical UI",[6,2,8],11,520,300,0,0],
[5,"scene:narrative","Story Podium & Conservation",[1.5,2.2,-1],8.5,350,260,0,0],
[6,"scene:lawful","Suspension Mover Track",[16.5,1.2,0],11,620,350,0,1],
[7,"scene:uorc","Pavilion Vault & Semiring Dome",[0,5.2,2],12.5,-420,220,0,0],
].map(([id,query,name,target,dist,yawMilli,pitchMilli,p,f])=>({id,query,name,target,dist,yawMilli,pitchMilli,possessExplorer:Boolean(p),followMover:Boolean(f)}));
const STRATA=[
["telos","01 · Ontological Telos","W5H 8/8",0,"dashboard"],
["uorc","02 · UORC & Semirings","15 Kernels",7,"code"],
["lawful","03 · Swept BLAS & XPBD","Zero-Tunnel",3,"code"],
["character","04 · 22-Joint IK & Nav","4,608 Polys",2,"gameplay"],
["acoustics","05 · Sabine Acoustics","RT60 + ITD",1,"audio"],
["ui","06 · Tropical Spatial UI","DTCG + AA",4,"design"],
["narrative","07 · Story & Stitch P2P","2-of-3 XOR",5,"narrative"],
].map(([key,label,tag,nodeIdx,roleKey])=>({key,label,tag,nodeIdx,query:`scene:${key}`,roleKey}));
const ROLES={dashboard:"telos",code:"lawful",design:"ui",audio:"acoustics",narrative:"narrative",director:"telos",gameplay:"character"};
const ORGANS=["wasm-core","offscreen-canvas","service-worker","audio-worklet","opfs-storage","gamepad-hid","device-orientation","webcrypto-prism-id","broadcast-channel","webrtc-stitch-mesh","webnn-local"];
const ORG_DEF=[1,1,1,0,0,0,0,1,1,0,0];
const ACOUST={quantumFrames:128,stereoChannels:2,fundamentalHz:110,modeNumers:[9,25,49,81,121],modeDenom:9,rt60Ms:3000};
const pConf=(l)=>{
if(!l?.startsWith("conformant:"))return null;
const o={};
for(const p of l.slice(11).split(",")){const e=p.indexOf("=");if(e>0)o[p.slice(0,e)]=p.slice(e+1);}
return o;
};
const pRat=(r,fb)=>{if(!r)return fb;const[n,d]=r.split("/").map(Number);return Number.isFinite(n)&&Number.isFinite(d)&&d?n/d:fb;};
const pTri=(r,s,fb)=>{if(!r)return fb;const p=r.split("/").map(Number);return p.length===3&&p.every(Number.isFinite)?p.map((v)=>v/s):fb;};
const yieldT=()=>new Promise((r)=>setTimeout(r,0));
let preW=null;
try{if(typeof Worker==="function")preW=new Worker("./engine-worker.mjs",{type:"module"});}catch{preW=null;}
export default async function start({app,reason,$,chip,show,parse}){
const canvas=$("ve-canvas"),vpEl=canvas?.parentElement,statusEl=$("ve-status");
const[lodBtn,uorcBtn,renderBtn,importBtn,glbIn,studioBtn,meshBtn,simBtn,camBtn,audioBtn,glbBtn,snapBtn,promptForm,promptInput,hudLod,hudStats,tele,touchPad,interactBtn,lockBtn]=
["lod-btn","uorc-btn","render-btn","import-btn","glb-file","studio-btn","mesh-btn","sim-btn","cam-btn","audio-btn","glb-btn","snap-btn","prompt-form","prompt-input","hud-lod","hud-stats","telemetry","touch-pad","interact-btn","lock-btn"].map((k)=>$(`ve-${k}`));
let lodIdx=2,simLive=false,camPreset="landmark",renderMode="webgl2",fMov=false;
let activeProject="Sunspire Harbor Regatta",activeStratum="telos",activeRole="director";
let aSlot=0,gazeMs=300,pYaw=0,pPit=0,pDst=0,jumpQ=false;
let sEye=[10.2,8.5,25.3],sTgt=[2.2,3.8,9.2],dragging=false,rafId=0;
let lTris=0,lVis=24,lClr=290,isSw=false,offOk=false;
let nextId=1,workerFramesInFlight=0,lSent=0,lW=0,lH=0,fbR=null,fbM=null,stM=null;
const organModes=new Uint8Array(11),stratumBtns=new Map(),keys=new Set(),pCalls=new Map();
const getStudio=async()=>(stM??=await import("./studio.mjs"));
const isOrganAllowed=(i,g=false)=>organModes[i]===2?false:Boolean(organModes[i]===1||ORG_DEF[i]===1||g);
const refuseOrgan=(i)=>chip(statusEl,`Refused: ContainerOrganDenied(${ORGANS[i]||i})`,"warn");
const syncPos=()=>vpEl?.classList.toggle("possessed",camPreset!=="landmark");
const syncTitle=(suf=null)=>{if(hudLod)hudLod.textContent=`Verdant Engine · ${suf||`${renderMode==="tile"?"Wasm 16x16 Tile":"WebGL2 PBR"} · ${LODS[lodIdx].label}`}`;};
const syncStratum=(k)=>{
if(STRATA.some((s)=>s.key===k))activeStratum=k;
for(const[sk,b]of stratumBtns)b.setAttribute("aria-current",String(sk===activeStratum));
};
const syncRole=(rk)=>{if(ROLES[rk]){activeRole=rk;syncStratum(ROLES[rk]);}};
const hK=(...a)=>a.some((k)=>keys.has(k));
const hasMv=()=>hK("KeyW","KeyA","KeyS","KeyD","ArrowUp","ArrowDown","ArrowLeft","ArrowRight","Space");
const isAnim=()=>simLive||dragging||fMov||hasMv();
const pxDims=()=>{
if(!canvas)return{w:1,h:1};
const d=Math.min(window.devicePixelRatio||1,isSw?1:2);
return{w:Math.max(1,Math.floor((canvas.clientWidth||320)*d)),h:Math.max(1,Math.floor((canvas.clientHeight||240)*d))};
};
function mvInt(dt){
const pad=isOrganAllowed(5)?navigator.getGamepads?.()?.[0]:null,dz=(v)=>(Math.abs(v)>0.16?v:0);
const gx=pad?dz(pad.axes[0]||0):0,gy=pad?-dz(pad.axes[1]||0):0,rx=pad?dz(pad.axes[2]||0):0,ry=pad?dz(pad.axes[3]||0):0;
const kf=(hK("KeyW","ArrowUp")?1:0)-(hK("KeyS","ArrowDown")?1:0),ks=(hK("KeyD","ArrowRight")?1:0)-(hK("KeyA","ArrowLeft")?1:0);
const fwd=Math.max(-1,Math.min(1,kf+gy)),strafe=Math.max(-1,Math.min(1,ks+gx)),speedScale=hK("ShiftLeft","ShiftRight")?1.65:1;
const jump=jumpQ||keys.has("Space")||Boolean(pad?.buttons?.[0]?.pressed);jumpQ=false;
let dYaw=pYaw-rx*dt*1.6,dPitch=pPit+ry*dt*1.1;
if(simLive&&!dragging&&!fMov&&camPreset==="landmark"&&!fwd&&!strafe)dYaw+=dt*0.14;
const dDist=pDst;pYaw=pPit=pDst=0;
return{fwd,strafe,speedScale,jump,dYaw,dPitch,dDist,moving:Boolean(fwd||strafe||jump)};
}
const hudStr=(fps)=>`${fps?`${fps}· `:""}Holo Verified · 15 Semirings · ${lTris>0?`${lTris.toLocaleString()} tris · `:""}Clear ${lClr}mm`;
let eW=null,prevNow=performance.now(),sClk=0,frames=0,lastFps=prevNow;
const reqR=()=>{if(!rafId){prevNow=performance.now();rafId=requestAnimationFrame(frame);}};
if(typeof Worker==="function"&&canvas){
try{
eW=preW||new Worker("./engine-worker.mjs",{type:"module"});preW=null;
eW.onmessage=({data:m})=>{
if(!m)return;
if(m.id&&pCalls.has(m.id)){const r=pCalls.get(m.id);pCalls.delete(m.id);r(m);return;}
if(m.kind==="frame-dropped"){workerFramesInFlight=Math.max(0,workerFramesInFlight-1);const d=pxDims();if((d.w!==lW||d.h!==lH)&&!rafId)reqR();return;}
if(m.kind==="telemetry"){
workerFramesInFlight=Math.max(0,workerFramesInFlight-1);lTris=m.tris||lTris;lVis=m.bvhVisible??lVis;lClr=m.camClearanceMm??lClr;
if(Array.isArray(m.eye))sEye=m.eye;if(Array.isArray(m.lookTarget))sTgt=m.lookTarget;
stM?.pushStudioTelemetry(m);stM?.syncModalAudioSpatial(m.binauralPan,m.binauralGain,m.binauralItdMicros,m.pavilionContact,m.attendedSlot);
if(!isAnim()&&hudStats)hudStats.textContent=hudStr("");
const d=pxDims();if((pYaw||pPit||pDst||jumpQ||d.w!==lW||d.h!==lH)&&!rafId)reqR();
}
};
eW.onerror=()=>{workerFramesInFlight=0;for(const r of pCalls.values())r(null);pCalls.clear();};
}catch{eW=null;}
}
const cW=(kind,payload={},transfer=[])=>!eW?Promise.resolve(null):new Promise((res)=>{
const id=nextId++,wd=setTimeout(()=>{if(pCalls.delete(id))res(null);},15000);
pCalls.set(id,(v)=>{clearTimeout(wd);res(v);});eW.postMessage({kind,id,...payload},transfer);
});
function setSim(n){
simLive=Boolean(n);
if(simBtn){simBtn.setAttribute("aria-pressed",String(simLive));simBtn.textContent=simLive?"Pause Preview":"Play Preview";}
reqR();
}
const apCam=(r)=>{if(r?.camera){sEye=r.camera.eye;sTgt=r.camera.lookTarget;lClr=r.camera.clearanceMm??lClr;}if(r?.tris)lTris=r.tris;};
async function syncFb(){
if(offOk||!fbR||!eW)return;
const r=await cW("get-packed-buffers",{simClockSec:sClk});
if(r?.ok){fbR.uploadPackedBuffers(r.pos,r.norm,r.col,r.idx);lTris=r.tris||0;apCam(r);}
}
async function cycCam(force=null){
const o=["landmark","third","first"];camPreset=force||o[(o.indexOf(camPreset)+1)%3];fMov=false;syncPos();
const sp=camPreset==="landmark"
?["Camera: Orbit","camera-focus-landmark",{target:[2.2,3.8,9.2],dist:18.5,yawMilli:460,pitchMilli:260},"Orbit Camera · Guarded (>=290mm)"]
:camPreset==="third"
?["Camera: 3rd IK","camera-set-mode",{mode:0,simClockSec:sClk},"3rd-Person Rig · WASD + Foot IK"]
:["Camera: 1st Eye","camera-set-mode",{mode:1,simClockSec:sClk},"1st-Person Eye · Head Masked"];
if(camBtn)camBtn.textContent=sp[0];apCam(await cW(sp[1],sp[2]));chip(statusEl,sp[3],"good");await syncFb();reqR();
}
async function doStory(){
const r=await cW("story-interact");if(!r?.ok)return null;
chip(statusEl,r.advanced?`StoryAutomaton -> ${r.title} · Conserved ${r.supply}`:`${r.title} · ${r.objective}`,r.advanced?"good":"wait");
return r;
}
const qVer=async(q)=>{try{return app?show(await app.dispatch(parse(q))):null;}catch{return null;}};
async function syncB(q=LODS[lodIdx].key,pre=null){
if(!app){chip(statusEl,reason?"Verifier unavailable":"Runtime offline","bad");return;}
const r=await qVer(q);if(!r)return;
if(r.startsWith("conformant:"))chip(statusEl,pre?`${pre} · ${q} Conformant`:`Runtime Conformant · ${LODS[lodIdx].label} · 15 Semirings`,"good");
else chip(statusEl,r.startsWith("refused:")?r.replace("refused:","Refused: "):r,r.startsWith("refused:")?"warn":"bad");
}
async function togRnd(force=null){
renderMode=force||(renderMode==="webgl2"?"tile":"webgl2");
if(renderBtn)renderBtn.textContent=renderMode==="tile"?"Raster: Wasm Tile":"Raster: WebGL2";
const r=await cW("set-render-mode",{mode:renderMode,simClockSec:sClk});
syncTitle();if(hudStats)hudStats.textContent=hudStr("");
chip(statusEl,`${renderMode==="tile"?"Wasm 16x16 Software Tile":"WebGL2 3-Pass PBR"} (${r?.tris??lTris} tris)`,"good");reqR();
}
const impGlb=async(name,buf)=>(await getStudio()).importCardGlb(name,buf,{
chip,statusEl,cW,isOrganAllowed,onImported:async(t,suf)=>{if(t)lTris=t;await syncFb();syncTitle(suf);reqR();},
});
const togAudio=async()=>!isOrganAllowed(3,true)?refuseOrgan(3):(await getStudio()).toggleModalAudio(audioBtn,ACOUST,()=>sEye);
const openStitch=async()=>(await import("./stitch-mesh.mjs")).openStitchMeshModal({callWorker:cW,queryVerifier:qVer,isOrganAllowed,onRefuseOrgan:refuseOrgan,onRestoreSnapshot:rstSnap});
const execP=async(t)=>(await getStudio()).routeStudioPrompt(t,{
callWorker:cW,queryVerifier:qVer,chip,statusEl,streamUorcToRank:strUorc,cycleCameraMode:cycCam,
interactChronicle:doStory,focusNodePreset:focNd,setSimLive:setSim,toggleRenderBackend:togRnd,
restoreRawSnapshot:rstSnap,launchStudioModal:openSt,openStitchModal:openStitch,
});
async function openSt(tab=null){
if(typeof tab==="string")syncRole(tab);
const mod=await getStudio();
mod.launchCardStudio(mod,typeof tab==="string"?tab:activeRole,{
cW,qVer,chip,statusEl,onPickGlb:()=>glbIn?.click(),
onReset:async(t)=>{if(t)lTris=t;syncTitle();await syncFb();await syncB("summary");reqR();},
getLodShort:()=>LODS[lodIdx].short,togRnd,strUorc,cycCam,setSim,togAudio,focNd,
onFocusPointRaw:async(target,dist,yawMilli,pitchMilli)=>{
camPreset="landmark";fMov=false;syncPos();if(camBtn)camBtn.textContent="Camera: Orbit";
apCam(await cW("camera-focus-landmark",{target,dist,yawMilli,pitchMilli,simClockSec:sClk}));await syncFb();reqR();
},
onSeeded:async(r)=>{apCam(r);await syncFb();reqR();chip(statusEl,`Runtime Seed ${r.seed} · ${LODS[lodIdx].label}`,"good");},
onStreamOpfs:(customStream)=>{const rk=Math.min(3,Math.max(2,lodIdx));return cW("stream-uorc",{targetRank:rk,customStream,lod:LODS[rk],simClockSec:sClk});},
onMountedOpfs:async(name,t)=>{const rk=Math.min(3,Math.max(2,lodIdx));lodIdx=rk;if(lodBtn)lodBtn.textContent=`Fidelity: ${LODS[rk].short}`;syncTitle(`OPFS ${name} (${t} tris)`);lTris=t||lTris;await syncFb();reqR();},
openStitch,execP,doStory,syncRole,
onProjectChange:(t)=>{if(t){activeProject=t;syncTitle();}},getActiveProject:()=>activeProject,
getAcousticSpec:()=>ACOUST,isOrganAllowed,getOrganMode:(i)=>organModes[i]??0,
onOrganGrantChange:(i,m)=>{organModes[i]=m;if(i===3&&m===2)stM?.suspendModalAudio?.(audioBtn);},
refuseOrgan,getRenderMode:()=>renderMode,getLodRank:()=>lodIdx,
});
}
async function setLod(n,upd=true){
lodIdx=((n%LODS.length)+LODS.length)%LODS.length;const lod=LODS[lodIdx];
if(lodBtn)lodBtn.textContent=`Fidelity: ${lod.short}`;syncTitle();if(lod.rank<2)fMov=false;
if(eW){eW.postMessage({kind:"set-lod",lod,simClockSec:sClk});await syncFb();}
reqR();if(upd)syncB(lod.key);
}
async function focNd(idx,title=null,query=null){
const n=NODES[idx];if(!n)return;
aSlot=Math.min(6,n.id);gazeMs=1400;
if(lodIdx<3){lodIdx=3;if(lodBtn)lodBtn.textContent=`Fidelity: ${LODS[3].short}`;syncTitle();}
fMov=n.followMover;if(fMov||n.id===3)setSim(true);
camPreset=n.possessExplorer?"third":"landmark";syncPos();
if(camBtn)camBtn.textContent=n.possessExplorer?"Camera: 3rd IK":"Camera: Orbit";
apCam(await cW("camera-focus-landmark",{possessExplorer:n.possessExplorer,target:n.target,dist:n.dist,yawMilli:n.yawMilli,pitchMilli:n.pitchMilli,lod:LODS[lodIdx],simClockSec:sClk}));
if(hudStats)hudStats.textContent=hudStr("");await syncFb();if(isAnim()||!offOk)reqR();
await syncB(query||n.query,title||n.name);
}
async function strUorc(rk){
const c=Math.max(0,Math.min(LODS.length-1,rk|0));lodIdx=c;const lod=LODS[c];
if(lodBtn)lodBtn.textContent=`Fidelity: ${lod.short}`;if(uorcBtn)uorcBtn.textContent=c>=4?"Stream LOD0 Proxy":"Stream LOD4 HD";
syncTitle();if(lod.rank<2)fMov=false;chip(statusEl,`Streaming .uorc -> ${lod.label}…`,"wait");
if(eW){
const r=await cW("stream-uorc",{targetRank:c,lod,simClockSec:sClk});if(r?.tris)lTris=r.tris;await syncFb();reqR();
if(r?.ok){chip(statusEl,`Streamed ${lod.label} (${lTris} tris · Σ=${r.codedChecksum})`,"good");return r;}
}
await setLod(c,true);return null;
}
const on=(el,fn)=>el?.addEventListener("click",fn);
on(lodBtn,()=>setLod(lodIdx+1));on(uorcBtn,()=>strUorc(lodIdx<4?4:0));on(renderBtn,()=>togRnd());on(importBtn,()=>glbIn?.click());
glbIn?.addEventListener("change",async()=>{const f=glbIn.files?.[0];if(f){await impGlb(f.name,await f.arrayBuffer());glbIn.value="";}});
on(studioBtn,()=>openSt());on(meshBtn,openStitch);on(simBtn,()=>setSim(!simLive));on(camBtn,()=>cycCam());
promptForm?.addEventListener("submit",async(e)=>{e.preventDefault();await execP(promptInput?.value);if(promptInput)promptInput.value="";});
vpEl?.addEventListener("dragover",(e)=>{e.preventDefault();vpEl.classList.add("drag-over");});
vpEl?.addEventListener("dragleave",()=>vpEl.classList.remove("drag-over"));
vpEl?.addEventListener("drop",async(e)=>{e.preventDefault();vpEl.classList.remove("drag-over");const f=e.dataTransfer?.files?.[0];if(f)await impGlb(f.name,await f.arrayBuffer());});
on(audioBtn,togAudio);on(glbBtn,async()=>(await getStudio()).exportGlbDownload(cW,LODS[lodIdx].short));
on(snapBtn,async()=>(await getStudio()).shareUorcSnapshot(cW,chip,statusEl));on(interactBtn,()=>doStory());
on(lockBtn,()=>document.pointerLockElement===canvas?document.exitPointerLock?.():canvas?.requestPointerLock?.());
document.addEventListener("pointerlockchange",()=>{const l=document.pointerLockElement===canvas;lockBtn?.classList.toggle("active",l);if(lockBtn)lockBtn.textContent=l?"Unlock":"Look";});
touchPad?.querySelectorAll("button[data-move]").forEach((btn)=>{
const code=btn.dataset.move,rel=()=>{keys.delete(code);btn.classList.remove("active");};
btn.addEventListener("pointerdown",(e)=>{e.preventDefault();keys.add(code);btn.classList.add("active");if(camPreset==="landmark")cycCam("third");else reqR();});
["pointerup","pointercancel","pointerleave"].forEach((ev)=>btn.addEventListener(ev,rel));
});
async function rstSnap(bytes,label="UORC"){
const r=await cW("restore-snapshot",{bytes});
if(r?.ok){await setLod(r.lodRank??3,false);chip(statusEl,`Restored ${label} · ${LODS[lodIdx].label} (${r.byteLen}B)`,"good");return true;}
chip(statusEl,`Refused tampered ${label} snapshot`,"warn");return false;
}
async function rstHash(){
const m=/^#(uorc|vdnt)=(.+)$/.exec(location.hash);if(!m)return false;
const b=(await getStudio()).base64UrlToBytes(m[2].trim());
if(!b?.byteLength){chip(statusEl,"Refused malformed UORC snapshot","warn");return true;}
await rstSnap(b,"UORC");return true;
}
addEventListener("hashchange",()=>{if(/^#(uorc|vdnt)=/.test(location.hash))rstHash();else if(location.hash.startsWith("#studio="))openSt(location.hash.slice(8).trim());});
let lx=0,ly=0;const onC=(ev,fn,opt)=>canvas?.addEventListener(ev,fn,opt);
onC("pointerdown",(e)=>{dragging=true;lx=e.clientX;ly=e.clientY;canvas.setPointerCapture?.(e.pointerId);reqR();});
onC("pointermove",(e)=>{
const l=document.pointerLockElement===canvas;if(!dragging&&!l)return;
const dx=l?(e.movementX||0):e.clientX-lx,dy=l?(e.movementY||0):e.clientY-ly;
lx=e.clientX;ly=e.clientY;gazeMs=Math.max(80,gazeMs-40);pYaw-=dx*0.007;pPit+=camPreset==="first"?-dy*0.0055:dy*0.006;reqR();
});
onC("pointerup",()=>{dragging=false;});onC("pointercancel",()=>{dragging=false;});
onC("wheel",(e)=>{e.preventDefault();pDst+=e.deltaY*0.018;reqR();},{passive:false});
addEventListener("deviceorientation",(e)=>{if(!isOrganAllowed(6)||e.gamma==null||e.beta==null)return;pYaw-=Math.max(-1,Math.min(1,e.gamma/45))*0.012;reqR();});
addEventListener("keydown",(e)=>{
if(/^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName??""))return;
if(e.code==="KeyE"&&!e.repeat){doStory();return;}
if(e.code==="KeyG"&&!e.repeat){openSt("code");return;}
if(e.code==="Space")jumpQ=true;keys.add(e.code);
if(hasMv()&&camPreset==="landmark"){cycCam("third");return;}
reqR();
});
addEventListener("keyup",(e)=>keys.delete(e.code));addEventListener("blur",()=>keys.clear());addEventListener("resize",reqR);
document.addEventListener("visibilitychange",()=>{if(document.hidden)keys.clear();else{prevNow=performance.now();reqR();}});
if(typeof ResizeObserver!=="undefined"&&vpEl)new ResizeObserver(reqR).observe(vpEl);
function frame(now){
rafId=0;if(document.hidden)return;
if(offOk&&eW&&workerFramesInFlight>=2){if(now-lSent>1500)workerFramesInFlight=0;else{if(isAnim())rafId=requestAnimationFrame(frame);return;}}
const dt=Math.min(0.05,Math.max(0.001,(now-prevNow)*0.001));prevNow=now;
const it=mvInt(dt),step=simLive||fMov||it.moving;if(step)sClk+=dt;
if(!dragging)gazeMs=Math.min(3000,gazeMs+Math.round(dt*1000));
const lod=LODS[lodIdx],{w,h}=pxDims();
if(offOk&&eW){
workerFramesInFlight++;lSent=now;lW=w;lH=h;
eW.postMessage({kind:"frame",width:w,height:h,dt,stepPhysics:step,followMover:fMov,fwd:it.fwd,strafe:it.strafe,speedScale:it.speedScale,jump:it.jump,deltaYaw:it.dYaw,deltaPitch:it.dPitch,deltaDist:it.dDist,gazedSlot:aSlot,dwellMs:gazeMs,angSpeedMilli:dragging?600:60,lod,simClockSec:sClk});
}else if(fbR&&fbM&&canvas){
if(canvas.width!==w||canvas.height!==h){canvas.width=w;canvas.height=h;}
const proj=fbM.mat4Perspective(Math.PI / 3.4, Math.max(0.2, w /h),0.25,140.0);
fbR.render(fbM.mat4Mul(proj,fbM.mat4LookAt(sEye,sTgt,[0,1,0])),sEye,lod,sClk,sTgt,Math.PI/3.4);
}
frames++;
if(!isAnim()){if(hudStats&&lTris>0)hudStats.textContent=hudStr("");}
else if(now-lastFps>=600){const fps=Math.max(1,Math.round((frames*1000)/(now-lastFps)));frames=0;lastFps=now;if(hudStats)hudStats.textContent=hudStr(`${fps} fps`);}
if(isAnim())rafId=requestAnimationFrame(frame);
}
const badges=[],el=(t,c,x)=>{const n=document.createElement(t);if(c)n.className=c;if(x)n.textContent=x;return n;};
if(tele){
tele.replaceChildren();
for(const s of STRATA){
const li=el("li"),b=el("button","again"),bg=el("span","ve-stratum-sub",s.tag);
b.type="button";b.id=`ve-stratum-${s.key}-btn`;b.dataset.stratumKey=s.key;
b.setAttribute("aria-current",String(s.key===activeStratum));
b.title=`Frame ${s.label} in the 3D stage and verify ${s.query}`;
b.append(el("span",0,s.label),bg);
b.onclick=async()=>{syncStratum(s.key);activeRole=s.roleKey;await focNd(s.nodeIdx,s.label,s.query);};
li.append(b);tele.append(li);stratumBtns.set(s.key,b);badges.push({s,bg,b});
}
}
const apInit=(r)=>{if(!r)return;apCam(r);if(r.audioSpec)Object.assign(ACOUST,r.audioSpec);};
const iD=pxDims();let cXfer=false;
if(eW&&canvas&&typeof canvas.transferControlToOffscreen==="function"){
try{
const oc=canvas.transferControlToOffscreen();cXfer=true;
const r=await cW("init",{canvas:oc,width:iD.w,height:iD.h,lod:LODS[lodIdx]},[oc]);
if(r?.kind==="ready"){offOk=true;lW=iD.w;lH=iD.h;isSw=Boolean(r.isSoftware);apInit(r);if(hudStats)hudStats.textContent=hudStr("");}
}catch{offOk=false;}
}
if(!offOk&&!cXfer&&canvas){
fbM=await import("./scene-renderer.mjs");fbR=await fbM.createBestRenderer(canvas);isSw=Boolean(fbR?.isSoftware);
if(eW){apInit(await cW("init",{canvas:null,width:iD.w,height:iD.h,lod:LODS[lodIdx]}));await syncFb();}
}
syncTitle();
const bootSnap=await rstHash();
if(location.hash.startsWith("#studio="))openSt(location.hash.slice(8).trim());
if(!bootSnap)chip(statusEl,`Runtime Conformant · ${LODS[lodIdx].label} · 15 Semirings`,"good");
reqR();
(async()=>{
if(!app){if(!bootSnap)chip(statusEl,reason?"Verifier unavailable":"Runtime offline","bad");return;}
for(let w=0;w<40;w++){if(/^\d+\/\d+$/.test(document.getElementById("frame")?.getAttribute("examples")||""))break;await new Promise((r)=>setTimeout(r,50));}
if(!bootSnap)await syncB("summary");
for(let i=0;i<LODS.length;i++){
await yieldT();const f=pConf(await qVer(`scene:lod:${i}`));if(!f)continue;
const l=LODS[i];l.res=pRat(f.res,l.res);l.density=pRat(f.density,l.density);l.lossless=f.lossless==="1";l.sky=pTri(f.sky,100,l.sky);l.fog=pTri(f.fog,100,l.fog);
}
for(const{s,bg,b}of badges){
await yieldT();if(!pConf(await qVer(s.query)))continue;
bg.className="ve-stratum-sub";bg.textContent=s.tag;b.dataset.holoConformant="true";b.title=`${s.label} (${s.tag}) — ${s.query} verified`;
}
await yieldT();const af=pConf(await qVer("scene:acoustics"));
if(af?.modes&&af?.denom){const nums=af.modes.split("/").map(Number).filter(Number.isFinite);if(nums.length){ACOUST.modeNumers=nums;ACOUST.modeDenom=Number(af.denom)||9;}}
reqR();
})();
}
