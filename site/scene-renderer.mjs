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
  const hHf = Boolean(gl.getExtension("EXT_color_buffer_half_float"));
  const sSz = isSoftware ? 1024 : 2048;

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
  const setTexClamp = (f) => {
    gl.texParameteri(T, gl.TEXTURE_MIN_FILTER, f); gl.texParameteri(T, gl.TEXTURE_MAG_FILTER, f);
    gl.texParameteri(T, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE); gl.texParameteri(T, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  };
  const uniforms = (prog, names) => Object.fromEntries(names.map((n) => [n, gl.getUniformLocation(prog, `u_${n}`)]));

  const instDecl = `layout(location=3) in vec4 i_c0;layout(location=4) in vec4 i_c1;layout(location=5) in vec4 i_c2;layout(location=6) in vec4 i_c3;uniform float u_inst;`;
  const instPos = `vec3 wp=u_inst>0.5?(i_c0.xyz*a_pos.x+i_c1.xyz*a_pos.y+i_c2.xyz*a_pos.z+i_c3.xyz):a_pos;`;
  const B = gl.ARRAY_BUFFER, E = gl.ELEMENT_ARRAY_BUFFER, T = gl.TEXTURE_2D, F = gl.FRAMEBUFFER, R = gl.RENDERBUFFER;
  let sPrg, kPrg, pPrg, oPrg, bPrg, sGpu, dGpu, iGpu, qVao, sShd, shd, hTex, hDep, hFbo, bTex, uSh, uSk, uPb, uPo;
  let contextLost = false, cKey = "", hdrW = 0, hdrH = 0, rScale = 1, lSB = 0, lDB = 0, fence = null;
  const buildProgs = () => {
    sPrg = link(
      `#version 300 es\nlayout(location=0) in vec3 a_pos;${instDecl}uniform mat4 u_lmvp;void main(){${instPos}gl_Position=u_lmvp*vec4(wp,1.0);}`,
      `#version 300 es\nprecision mediump float;void main(){}`,
    );
    kPrg = link(
      `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 vn;void main(){vn=a_uv*2.0-1.0;gl_Position=vec4(vn,0.9999,1.0);}`,
      `#version 300 es\nprecision highp float;in vec2 vn;uniform vec3 u_fwd,u_right,u_up,u_sky,u_fog,u_sun;uniform float u_aspect,u_th;out vec4 o_hdr;void main(){vec3 rd=normalize(u_fwd+vn.x*u_aspect*u_th*u_right+vn.y*u_th*u_up);float h=clamp(rd.y,-0.25,1.0);vec3 z=pow(max(u_sky,vec3(0.03)),vec3(1.5))*vec3(0.85,1.05,1.38)+vec3(0.015,0.04,0.09),hz=pow(max(u_fog,vec3(0.04)),vec3(1.5))*vec3(1.22,1.18,1.12)+vec3(0.05,0.055,0.05),c=mix(hz,z,pow(clamp(h,0.0,1.0),0.52));if(h<0.0)c=mix(hz,z*0.35,clamp(-h*3.2,0.0,1.0));float mu=max(dot(rd,u_sun),0.0);c+=vec3(1.05,0.8,0.46)*pow(mu,9.0)*0.32+vec3(2.2,1.9,1.45)*smoothstep(0.9984,0.9997,mu);o_hdr=vec4(max(c,vec3(0.0)),1.0);}`,
    );
    pPrg = link(
      `#version 300 es\nlayout(location=0) in vec3 a_pos;layout(location=1) in vec3 a_norm;layout(location=2) in vec3 a_col;${instDecl}uniform mat4 u_mvp,u_lmvp;out vec3 v_pos,v_norm,v_col;out vec4 v_lp;void main(){${instPos}vec3 wn=u_inst>0.5?normalize(i_c0.xyz*a_norm.x+i_c1.xyz*a_norm.y+i_c2.xyz*a_norm.z):a_norm;v_pos=wp;v_norm=wn;v_col=u_inst>0.5?a_col*vec3(i_c0.w,i_c1.w,i_c2.w):a_col;v_lp=u_lmvp*vec4(wp+wn*0.04,1.0);gl_Position=u_mvp*vec4(wp,1.0);}`,
      `#version 300 es\nprecision highp float;precision highp sampler2DShadow;in vec3 v_pos,v_norm,v_col;in vec4 v_lp;uniform vec3 u_eye,u_fog,u_sky,u_sun;uniform float u_time,u_sh,u_is,u_q;uniform sampler2DShadow u_smap;out vec4 o_hdr;float h3(vec3 p){p=fract(p*vec3(0.1031,0.103,0.0973));p+=dot(p,p.yxz+33.33);return fract((p.x+p.y)*p.z);}float tG(vec3 p){vec3 i=floor(p),f=fract(p);f=f*f*(3.0-2.0*f);return mix(mix(h3(i),h3(i+vec3(1,0,0)),f.x),mix(h3(i+vec3(0,0,1)),h3(i+vec3(1,0,1)),f.x),f.z);}float sS(vec4 lp,float ndl){if(u_sh<0.5)return 1.0;vec3 p=lp.xyz/max(lp.w,1e-4)*0.5+0.5;vec2 e=smoothstep(vec2(0.0),vec2(0.07),p.xy)*(1.0-smoothstep(vec2(0.93),vec2(1.0),p.xy));float m=e.x*e.y*step(0.0,p.z)*step(p.z,1.0);if(m<=0.001)return 1.0;float b=max(0.0016*(1.0-ndl),0.00045),a=0.0;if(u_q<1.5)return mix(1.0,texture(u_smap,vec3(p.xy,p.z-b)),m);for(int y=-1;y<=1;y++)for(int x=-1;x<=1;x++)a+=texture(u_smap,vec3(p.xy+vec2(float(x),float(y))*u_is,p.z-b));return mix(1.0,a/9.0,m);}void main(){vec3 te=u_eye-v_pos;float d=length(te);vec3 n=normalize(v_norm),v=te/max(d,1e-4),l=u_sun;bool iW=(v_col.b>v_col.r*1.35&&abs(n.y)>0.92),iV=(v_col.b>0.85&&v_col.g>0.72&&v_col.r<0.35),iM=(v_col.r>v_col.b*1.25&&v_col.g>v_col.b*1.05);float g1=0.5,g2=0.5;if(iW){float w1=sin(v_pos.x*3.1+u_time*2.3)*cos(v_pos.z*2.7-u_time*1.8),w2=sin((v_pos.x+v_pos.z)*5.5-u_time*3.0)*0.5;n=normalize(n+vec3((w1+w2)*0.042,0.0,(w1-w2)*0.042));}else if(u_q>=1.5&&d<30.0){g1=tG(v_pos*5.2);g2=tG(v_pos*18.5);float mc=(g1-0.5)*0.055+(g2-0.5)*0.028;vec3 c1=cross(dFdx(v_pos),n),c2=cross(n,dFdy(v_pos));n=normalize(n+((c1/max(length(c1),1e-4))*dFdx(mc)+(c2/max(length(c2),1e-4))*dFdy(mc))*(iM?0.55:1.15));}vec3 ab=pow(max(v_col,vec3(0.02)),vec3(2.0));if(!iW&&!iV)ab*=0.93+0.14*g1;float mt=iW?0.04:(iM?0.72:0.05),rg=iW?0.07:(iM?0.28:clamp(0.66-0.15*g2,0.24,0.88)),ndl=max(dot(n,l),0.0),ndv=max(dot(n,v),1e-3);vec3 h=normalize(l+v);float ndh=max(dot(n,h),0.0),vdh=max(dot(v,h),0.0),sw=sS(v_lp,ndl),a=rg*rg,a2=a*a,dn=ndh*ndh*(a2-1.0)+1.0,dg=a2/max(3.14159265*dn*dn,1e-4),k=(rg+1.0)*(rg+1.0)*0.125,gs=(ndv/(ndv*(1.0-k)+k))*(ndl/(ndl*(1.0-k)+k));vec3 f0=mix(vec3(0.04),ab,mt),fr=f0+(1.0-f0)*pow(1.0-vdh,5.0),sp=(dg*gs*fr)/max(4.0*ndv*ndl,1e-3),kd=(1.0-fr)*(1.0-mt),sr=vec3(2.65,2.42,2.02)*sw,dr=(kd*ab/3.14159265+sp)*sr*ndl;float ao=u_q>=1.5?clamp(1.0-length(fwidth(n))*0.38,0.56,1.0):0.92;vec3 sl=pow(max(u_sky,vec3(0.04)),vec3(1.5))*1.35+vec3(0.08,0.12,0.18),fl=pow(max(u_fog,vec3(0.04)),vec3(1.5))*1.15+vec3(0.05),hm=mix(vec3(0.09,0.08,0.07),sl,0.5+0.5*n.y),rD=reflect(-v,n),ib=mix(fl,sl,clamp(rD.y*0.5+0.5,0.0,1.0))*(1.0-rg*0.72),fE=f0+(max(vec3(1.0-rg),f0)-f0)*pow(1.0-ndv,5.0),lt=dr+(kd*ab*hm+ib*fE)*ao;if(iV)lt+=ab*0.38;if(iW)lt=mix(ab*(hm+0.4*sw),sl+sr*pow(ndh,96.0),clamp(0.18+0.78*pow(1.0-ndv,4.0),0.0,0.95));o_hdr=vec4(max(mix(lt,fl,clamp(1.0-exp(-d*0.0115),0.0,0.74)),vec3(0.0)),1.0);}`,
    );
    oPrg = link(
      `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 v_uv;void main(){v_uv=a_uv;gl_Position=vec4(a_uv*2.0-1.0,0.0,1.0);}`,
      `#version 300 es\nprecision highp float;in vec2 v_uv;uniform sampler2D u_hdr;uniform vec2 u_ir;out vec4 o_col;vec3 ac(vec3 x){return clamp((x*(2.51*x+0.03))/(x*(2.43*x+0.59)+0.14),0.0,1.0);}void main(){vec3 c=texture(u_hdr,v_uv).rgb,av=(texture(u_hdr,v_uv+vec2(0.0,u_ir.y)).rgb+texture(u_hdr,v_uv-vec2(0.0,u_ir.y)).rgb+texture(u_hdr,v_uv+vec2(u_ir.x,0.0)).rgb+texture(u_hdr,v_uv-vec2(u_ir.x,0.0)).rgb)*0.25,aa=mix(c,(c+av)*0.5,clamp(length(c-av)*2.2,0.0,0.45));o_col=vec4(pow(ac(aa*1.06),vec3(1.0/2.2)),1.0);}`,
    );
    bPrg = link(
      `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 v_uv;void main(){v_uv=vec2(a_uv.x,1.0-a_uv.y);gl_Position=vec4(a_uv*2.0-1.0,0.0,1.0);}`,
      `#version 300 es\nprecision mediump float;in vec2 v_uv;uniform sampler2D u_tex;out vec4 o_col;void main(){o_col=texture(u_tex,v_uv);}`,
    );
    return Boolean(sPrg && kPrg && pPrg && oPrg);
  };

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
  const makeShadow = () => {
    const tex = gl.createTexture(), fbo = gl.createFramebuffer();
    gl.bindTexture(T, tex); gl.texImage2D(T, 0, gl.DEPTH_COMPONENT24, sSz, sSz, 0, gl.DEPTH_COMPONENT, gl.UNSIGNED_INT, null);
    setTexClamp(gl.LINEAR); gl.texParameteri(T, gl.TEXTURE_COMPARE_MODE, gl.COMPARE_REF_TO_TEXTURE); gl.texParameteri(T, gl.TEXTURE_COMPARE_FUNC, gl.LEQUAL);
    gl.bindFramebuffer(F, fbo); gl.framebufferTexture2D(F, gl.DEPTH_ATTACHMENT, T, tex, 0); gl.drawBuffers([gl.NONE]); gl.readBuffer(gl.NONE);
    return { tex, fbo };
  };
  const buildGpu = () => {
    gl.getExtension("EXT_color_buffer_half_float");
    if (!buildProgs()) return false;
    sGpu = makeVao(false); dGpu = makeVao(false); iGpu = makeVao(true);
    qVao = gl.createVertexArray(); const qBuf = gl.createBuffer();
    gl.bindVertexArray(qVao); gl.bindBuffer(B, qBuf);
    gl.bufferData(B, new Float32Array([0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1]), gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0); gl.bindVertexArray(null);
    sShd = makeShadow(); shd = makeShadow();
    hTex = gl.createTexture(); hDep = gl.createRenderbuffer(); hFbo = gl.createFramebuffer();
    bTex = gl.createTexture(); gl.bindTexture(T, bTex); setTexClamp(gl.NEAREST); gl.bindFramebuffer(F, null);
    uSh = uniforms(sPrg, ["lmvp", "inst"]);
    uSk = uniforms(kPrg, ["fwd", "right", "up", "sky", "fog", "sun", "aspect", "th"]);
    uPb = uniforms(pPrg, ["mvp", "lmvp", "eye", "fog", "sky", "sun", "time", "sh", "is", "smap", "inst", "q"]);
    uPo = uniforms(oPrg, ["hdr", "ir"]);
    hdrW = hdrH = 0; cKey = ""; fence = null;
    return true;
  };
  if (!buildGpu()) return null;
  canvas.addEventListener?.("webglcontextlost", (e) => { e.preventDefault?.(); contextLost = true; fence = null; });
  canvas.addEventListener?.("webglcontextrestored", () => { contextLost = false; buildGpu(); });
  const drawQuad = () => { gl.bindVertexArray(qVao); gl.drawArrays(gl.TRIANGLES, 0, 6); };
  const eHdr = (w, h) => {
    if (w === hdrW && h === hdrH) return true;
    hdrW = w; hdrH = h;
    gl.bindTexture(T, hTex); gl.texImage2D(T, 0, hHf ? gl.RGBA16F : gl.RGBA8, w, h, 0, gl.RGBA, hHf ? gl.HALF_FLOAT : gl.UNSIGNED_BYTE, null); setTexClamp(gl.LINEAR);
    gl.bindRenderbuffer(R, hDep); gl.renderbufferStorage(R, gl.DEPTH_COMPONENT24, w, h);
    gl.bindFramebuffer(F, hFbo); gl.framebufferTexture2D(F, gl.COLOR_ATTACHMENT0, T, hTex, 0); gl.framebufferRenderbuffer(F, gl.DEPTH_ATTACHMENT, R, hDep);
    const ok = gl.checkFramebufferStatus(F) === gl.FRAMEBUFFER_COMPLETE; gl.bindFramebuffer(F, null); return ok;
  };
  const lightOrtho = mat4Ortho(-34, 34, -34, 34, -48, 64);
  const sUp = (t, b, v, k) => {
    gl.bindBuffer(t, b);
    if (v.byteLength > dGpu[k]) { dGpu[k] = Math.max(v.byteLength * 2, 16384); gl.bufferData(t, dGpu[k], gl.STREAM_DRAW); }
    if (v.byteLength > 0) gl.bufferSubData(t, 0, v);
  };

  return {
    backend: "webgl2",
    isSoftware,
    setRenderScale(pm) { rScale = Math.max(0.65, Math.min(1, (pm || 1000) / 1000)); },
    getUploadTelemetry() { return { staticBytes: lSB, dynBytes: lDB, staticEpoch: sGpu.epoch, instCount: iGpu.instCount, contextLost }; },
    uploadSplitBuffers(sv, dv, iv = null) {
      if (contextLost || gl.isContextLost()) return;
      lSB = 0; lDB = 0;
      if (sv && sv.epoch !== sGpu.epoch) {
        [[B, sGpu.pos, sv.pos], [B, sGpu.norm, sv.norm], [B, sGpu.col, sv.col], [E, sGpu.idx, sv.idx]].forEach(([t, b, d]) => {
          gl.bindBuffer(t, b); gl.bufferData(t, d, gl.STATIC_DRAW); lSB += d.byteLength;
        });
        sGpu.count = sv.idx.length; sGpu.epoch = sv.epoch; cKey = "";
      }
      if (dv && dv.idx.length > 0) {
        sUp(B, dGpu.pos, dv.pos, "capPos");
        [[dGpu.norm, dv.norm], [dGpu.col, dv.col]].forEach(([b, v]) => {
          gl.bindBuffer(B, b); if (v.byteLength > dGpu.capPos) gl.bufferData(B, dGpu.capPos, gl.STREAM_DRAW); gl.bufferSubData(B, 0, v);
        });
        sUp(E, dGpu.idx, dv.idx, "capIdx");
        dGpu.count = dv.idx.length; lDB = dv.pos.byteLength * 3 + dv.idx.byteLength;
      } else dGpu.count = 0;
      if (iv && iv.instCount > 0) {
        if (iv.epoch !== iGpu.epoch) {
          [[B, iGpu.pos, iv.pos], [B, iGpu.norm, iv.norm], [B, iGpu.col, iv.col], [E, iGpu.idx, iv.idx], [B, iGpu.inst, iv.data]].forEach(([t, b, d]) => {
            gl.bindBuffer(t, b); gl.bufferData(t, d, gl.STATIC_DRAW);
          });
          iGpu.count = iv.idx.length; iGpu.instCount = iv.instCount; iGpu.epoch = iv.epoch;
        }
      } else iGpu.instCount = 0;
    },
    uploadPackedBuffers(posF32, normF32, colF32, idxU32) {
      if (contextLost || gl.isContextLost()) return;
      [[B, sGpu.pos, posF32], [B, sGpu.norm, normF32], [B, sGpu.col, colF32], [E, sGpu.idx, idxU32]].forEach(([t, b, d]) => {
        gl.bindBuffer(t, b); gl.bufferData(t, d, gl.STATIC_DRAW);
      });
      sGpu.count = idxU32.length; sGpu.epoch = (sGpu.epoch + 1) >>> 0; dGpu.count = 0; cKey = "";
    },
    render(mvp, eye, lodSpec, timeSec, lookTarget = [0, 1.8, 0], fovRad = Math.PI / 3.4) {
      const w = canvas.width | 0, h = canvas.height | 0;
      if (contextLost || w <= 0 || h <= 0 || gl.isContextLost()) return;
      if (fence) {
        if (gl.getSyncParameter(fence, gl.SYNC_STATUS) === gl.UNSIGNALED && isSoftware) return;
        gl.deleteSync(fence); fence = null;
      }
      const rank = lodSpec?.rank ?? 3, quality = isSoftware ? Math.min(1, rank) : (rank >= 2 ? 2 : 1);
      const sky = lodSpec?.sky ?? [0.18, 0.3, 0.38], fog = lodSpec?.fog ?? [0.3, 0.4, 0.44];
      const totalIdx = sGpu.count + dGpu.count + (iGpu.instCount > 0 ? iGpu.count : 0);
      const shOn = rank >= 1 && totalIdx > 0 ? 1 : 0;
      const fx = lookTarget[0] ?? 0, fy = lookTarget[1] ?? 1.5, fz = lookTarget[2] ?? 0;
      const sfx = Math.round(fx * 0.5) * 2, sfy = Math.round(fy * 0.5) * 2, sfz = Math.round(fz * 0.5) * 2;
      const lmvp = mat4Mul(lightOrtho, mat4LookAt([sfx + SUN_DIR[0] * 14, sfy + SUN_DIR[1] * 14, sfz + SUN_DIR[2] * 14], [sfx, sfy, sfz], [0, 1, 0]));

      if (shOn) {
        const key = `${sGpu.epoch}:${sfx},${sfy},${sfz}`;
        gl.colorMask(false, false, false, false); gl.enable(gl.DEPTH_TEST); gl.depthMask(true);
        gl.enable(gl.POLYGON_OFFSET_FILL); gl.polygonOffset(1.8, 3);
        gl.useProgram(sPrg); gl.uniformMatrix4fv(uSh.lmvp, false, lmvp);
        if (key !== cKey) {
          gl.bindFramebuffer(F, sShd.fbo); gl.viewport(0, 0, sSz, sSz); gl.clear(gl.DEPTH_BUFFER_BIT);
          if (sGpu.count > 0) { gl.uniform1f(uSh.inst, 0); gl.bindVertexArray(sGpu.vao); gl.drawElements(gl.TRIANGLES, sGpu.count, gl.UNSIGNED_INT, 0); }
          cKey = key;
        }
        gl.bindFramebuffer(gl.READ_FRAMEBUFFER, sShd.fbo); gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, shd.fbo);
        gl.blitFramebuffer(0, 0, sSz, sSz, 0, 0, sSz, sSz, gl.DEPTH_BUFFER_BIT, gl.NEAREST);
        gl.bindFramebuffer(F, shd.fbo); gl.viewport(0, 0, sSz, sSz);
        if (dGpu.count > 0) { gl.uniform1f(uSh.inst, 0); gl.bindVertexArray(dGpu.vao); gl.drawElements(gl.TRIANGLES, dGpu.count, gl.UNSIGNED_INT, 0); }
        if (iGpu.instCount > 0 && iGpu.count > 0 && !isSoftware) {
          gl.uniform1f(uSh.inst, 1); gl.bindVertexArray(iGpu.vao); gl.drawElementsInstanced(gl.TRIANGLES, iGpu.count, gl.UNSIGNED_INT, 0, iGpu.instCount);
        }
        gl.disable(gl.POLYGON_OFFSET_FILL); gl.colorMask(true, true, true, true);
      }

      const sc = isSoftware ? Math.min(rScale, 0.85) : rScale;
      const rw = Math.max(16, Math.round(w * sc)), rh = Math.max(16, Math.round(h * sc)), useHdr = eHdr(rw, rh);
      gl.bindFramebuffer(F, useHdr ? hFbo : null); gl.viewport(0, 0, useHdr ? rw : w, useHdr ? rh : h);
      gl.clearColor(sky[0], sky[1], sky[2], 1); gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

      let fdx = fx - eye[0], fdy = fy - eye[1], fdz = fz - eye[2];
      const fl = Math.hypot(fdx, fdy, fdz) || 1; fdx /= fl; fdy /= fl; fdz /= fl;
      const rl = Math.hypot(-fdz, fdx) || 1, rdx = -fdz / rl, rdz = fdx / rl;
      gl.disable(gl.DEPTH_TEST); gl.depthMask(false); gl.useProgram(kPrg);
      gl.uniform3f(uSk.fwd, fdx, fdy, fdz); gl.uniform3f(uSk.right, rdx, 0, rdz); gl.uniform3f(uSk.up, -rdz * fdy, rdz * fdx - rdx * fdz, rdx * fdy);
      gl.uniform3fv(uSk.sky, sky); gl.uniform3fv(uSk.fog, fog); gl.uniform3fv(uSk.sun, SUN_DIR);
      gl.uniform1f(uSk.aspect, w / h); gl.uniform1f(uSk.th, Math.tan(fovRad * 0.5)); drawQuad();

      gl.depthMask(true); gl.enable(gl.DEPTH_TEST);
      if (totalIdx > 0 && mvp) {
        gl.useProgram(pPrg);
        gl.uniformMatrix4fv(uPb.mvp, false, mvp); gl.uniformMatrix4fv(uPb.lmvp, false, lmvp);
        gl.uniform3fv(uPb.eye, eye); gl.uniform3fv(uPb.fog, fog); gl.uniform3fv(uPb.sky, sky); gl.uniform3fv(uPb.sun, SUN_DIR);
        gl.uniform1f(uPb.time, timeSec); gl.uniform1f(uPb.sh, shOn); gl.uniform1f(uPb.is, 1 / sSz); gl.uniform1f(uPb.q, quality);
        gl.activeTexture(gl.TEXTURE0); gl.bindTexture(T, shd.tex); gl.uniform1i(uPb.smap, 0);
        if (sGpu.count > 0) { gl.uniform1f(uPb.inst, 0); gl.bindVertexArray(sGpu.vao); gl.drawElements(gl.TRIANGLES, sGpu.count, gl.UNSIGNED_INT, 0); }
        if (dGpu.count > 0) { gl.uniform1f(uPb.inst, 0); gl.bindVertexArray(dGpu.vao); gl.drawElements(gl.TRIANGLES, dGpu.count, gl.UNSIGNED_INT, 0); }
        if (iGpu.instCount > 0 && iGpu.count > 0) {
          gl.uniform1f(uPb.inst, 1); gl.bindVertexArray(iGpu.vao); gl.drawElementsInstanced(gl.TRIANGLES, iGpu.count, gl.UNSIGNED_INT, 0, iGpu.instCount);
        }
      }
      if (useHdr) {
        gl.bindFramebuffer(F, null); gl.viewport(0, 0, w, h); gl.disable(gl.DEPTH_TEST); gl.useProgram(oPrg);
        gl.activeTexture(gl.TEXTURE0); gl.bindTexture(T, hTex); gl.uniform1i(uPo.hdr, 0); gl.uniform2f(uPo.ir, 1 / rw, 1 / rh); drawQuad();
      }
      gl.bindVertexArray(null);
      if (typeof gl.fenceSync === "function") { fence = gl.fenceSync(gl.SYNC_GPU_COMMANDS_COMPLETE, 0); gl.flush(); }
    },
    blitTensorRgba(rgbaBytes, width, height) {
      if (contextLost || !bPrg || !rgbaBytes || width <= 0 || height <= 0 || gl.isContextLost()) return;
      gl.bindFramebuffer(F, null); gl.viewport(0, 0, canvas.width, canvas.height); gl.disable(gl.DEPTH_TEST); gl.useProgram(bPrg);
      gl.activeTexture(gl.TEXTURE0); gl.bindTexture(T, bTex); gl.texImage2D(T, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, rgbaBytes);
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
