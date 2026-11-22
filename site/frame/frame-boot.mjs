// Verdant Blank Frame Bootloader (GF(2^8) k-of-n RS + SHA-256 + OPFS + WebRTC Peer Mesh + #holo User Pins)
const HOLO_MAGIC="VDNT_HOLO_REL_V1",STCH2_MAGIC="STCH2_RS",EPOCH_KEY="verdant_stitch_last_epoch_v1",PRISM_ID_KEY="verdant_zero_account_prism_id_v1",VAULT_KEY="verdant_private_boundary_shards_v1";
const GF_EXP=new Uint8Array(512),GF_LOG=new Uint8Array(256);
for(let i=0,x=1;i<255;i++){GF_EXP[i]=x;GF_LOG[x]=i;x=(x<<1)^(x&0x80?0x11d:0);}
for(let i=255;i<512;i++)GF_EXP[i]=GF_EXP[i-255];
const gfMul=(a,b)=>(a===0||b===0?0:GF_EXP[GF_LOG[a]+GF_LOG[b]]);
const gfInv=(a)=>{if(!a)throw new Error("GF(2^8) div0");return GF_EXP[255-GF_LOG[a]];};
const cauchy=(p,c,k)=>gfInv(p^(k+c));
const td=new TextDecoder(),te=new TextEncoder();
const sha256=async(b)=>new Uint8Array(await crypto.subtle.digest("SHA-256",b));
const toHex=(u)=>Array.from(u,(b)=>b.toString(16).padStart(2,"0")).join("");
const fromHex=(h)=>Uint8Array.from({length:h.length/2},(_,i)=>parseInt(h.slice(i*2,i*2+2),16));
const fromB64Url=(s)=>{const p=s.replace(/-/g,"+").replace(/_/g,"/");const bin=atob(p+"=".repeat((4-(p.length%4))%4));return Uint8Array.from(bin,(c)=>c.charCodeAt(0));};
const eq=(a,b)=>a.length===b.length&&a.reduce((d,v,i)=>d|(v^b[i]),0)===0;
function invertGf256(mat){
const k=mat.length,a=mat.map((r)=>r.slice()),inv=Array.from({length:k},(_,i)=>{const r=new Uint8Array(k);r[i]=1;return r;});
for(let col=0;col<k;col++){
let piv=col;while(piv<k&&a[piv][col]===0)piv++;if(piv===k)throw new Error("Singular RS");
if(piv!==col){[a[col],a[piv]]=[a[piv],a[col]];[inv[col],inv[piv]]=[inv[piv],inv[col]];}
const f=gfInv(a[col][col]);
for(let j=0;j<k;j++){a[col][j]=gfMul(a[col][j],f);inv[col][j]=gfMul(inv[col][j],f);}
for(let row=0;row<k;row++){if(row===col||!a[row][col])continue;const m=a[row][col];for(let j=0;j<k;j++){a[row][j]^=gfMul(m,a[col][j]);inv[row][j]^=gfMul(m,inv[col][j]);}}
}return inv;}
export function sanitizeStitchDeepLink(raw){
const uri=String(raw||"").trim();if(!uri.startsWith("stitch://"))throw new Error(`Refused non-stitch URI: ${uri}`);
const rest=uri.slice(9);if(!rest||rest.includes("..")||/[<>"'\\\x00-\x1f]/.test(rest))throw new Error(`Unsafe stitch URI: ${uri}`);
const s=rest.indexOf("/"),authority=s===-1?rest:rest.slice(0,s),sub=(s===-1?"":rest.slice(s+1)).replace(/^\/+|\/+$/g,"");
if(authority!=="verdant"&&!/^[0-9a-fA-F]{64}$/.test(authority))throw new Error(`Bad authority: ${authority}`);
const map={"":"studio/index.html",studio:"studio/index.html","studio/index.html":"studio/index.html",catalogs:"catalogs/index.html","catalogs/index.html":"catalogs/index.html",demo:"demo/index.html","demo/index.html":"demo/index.html",docs:"docs/index.html","docs/index.html":"docs/index.html",starter:"starter/index.html","starter/index.html":"starter/index.html",card:"card.html","card.html":"card.html"};
const virtualRoute=map[sub]||(/^[A-Za-z0-9/._-]+$/.test(sub)?sub:null);if(!virtualRoute)throw new Error(`Bad route: ${sub}`);
return{canonicalUri:`stitch://${authority}/${virtualRoute}`,authority,virtualRoute};}
export async function parseAndVerifyStch2Shard(buf){
const u8=new Uint8Array(buf);if(u8.length<=80||td.decode(u8.subarray(0,8))!==STCH2_MAGIC)throw new Error("Invalid STCH2");
const k=u8[8],n=u8[9],index=u8[10],originalByteLen=new DataView(u8.buffer,u8.byteOffset,u8.byteLength).getUint32(12,true);
const manifestRoot=u8.subarray(16,48),payloadRoot=u8.subarray(48,80),payload=u8.subarray(80);
if(!eq(await sha256(payload),payloadRoot))throw new Error(`Shard ${index} SHA-256 mismatch`);
return{k,n,index,originalByteLen,manifestRoot,payloadRoot,payload};}
export async function recoverKofNShards(shards){
if(!shards.length)throw new Error("No shards");
const{k,n,originalByteLen,manifestRoot}=shards[0],shardLen=shards[0].payload.length,seen=new Set(),sel=[];
for(const s of shards){
if(s.k!==k||s.n!==n||s.originalByteLen!==originalByteLen||s.payload.length!==shardLen||!eq(s.manifestRoot,manifestRoot))throw new Error("Incompatible STCH2 shards");
if(!seen.has(s.index)){seen.add(s.index);sel.push(s);if(sel.length===k)break;}
}
if(sel.length<k)throw new Error(`Need ${k} shards, got ${sel.length}`);
const subMat=sel.map((s)=>{const r=new Uint8Array(k);if(s.index<k)r[s.index]=1;else for(let c=0;c<k;c++)r[c]=cauchy(s.index-k,c,k);return r;});
const inv=invertGf256(subMat),out=new Uint8Array(k*shardLen);
for(let r=0;r<k;r++){const dst=out.subarray(r*shardLen,(r+1)*shardLen);for(let si=0;si<k;si++){const cf=inv[r][si];if(!cf)continue;const src=sel[si].payload;for(let b=0;b<shardLen;b++)dst[b]^=gfMul(cf,src[b]);}}
const exact=out.subarray(0,originalByteLen);if(!eq(await sha256(exact),manifestRoot))throw new Error("Recovered manifestRoot mismatch");
return exact;}
export async function verifyAndUnpackHoloCapsule(raw){
const u8=raw instanceof Uint8Array?raw:new Uint8Array(raw);
if(u8.length<20||td.decode(u8.subarray(0,16))!==HOLO_MAGIC)throw new Error("Bad .holo magic");
const mLen=new DataView(u8.buffer,u8.byteOffset,u8.byteLength).getUint32(16,true),bodyStart=20+mLen;
if(!mLen||bodyStart>u8.length)throw new Error("Truncated .holo manifest");
const manifest=JSON.parse(td.decode(u8.subarray(20,bodyStart)));
if(manifest.spec!=="verdant/release-capsule/1"||!Array.isArray(manifest.files))throw new Error("Bad manifest");
let body=u8.subarray(bodyStart);
if(manifest.compression==="gzip"){
body=new Uint8Array(await new Response(new Blob([body]).stream().pipeThrough(new DecompressionStream("gzip"))).arrayBuffer());
}else if(manifest.compression){throw new Error(`Unsupported .holo compression: ${manifest.compression}`);}
let cur=0;const files=[];
for(const e of manifest.files){
const end=cur+e.byteLen;if(end>body.length)throw new Error(`Truncated ${e.path}`);
const bytes=body.slice(cur,end);if(toHex(await sha256(bytes))!==e.sha256Hex)throw new Error(`SHA-256 mismatch: ${e.path}`);
files.push({path:e.path,mimeType:e.mimeType,bytes});cur=end;
}
if(cur!==body.length||cur!==manifest.totalPayloadBytes)throw new Error("Trailing unverified .holo bytes");
return{manifest,files,capsuleSha256Hex:toHex(await sha256(u8)),rawBytes:u8};}
async function getLocalPrismId(){
let rec=null;try{rec=JSON.parse(localStorage.getItem(PRISM_ID_KEY)||"null");}catch{}
if(rec?.prismId&&rec?.secretHex)return rec;
const sec=crypto.getRandomValues(new Uint8Array(32));
rec={prismId:`prism:local-${toHex(await sha256(sec)).slice(0,16)}`,secretHex:toHex(sec)};
try{localStorage.setItem(PRISM_ID_KEY,JSON.stringify(rec));}catch{}return rec;}
export async function sealPrivateBoundaryZeroAccount(boundaryId,plaintextUtf8,k=4,n=8){
const id=await getLocalPrismId(),key=await crypto.subtle.importKey("raw",fromHex(id.secretHex),{name:"AES-GCM"},false,["encrypt"]),iv=crypto.getRandomValues(new Uint8Array(12)),plain=te.encode(plaintextUtf8),pad=new Uint8Array(Math.ceil((plain.length+1)/256)*256);
pad.set(plain);pad[plain.length]=0x80;
const aad=te.encode(`${boundaryId}|${id.prismId}|epoch:1`),ct=new Uint8Array(await crypto.subtle.encrypt({name:"AES-GCM",iv,additionalData:aad},key,pad));
const envBytes=te.encode(JSON.stringify({spec:"verdant/private-boundary/1",boundaryId,ownerPrismId:id.prismId,visibility:"private-encrypted",plaintextLen:plain.length,ivHex:toHex(iv),ciphertextHex:toHex(ct)}));
const mRoot=await sha256(envBytes),sLen=Math.ceil(envBytes.length/k),dBlocks=Array.from({length:k},(_,i)=>{const blk=new Uint8Array(sLen),st=i*sLen;if(st<envBytes.length)blk.set(envBytes.subarray(st,Math.min(st+sLen,envBytes.length)));return blk;});
const shards=[];
for(let i=0;i<n;i++){
let blk=i<k?dBlocks[i]:new Uint8Array(sLen);
if(i>=k)for(let c=0;c<k;c++){const cf=cauchy(i-k,c,k);for(let b=0;b<sLen;b++)blk[b]^=gfMul(cf,dBlocks[c][b]);}
const pRoot=await sha256(blk),f=new Uint8Array(80+blk.length);
f.set(te.encode(STCH2_MAGIC),0);f[8]=k;f[9]=n;f[10]=i;new DataView(f.buffer).setUint32(12,envBytes.length,true);f.set(mRoot,16);f.set(pRoot,48);f.set(blk,80);shards.push(f);
}
try{localStorage.setItem(VAULT_KEY,JSON.stringify(shards.map(toHex)));}catch{}
return{ownerPrismId:id.prismId,shardCount:shards.length,shards,envelopeKappaHex:toHex(mRoot)};}
export async function recoverPrivateBoundaryZeroAccount(rawShards){
const id=await getLocalPrismId(),parsed=await Promise.all(rawShards.map(parseAndVerifyStch2Shard)),env=JSON.parse(td.decode(await recoverKofNShards(parsed)));
if(env.ownerPrismId!==id.prismId)throw new Error("Unauthorized Prism ID");
const key=await crypto.subtle.importKey("raw",fromHex(id.secretHex),{name:"AES-GCM"},false,["decrypt"]),aad=te.encode(`${env.boundaryId}|${env.ownerPrismId}|epoch:1`);
const pad=new Uint8Array(await crypto.subtle.decrypt({name:"AES-GCM",iv:fromHex(env.ivHex),additionalData:aad},key,fromHex(env.ciphertextHex)));
return{ownerPrismId:env.ownerPrismId,boundaryId:env.boundaryId,plaintextUtf8:td.decode(pad.subarray(0,env.plaintextLen))};}
async function mountSwVfs(capsule){
if(!("serviceWorker"in navigator))throw new Error("ServiceWorker unavailable");
const reg=await navigator.serviceWorker.register("./frame-sw.js",{scope:"./"});await navigator.serviceWorker.ready;
const sw=reg.active||navigator.serviceWorker.controller;if(!sw)throw new Error("ServiceWorker inactive");
return new Promise((res,rej)=>{const ch=new MessageChannel();ch.port1.onmessage=(e)=>(e.data?.ok?res(e.data):rej(new Error(e.data?.error||"VFS mount failed")));sw.postMessage({type:"MOUNT_CAPSULE_VFS",files:capsule.files},[ch.port2]);});}
async function cacheOpfs(name,bytes){try{if(!navigator.storage?.getDirectory)return false;const fh=await(await navigator.storage.getDirectory()).getFileHandle(name,{create:true});const w=await fh.createWritable();await w.write(bytes);await w.close();return true;}catch{return false;}}
async function readOpfs(name){try{if(!navigator.storage?.getDirectory)return null;const fh=await(await navigator.storage.getDirectory()).getFileHandle(name);return new Uint8Array(await(await fh.getFile()).arrayBuffer());}catch{return null;}}
export function createStitchPeerMesh(localId,capBytes=16777216){
const shardCache=new Map(),usage=new Map(),stats={servedShards:0,receivedShards:0,refusedOverBudget:0,webrtcChannels:0};
const bc=typeof BroadcastChannel==="function"?new BroadcastChannel("verdant-stitch-peers-v1"):null;
const admit=(pid,n)=>{if(!String(pid||"").startsWith("prism:")||n<=0)throw new Error("Bad peer id");const next=(usage.get(pid)||0)+n;if(next>capBytes){stats.refusedOverBudget++;throw new Error("Over peer budget");}usage.set(pid,next);};
const cacheVerified=async(pid,raw)=>{const u8=raw instanceof Uint8Array?raw:new Uint8Array(raw),parsed=await parseAndVerifyStch2Shard(u8),key=`${toHex(parsed.manifestRoot)}:${parsed.index}`;admit(pid,u8.byteLength);shardCache.set(key,u8);stats.receivedShards++;return{key,parsed};};
const serveToPeer=(reqId,key)=>{const raw=shardCache.get(key);if(!raw)throw new Error(`No shard ${key}`);admit(reqId,raw.byteLength);stats.servedShards++;return raw;};
const openLoopbackWebRtcDemo=async(key)=>{
if(typeof RTCPeerConnection!=="function")return{supported:false};
const raw=shardCache.get(key);if(!raw)return{supported:true,transferred:0};
const a=new RTCPeerConnection(),b=new RTCPeerConnection();
a.onicecandidate=(e)=>e.candidate&&b.addIceCandidate(e.candidate).catch(()=>{});
b.onicecandidate=(e)=>e.candidate&&a.addIceCandidate(e.candidate).catch(()=>{});
const dc=a.createDataChannel("verdant-stch2-v1");
const done=new Promise((res,rej)=>{
const tm=setTimeout(()=>rej(new Error("WebRTC timeout")),3000);
b.ondatachannel=(ev)=>{stats.webrtcChannels++;const parts=[];ev.channel.binaryType="arraybuffer";ev.channel.onmessage=async(m)=>{if(m.data==="EOF"){clearTimeout(tm);const tot=parts.reduce((s,c)=>s+c.byteLength,0),buf=new Uint8Array(tot);let o=0;for(const c of parts){buf.set(c,o);o+=c.byteLength;}try{await parseAndVerifyStch2Shard(buf);res({supported:true,verified:true,transferred:tot});}catch(e){rej(e);}}else parts.push(new Uint8Array(m.data));};};
dc.onopen=()=>{for(let o=0;o<raw.byteLength;o+=16384)dc.send(raw.subarray(o,Math.min(o+16384,raw.byteLength)));dc.send("EOF");};
});
const offer=await a.createOffer();await a.setLocalDescription(offer);await b.setRemoteDescription(offer);
const ans=await b.createAnswer();await b.setLocalDescription(ans);await a.setRemoteDescription(ans);
try{return await done;}finally{a.close();b.close();}};
if(bc)bc.onmessage=(e)=>{const m=e.data;if(m?.type==="STCH2_WANT"&&m.from!==localId&&shardCache.has(m.key))try{bc.postMessage({type:"STCH2_HAVE",from:localId,key:m.key,bytes:serveToPeer(m.from,m.key)});}catch{}};
return{localPrismId:localId,shardCache,stats,cacheVerified,serveToPeer,openLoopbackWebRtcDemo};}
async function fetchShards(pin,relayOverride,gwOverride,mesh){
const gwRelays=gwOverride?gwOverride.split(",").map((s)=>s.trim().replace(/\/+$/,"")).filter(Boolean).map((u)=>`${u}/shards`):[];
const baseRelays=relayOverride?relayOverride.split(",").map((s)=>s.trim()).filter(Boolean):[...gwRelays,...(pin.stitchRelays||[]),new URL("./shards",location.href).href];
const relays=[...new Set(baseRelays)];
const k=pin.dataShardsK||4,collected=[],relayHits={},failedRelays={};let dropped=0;
const raceOne=(fn)=>Promise.any(relays.map(async(base)=>{
const clean=base.replace(/\/+$/,""),res=await fetch(`${clean}/${fn}`,{cache:"no-store"});
if(!res.ok){failedRelays[clean]=(failedRelays[clean]||0)+1;throw new Error(`HTTP ${res.status}`);}
const raw=new Uint8Array(await res.arrayBuffer()),shard=await parseAndVerifyStch2Shard(raw);
relayHits[clean]=(relayHits[clean]||0)+1;if(mesh)try{await mesh.cacheVerified(mesh.localPrismId,raw);}catch{}return shard;}));
for(const r of await Promise.allSettled((pin.shardFiles||[]).map(raceOne))){if(r.status==="fulfilled"&&collected.length<k)collected.push(r.value);else if(r.status==="rejected")dropped++;}
const activeGateway=Object.keys(relayHits)[0]||null;
return{collected,dropped,relayHits,failedRelays,relayCount:relays.length,activeGateway};}
export async function bootBlankFrame(){
const $=(id)=>document.getElementById(id),setStatus=(t,d)=>{if($("frame-status"))$("frame-status").textContent=t;if($("frame-detail"))$("frame-detail").textContent=d;};
const id=await getLocalPrismId();if($("prism-id-badge"))$("prism-id-badge").textContent=`${id.prismId} (zero-account local key)`;
const peerMesh=createStitchPeerMesh(id.prismId);
window.__VERDANT_BLANK_FRAME__={status:"booting",sanitizeStitchDeepLink,sealPrivateBoundaryZeroAccount,recoverPrivateBoundaryZeroAccount,verifyAndUnpackHoloCapsule,peerMesh};
const params=new URLSearchParams(location.search);
async function mountAndShow(capsule,route,meta){
const swRes=await mountSwVfs(capsule);await cacheOpfs(`verdant-v${capsule.manifest.version}.holo`,capsule.rawBytes);
const iframe=document.createElement("iframe");iframe.id="verdant-lifted-frame";iframe.title="Verdant Self-Hosted Application";iframe.src=`./app/${route}`;iframe.setAttribute("allow","fullscreen; gamepad");
$("frame-mount").replaceChildren(iframe);$("frame-mount").hidden=false;if(params.get("embed")!=="shell")$("frame-shell").classList.add("compact");
let govBadge="";try{const ke=capsule.files.find((f)=>f.path==="generated/roadmap-kpi-report.json");if(ke){const k=JSON.parse(td.decode(ke.bytes));govBadge=` · ${k.totalManagers} Managers · ${k.kpiPassedCount}/${k.kpiTotalCount} KPIs`;}}catch{}
setStatus(`Mounted Verdant v${capsule.manifest.version} via Stitch Virtual FS`,`${swRes.mountedFiles} verified files${govBadge} · kappa ${capsule.capsuleSha256Hex.slice(0,16)}… · /${route}`);
window.__VERDANT_BLANK_FRAME__={...window.__VERDANT_BLANK_FRAME__,status:"mounted",version:capsule.manifest.version,capsuleSha256Hex:capsule.capsuleSha256Hex,mountedFiles:swRes.mountedFiles,targetRoute:route,...meta};}
$("holo-file-input")?.addEventListener("change",async(e)=>{const f=e.target.files?.[0];if(!f)return;try{const cap=await verifyAndUnpackHoloCapsule(new Uint8Array(await f.arrayBuffer()));await mountAndShow(cap,sanitizeStitchDeepLink($("pin-uri-input")?.value||"stitch://verdant/studio").virtualRoute,{source:"offline-holo-file"});}catch(err){setStatus("Offline .holo failed",String(err));}});
$("vault-test-btn")?.addEventListener("click",async()=>{try{const s=await sealPrivateBoundaryZeroAccount("stitch://boundary/local-vault",$("vault-note-input")?.value||"Private note",4,8);const r=await recoverPrivateBoundaryZeroAccount([s.shards[1],s.shards[3],s.shards[5],s.shards[7]]);if($("vault-output"))$("vault-output").textContent=`Sealed under ${s.ownerPrismId} (4 of 8 dropped) → "${r.plaintextUtf8}"`;}catch(err){if($("vault-output"))$("vault-output").textContent=`Vault error: ${err}`;}});
$("pin-form")?.addEventListener("submit",(e)=>{e.preventDefault();try{const p=sanitizeStitchDeepLink($("pin-uri-input").value),u=new URL(location.href);u.searchParams.set("pin",p.canonicalUri);location.href=u.href;}catch(err){setStatus("Refused unsafe deep-link URI",String(err));}});
try{
const reqPin=params.get("pin"),hashHolo=/^#holo=([A-Za-z0-9_-]+)$/.exec(location.hash||"");
if(hashHolo){
const target=sanitizeStitchDeepLink(reqPin||"stitch://verdant/studio");if($("pin-uri-input"))$("pin-uri-input").value=target.canonicalUri;
const cap=await verifyAndUnpackHoloCapsule(fromB64Url(hashHolo[1]));
if(target.authority!=="verdant"&&cap.capsuleSha256Hex!==target.authority.toLowerCase())throw new Error("Pinned kappa mismatch");
await cacheOpfs(`verdant-app-${cap.capsuleSha256Hex}.holo`,cap.rawBytes);
await mountAndShow(cap,target.virtualRoute,{source:"stitch-user-pin-holo"});return;}
if(reqPin){const t=sanitizeStitchDeepLink(reqPin);if(/^[0-9a-f]{64}$/i.test(t.authority)){const cached=await readOpfs(`verdant-app-${t.authority.toLowerCase()}.holo`);if(cached){const cap=await verifyAndUnpackHoloCapsule(cached);if(cap.capsuleSha256Hex!==t.authority.toLowerCase())throw new Error("OPFS kappa mismatch");if($("pin-uri-input"))$("pin-uri-input").value=t.canonicalUri;await mountAndShow(cap,t.virtualRoute,{source:"stitch-user-pin-opfs"});return;}}}
const pinRes=await fetch("./frame-pin.json",{cache:"no-store"});if(!pinRes.ok)throw new Error(`frame-pin.json HTTP ${pinRes.status}`);
const pin=await pinRes.json(),target=sanitizeStitchDeepLink(reqPin||pin.defaultStitchUri||"stitch://verdant/studio");
if($("pin-uri-input"))$("pin-uri-input").value=target.canonicalUri;
const lastEpoch=Number(localStorage.getItem(EPOCH_KEY)||"0");if(Number(pin.releaseEpoch)<lastEpoch)throw new Error(`Rollback refused: ${pin.releaseEpoch} < ${lastEpoch}`);
const{collected,dropped,relayHits,failedRelays,relayCount,activeGateway}=await fetchShards(pin,params.get("relay")||"",params.get("gateway")||"",peerMesh);
const parts=["studio","catalogs","demo","docs","starter","card"];
if(collected.length<pin.dataShardsK){
const cached=await readOpfs(`verdant-v${pin.releaseVersion}.holo`);
if(cached){const offCap=await verifyAndUnpackHoloCapsule(cached);if(offCap.capsuleSha256Hex===pin.releaseKappaHex){await mountAndShow(offCap,target.virtualRoute,{source:"stitch-opfs-offline-fallback",relayCount,failedRelays,activeGateway:"opfs-cache",availableParts:parts});return;}}
throw new Error(`Only ${collected.length}/${pin.dataShardsK} shards reachable — drop a portable .holo for offline boot.`);}
const cap=await verifyAndUnpackHoloCapsule(await recoverKofNShards(collected));if(cap.capsuleSha256Hex!==pin.releaseKappaHex)throw new Error("Recovered .holo kappa mismatch");
try{localStorage.setItem(EPOCH_KEY,String(pin.releaseEpoch));}catch{}
await mountAndShow(cap,target.virtualRoute,{source:"stitch-4-of-8-pull-through",shardsUsed:collected.length,shardsDropped:dropped,relayHits,failedRelays,relayCount,activeGateway,availableParts:parts});
}catch(err){setStatus("Blank Frame ready (awaiting Stitch shards or offline .holo)",String(err));window.__VERDANT_BLANK_FRAME__={...window.__VERDANT_BLANK_FRAME__,status:"error",error:String(err)};}}
if(document.readyState==="loading")document.addEventListener("DOMContentLoaded",()=>bootBlankFrame());else bootBlankFrame();
