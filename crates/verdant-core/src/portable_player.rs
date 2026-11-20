//! Self-contained interactive WebGL2 3D Player embedded into zero-account portable `.holo`
//! user applications (`VE-57`, refined in `VE-58`).
//!
//! Renders all 5 Verdant Studio primitive shapes faithfully (`0=Box`, `1=Platform`, `2=Pillar`,
//! `3=Ramp`, `4=Beacon Orb`) with domain sky/horizon palettes, ground grid lines, per-object
//! PBR/emissive lighting, drag/touch orbit, wheel zoom, and WASD/Arrow camera panning — 100%
//! offline with zero accounts and zero external CDN fetches.

/// Escapes untrusted text for safe inclusion inside HTML text nodes and JSON script blocks.
#[must_use]
pub fn escape_html_text(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Synthesizes a compact (~6.5 KiB), zero-dependency interactive WebGL2 3D HTML application for
/// a portable `.holo` capsule committed via `VerdantVcs` (`VE-57`, `VE-58`).
#[must_use]
pub fn render_portable_3d_player_html(app_title: &str, author: &str, scene_json: &str) -> String {
    let safe_title = escape_html_text(app_title);
    let safe_author = escape_html_text(author);
    let safe_json_text = escape_html_text(scene_json);
    let safe_json_script = scene_json.replace("</", "<\\/");
    format!(
        r##"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{safe_title}</title><style>*{{box-sizing:border-box}}html,body{{margin:0;height:100%;background:#0e1318;color:#ecf3f8;font-family:system-ui,sans-serif;overflow:hidden}}#hud{{position:fixed;top:.75rem;left:.75rem;right:.75rem;display:flex;flex-wrap:wrap;justify-content:space-between;align-items:center;gap:.5rem;padding:.55rem .85rem;border-radius:.6rem;background:rgba(14,19,24,.84);border:1px solid rgba(132,174,196,.28);z-index:2;font-size:.84rem}}#hud h1{{margin:0;font-size:.98rem}}#hud .meta{{color:#9fb6c5;font-family:ui-monospace,monospace;font-size:.76rem}}#hud button{{padding:.35rem .65rem;border-radius:.4rem;border:1px solid rgba(132,174,196,.4);background:#1b2833;color:#ecf3f8;cursor:pointer;font-size:.78rem}}#verdant-shared-3d-canvas{{display:block;width:100vw;height:100vh;touch-action:none}}#verdant-scene-data{{position:fixed;bottom:.75rem;left:.75rem;right:.75rem;max-height:36vh;overflow:auto;margin:0;padding:.75rem;border-radius:.5rem;background:rgba(10,14,18,.94);border:1px solid rgba(132,174,196,.35);font-size:.74rem;z-index:3}}</style></head><body><header id="hud"><div><h1>{safe_title}</h1><div class="meta" id="scene-meta">Committed by {safe_author} · VerdantVcs portable .holo</div></div><div><button id="btn-orbit" type="button">Pause Orbit</button> <button id="btn-reset" type="button">Reset View</button> <button id="btn-json" type="button">Scene JSON</button></div></header><canvas id="verdant-shared-3d-canvas" aria-label="{safe_title} interactive 3D viewport"></canvas><pre id="verdant-scene-data" hidden>{safe_json_text}</pre><script type="application/json" id="verdant-scene-payload">{safe_json_script}</script><script>
(()=>{{
const raw=document.getElementById("verdant-scene-payload")?.textContent||"{{}}";
let scene={{}};try{{scene=JSON.parse(raw);}}catch{{}}
const objs=Array.isArray(scene.objects)?scene.objects:[];
const dom=String(scene.domain||"sunspire-regatta");
const palettes={{"sunspire-regatta":[.14,.26,.38,.24,.30,.28],"meridian-observatory":[.08,.10,.22,.16,.18,.28],"glasshouse-chronicle":[.15,.28,.22,.22,.32,.24],"starfall-bastion":[.22,.12,.15,.20,.16,.18]}};
const pal=palettes[dom]||palettes["sunspire-regatta"];
const c=document.getElementById("verdant-shared-3d-canvas"),meta=document.getElementById("scene-meta"),pre=document.getElementById("verdant-scene-data");
document.getElementById("btn-json").onclick=()=>{{pre.hidden=!pre.hidden;}};
let autoOrbit=true,yaw=.65,pit=.38,dist=12,tx=0,tz=0,drag=false,lx=0,ly=0,frames=0;
const keys=new Set();
document.getElementById("btn-orbit").onclick=(e)=>{{autoOrbit=!autoOrbit;e.target.textContent=autoOrbit?"Pause Orbit":"Auto Orbit";}};
document.getElementById("btn-reset").onclick=()=>{{yaw=.65;pit=.38;dist=12;tx=0;tz=0;}};
window.addEventListener("keydown",(e)=>{{keys.add(e.code);}});
window.addEventListener("keyup",(e)=>{{keys.delete(e.code);}});
c.onpointerdown=(e)=>{{drag=true;lx=e.clientX;ly=e.clientY;c.setPointerCapture?.(e.pointerId);}};
c.onpointermove=(e)=>{{if(!drag)return;yaw-=(e.clientX-lx)*.008;pit=Math.max(.08,Math.min(1.4,pit+(e.clientY-ly)*.006));lx=e.clientX;ly=e.clientY;}};
c.onpointerup=c.onpointercancel=()=>{{drag=false;}};
c.addEventListener("wheel",(e)=>{{e.preventDefault();dist=Math.max(4,Math.min(44,dist+e.deltaY*.015));}},{{passive:false}});
const gl=c.getContext("webgl2",{{antialias:true}})||c.getContext("webgl",{{antialias:true}});
const state={{ready:false,renderedFrames:0,objectCount:objs.length,shapeKinds:5,title:String(scene.project||document.title),domain:dom,authorPrismId:"{safe_author}",backend:gl?"webgl":"2d"}};
window.__VERDANT_SHARED_APP__=state;
if(!gl){{const ctx=c.getContext("2d");const draw2d=()=>{{c.width=innerWidth;c.height=innerHeight;ctx.fillStyle="#16242f";ctx.fillRect(0,0,c.width,c.height);ctx.fillStyle="#6fe3b0";ctx.font="16px monospace";ctx.fillText(state.title+" ("+objs.length+" 3D objects)",24,100);frames++;state.renderedFrames=frames;state.ready=true;}};draw2d();return;}}
const sh=(t,s)=>{{const o=gl.createShader(t);gl.shaderSource(o,s);gl.compileShader(o);return o;}};
const pr=gl.createProgram();
gl.attachShader(pr,sh(gl.VERTEX_SHADER,"attribute vec3 p,n;uniform mat4 mvp,m;varying vec3 vn,wp;void main(){{vec4 w=m*vec4(p,1.);wp=w.xyz;vn=mat3(m)*n;gl_Position=mvp*vec4(p,1.);}}"));
gl.attachShader(pr,sh(gl.FRAGMENT_SHADER,"precision mediump float;varying vec3 vn,wp;uniform vec3 col,sky;uniform float em,met,grd;void main(){{vec3 N=normalize(vn),L=normalize(vec3(.55,.82,.42)),V=normalize(vec3(0.,8.,12.)-wp),H=normalize(L+V);float d=max(dot(N,L),0.),sp=pow(max(dot(N,H),0.),mix(16.,64.,met))*mix(.18,.85,met);float hemi=.32+.28*N.y;vec3 base=col;if(grd>.5){{vec2 g=abs(fract(wp.xz*.5-.5)-.5);float ln=1.-smoothstep(.0,.045,min(g.x,g.y));base=mix(base,base*1.38+vec3(.05),ln*.42);}}vec3 c=base*(hemi+.78*d)+vec3(sp)+base*em;float fog=clamp(length(wp.xz)/75.,0.,.55);gl_FragColor=vec4(mix(c,sky,fog),1.);}}"));
gl.linkProgram(pr);gl.useProgram(pr);
const mk=(t,d)=>{{const b=gl.createBuffer();gl.bindBuffer(t,b);gl.bufferData(t,d,gl.STATIC_DRAW);return b;}};
const packMesh=(v,n,ix)=>({{bp:mk(gl.ARRAY_BUFFER,new Float32Array(v)),bn:mk(gl.ARRAY_BUFFER,new Float32Array(n)),bi:mk(gl.ELEMENT_ARRAY_BUFFER,new Uint16Array(ix)),cnt:ix.length}});
const mkBox=()=>packMesh([-1,-1,1,1,-1,1,1,1,1,-1,1,1,1,-1,-1,-1,-1,-1,-1,1,-1,1,1,-1,-1,1,1,1,1,1,1,1,-1,-1,1,-1,-1,-1,-1,1,-1,-1,1,-1,1,-1,-1,1,1,-1,1,1,-1,-1,1,1,-1,1,1,1,-1,-1,-1,-1,-1,1,-1,1,1,-1,1,-1],[0,0,1,0,0,1,0,0,1,0,0,1,0,0,-1,0,0,-1,0,0,-1,0,0,-1,0,1,0,0,1,0,0,1,0,0,1,0,0,-1,0,0,-1,0,0,-1,0,0,-1,0,1,0,0,1,0,0,1,0,0,1,0,0,-1,0,0,-1,0,0,-1,0,0,-1,0,0],[0,1,2,0,2,3,4,5,6,4,6,7,8,9,10,8,10,11,12,13,14,12,14,15,16,17,18,16,18,19,20,21,22,20,22,23]);
const mkCyl=(seg=12)=>{{const v=[],n=[],ix=[];for(let i=0;i<=seg;i++){{const a=i/seg*Math.PI*2,cx=Math.cos(a),cz=Math.sin(a),r=1-.06*Math.abs(Math.sin(a*6));v.push(cx*r,-1,cz*r,cx*r,1,cz*r);n.push(cx,0,cz,cx,0,cz);}}for(let i=0;i<seg;i++){{const k=i*2;ix.push(k,k+2,k+3,k,k+3,k+1);}}const tc=v.length/3;v.push(0,1,0);n.push(0,1,0);for(let i=0;i<=seg;i++){{const a=i/seg*Math.PI*2;v.push(Math.cos(a),1,Math.sin(a));n.push(0,1,0);}}for(let i=0;i<seg;i++)ix.push(tc,tc+1+i+1,tc+1+i);return packMesh(v,n,ix);}};
const mkRamp=()=>{{const sl=[0,.7071,-.7071];return packMesh([-1,-1,1,1,-1,1,1,1,1,-1,1,1,-1,-1,-1,1,-1,-1,1,1,1,-1,1,1,-1,-1,-1,1,-1,-1,1,-1,1,-1,-1,1,-1,-1,-1,-1,-1,1,-1,1,1,1,-1,1,1,-1,-1,1,1,1],[0,0,1,0,0,1,0,0,1,0,0,1,sl[0],sl[1],sl[2],sl[0],sl[1],sl[2],sl[0],sl[1],sl[2],sl[0],sl[1],sl[2],0,-1,0,0,-1,0,0,-1,0,0,-1,0,-1,0,0,-1,0,0,-1,0,0,1,0,0,1,0,0,1,0,0],[0,1,2,0,2,3,4,5,6,4,6,7,8,9,10,8,10,11,12,13,14,15,16,17]);}};
const mkOrb=(lat=8,lon=12)=>{{const v=[],n=[],ix=[];for(let y=0;y<=lat;y++){{const ph=y/lat*Math.PI,sy=Math.sin(ph),cy=Math.cos(ph);for(let x=0;x<=lon;x++){{const th=x/lon*Math.PI*2,px=sy*Math.cos(th),pz=sy*Math.sin(th);v.push(px,cy,pz);n.push(px,cy,pz);}}}}for(let y=0;y<lat;y++)for(let x=0;x<lon;x++){{const a=y*(lon+1)+x,b=a+lon+1;ix.push(a,b,a+1,b,b+1,a+1);}}return packMesh(v,n,ix);}};
const boxM=mkBox(),meshes=[boxM,boxM,mkCyl(12),mkRamp(),mkOrb(8,12)];
const ap=gl.getAttribLocation(pr,"p"),an=gl.getAttribLocation(pr,"n"),uMvp=gl.getUniformLocation(pr,"mvp"),uM=gl.getUniformLocation(pr,"m"),uC=gl.getUniformLocation(pr,"col"),uS=gl.getUniformLocation(pr,"sky"),uE=gl.getUniformLocation(pr,"em"),uMt=gl.getUniformLocation(pr,"met"),uG=gl.getUniformLocation(pr,"grd");
const mul=(a,b)=>{{const o=new Float32Array(16);for(let r=0;r<4;r++)for(let q=0;q<4;q++)o[q*4+r]=a[r]*b[q*4]+a[4+r]*b[q*4+1]+a[8+r]*b[q*4+2]+a[12+r]*b[q*4+3];return o;}};
const persp=(f,a,n,fa)=>{{const t=1/Math.tan(f/2);return new Float32Array([t/a,0,0,0,0,t,0,0,0,0,(fa+n)/(n-fa),-1,0,0,(2*fa*n)/(n-fa),0]);}};
const look=(e,t)=>{{const sub=(a,b)=>[a[0]-b[0],a[1]-b[1],a[2]-b[2]],norm=v=>{{const l=Math.hypot(...v)||1;return v.map(x=>x/l);}},cr=(a,b)=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]],dot=(a,b)=>a[0]*b[0]+a[1]*b[1]+a[2]*b[2];const z=norm(sub(e,t)),x=norm(cr([0,1,0],z)),y=cr(z,x);return new Float32Array([x[0],y[0],z[0],0,x[1],y[1],z[1],0,x[2],y[2],z[2],0,-dot(x,e),-dot(y,e),-dot(z,e),1]);}};
const trs=(p,s,y)=>{{const cy=Math.cos(y),sy=Math.sin(y);return new Float32Array([cy*s[0],0,-sy*s[0],0,0,s[1],0,0,sy*s[2],0,cy*s[2],0,p[0],p[1],p[2],1]);}};
const drawMesh=(mesh,vp,pos,scl,y,col,em,met,grd)=>{{gl.bindBuffer(gl.ARRAY_BUFFER,mesh.bp);gl.enableVertexAttribArray(ap);gl.vertexAttribPointer(ap,3,gl.FLOAT,false,0,0);gl.bindBuffer(gl.ARRAY_BUFFER,mesh.bn);gl.enableVertexAttribArray(an);gl.vertexAttribPointer(an,3,gl.FLOAT,false,0,0);gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER,mesh.bi);const m=trs(pos,scl,y),mvp=mul(vp,m);gl.uniformMatrix4fv(uMvp,false,mvp);gl.uniformMatrix4fv(uM,false,m);gl.uniform3fv(uC,col);gl.uniform1f(uE,em);gl.uniform1f(uMt,met);gl.uniform1f(uG,grd);gl.drawElements(gl.TRIANGLES,mesh.cnt,gl.UNSIGNED_SHORT,0);}};
const render=()=>{{
const dpr=Math.min(devicePixelRatio||1,2),w=Math.max(1,Math.floor(innerWidth*dpr)),h=Math.max(1,Math.floor(innerHeight*dpr));
if(c.width!==w||c.height!==h){{c.width=w;c.height=h;}}
gl.viewport(0,0,w,h);gl.enable(gl.DEPTH_TEST);gl.clearColor(pal[0],pal[1],pal[2],1);gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT);
gl.uniform3f(uS,pal[0],pal[1],pal[2]);
const sp=.12,fx=-Math.sin(yaw),fz=-Math.cos(yaw),rx=Math.cos(yaw),rz=-Math.sin(yaw);
if(keys.has("KeyW")||keys.has("ArrowUp")){{tx+=fx*sp;tz+=fz*sp;}}if(keys.has("KeyS")||keys.has("ArrowDown")){{tx-=fx*sp;tz-=fz*sp;}}
if(keys.has("KeyD")||keys.has("ArrowRight")){{tx+=rx*sp;tz+=rz*sp;}}if(keys.has("KeyA")||keys.has("ArrowLeft")){{tx-=rx*sp;tz-=rz*sp;}}
if(autoOrbit&&!drag)yaw+=.0055;
const eye=[tx+Math.sin(yaw)*Math.cos(pit)*dist,Math.sin(pit)*dist+1.6,tz+Math.cos(yaw)*Math.cos(pit)*dist];
const vp=mul(persp(.85,w/h,.1,180),look(eye,[tx,.8,tz]));
drawMesh(boxM,vp,[0,-.22,0],[18,.22,18],0,[pal[3],pal[4],pal[5]],0,0,1);
const list=objs.length?objs:[{{kind:0,pos:[0,0,0],scale:[1.6,1.6,1.6],yawMilli:0,color:[.86,.38,.18]}}];
for(const o of list){{
const k=Math.max(0,Math.min(4,Number(o.kind??o.primKind??0)|0));
const p=o.pos||[0,0,0],s=o.scale||[1.4,1.4,1.4],y=(o.yawMilli||0)/1000,cl=o.color||[.82,.42,.22];
const mc=Math.floor(cl[0]||0),em=mc>=3?.58+.14*Math.sin(frames*.06):0,met=mc===2?.85:.12,rgb=[Math.max(.08,cl[0]-mc),cl[1]||.4,cl[2]||.25];
if(k===1){{drawMesh(boxM,vp,[p[0],p[1]+s[1]*.35,p[2]],[s[0]*.55,s[1]*.35,s[2]*.55],y,rgb,em,met,0);drawMesh(boxM,vp,[p[0],p[1]+s[1]*.82,p[2]],[s[0]*.46,s[1]*.18,s[2]*.46],y,rgb.map(v=>Math.min(1,v*1.08)),em,met,0);}}
else drawMesh(meshes[k],vp,[p[0],p[1]+s[1]*.5,p[2]],[s[0]*.5,s[1]*.5,s[2]*.5],y,rgb,em,met,0);
}}
frames++;state.renderedFrames=frames;state.ready=true;
if(meta&&frames===1)meta.textContent=`${{dom}} · ${{objs.length}} authored 3D objects (5 Studio shapes) · ${{state.backend.toUpperCase()}} · drag/WASD · {safe_author}`;
requestAnimationFrame(render);
}};
requestAnimationFrame(render);
}})();
</script></body></html>
"##
    )
}
