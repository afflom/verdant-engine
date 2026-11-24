//! **Public-Benefit Stitch Gateway Execution, Portable Offline HTML Envelope & Volunteer `/swarm` Bridge (`gateway_swarm.rs`, `VE-77`, `VE-79`)**.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::time::Duration;

use verdant_tensor::scalars::{
    eight_u32, four_u32, gateway_blank_frame_budget_ok, gateway_http_status_no_content,
    gateway_http_status_ok, gateway_http_status_redirect, gateway_http_status_unavailable,
    gateway_immutable_cache_max_age_secs, gateway_provider_families_independent,
    stitch_l5_chunk_bytes_max, stitch_l5_epoch_request_cap, stitch_l5_transfer_budget_bytes,
    stitch_l5_wire_bytes_max, two_u32,
};

use crate::digest::sha256_hex;
use crate::error::ObserverModelError;
use crate::gateway::{
    build_multi_gateway_manifest, GatewayFailoverReport, GatewayOp, GatewayOpOutcome,
    GatewayServeReport, MultiGatewayManifest,
};
use crate::observer::CommunicationChannelKind;
use crate::vcs::FramePinManifest;

/// Executes a native public-benefit multi-gateway Stitch hosting operation (`PackBundle`, `Serve`, or
/// `VerifyFailover`), providing GitHub + Stitch Layer-5 Swarm + IPFS + Zenodo + Volunteer Self-Hosted parity (`VE-77`, `VE-78`).
#[cfg(not(target_arch = "wasm32"))]
pub fn execute_gateway_op(
    op: &GatewayOp,
) -> Result<(GatewayOpOutcome, String), ObserverModelError> {
    let make_err = |detail: String| ObserverModelError::CommunicationEnvelopeRejected {
        channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
        detail,
    };

    match op {
        GatewayOp::PackBundle {
            repo_root,
            release_dir,
            out_dir,
            extra_gateways,
            crates_dir,
        } => {
            let pin_raw = std::fs::read_to_string(release_dir.join("frame-pin.json"))
                .map_err(|e| make_err(format!("read release frame-pin.json: {e}")))?;
            let mut pin: FramePinManifest = serde_json::from_str(&pin_raw)
                .map_err(|e| make_err(format!("parse release frame-pin.json: {e}")))?;
            let manifest = build_multi_gateway_manifest(&pin, extra_gateways)?;

            // Merge every verified gateway shard relay URL into the bundle's frame-pin.json:
            let mut merged_relays: Vec<String> = Vec::new();
            let mut seen_r: BTreeSet<String> = BTreeSet::new();
            for g in &manifest.gateways {
                if seen_r.insert(g.shard_relay_url.clone()) {
                    merged_relays.push(g.shard_relay_url.clone());
                }
            }
            if !merged_relays.is_empty() {
                pin.stitch_relays = merged_relays;
            }

            let shards_out = out_dir.join("shards");
            let versioned_shards_out = out_dir
                .join("stitch")
                .join(format!("v{}", pin.release_version))
                .join("shards");
            let holo_out = out_dir.join("holo");
            std::fs::create_dir_all(&shards_out)
                .map_err(|e| make_err(format!("create gateway shards dir: {e}")))?;
            std::fs::create_dir_all(&versioned_shards_out)
                .map_err(|e| make_err(format!("create versioned shards dir: {e}")))?;
            std::fs::create_dir_all(&holo_out)
                .map_err(|e| make_err(format!("create gateway holo dir: {e}")))?;

            // Copy Blank Frame static shell (`<= 30 KiB`) to bundle root:
            let frame_src = repo_root.join("site/frame");
            let mut frame_total_bytes = 0u64;
            for name in ["index.html", "frame-boot.mjs", "frame-sw.js"] {
                let bytes = std::fs::read(frame_src.join(name))
                    .map_err(|e| make_err(format!("read site/frame/{name}: {e}")))?;
                frame_total_bytes = frame_total_bytes.saturating_add(bytes.len() as u64);
                std::fs::write(out_dir.join(name), &bytes)
                    .map_err(|e| make_err(format!("write gateway {name}: {e}")))?;
            }
            let pin_pretty = format!(
                "{}\n",
                serde_json::to_string_pretty(&pin)
                    .map_err(|e| make_err(format!("serialize bundle frame-pin.json: {e}")))?
            );
            frame_total_bytes = frame_total_bytes.saturating_add(pin_pretty.len() as u64);
            if !gateway_blank_frame_budget_ok(frame_total_bytes) {
                return Err(make_err(format!(
                    "gateway Blank Frame exceeds 30 KiB budget ({frame_total_bytes} B)"
                )));
            }
            std::fs::write(out_dir.join("frame-pin.json"), pin_pretty.as_bytes())
                .map_err(|e| make_err(format!("write gateway frame-pin.json: {e}")))?;

            // Copy all 8 STCH2 shards into both `/shards/` (portable/edge/IPFS) & `/stitch/v<ver>/shards/` (GitHub Pages):
            for fname in &pin.shard_files {
                let raw = std::fs::read(release_dir.join("shards").join(fname))
                    .map_err(|e| make_err(format!("read release shard {fname}: {e}")))?;
                let _ = crate::network::StitchRsShard::from_binary_stch2(&raw)
                    .map_err(|e| make_err(format!("verify release shard {fname}: {e}")))?;
                std::fs::write(shards_out.join(fname), &raw)
                    .map_err(|e| make_err(format!("write gateway shard {fname}: {e}")))?;
                std::fs::write(versioned_shards_out.join(fname), &raw)
                    .map_err(|e| make_err(format!("write versioned shard {fname}: {e}")))?;
            }

            // Copy verified `.holo` capsule:
            let holo_name = format!("verdant-v{}.holo", pin.release_version);
            let holo_bytes = std::fs::read(release_dir.join(&holo_name))
                .map_err(|e| make_err(format!("read {holo_name}: {e}")))?;
            if sha256_hex(&holo_bytes) != pin.release_kappa_hex {
                return Err(make_err(format!(
                    "{holo_name} SHA-256 does not match frame-pin releaseKappaHex"
                )));
            }
            std::fs::write(holo_out.join(&holo_name), &holo_bytes)
                .map_err(|e| make_err(format!("write gateway {holo_name}: {e}")))?;

            // Emit static redirect shims under `/part/<part>/index.html` so GitHub Pages, IPFS,
            // volunteer HTTP/OCI mirrors, and offline archives all serve clean stakeholder part links natively:
            for part in &manifest.stakeholder_parts {
                let pdir = out_dir.join("part").join(&part.part_id);
                std::fs::create_dir_all(&pdir)
                    .map_err(|e| make_err(format!("create part dir {}: {e}", part.part_id)))?;
                let target = format!("../../?pin={}", part.stitch_uri);
                let html = format!(
                    "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta http-equiv=\"refresh\" content=\"0;url={target}\"><title>Verdant — {}</title></head><body><p>Mounting <a href=\"{target}\">{}</a> via Stitch Blank Frame…</p><script>location.replace({:?});</script></body></html>\n",
                    part.label, part.stitch_uri, target
                );
                std::fs::write(pdir.join("index.html"), html.as_bytes()).map_err(|e| {
                    make_err(format!("write part/{}/index.html: {e}", part.part_id))
                })?;
            }

            // Emit `gateway-manifest.json`:
            let manifest_pretty = format!(
                "{}\n",
                serde_json::to_string_pretty(&manifest)
                    .map_err(|e| make_err(format!("serialize gateway-manifest.json: {e}")))?
            );
            std::fs::write(
                out_dir.join("gateway-manifest.json"),
                manifest_pretty.as_bytes(),
            )
            .map_err(|e| make_err(format!("write gateway-manifest.json: {e}")))?;

            // Emit generic static HTTP `_headers` + `_redirects` and public-benefit Stitch Layer-5 swarm descriptor
            // (zero commercial cloud provider manifests such as wrangler.toml or netlify.toml):
            let max_age = gateway_immutable_cache_max_age_secs();
            let headers_txt = format!(
                "/*\n  Access-Control-Allow-Origin: *\n  Cross-Origin-Resource-Policy: cross-origin\n  X-Verdant-Gateway: stitch-v1\n\n/shards/*\n  Cache-Control: public, max-age={max_age}, immutable\n  Content-Type: application/octet-stream\n\n/stitch/*\n  Cache-Control: public, max-age={max_age}, immutable\n  Content-Type: application/octet-stream\n\n/frame-pin.json\n  Cache-Control: no-store\n\n/gateway-manifest.json\n  Cache-Control: no-store\n"
            );
            std::fs::write(out_dir.join("_headers"), headers_txt.as_bytes())
                .map_err(|e| make_err(format!("write _headers: {e}")))?;

            let redirects_txt = "/part/:part /?pin=stitch://verdant/:part 302\n/healthz /gateway-manifest.json 200\n";
            std::fs::write(out_dir.join("_redirects"), redirects_txt.as_bytes())
                .map_err(|e| make_err(format!("write _redirects: {e}")))?;

            let kappa_prefix = &manifest.release_kappa_hex[..manifest
                .release_kappa_hex
                .len()
                .min((eight_u32() + eight_u32()) as usize)];
            let wire_max = 1u64 << (eight_u32() + four_u32());
            let swarm_desc = serde_json::json!({
                "spec": "verdant/stitch-layer5-swarm/1",
                "protocol": "stitch/1",
                "zeroCommercialRegistration": true,
                "releaseVersion": manifest.release_version,
                "releaseEpoch": manifest.release_epoch,
                "releaseKappaHex": manifest.release_kappa_hex,
                "opaqueTopic": format!("verdant-stitch-{kappa_prefix}"),
                "optInConsentRequired": true,
                "framingLimits": {
                    "wireBytesMax": wire_max,
                    "erasureDataShardsK": manifest.data_shards_k,
                    "erasureTotalShardsN": manifest.total_shards_n,
                },
                "publicBenefitCarriers": [
                    "stitch-layer5-webrtc-swarm",
                    "volunteer-stitch-http-gateway",
                    "ipfs-content-addressed-root",
                    "zenodo-permanent-doi",
                    "offline-opfs-portable-envelope"
                ]
            });
            std::fs::write(
                out_dir.join("stitch-swarm-config.json"),
                format!(
                    "{}\n",
                    serde_json::to_string_pretty(&swarm_desc).unwrap_or_default()
                ),
            )
            .map_err(|e| make_err(format!("write stitch-swarm-config.json: {e}")))?;

            // Emit IPFS / IPNS content-addressed root descriptor (100% relative paths):
            let ipfs_desc = serde_json::json!({
                "spec": "verdant/ipfs-gateway-root/1",
                "releaseVersion": manifest.release_version,
                "releaseEpoch": manifest.release_epoch,
                "releaseKappaHex": manifest.release_kappa_hex,
                "relativeFramePin": "./frame-pin.json",
                "relativeShardsDir": "./shards",
                "relativeManifest": "./gateway-manifest.json",
                "compatiblePublicGateways": [
                    "https://ipfs.io/ipfs/<CID>/",
                    "https://dweb.link/ipfs/<CID>/",
                    "https://<CID>.ipfs.w3s.link/"
                ]
            });
            std::fs::write(
                out_dir.join(".ipfs-gateway-root.json"),
                format!(
                    "{}\n",
                    serde_json::to_string_pretty(&ipfs_desc).unwrap_or_default()
                ),
            )
            .map_err(|e| make_err(format!("write .ipfs-gateway-root.json: {e}")))?;

            // Emit portable OCI / Docker static gateway Dockerfile:
            let dockerfile = "FROM nginx:1.27-alpine\nCOPY . /usr/share/nginx/html\nRUN printf 'server { listen 8080; root /usr/share/nginx/html; index index.html; add_header Access-Control-Allow-Origin * always; add_header Cross-Origin-Resource-Policy cross-origin always; add_header X-Verdant-Gateway stitch-v1 always; location = /healthz { try_files /gateway-manifest.json =404; } location / { try_files $uri $uri/ =404; } }\\n' > /etc/nginx/conf.d/default.conf\nEXPOSE 8080\n";
            std::fs::write(out_dir.join("Dockerfile"), dockerfile.as_bytes())
                .map_err(|e| make_err(format!("write gateway Dockerfile: {e}")))?;

            // Emit Offline Portable Single-File HTML Envelope (`verdant-portable-v<ver>.html`, `VE-79`):
            // Zero-network self-verifying HTML embedding the Base64 gzipped `.holo` capsule, inline
            // WebCrypto SHA-256 verification against `releaseKappaHex`, and `DecompressionStream("gzip")`.
            let b64_alpha = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let mut holo_b64 = String::with_capacity(
                (holo_bytes.len() / (two_u32() as usize + 1) + 1) * (four_u32() as usize),
            );
            let s6 = (four_u32() + two_u32()) as usize;
            let s4 = four_u32() as usize;
            let s2 = two_u32() as usize;
            let m6 = (1usize << s6) - 1;
            let mut bi = 0usize;
            while bi < holo_bytes.len() {
                let b0 = usize::from(holo_bytes[bi]);
                let has1 = bi + 1 < holo_bytes.len();
                let has2 = bi + s2 < holo_bytes.len();
                let b1 = if has1 {
                    usize::from(holo_bytes[bi + 1])
                } else {
                    0
                };
                let b2 = if has2 {
                    usize::from(holo_bytes[bi + s2])
                } else {
                    0
                };
                let n24 = (b0 << (s4 * s4)) | (b1 << (s4 * s2)) | b2;
                holo_b64.push(b64_alpha[(n24 >> (s6 + s6 + s6)) & m6] as char);
                holo_b64.push(b64_alpha[(n24 >> (s6 + s6)) & m6] as char);
                holo_b64.push(if has1 {
                    b64_alpha[(n24 >> s6) & m6] as char
                } else {
                    '='
                });
                holo_b64.push(if has2 {
                    b64_alpha[n24 & m6] as char
                } else {
                    '='
                });
                bi += s2 + 1;
            }
            let portable_name = format!("verdant-portable-v{}.html", manifest.release_version);
            let portable_html = format!(
                "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Verdant Engine v{} — Offline Portable Envelope</title><style>body{{font-family:system-ui,sans-serif;background:#0a0a0a;color:#fafafa;margin:0;padding:1.5rem}}main{{max-width:48rem;margin:0 auto}}code,pre{{font-family:ui-monospace,monospace;background:#141414;padding:.2rem .4rem;border-radius:4px}}button{{background:#fafafa;color:#0a0a0a;border:0;padding:.6rem 1rem;border-radius:6px;font-weight:600;cursor:pointer;margin:.25rem}}</style></head><body><main><h1>Verdant Engine v{} (Offline Portable Stitch Envelope)</h1><p>Zero-account, zero-network single-file archive. Verifies SHA-256 <code>{}</code> and embedded Holo v6 self-carrying proof record offline before unpacking via <code>DecompressionStream(\"gzip\")</code>.</p><p id=\"vp-status\">Verifying embedded .holo capsule…</p><div id=\"vp-parts\"></div><pre id=\"vp-log\"></pre></main><script id=\"verdant-holo-b64\" type=\"application/octet-stream\">{}</script><script type=\"module\">const KAPPA={:?},VER={:?};const td=new TextDecoder();const toHex=(u)=>Array.from(u,(b)=>b.toString(16).padStart(2,\"0\")).join(\"\");const sha256=async(u)=>new Uint8Array(await crypto.subtle.digest(\"SHA-256\",u));const eq=(a,b)=>a.length===b.length&&a.reduce((d,v,i)=>d|(v^b[i]),0)===0;const B3_IV=Uint32Array.of(0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19),B3_P=[2,6,3,10,7,0,4,13,1,11,12,5,9,14,15,8],ror=(x,n)=>((x>>>n)|(x<<(32-n)))>>>0;function b3Comp(cv,blk,cnt,bLen,flg){{const s=new Uint32Array(16);s.set(cv,0);s[8]=B3_IV[0];s[9]=B3_IV[1];s[10]=B3_IV[2];s[11]=B3_IV[3];s[12]=cnt>>>0;s[13]=Math.floor(cnt/4294967296)>>>0;s[14]=bLen>>>0;s[15]=flg>>>0;let m=Uint32Array.from(blk);const g=(a,b,c,d,mx,my)=>{{s[a]=(s[a]+s[b]+mx)>>>0;s[d]=ror(s[d]^s[a],16);s[c]=(s[c]+s[d])>>>0;s[b]=ror(s[b]^s[c],12);s[a]=(s[a]+s[b]+my)>>>0;s[d]=ror(s[d]^s[a],8);s[c]=(s[c]+s[d])>>>0;s[b]=ror(s[b]^s[c],7);}};for(let r=0;r<7;r++){{g(0,4,8,12,m[0],m[1]);g(1,5,9,13,m[2],m[3]);g(2,6,10,14,m[4],m[5]);g(3,7,11,15,m[6],m[7]);g(0,5,10,15,m[8],m[9]);g(1,6,11,12,m[10],m[11]);g(2,7,8,13,m[12],m[13]);g(3,4,9,14,m[14],m[15]);if(r<6)m=Uint32Array.from(B3_P,(i)=>m[i]);}}for(let i=0;i<8;i++){{s[i]^=s[i+8];s[i+8]^=cv[i];}}return s;}}const b3Words=(u)=>{{const b=new Uint8Array(64);b.set(u);const dv=new DataView(b.buffer),w=new Uint32Array(16);for(let i=0;i<16;i++)w[i]=dv.getUint32(i*4,true);return w;}},b3Parent=(l,r)=>{{const w=new Uint32Array(16);w.set(l,0);w.set(r,8);return w;}};function b3Chunk(u,cnt){{let cv=Uint32Array.from(B3_IV);if(!u.length)return{{cv,w:new Uint32Array(16),cnt,bLen:0,flg:3}};let p=0,bi=0;while(p+64<u.length){{cv=b3Comp(cv,b3Words(u.subarray(p,p+64)),cnt,64,bi===0?1:0).subarray(0,8);p+=64;bi++;}}return{{cv,w:b3Words(u.subarray(p)),cnt,bLen:u.length-p,flg:2|(bi===0?1:0)}};}}function blake3Bytes(u){{const fin=(o)=>{{const s=b3Comp(o.cv,o.w,0,o.bLen,o.flg|8),out=new Uint8Array(32),dv=new DataView(out.buffer);for(let i=0;i<8;i++)dv.setUint32(i*4,s[i],true);return out;}};if(u.length<=1024)return fin(b3Chunk(u,0));const stk=[];let p=0,cnt=0;while(p+1024<u.length){{let cv=b3Comp(...((c)=>[c.cv,c.w,c.cnt,c.bLen,c.flg])(b3Chunk(u.subarray(p,p+1024),cnt))).subarray(0,8),tot=cnt+1;while((tot&1)===0){{cv=b3Comp(B3_IV,b3Parent(stk.pop(),cv),0,64,4).subarray(0,8);tot=Math.floor(tot/2);}}stk.push(cv);p+=1024;cnt++;}}let o=b3Chunk(u.subarray(p),cnt);while(stk.length){{const l=stk.pop(),rc=b3Comp(o.cv,o.w,o.cnt,o.bLen,o.flg).subarray(0,8);o={{cv:B3_IV,w:b3Parent(l,rc),cnt:0,bLen:64,flg:4}};}}return fin(o);}}async function verifyHoloV6(u,exp){{if(u.length<=40||u[0]!==0x55||u[1]!==0x4f||u[2]!==0x52||u[3]!==0x43||u[4]!==2||u[5]!==2)throw new Error(\"Bad Holo v6 magic\");const pre=u.subarray(0,u.length-32),seal=u.subarray(u.length-32);if(!eq(blake3Bytes(pre),seal))throw new Error(\"Holo v6 BLAKE3 seal mismatch\");let p=8;const uleb=()=>{{let v=0,sc=1,st=p;for(;;){{if(p>=pre.length||p-st>7)throw new Error(\"Bad ULEB\");const b=pre[p++];v+=(b&0x7f)*sc;sc*=128;if(b<128)return v;}}}};const cnt=uleb(),lens=Array.from({{length:cnt}},uleb),bLen=uleb();if(cnt<10||p+bLen!==pre.length)throw new Error(\"Holo v6 bounds mismatch\");const objs=lens.map((l)=>{{const s=pre.subarray(p,p+l);p+=l;return s;}}),man=JSON.parse(td.decode(objs[0])),roles={{}};(man.objects||[]).forEach((r,i)=>{{const b=objs[i===0?1:i+2];if(!b||b.length!==r.length||`blake3:${{toHex(blake3Bytes(b))}}`!==r.kappa)throw new Error(`Holo v6 ${{r.role}} mismatch`);roles[r.role]=b;}});const snap=JSON.parse(td.decode(roles.snapshot)),lines=td.decode(objs[objs.length-1]).trim().split(\"\\n\").filter(Boolean);let stmt=null;for(const ln of lines){{const bnd=JSON.parse(ln),b64=bnd.dsseEnvelope.payload.replace(/-/g,\"+\").replace(/_/g,\"/\"),st=JSON.parse(td.decode(Uint8Array.from(atob(b64+\"=\".repeat((4-(b64.length%4))%4)),(c)=>c.charCodeAt(0))));if(st.predicateType===\"https://uor.foundation/prism/kernel-proof/v1\")stmt=st;}}if(!stmt)throw new Error(\"Missing kernel-proof/v1\");for(const[nm,bytes]of[[\"model\",roles.model],[\"snapshot\",roles.snapshot],[\"sources\",roles.sources]]){{const b3=toHex(blake3Bytes(bytes)),s2=toHex(await sha256(bytes));if(!(stmt.subject||[]).some((x)=>x.name===nm&&x.digest?.blake3===b3&&x.digest?.sha256===s2))throw new Error(`Subject ${{nm}} digest mismatch`);}}const thms=stmt.predicate?.theorems||[];if(stmt.predicate?.semantic_id!==snap.semantic_id||!thms.length||thms.some((t)=>t.status!==\"proved\"||(t.axioms||[]).includes(\"Classical.choice\")))throw new Error(\"Unproved theorem\");const rec={{holoKappa:`blake3:${{toHex(blake3Bytes(u))}}`,semanticId:snap.semantic_id,theoremsProved:thms.length}};if(exp&&(rec.holoKappa!==exp.holoKappa||rec.semanticId!==exp.semanticId||rec.theoremsProved!==exp.theoremsProved))throw new Error(\"Manifest proofRecord mismatch\");return rec;}}const raw=Uint8Array.from(atob(document.getElementById(\"verdant-holo-b64\").textContent.trim()),(c)=>c.charCodeAt(0));const digest=toHex(await sha256(raw));if(digest!==KAPPA){{document.getElementById(\"vp-status\").textContent=\"REFUSED: SHA-256 mismatch \"+digest;throw new Error(\"Tampered portable envelope\");}}const mLen=new DataView(raw.buffer).getUint32(16,true);const man=JSON.parse(td.decode(raw.subarray(20,20+mLen)));let body=raw.subarray(20+mLen);if(man.compression===\"gzip\")body=new Uint8Array(await new Response(new Blob([body]).stream().pipeThrough(new DecompressionStream(\"gzip\"))).arrayBuffer());let cur=0;const files=[];for(const f of man.files){{const sl=body.slice(cur,cur+f.byteLen);if(toHex(await sha256(sl))!==f.sha256Hex)throw new Error(\"File SHA-256 mismatch: \"+f.path);files.push({{...f,bytes:sl}});cur+=f.byteLen;}}const pf=files.find((f)=>f.path===\"generated/stitch-proof.holo\");let proofRecord=null;if(man.proofRecord||pf){{if(!man.proofRecord||!pf)throw new Error(\"Unbound Holo v6 proofRecord\");proofRecord=await verifyHoloV6(pf.bytes,man.proofRecord);}}window.__VERDANT_PORTABLE__={{verified:true,version:VER,kappa:digest,proofRecord}};const prTxt=proofRecord?` · Holo v6 proof (${{proofRecord.theoremsProved}} proved, ${{proofRecord.holoKappa.slice(0,19)}}…)`:'';document.getElementById(\"vp-status\").textContent=`VERIFIED offline v${{VER}} · ${{man.files.length}} lifted files${{prTxt}} · kappa ${{digest.slice(0,16)}}…`;const dl=document.createElement(\"button\");dl.textContent=`Save verdant-v${{VER}}.holo (${{raw.byteLength}} B)`;dl.onclick=()=>{{const a=document.createElement(\"a\");a.href=URL.createObjectURL(new Blob([raw],{{type:\"application/octet-stream\"}}));a.download=`verdant-v${{VER}}.holo`;a.click();}};document.getElementById(\"vp-parts\").append(dl);document.getElementById(\"vp-log\").textContent=man.files.map((f)=>`${{f.path}} (${{f.byteLen}} B)`).join(\"\\n\");</script></body></html>\n",
                manifest.release_version,
                manifest.release_version,
                manifest.release_kappa_hex,
                holo_b64,
                manifest.release_kappa_hex,
                manifest.release_version
            );
            std::fs::write(out_dir.join(&portable_name), portable_html.as_bytes())
                .map_err(|e| make_err(format!("write {portable_name}: {e}")))?;

            // Copy optional RFC 2789 Cargo Sparse Registry tree into `out_dir/crates`:
            if let Some(src_crates) = crates_dir {
                if src_crates.is_dir() {
                    let mut stack: Vec<(PathBuf, PathBuf)> =
                        vec![(src_crates.clone(), out_dir.join("crates"))];
                    while let Some((src, dst)) = stack.pop() {
                        std::fs::create_dir_all(&dst)
                            .map_err(|e| make_err(format!("create crates subdir: {e}")))?;
                        let rd = std::fs::read_dir(&src)
                            .map_err(|e| make_err(format!("read crates dir: {e}")))?;
                        for entry in rd.flatten() {
                            let ep = entry.path();
                            let target = dst.join(entry.file_name());
                            if ep.is_dir() {
                                stack.push((ep, target));
                            } else if ep.is_file() {
                                std::fs::copy(&ep, &target)
                                    .map_err(|e| make_err(format!("copy crate asset: {e}")))?;
                            }
                        }
                    }
                }
            }

            let outcome = GatewayOpOutcome::Packed(manifest);
            let pretty = serde_json::to_string_pretty(&outcome)
                .map_err(|e| make_err(format!("serialize gateway outcome: {e}")))?;
            Ok((outcome, pretty))
        }

        GatewayOp::Serve {
            bundle_dir,
            bind_addr,
            max_requests,
            swarm_relay,
        } => {
            let root = bundle_dir
                .canonicalize()
                .map_err(|e| make_err(format!("resolve gateway bundle dir: {e}")))?;
            let listener = TcpListener::bind(bind_addr)
                .map_err(|e| make_err(format!("bind Stitch gateway on {bind_addr}: {e}")))?;
            let bound_port = listener
                .local_addr()
                .map_err(|e| make_err(format!("read bound port: {e}")))?
                .port();

            let read_timeout_secs = u64::from(two_u32());
            let buf_len = 1usize << (eight_u32() + four_u32());
            let max_age = gateway_immutable_cache_max_age_secs();

            let mut requests_served = 0usize;
            let mut shard_requests = 0usize;
            let mut part_redirects = 0usize;
            let mut healthz_checks = 0usize;
            let mut manifest_requests = 0usize;
            let mut swarm_requests = 0usize;
            let mut not_found_count = 0usize;

            for stream_res in listener.incoming() {
                if let Some(limit) = max_requests {
                    if requests_served >= *limit {
                        break;
                    }
                }
                let Ok(mut stream) = stream_res else {
                    continue;
                };
                let _ = stream.set_read_timeout(Some(Duration::from_secs(read_timeout_secs)));
                let mut req_buf = vec![0u8; buf_len];
                let n = stream.read(&mut req_buf).unwrap_or(0);
                if n == 0 {
                    continue;
                }
                requests_served = requests_served.saturating_add(1);
                let req_str = String::from_utf8_lossy(&req_buf[..n]);
                let first_line = req_str.lines().next().unwrap_or("");
                let mut parts = first_line.split_whitespace();
                let method = parts.next().unwrap_or("GET");
                let raw_target = parts.next().unwrap_or("/");
                let url_path = raw_target.split('?').next().unwrap_or("/");

                let cors_hdr = "Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, HEAD, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nCross-Origin-Resource-Policy: cross-origin\r\nX-Verdant-Gateway: stitch-v1\r\n";

                if method.eq_ignore_ascii_case("OPTIONS") {
                    let resp = format!(
                        "HTTP/1.1 204 No Content\r\n{cors_hdr}Content-Length: 0\r\nConnection: close\r\n\r\n"
                    );
                    let _ = stream.write_all(resp.as_bytes());
                    if let Some(limit) = max_requests {
                        if requests_served >= *limit {
                            break;
                        }
                    }
                    continue;
                }

                let rel = url_path.trim_start_matches('/');
                if rel.contains("..") || rel.contains('\\') {
                    let resp = format!(
                        "HTTP/1.1 400 Bad Request\r\n{cors_hdr}Content-Type: text/plain; charset=utf-8\r\nContent-Length: 8\r\nConnection: close\r\n\r\nbad path"
                    );
                    let _ = stream.write_all(resp.as_bytes());
                    continue;
                }

                if rel == "healthz" {
                    healthz_checks = healthz_checks.saturating_add(1);
                    let manifest_opt: Option<MultiGatewayManifest> =
                        std::fs::read_to_string(root.join("gateway-manifest.json"))
                            .ok()
                            .and_then(|s| serde_json::from_str(&s).ok());
                    let mut avail_shards = 0u32;
                    if let Some(ref m) = manifest_opt {
                        for sf in &m.shard_files {
                            if root.join("shards").join(sf).is_file() {
                                avail_shards = avail_shards.saturating_add(1);
                            }
                        }
                    }
                    let (status_line, body_json) = match manifest_opt {
                        Some(m) if avail_shards >= u32::from(m.data_shards_k) => (
                            "HTTP/1.1 200 OK",
                            serde_json::json!({
                                "status": "ok",
                                "provider": "self-hosted-stitch-node",
                                "releaseVersion": m.release_version,
                                "releaseEpoch": m.release_epoch,
                                "releaseKappaHex": m.release_kappa_hex,
                                "availableShards": avail_shards,
                                "requiredShardsK": m.data_shards_k,
                                "totalShardsN": m.total_shards_n,
                                "stakeholderParts": m.stakeholder_parts.iter().map(|p| p.part_id.clone()).collect::<Vec<_>>(),
                            }),
                        ),
                        _ => (
                            "HTTP/1.1 503 Service Unavailable",
                            serde_json::json!({
                                "status": "degraded",
                                "availableShards": avail_shards,
                            }),
                        ),
                    };
                    let body = body_json.to_string().into_bytes();
                    let hdr = format!(
                        "{status_line}\r\n{cors_hdr}Content-Type: application/json; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(hdr.as_bytes());
                    let _ = stream.write_all(&body);
                    if let Some(limit) = max_requests {
                        if requests_served >= *limit {
                            break;
                        }
                    }
                    continue;
                }

                if rel == "swarm" {
                    swarm_requests = swarm_requests.saturating_add(1);
                    let manifest_opt: Option<MultiGatewayManifest> =
                        std::fs::read_to_string(root.join("gateway-manifest.json"))
                            .ok()
                            .and_then(|s| serde_json::from_str(&s).ok());
                    let mut avail_idx: Vec<usize> = Vec::new();
                    if let Some(ref m) = manifest_opt {
                        for (idx, sf) in m.shard_files.iter().enumerate() {
                            if root.join("shards").join(sf).is_file() {
                                avail_idx.push(idx);
                            }
                        }
                    }
                    let (status_line, body_json) = match manifest_opt {
                        Some(m) => {
                            let kp = &m.release_kappa_hex[..m
                                .release_kappa_hex
                                .len()
                                .min((eight_u32() + eight_u32()) as usize)];
                            let topic = format!("verdant-stitch-{kp}");
                            let relay_url = swarm_relay.as_deref().unwrap_or("https://ntfy.sh");
                            let ann = serde_json::json!({
                                "v": "stitch/1",
                                "p": "signal",
                                "c": m.release_kappa_hex,
                                "i": 0,
                                "n": 1,
                                "topic": topic,
                                "availableShardIndices": avail_idx,
                            });
                            (
                                "HTTP/1.1 200 OK",
                                serde_json::json!({
                                    "spec": "verdant/stitch-volunteer-swarm/1",
                                    "protocol": "stitch/1",
                                    "zeroCommercialRegistration": true,
                                    "releaseVersion": m.release_version,
                                    "releaseEpoch": m.release_epoch,
                                    "releaseKappaHex": m.release_kappa_hex,
                                    "opaqueTopic": topic,
                                    "swarmRelayUrl": relay_url,
                                    "wireBytesMax": stitch_l5_wire_bytes_max(),
                                    "chunkBytesMax": stitch_l5_chunk_bytes_max(),
                                    "resourceAdmissionBudget": {
                                        "epochRequestCap": stitch_l5_epoch_request_cap(),
                                        "transferBudgetBytes": stitch_l5_transfer_budget_bytes(),
                                        "storageBudgetBytes": stitch_l5_transfer_budget_bytes(),
                                    },
                                    "availableShardIndices": avail_idx,
                                    "announceFrame": ann,
                                }),
                            )
                        }
                        None => (
                            "HTTP/1.1 503 Service Unavailable",
                            serde_json::json!({"status": "missing-manifest"}),
                        ),
                    };
                    let body = body_json.to_string().into_bytes();
                    let hdr = format!(
                        "{status_line}\r\n{cors_hdr}Content-Type: application/json; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(hdr.as_bytes());
                    let _ = stream.write_all(&body);
                    if max_requests.is_some_and(|l| requests_served >= l) {
                        break;
                    }
                    continue;
                }

                if rel == "portable" {
                    let portable_file = std::fs::read_to_string(root.join("gateway-manifest.json"))
                        .ok()
                        .and_then(|s| serde_json::from_str::<MultiGatewayManifest>(&s).ok())
                        .map(|m| format!("verdant-portable-v{}.html", m.release_version));
                    if let Some(pf) = portable_file {
                        if let Ok(body) = std::fs::read(root.join(&pf)) {
                            let hdr = format!(
                                "HTTP/1.1 200 OK\r\n{cors_hdr}Content-Type: text/html; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                                body.len()
                            );
                            let _ = stream.write_all(hdr.as_bytes());
                            if !method.eq_ignore_ascii_case("HEAD") {
                                let _ = stream.write_all(&body);
                            }
                            if max_requests.is_some_and(|l| requests_served >= l) {
                                break;
                            }
                            continue;
                        }
                    }
                }

                if let Some(part_raw) = rel.strip_prefix("part/") {
                    let slug = part_raw.trim_end_matches('/');
                    if matches!(
                        slug,
                        "studio" | "catalogs" | "demo" | "docs" | "starter" | "card"
                    ) {
                        part_redirects = part_redirects.saturating_add(1);
                        let loc = format!("/?pin=stitch://verdant/{slug}");
                        let resp = format!(
                            "HTTP/1.1 302 Found\r\n{cors_hdr}Location: {loc}\r\nCache-Control: no-store\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        );
                        let _ = stream.write_all(resp.as_bytes());
                        if let Some(limit) = max_requests {
                            if requests_served >= *limit {
                                break;
                            }
                        }
                        continue;
                    }
                }

                let mapped_rel = if rel.is_empty() || rel == "frame" || rel == "frame/" {
                    "index.html"
                } else if let Some(after_frame) = rel.strip_prefix("frame/") {
                    after_frame
                } else {
                    rel
                };
                let full_path = root.join(mapped_rel);
                match std::fs::read(&full_path) {
                    Ok(body) => {
                        if mapped_rel.ends_with(".stch") {
                            shard_requests = shard_requests.saturating_add(1);
                        } else if mapped_rel == "gateway-manifest.json" {
                            manifest_requests = manifest_requests.saturating_add(1);
                        }
                        let content_type = if mapped_rel.ends_with(".html") {
                            "text/html; charset=utf-8"
                        } else if mapped_rel.ends_with(".mjs") || mapped_rel.ends_with(".js") {
                            "text/javascript; charset=utf-8"
                        } else if mapped_rel.ends_with(".json") {
                            "application/json; charset=utf-8"
                        } else if mapped_rel.ends_with(".crate") {
                            "application/gzip"
                        } else {
                            "application/octet-stream"
                        };
                        let cache_hdr = if mapped_rel.ends_with(".stch")
                            || mapped_rel.ends_with(".holo")
                            || mapped_rel.ends_with(".crate")
                        {
                            format!("public, max-age={max_age}, immutable")
                        } else {
                            "no-store".to_string()
                        };
                        let hdr = format!(
                            "HTTP/1.1 200 OK\r\n{cors_hdr}Content-Type: {content_type}\r\nCache-Control: {cache_hdr}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            body.len()
                        );
                        let _ = stream.write_all(hdr.as_bytes());
                        if !method.eq_ignore_ascii_case("HEAD") {
                            let _ = stream.write_all(&body);
                        }
                    }
                    Err(_) => {
                        not_found_count = not_found_count.saturating_add(1);
                        let resp = format!(
                            "HTTP/1.1 404 Not Found\r\n{cors_hdr}Content-Type: text/plain; charset=utf-8\r\nContent-Length: 9\r\nConnection: close\r\n\r\nnot found"
                        );
                        let _ = stream.write_all(resp.as_bytes());
                    }
                }
                if let Some(limit) = max_requests {
                    if requests_served >= *limit {
                        break;
                    }
                }
            }

            let outcome = GatewayOpOutcome::Served(GatewayServeReport {
                bind_addr: bind_addr.clone(),
                bound_port,
                requests_served,
                shard_requests,
                part_redirects,
                healthz_checks,
                manifest_requests,
                swarm_requests,
                not_found_count,
            });
            let pretty = serde_json::to_string_pretty(&outcome)
                .map_err(|e| make_err(format!("serialize serve report: {e}")))?;
            Ok((outcome, pretty))
        }

        GatewayOp::VerifyFailover { bundle_dir } => {
            let root = bundle_dir
                .canonicalize()
                .map_err(|e| make_err(format!("resolve bundle dir for verify: {e}")))?;
            for forbidden in ["wrangler.toml", "netlify.toml", "vercel.json"] {
                if root.join(forbidden).exists() {
                    return Err(make_err(format!(
                        "public-benefit Stitch bundle refuses commercial cloud provider manifest `{forbidden}`"
                    )));
                }
            }
            let required_adapters = [
                "_headers",
                "_redirects",
                "stitch-swarm-config.json",
                ".ipfs-gateway-root.json",
                "Dockerfile",
            ];
            let mut adapters_verified = Vec::new();
            for adapter in required_adapters {
                if !root.join(adapter).is_file() {
                    return Err(make_err(format!(
                        "multi-gateway bundle missing required adapter `{adapter}`"
                    )));
                }
                adapters_verified.push(adapter.to_string());
            }

            let pin_raw = std::fs::read_to_string(root.join("frame-pin.json"))
                .map_err(|e| make_err(format!("read bundle frame-pin.json: {e}")))?;
            let pin: FramePinManifest = serde_json::from_str(&pin_raw)
                .map_err(|e| make_err(format!("parse bundle frame-pin.json: {e}")))?;
            let manifest_raw = std::fs::read_to_string(root.join("gateway-manifest.json"))
                .map_err(|e| make_err(format!("read bundle gateway-manifest.json: {e}")))?;
            let manifest: MultiGatewayManifest = serde_json::from_str(&manifest_raw)
                .map_err(|e| make_err(format!("parse bundle gateway-manifest.json: {e}")))?;
            if !gateway_provider_families_independent(
                manifest.distinct_provider_families.len() as u32
            ) {
                return Err(make_err(
                    "gateway-manifest.json failed >= 2 independent provider families check"
                        .to_string(),
                ));
            }
            for part in &manifest.stakeholder_parts {
                if !root
                    .join("part")
                    .join(&part.part_id)
                    .join("index.html")
                    .is_file()
                {
                    return Err(make_err(format!(
                        "bundle missing static stakeholder part redirect `part/{}/index.html`",
                        part.part_id
                    )));
                }
            }

            // Prepare a shadow secondary gateway directory with 4 of 8 shards DELIBERATELY WITHHELD:
            static VERIFY_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
            let verify_seq = VERIFY_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let shadow_dir = std::env::temp_dir().join(format!(
                "verdant-gw-verify-{}-{verify_seq}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&shadow_dir);
            std::fs::create_dir_all(shadow_dir.join("shards"))
                .map_err(|e| make_err(format!("mkdir shadow gateway shards: {e}")))?;
            let portable_name = format!("verdant-portable-v{}.html", manifest.release_version);
            let portable_txt = std::fs::read_to_string(root.join(&portable_name)).map_err(|e| {
                make_err(format!(
                    "read offline portable envelope {portable_name}: {e}"
                ))
            })?;
            if !portable_txt.contains(&manifest.release_kappa_hex)
                || !portable_txt.contains("DecompressionStream")
                || !portable_txt.contains("SHA-256")
            {
                return Err(make_err(format!(
                    "{portable_name} missing kappa lock or offline SHA-256/gzip verifier"
                )));
            }
            for f in [
                "index.html",
                "frame-boot.mjs",
                "frame-sw.js",
                "frame-pin.json",
                "gateway-manifest.json",
                portable_name.as_str(),
            ] {
                std::fs::copy(root.join(f), shadow_dir.join(f))
                    .map_err(|e| make_err(format!("copy shadow {f}: {e}")))?;
            }
            let stride = two_u32() as usize;
            for (idx, fname) in pin.shard_files.iter().enumerate() {
                if idx % stride == 1 {
                    std::fs::copy(
                        root.join("shards").join(fname),
                        shadow_dir.join("shards").join(fname),
                    )
                    .map_err(|e| make_err(format!("copy shadow shard {fname}: {e}")))?;
                }
            }

            // Gateway A (simulated GitHub primary outage -> HTTP 503):
            let primary_listener = TcpListener::bind("127.0.0.1:0")
                .map_err(|e| make_err(format!("bind simulated primary outage server: {e}")))?;
            let primary_port = primary_listener
                .local_addr()
                .map_err(|e| make_err(format!("read primary port: {e}")))?
                .port();
            let primary_stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let primary_stop_srv = std::sync::Arc::clone(&primary_stop);
            let primary_reqs = pin.shard_files.len();
            let primary_thread = std::thread::spawn(move || {
                let mut handled = 0usize;
                for stream_res in primary_listener.incoming() {
                    if primary_stop_srv.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                    let Ok(mut sock) = stream_res else {
                        continue;
                    };
                    let mut buf = [0u8; 1 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)];
                    let n = sock.read(&mut buf).unwrap_or(0);
                    if n == 0 {
                        continue;
                    }
                    let _ = sock.write_all(
                        b"HTTP/1.1 503 Service Unavailable\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 14\r\nConnection: close\r\n\r\nprimary outage",
                    );
                    handled = handled.saturating_add(1);
                    if handled >= primary_reqs {
                        break;
                    }
                }
            });

            // Gateway B (real Verdant native Stitch Gateway on loopback):
            let sec_probe_listener = TcpListener::bind("127.0.0.1:0")
                .map_err(|e| make_err(format!("allocate secondary gateway port: {e}")))?;
            let sec_port = sec_probe_listener
                .local_addr()
                .map_err(|e| make_err(format!("read secondary port: {e}")))?
                .port();
            drop(sec_probe_listener);

            let sec_expected_reqs = pin.shard_files.len() + ((four_u32() + two_u32()) as usize);
            let sec_dir = shadow_dir.clone();
            let sec_addr = format!("127.0.0.1:{sec_port}");
            let sec_thread = std::thread::spawn(move || {
                execute_gateway_op(&GatewayOp::Serve {
                    bundle_dir: sec_dir,
                    bind_addr: sec_addr,
                    max_requests: Some(sec_expected_reqs),
                    swarm_relay: Some("https://ntfy.sh".to_string()),
                })
            });

            // Wait briefly for secondary listener to bind:
            let retry_cap = eight_u32() + eight_u32();
            let mut connected = false;
            for _ in 0..retry_cap {
                if TcpStream::connect(format!("127.0.0.1:{sec_port}")).is_ok() {
                    // Opening and closing sends 0 bytes, which `Serve` skips cleanly without counting!
                    connected = true;
                    break;
                }
                std::thread::sleep(Duration::from_millis(u64::from(eight_u32() + eight_u32())));
            }
            if !connected {
                let _ = std::fs::remove_dir_all(&shadow_dir);
                return Err(make_err(
                    "secondary Stitch gateway did not bind in time".to_string(),
                ));
            }

            let http_req = |port: u16,
                            method: &str,
                            path: &str|
             -> Result<(u16, String, Vec<u8>), ObserverModelError> {
                let mut sock = TcpStream::connect(format!("127.0.0.1:{port}"))
                    .map_err(|e| make_err(format!("connect 127.0.0.1:{port}{path}: {e}")))?;
                let req = format!(
                    "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
                );
                sock.write_all(req.as_bytes())
                    .map_err(|e| make_err(format!("write HTTP request {path}: {e}")))?;
                let mut raw = Vec::new();
                sock.read_to_end(&mut raw)
                    .map_err(|e| make_err(format!("read HTTP response {path}: {e}")))?;
                let sep = b"\r\n\r\n";
                let split_at = raw
                    .windows(sep.len())
                    .position(|w| w == sep)
                    .ok_or_else(|| make_err(format!("malformed HTTP response for {path}")))?;
                let hdr = String::from_utf8_lossy(&raw[..split_at]).to_string();
                let body = raw[split_at + sep.len()..].to_vec();
                let code = hdr
                    .lines()
                    .next()
                    .and_then(|l| l.split_whitespace().nth(1))
                    .and_then(|s| s.parse::<u16>().ok())
                    .unwrap_or(0);
                Ok((code, hdr, body))
            };

            let verify_res = (|| -> Result<GatewayFailoverReport, ObserverModelError> {
                let (health_code, health_hdr, health_body) = http_req(sec_port, "GET", "/healthz")?;
                if !gateway_http_status_ok(health_code)
                    || !health_hdr
                        .to_ascii_lowercase()
                        .contains("access-control-allow-origin: *")
                {
                    return Err(make_err(format!(
                        "secondary gateway /healthz failed: HTTP {health_code}"
                    )));
                }
                let health_json: serde_json::Value = serde_json::from_slice(&health_body)
                    .map_err(|e| make_err(format!("parse /healthz JSON: {e}")))?;
                if health_json.get("status").and_then(|v| v.as_str()) != Some("ok") {
                    return Err(make_err(format!(
                        "secondary gateway /healthz not ok: {health_json}"
                    )));
                }

                let (man_code, _, man_body) = http_req(sec_port, "GET", "/gateway-manifest.json")?;
                if !gateway_http_status_ok(man_code) || man_body.is_empty() {
                    return Err(make_err(format!(
                        "secondary gateway /gateway-manifest.json failed: HTTP {man_code}"
                    )));
                }

                let (part_code, part_hdr, _) = http_req(sec_port, "GET", "/part/studio")?;
                if !gateway_http_status_redirect(part_code)
                    || !part_hdr.contains("Location: /?pin=stitch://verdant/studio")
                {
                    return Err(make_err(format!(
                        "secondary gateway /part/studio redirect failed: HTTP {part_code}"
                    )));
                }

                let (opt_code, opt_hdr, _) = http_req(sec_port, "OPTIONS", "/shards/shard-1.stch")?;
                if !gateway_http_status_no_content(opt_code)
                    || !opt_hdr
                        .to_ascii_lowercase()
                        .contains("access-control-allow-origin: *")
                {
                    return Err(make_err(format!(
                        "secondary gateway CORS preflight failed: HTTP {opt_code}"
                    )));
                }

                let (swarm_code, swarm_hdr, swarm_body) = http_req(sec_port, "GET", "/swarm")?;
                if !gateway_http_status_ok(swarm_code)
                    || !swarm_hdr
                        .to_ascii_lowercase()
                        .contains("access-control-allow-origin: *")
                {
                    return Err(make_err(format!(
                        "secondary volunteer gateway /swarm failed: HTTP {swarm_code}"
                    )));
                }
                let swarm_json: serde_json::Value = serde_json::from_slice(&swarm_body)
                    .map_err(|e| make_err(format!("parse /swarm JSON: {e}")))?;
                if swarm_json.get("protocol").and_then(|v| v.as_str()) != Some("stitch/1")
                    || swarm_json
                        .get("zeroCommercialRegistration")
                        .and_then(serde_json::Value::as_bool)
                        != Some(true)
                {
                    return Err(make_err(format!(
                        "secondary volunteer gateway /swarm contract mismatch: {swarm_json}"
                    )));
                }

                let (port_code, _, port_body) = http_req(sec_port, "GET", "/portable")?;
                let port_str = String::from_utf8_lossy(&port_body);
                if !gateway_http_status_ok(port_code)
                    || !port_str.contains(&manifest.release_kappa_hex)
                {
                    return Err(make_err(format!(
                        "secondary gateway /portable offline HTML failed: HTTP {port_code}"
                    )));
                }

                let mut primary_outage_status = 0u16;
                let mut surviving = Vec::new();
                let mut withheld_shards_tolerated = 0u32;
                for fname in &pin.shard_files {
                    let shard_url = format!("/shards/{fname}");
                    let (p_code, _, _) = http_req(primary_port, "GET", &shard_url)?;
                    if !gateway_http_status_unavailable(p_code) {
                        return Err(make_err(format!(
                            "expected primary simulated outage 503, got {p_code}"
                        )));
                    }
                    primary_outage_status = p_code;

                    // Failover to independent secondary Stitch gateway:
                    let (s_code, s_hdr, s_body) = http_req(sec_port, "GET", &shard_url)?;
                    if gateway_http_status_ok(s_code) {
                        if !s_hdr
                            .to_ascii_lowercase()
                            .contains("access-control-allow-origin: *")
                        {
                            return Err(make_err(format!(
                                "secondary shard {fname} missing CORS wildcard header"
                            )));
                        }
                        let parsed = crate::network::StitchRsShard::from_binary_stch2(&s_body)
                            .map_err(|e| {
                                make_err(format!("verify secondary shard {fname}: {e}"))
                            })?;
                        surviving.push(parsed);
                    } else {
                        withheld_shards_tolerated = withheld_shards_tolerated.saturating_add(1);
                    }
                }

                let mut trusted = BTreeMap::new();
                trusted.insert(
                    "prism:verdant-observer-executive".to_string(),
                    b"verdant-observer-executive-quorum-key".to_vec(),
                );
                trusted.insert(
                    "prism:verdant-release-bot".to_string(),
                    b"verdant-release-bot-quorum-key".to_vec(),
                );
                let recovered = crate::vcs::VerdantReleaseCapsule::recover_from_stitch(
                    &surviving,
                    &pin,
                    0,
                    Some(pin.prev_kappa_hex.as_str()),
                    &trusted,
                )
                .map_err(|e| make_err(format!("failover 4-of-8 capsule recovery failed: {e}")))?;

                Ok(GatewayFailoverReport {
                    spec: "verdant/gateway-failover-report/1".to_string(),
                    release_version: recovered.manifest.version,
                    release_epoch: recovered.manifest.release_epoch,
                    release_kappa_hex: recovered.capsule_sha256_hex,
                    primary_outage_status,
                    secondary_healthz_status: health_code,
                    part_redirect_status: part_code,
                    cors_preflight_status: opt_code,
                    swarm_endpoint_status: swarm_code,
                    portable_html_verified: true,
                    surviving_shards_fetched: surviving.len() as u32,
                    withheld_shards_tolerated,
                    recovered_files_count: recovered.manifest.files.len(),
                    adapters_verified,
                    distinct_provider_families: manifest.distinct_provider_families,
                    all_passed: true,
                })
            })();

            primary_stop.store(true, std::sync::atomic::Ordering::SeqCst);
            let _ = TcpStream::connect(format!("127.0.0.1:{primary_port}"));
            let _ = primary_thread.join();
            if verify_res.is_ok() {
                let _ = sec_thread.join();
            } else {
                for _ in 0..sec_expected_reqs {
                    if let Ok(mut s) = TcpStream::connect(format!("127.0.0.1:{sec_port}")) {
                        let _ = s.write_all(
                            b"OPTIONS / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
                        );
                    } else {
                        break;
                    }
                }
                let _ = sec_thread.join();
            }
            let _ = std::fs::remove_dir_all(&shadow_dir);
            let outcome = GatewayOpOutcome::Verified(verify_res?);
            let pretty = serde_json::to_string_pretty(&outcome)
                .map_err(|e| make_err(format!("serialize failover report: {e}")))?;
            Ok((outcome, pretty))
        }
    }
}
