export function mat4Perspective(fovRad, aspect, near, far) {
  const f = 1 / Math.tan(fovRad * 0.5), nf = 1 / (near - far);
  return new Float32Array([f / aspect, 0, 0, 0, 0, f, 0, 0, 0, 0, (far + near) * nf, -1, 0, 0, 2 * far * near * nf, 0]);
}

export function mat4Ortho(l, r, b, t, n, f) {
  const rl = 1 / (r - l), tb = 1 / (t - b), fn = 1 / (f - n);
  return new Float32Array([2 * rl, 0, 0, 0, 0, 2 * tb, 0, 0, 0, 0, -2 * fn, 0, -(r + l) * rl, -(t + b) * tb, -(f + n) * fn, 1]);
}

export function mat4LookAt(eye, target, up) {
  let zx = eye[0] - target[0], zy = eye[1] - target[1], zz = eye[2] - target[2];
  const zl = Math.hypot(zx, zy, zz) || 1;
  zx /= zl; zy /= zl; zz /= zl;
  let xx = up[1] * zz - up[2] * zy, xy = up[2] * zx - up[0] * zz, xz = up[0] * zy - up[1] * zx;
  const xl = Math.hypot(xx, xy, xz) || 1;
  xx /= xl; xy /= xl; xz /= xl;
  const yx = zy * xz - zz * xy, yy = zz * xx - zx * xz, yz = zx * xy - zy * xx;
  return new Float32Array([
    xx, yx, zx, 0, xy, yy, zy, 0, xz, yz, zz, 0,
    -(xx * eye[0] + xy * eye[1] + xz * eye[2]), -(yx * eye[0] + yy * eye[1] + yz * eye[2]), -(zx * eye[0] + zy * eye[1] + zz * eye[2]), 1,
  ]);
}

export function mat4Mul(a, b) {
  const out = new Float32Array(16);
  for (let c = 0; c < 4; c++)
    for (let r = 0; r < 4; r++)
      out[c * 4 + r] = a[r] * b[c * 4] + a[4 + r] * b[c * 4 + 1] + a[8 + r] * b[c * 4 + 2] + a[12 + r] * b[c * 4 + 3];
  return out;
}

const SUN_DIR = (() => {
  const x = 0.54, y = 0.78, z = 0.31, l = Math.hypot(x, y, z);
  return new Float32Array([x / l, y / l, z / l]);
})();

export async function createGlRenderer(canvas) {
  const gl = canvas.getContext("webgl2", { antialias: false, depth: true, alpha: false, preserveDrawingBuffer: true, powerPreference: "high-performance" });
  if (!gl) return null;
  const dbg = gl.getExtension("WEBGL_debug_renderer_info");
  const isSoftware = /SwiftShader|llvmpipe|Software|Basic Render/i.test(String((dbg ? gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER)) || ""));
  const hasHalfFloat = Boolean(gl.getExtension("EXT_color_buffer_half_float"));
  const shadowSize = isSoftware ? 1024 : 2048;

  const compile = (t, src) => {
    const s = gl.createShader(t);
    gl.shaderSource(s, src); gl.compileShader(s);
    return gl.getShaderParameter(s, gl.COMPILE_STATUS) ? s : null;
  };
  const link = (vsSrc, fsSrc) => {
    const vs = compile(gl.VERTEX_SHADER, vsSrc), fs = compile(gl.FRAGMENT_SHADER, fsSrc);
    if (!vs || !fs) return null;
    const p = gl.createProgram();
    gl.attachShader(p, vs); gl.attachShader(p, fs); gl.linkProgram(p);
    return gl.getProgramParameter(p, gl.LINK_STATUS) ? p : null;
  };
  const setTexClamp = (filter) => {
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, filter);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, filter);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  };
  const uniforms = (prog, names) => Object.fromEntries(names.map((n) => [n, gl.getUniformLocation(prog, `u_${n}`)]));

  const instDecl = `layout(location=3) in vec4 i_c0;layout(location=4) in vec4 i_c1;layout(location=5) in vec4 i_c2;layout(location=6) in vec4 i_c3;uniform float u_inst;`;
  const instPos = `vec3 wp=u_inst>0.5?(i_c0.xyz*a_pos.x+i_c1.xyz*a_pos.y+i_c2.xyz*a_pos.z+i_c3.xyz):a_pos;`;

  const shadowProg = link(
    `#version 300 es\nlayout(location=0) in vec3 a_pos;${instDecl}uniform mat4 u_lmvp;void main(){${instPos}gl_Position=u_lmvp*vec4(wp,1.0);}`,
    `#version 300 es\nprecision mediump float;void main(){}`,
  );
  const skyProg = link(
    `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 v_ndc;void main(){v_ndc=a_uv*2.0-1.0;gl_Position=vec4(v_ndc,0.9999,1.0);}`,
    `#version 300 es\nprecision highp float;in vec2 v_ndc;uniform vec3 u_fwd,u_right,u_up,u_sky,u_fog,u_sun;uniform float u_aspect,u_tanHalf;out vec4 o_hdr;
void main(){
vec3 rd=normalize(u_fwd+v_ndc.x*u_aspect*u_tanHalf*u_right+v_ndc.y*u_tanHalf*u_up);
float h=clamp(rd.y,-0.25,1.0);
vec3 zenith=pow(max(u_sky,vec3(0.03)),vec3(1.5))*vec3(0.85,1.05,1.38)+vec3(0.015,0.04,0.09);
vec3 horiz=pow(max(u_fog,vec3(0.04)),vec3(1.5))*vec3(1.22,1.18,1.12)+vec3(0.05,0.055,0.05);
vec3 col=mix(horiz,zenith,pow(clamp(h,0.0,1.0),0.52));
if(h<0.0)col=mix(horiz,zenith*0.35,clamp(-h*3.2,0.0,1.0));
float mu=max(dot(rd,u_sun),0.0);
col+=vec3(1.05,0.80,0.46)*pow(mu,9.0)*0.32+vec3(2.2,1.9,1.45)*smoothstep(0.9984,0.9997,mu);
o_hdr=vec4(max(col,vec3(0.0)),1.0);}`,
  );
  const pbrProg = link(
    `#version 300 es\nlayout(location=0) in vec3 a_pos;layout(location=1) in vec3 a_norm;layout(location=2) in vec3 a_col;${instDecl}uniform mat4 u_mvp,u_lmvp;out vec3 v_pos,v_norm,v_col;out vec4 v_lpos;
void main(){${instPos}vec3 wn=u_inst>0.5?normalize(i_c0.xyz*a_norm.x+i_c1.xyz*a_norm.y+i_c2.xyz*a_norm.z):a_norm;
v_pos=wp;v_norm=wn;v_col=u_inst>0.5?a_col*vec3(i_c0.w,i_c1.w,i_c2.w):a_col;v_lpos=u_lmvp*vec4(wp+wn*0.04,1.0);gl_Position=u_mvp*vec4(wp,1.0);}`,
    `#version 300 es\nprecision highp float;precision highp sampler2DShadow;in vec3 v_pos,v_norm,v_col;in vec4 v_lpos;
uniform vec3 u_eye,u_fog,u_sky,u_sun;uniform float u_time,u_shadows,u_invSmap,u_quality;uniform sampler2DShadow u_smap;out vec4 o_hdr;
float hash3(vec3 p){p=fract(p*vec3(0.1031,0.1030,0.0973));p+=dot(p,p.yxz+33.33);return fract((p.x+p.y)*p.z);}
float triGrain(vec3 p){vec3 i=floor(p),f=fract(p);f=f*f*(3.0-2.0*f);return mix(mix(hash3(i),hash3(i+vec3(1,0,0)),f.x),mix(hash3(i+vec3(0,0,1)),hash3(i+vec3(1,0,1)),f.x),f.z);}
float sampleShadow(vec4 lp,float ndl){
if(u_shadows<0.5)return 1.0;
vec3 p=lp.xyz/max(lp.w,1e-4)*0.5+0.5;
vec2 edge=smoothstep(vec2(0.0),vec2(0.07),p.xy)*(1.0-smoothstep(vec2(0.93),vec2(1.0),p.xy));
float mask=edge.x*edge.y*step(0.0,p.z)*step(p.z,1.0);
if(mask<=0.001)return 1.0;
float bias=max(0.0016*(1.0-ndl),0.00045),acc=0.0;
if(u_quality<1.5)return mix(1.0,texture(u_smap,vec3(p.xy,p.z-bias)),mask);
for(int y=-1;y<=1;y++)for(int x=-1;x<=1;x++)acc+=texture(u_smap,vec3(p.xy+vec2(float(x),float(y))*u_invSmap,p.z-bias));
return mix(1.0,acc/9.0,mask);}
void main(){
vec3 toE=u_eye-v_pos;float dist=length(toE);vec3 n=normalize(v_norm),v=toE/max(dist,1e-4),l=u_sun;
bool isWater=(v_col.b>v_col.r*1.35&&abs(n.y)>0.92),isVisor=(v_col.b>0.85&&v_col.g>0.72&&v_col.r<0.35),isMetal=(v_col.r>v_col.b*1.25&&v_col.g>v_col.b*1.05);
float g1=0.5,g2=0.5;
if(isWater){
float w1=sin(v_pos.x*3.1+u_time*2.3)*cos(v_pos.z*2.7-u_time*1.8),w2=sin((v_pos.x+v_pos.z)*5.5-u_time*3.0)*0.5;
n=normalize(n+vec3((w1+w2)*0.042,0.0,(w1-w2)*0.042));
}else if(u_quality>=1.5&&dist<30.0){
g1=triGrain(v_pos*5.2);g2=triGrain(v_pos*18.5);float micro=(g1-0.5)*0.055+(g2-0.5)*0.028;
vec3 c1=cross(dFdx(v_pos),n),c2=cross(n,dFdy(v_pos));
n=normalize(n+((c1/max(length(c1),1e-4))*dFdx(micro)+(c2/max(length(c2),1e-4))*dFdy(micro))*(isMetal?0.55:1.15));}
vec3 albedo=pow(max(v_col,vec3(0.02)),vec3(2.0));
if(!isWater&&!isVisor)albedo*=0.93+0.14*g1;
float metallic=isWater?0.04:(isMetal?0.72:0.05),rough=isWater?0.07:(isMetal?0.28:clamp(0.66-0.15*g2,0.24,0.88));
float ndl=max(dot(n,l),0.0),ndv=max(dot(n,v),1e-3);
vec3 h=normalize(l+v);
float ndh=max(dot(n,h),0.0),vdh=max(dot(v,h),0.0),shadow=sampleShadow(v_lpos,ndl);
float a=rough*rough,a2=a*a,denom=ndh*ndh*(a2-1.0)+1.0,dGgx=a2/max(3.14159265*denom*denom,1e-4);
float k=(rough+1.0)*(rough+1.0)*0.125,gSmith=(ndv/(ndv*(1.0-k)+k))*(ndl/(ndl*(1.0-k)+k));
vec3 f0=mix(vec3(0.04),albedo,metallic),fres=f0+(1.0-f0)*pow(1.0-vdh,5.0);
vec3 spec=(dGgx*gSmith*fres)/max(4.0*ndv*ndl,1e-3),kd=(1.0-fres)*(1.0-metallic),sunRad=vec3(2.65,2.42,2.02)*shadow;
vec3 direct=(kd*albedo/3.14159265+spec)*sunRad*ndl;
float ao=u_quality>=1.5?clamp(1.0-length(fwidth(n))*0.38,0.56,1.0):0.92;
vec3 skyLin=pow(max(u_sky,vec3(0.04)),vec3(1.5))*1.35+vec3(0.08,0.12,0.18),fogLin=pow(max(u_fog,vec3(0.04)),vec3(1.5))*1.15+vec3(0.05);
vec3 hemi=mix(vec3(0.09,0.08,0.07),skyLin,0.5+0.5*n.y),rDir=reflect(-v,n);
vec3 iblSpec=mix(fogLin,skyLin,clamp(rDir.y*0.5+0.5,0.0,1.0))*(1.0-rough*0.72);
vec3 fEnv=f0+(max(vec3(1.0-rough),f0)-f0)*pow(1.0-ndv,5.0),lit=direct+(kd*albedo*hemi+iblSpec*fEnv)*ao;
if(isVisor)lit+=albedo*0.38;
if(isWater)lit=mix(albedo*(hemi+0.4*shadow),skyLin+sunRad*pow(ndh,96.0),clamp(0.18+0.78*pow(1.0-ndv,4.0),0.0,0.95));
o_hdr=vec4(max(mix(lit,fogLin,clamp(1.0-exp(-dist*0.0115),0.0,0.74)),vec3(0.0)),1.0);}`,
  );
  const postProg = link(
    `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 v_uv;void main(){v_uv=a_uv;gl_Position=vec4(a_uv*2.0-1.0,0.0,1.0);}`,
    `#version 300 es\nprecision highp float;in vec2 v_uv;uniform sampler2D u_hdr;uniform vec2 u_invRes;out vec4 o_col;
vec3 aces(vec3 x){return clamp((x*(2.51*x+0.03))/(x*(2.43*x+0.59)+0.14),0.0,1.0);}
void main(){
vec3 c=texture(u_hdr,v_uv).rgb,avg=(texture(u_hdr,v_uv+vec2(0.0,u_invRes.y)).rgb+texture(u_hdr,v_uv-vec2(0.0,u_invRes.y)).rgb+texture(u_hdr,v_uv+vec2(u_invRes.x,0.0)).rgb+texture(u_hdr,v_uv-vec2(u_invRes.x,0.0)).rgb)*0.25;
vec3 aa=mix(c,(c+avg)*0.5,clamp(length(c-avg)*2.2,0.0,0.45));
o_col=vec4(pow(aces(aa*1.06),vec3(1.0/2.2)),1.0);}`,
  );
  const blitProg = link(
    `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 v_uv;void main(){v_uv=vec2(a_uv.x,1.0-a_uv.y);gl_Position=vec4(a_uv*2.0-1.0,0.0,1.0);}`,
    `#version 300 es\nprecision mediump float;in vec2 v_uv;uniform sampler2D u_tex;out vec4 o_col;void main(){o_col=texture(u_tex,v_uv);}`,
  );
  if (!shadowProg || !skyProg || !pbrProg || !postProg) return null;

  const B = gl.ARRAY_BUFFER, E = gl.ELEMENT_ARRAY_BUFFER, T = gl.TEXTURE_2D, F = gl.FRAMEBUFFER, R = gl.RENDERBUFFER;
  const makeVao = (instanced = false) => {
    const vao = gl.createVertexArray(), pos = gl.createBuffer(), norm = gl.createBuffer(), col = gl.createBuffer(), idx = gl.createBuffer();
    const inst = instanced ? gl.createBuffer() : null;
    gl.bindVertexArray(vao);
    [pos, norm, col].forEach((b, loc) => { gl.bindBuffer(B, b); gl.enableVertexAttribArray(loc); gl.vertexAttribPointer(loc, 3, gl.FLOAT, false, 0, 0); });
    if (inst) {
      gl.bindBuffer(B, inst);
      for (let k = 0; k < 4; k++) { gl.enableVertexAttribArray(3 + k); gl.vertexAttribPointer(3 + k, 4, gl.FLOAT, false, 64, k * 16); gl.vertexAttribDivisor(3 + k, 1); }
    }
    gl.bindBuffer(E, idx); gl.bindVertexArray(null);
    return { vao, pos, norm, col, idx, inst, count: 0, instCount: 0, capPos: 0, capIdx: 0, epoch: 0 };
  };
  const staticGpu = makeVao(false), dynGpu = makeVao(false), instGpu = makeVao(true);

  const quadVao = gl.createVertexArray(), quadBuf = gl.createBuffer();
  gl.bindVertexArray(quadVao); gl.bindBuffer(B, quadBuf);
  gl.bufferData(B, new Float32Array([0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1]), gl.STATIC_DRAW);
  gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0); gl.bindVertexArray(null);
  const drawQuad = () => { gl.bindVertexArray(quadVao); gl.drawArrays(gl.TRIANGLES, 0, 6); };

  const makeShadow = () => {
    const tex = gl.createTexture(), fbo = gl.createFramebuffer();
    gl.bindTexture(T, tex); gl.texImage2D(T, 0, gl.DEPTH_COMPONENT24, shadowSize, shadowSize, 0, gl.DEPTH_COMPONENT, gl.UNSIGNED_INT, null);
    setTexClamp(gl.LINEAR); gl.texParameteri(T, gl.TEXTURE_COMPARE_MODE, gl.COMPARE_REF_TO_TEXTURE); gl.texParameteri(T, gl.TEXTURE_COMPARE_FUNC, gl.LEQUAL);
    gl.bindFramebuffer(F, fbo); gl.framebufferTexture2D(F, gl.DEPTH_ATTACHMENT, T, tex, 0); gl.drawBuffers([gl.NONE]); gl.readBuffer(gl.NONE);
    return { tex, fbo };
  };
  const staticShadow = makeShadow(), shadow = makeShadow();
  let cachedShadowKey = "", hdrW = 0, hdrH = 0, renderScale = 1, lastStaticBytes = 0, lastDynBytes = 0;

  const hdrTex = gl.createTexture(), hdrDepth = gl.createRenderbuffer(), hdrFbo = gl.createFramebuffer();
  const ensureHdrFbo = (w, h) => {
    if (w === hdrW && h === hdrH) return true;
    hdrW = w; hdrH = h;
    gl.bindTexture(T, hdrTex); gl.texImage2D(T, 0, hasHalfFloat ? gl.RGBA16F : gl.RGBA8, w, h, 0, gl.RGBA, hasHalfFloat ? gl.HALF_FLOAT : gl.UNSIGNED_BYTE, null); setTexClamp(gl.LINEAR);
    gl.bindRenderbuffer(R, hdrDepth); gl.renderbufferStorage(R, gl.DEPTH_COMPONENT24, w, h);
    gl.bindFramebuffer(F, hdrFbo); gl.framebufferTexture2D(F, gl.COLOR_ATTACHMENT0, T, hdrTex, 0); gl.framebufferRenderbuffer(F, gl.DEPTH_ATTACHMENT, R, hdrDepth);
    const ok = gl.checkFramebufferStatus(F) === gl.FRAMEBUFFER_COMPLETE; gl.bindFramebuffer(F, null); return ok;
  };
  const blitTex = gl.createTexture(); gl.bindTexture(T, blitTex); setTexClamp(gl.NEAREST); gl.bindFramebuffer(F, null);

  const uShadow = uniforms(shadowProg, ["lmvp", "inst"]);
  const uSky = uniforms(skyProg, ["fwd", "right", "up", "sky", "fog", "sun", "aspect", "tanHalf"]);
  const uPbr = uniforms(pbrProg, ["mvp", "lmvp", "eye", "fog", "sky", "sun", "time", "shadows", "invSmap", "smap", "inst", "quality"]);
  const uPost = uniforms(postProg, ["hdr", "invRes"]);
  const lightOrtho = mat4Ortho(-34, 34, -34, 34, -48, 64);

  const streamUpload = (target, buf, view, capKey) => {
    gl.bindBuffer(target, buf);
    if (view.byteLength > dynGpu[capKey]) { dynGpu[capKey] = Math.max(view.byteLength * 2, 16384); gl.bufferData(target, dynGpu[capKey], gl.STREAM_DRAW); }
    if (view.byteLength > 0) gl.bufferSubData(target, 0, view);
  };

  return {
    backend: "webgl2",
    isSoftware,
    setRenderScale(pm) { renderScale = Math.max(0.65, Math.min(1, (pm || 1000) / 1000)); },
    getUploadTelemetry() { return { staticBytes: lastStaticBytes, dynBytes: lastDynBytes, staticEpoch: staticGpu.epoch, instCount: instGpu.instCount }; },
    uploadSplitBuffers(sv, dv, iv = null) {
      lastStaticBytes = 0; lastDynBytes = 0;
      if (sv && sv.epoch !== staticGpu.epoch) {
        [[B, staticGpu.pos, sv.pos], [B, staticGpu.norm, sv.norm], [B, staticGpu.col, sv.col], [E, staticGpu.idx, sv.idx]].forEach(([t, b, d]) => {
          gl.bindBuffer(t, b); gl.bufferData(t, d, gl.STATIC_DRAW); lastStaticBytes += d.byteLength;
        });
        staticGpu.count = sv.idx.length; staticGpu.epoch = sv.epoch; cachedShadowKey = "";
      }
      if (dv && dv.idx.length > 0) {
        streamUpload(B, dynGpu.pos, dv.pos, "capPos");
        [ [dynGpu.norm, dv.norm], [dynGpu.col, dv.col] ].forEach(([b, v]) => {
          gl.bindBuffer(B, b); if (v.byteLength > dynGpu.capPos) gl.bufferData(B, dynGpu.capPos, gl.STREAM_DRAW); gl.bufferSubData(B, 0, v);
        });
        streamUpload(E, dynGpu.idx, dv.idx, "capIdx");
        dynGpu.count = dv.idx.length; lastDynBytes = dv.pos.byteLength * 3 + dv.idx.byteLength;
      } else dynGpu.count = 0;
      if (iv && iv.instCount > 0) {
        if (iv.epoch !== instGpu.epoch) {
          [[B, instGpu.pos, iv.pos], [B, instGpu.norm, iv.norm], [B, instGpu.col, iv.col], [E, instGpu.idx, iv.idx], [B, instGpu.inst, iv.data]].forEach(([t, b, d]) => {
            gl.bindBuffer(t, b); gl.bufferData(t, d, gl.STATIC_DRAW);
          });
          instGpu.count = iv.idx.length; instGpu.instCount = iv.instCount; instGpu.epoch = iv.epoch;
        }
      } else instGpu.instCount = 0;
    },
    uploadPackedBuffers(posF32, normF32, colF32, idxU32) {
      [[B, staticGpu.pos, posF32], [B, staticGpu.norm, normF32], [B, staticGpu.col, colF32], [E, staticGpu.idx, idxU32]].forEach(([t, b, d]) => {
        gl.bindBuffer(t, b); gl.bufferData(t, d, gl.STATIC_DRAW);
      });
      staticGpu.count = idxU32.length; staticGpu.epoch = (staticGpu.epoch + 1) >>> 0; dynGpu.count = 0; cachedShadowKey = "";
    },
    render(mvp, eye, lodSpec, timeSec, lookTarget = [0, 1.8, 0], fovRad = Math.PI / 3.4) {
      const w = canvas.width | 0, h = canvas.height | 0;
      if (w <= 0 || h <= 0) return;
      const rank = lodSpec?.rank ?? 3, quality = isSoftware ? Math.min(1, rank) : (rank >= 2 ? 2 : 1);
      const sky = lodSpec?.sky ?? [0.18, 0.3, 0.38], fog = lodSpec?.fog ?? [0.3, 0.4, 0.44];
      const totalIdx = staticGpu.count + dynGpu.count + (instGpu.instCount > 0 ? instGpu.count : 0);
      const shadowsOn = rank >= 1 && totalIdx > 0 ? 1 : 0;
      const fx = lookTarget[0] ?? 0, fy = lookTarget[1] ?? 1.5, fz = lookTarget[2] ?? 0;
      const sfx = Math.round(fx * 0.5) * 2, sfy = Math.round(fy * 0.5) * 2, sfz = Math.round(fz * 0.5) * 2;
      const lmvp = mat4Mul(lightOrtho, mat4LookAt([sfx + SUN_DIR[0] * 14, sfy + SUN_DIR[1] * 14, sfz + SUN_DIR[2] * 14], [sfx, sfy, sfz], [0, 1, 0]));

      if (shadowsOn) {
        const key = `${staticGpu.epoch}:${sfx},${sfy},${sfz}`;
        gl.colorMask(false, false, false, false); gl.enable(gl.DEPTH_TEST); gl.depthMask(true);
        gl.enable(gl.POLYGON_OFFSET_FILL); gl.polygonOffset(1.8, 3);
        gl.useProgram(shadowProg); gl.uniformMatrix4fv(uShadow.lmvp, false, lmvp);
        if (key !== cachedShadowKey) {
          gl.bindFramebuffer(F, staticShadow.fbo); gl.viewport(0, 0, shadowSize, shadowSize); gl.clear(gl.DEPTH_BUFFER_BIT);
          if (staticGpu.count > 0) { gl.uniform1f(uShadow.inst, 0); gl.bindVertexArray(staticGpu.vao); gl.drawElements(gl.TRIANGLES, staticGpu.count, gl.UNSIGNED_INT, 0); }
          cachedShadowKey = key;
        }
        gl.bindFramebuffer(gl.READ_FRAMEBUFFER, staticShadow.fbo); gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, shadow.fbo);
        gl.blitFramebuffer(0, 0, shadowSize, shadowSize, 0, 0, shadowSize, shadowSize, gl.DEPTH_BUFFER_BIT, gl.NEAREST);
        gl.bindFramebuffer(F, shadow.fbo); gl.viewport(0, 0, shadowSize, shadowSize);
        if (dynGpu.count > 0) { gl.uniform1f(uShadow.inst, 0); gl.bindVertexArray(dynGpu.vao); gl.drawElements(gl.TRIANGLES, dynGpu.count, gl.UNSIGNED_INT, 0); }
        if (instGpu.instCount > 0 && instGpu.count > 0 && !isSoftware) {
          gl.uniform1f(uShadow.inst, 1); gl.bindVertexArray(instGpu.vao); gl.drawElementsInstanced(gl.TRIANGLES, instGpu.count, gl.UNSIGNED_INT, 0, instGpu.instCount);
        }
        gl.disable(gl.POLYGON_OFFSET_FILL); gl.colorMask(true, true, true, true);
      }

      const sc = isSoftware ? Math.min(renderScale, 0.85) : renderScale;
      const rw = Math.max(16, Math.round(w * sc)), rh = Math.max(16, Math.round(h * sc)), useHdr = ensureHdrFbo(rw, rh);
      gl.bindFramebuffer(F, useHdr ? hdrFbo : null); gl.viewport(0, 0, useHdr ? rw : w, useHdr ? rh : h);
      gl.clearColor(sky[0], sky[1], sky[2], 1); gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

      let fdx = fx - eye[0], fdy = fy - eye[1], fdz = fz - eye[2];
      const fl = Math.hypot(fdx, fdy, fdz) || 1; fdx /= fl; fdy /= fl; fdz /= fl;
      const rl = Math.hypot(-fdz, fdx) || 1, rdx = -fdz / rl, rdz = fdx / rl;
      gl.disable(gl.DEPTH_TEST); gl.depthMask(false); gl.useProgram(skyProg);
      gl.uniform3f(uSky.fwd, fdx, fdy, fdz); gl.uniform3f(uSky.right, rdx, 0, rdz); gl.uniform3f(uSky.up, -rdz * fdy, rdz * fdx - rdx * fdz, rdx * fdy);
      gl.uniform3fv(uSky.sky, sky); gl.uniform3fv(uSky.fog, fog); gl.uniform3fv(uSky.sun, SUN_DIR);
      gl.uniform1f(uSky.aspect, w / h); gl.uniform1f(uSky.tanHalf, Math.tan(fovRad * 0.5)); drawQuad();

      gl.depthMask(true); gl.enable(gl.DEPTH_TEST);
      if (totalIdx > 0 && mvp) {
        gl.useProgram(pbrProg);
        gl.uniformMatrix4fv(uPbr.mvp, false, mvp); gl.uniformMatrix4fv(uPbr.lmvp, false, lmvp);
        gl.uniform3fv(uPbr.eye, eye); gl.uniform3fv(uPbr.fog, fog); gl.uniform3fv(uPbr.sky, sky); gl.uniform3fv(uPbr.sun, SUN_DIR);
        gl.uniform1f(uPbr.time, timeSec); gl.uniform1f(uPbr.shadows, shadowsOn); gl.uniform1f(uPbr.invSmap, 1 / shadowSize); gl.uniform1f(uPbr.quality, quality);
        gl.activeTexture(gl.TEXTURE0); gl.bindTexture(T, shadow.tex); gl.uniform1i(uPbr.smap, 0);
        if (staticGpu.count > 0) { gl.uniform1f(uPbr.inst, 0); gl.bindVertexArray(staticGpu.vao); gl.drawElements(gl.TRIANGLES, staticGpu.count, gl.UNSIGNED_INT, 0); }
        if (dynGpu.count > 0) { gl.uniform1f(uPbr.inst, 0); gl.bindVertexArray(dynGpu.vao); gl.drawElements(gl.TRIANGLES, dynGpu.count, gl.UNSIGNED_INT, 0); }
        if (instGpu.instCount > 0 && instGpu.count > 0) {
          gl.uniform1f(uPbr.inst, 1); gl.bindVertexArray(instGpu.vao); gl.drawElementsInstanced(gl.TRIANGLES, instGpu.count, gl.UNSIGNED_INT, 0, instGpu.instCount);
        }
      }
      if (useHdr) {
        gl.bindFramebuffer(F, null); gl.viewport(0, 0, w, h); gl.disable(gl.DEPTH_TEST); gl.useProgram(postProg);
        gl.activeTexture(gl.TEXTURE0); gl.bindTexture(T, hdrTex); gl.uniform1i(uPost.hdr, 0); gl.uniform2f(uPost.invRes, 1 / rw, 1 / rh); drawQuad();
      }
      gl.bindVertexArray(null);
    },
    blitTensorRgba(rgbaBytes, width, height) {
      if (!blitProg || !rgbaBytes || width <= 0 || height <= 0) return;
      gl.bindFramebuffer(F, null); gl.viewport(0, 0, canvas.width, canvas.height); gl.disable(gl.DEPTH_TEST); gl.useProgram(blitProg);
      gl.activeTexture(gl.TEXTURE0); gl.bindTexture(T, blitTex); gl.texImage2D(T, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, rgbaBytes);
      drawQuad(); gl.bindVertexArray(null);
    },
  };
}

export function createCanvas2dTensorRenderer(canvas) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return null;
  return {
    backend: "tensor-2d",
    isSoftware: true,
    setRenderScale() {},
    getUploadTelemetry() { return { staticBytes: 0, dynBytes: 0, staticEpoch: 0, instCount: 0 }; },
    uploadSplitBuffers() {},
    uploadPackedBuffers() {},
    render(_mvp, _eye, lodSpec) {
      if (canvas.width <= 0 || canvas.height <= 0) return;
      const [r = 0.18, g = 0.3, b = 0.38] = lodSpec?.sky ?? [];
      ctx.fillStyle = `rgb(${Math.round(r * 255)},${Math.round(g * 255)},${Math.round(b * 255)})`;
      ctx.fillRect(0, 0, canvas.width, canvas.height);
    },
    blitTensorRgba(rgbaBytes, width, height) {
      if (!rgbaBytes || width <= 0 || height <= 0 || typeof ImageData !== "function") return;
      ctx.putImageData(new ImageData(new Uint8ClampedArray(rgbaBytes.slice().buffer), width, height), 0, 0);
    },
  };
}

export async function createBestRenderer(canvas) {
  return (await createGlRenderer(canvas)) || createCanvas2dTensorRenderer(canvas);
}
