export function mat4Perspective(fv, as, n, f) {
  const g = 1 / Math.tan(fv * 0.5), nf = 1 / (n - f);
  return new Float32Array([g / as, 0, 0, 0, 0, g, 0, 0, 0, 0, (f + n) * nf, -1, 0, 0, 2 * f * n * nf, 0]);
}
export function mat4Ortho(l, r, b, t, n, f) {
  const rl = 1 / (r - l), tb = 1 / (t - b), fn = 1 / (f - n);
  return new Float32Array([2 * rl, 0, 0, 0, 0, 2 * tb, 0, 0, 0, 0, -2 * fn, 0, -(r + l) * rl, -(t + b) * tb, -(f + n) * fn, 1]);
}
export function mat4LookAt(e, t, u) {
  let zx = e[0] - t[0], zy = e[1] - t[1], zz = e[2] - t[2], zl = Math.hypot(zx, zy, zz) || 1;
  zx /= zl; zy /= zl; zz /= zl;
  let xx = u[1] * zz - u[2] * zy, xy = u[2] * zx - u[0] * zz, xz = u[0] * zy - u[1] * zx, xl = Math.hypot(xx, xy, xz) || 1;
  xx /= xl; xy /= xl; xz /= xl;
  const yx = zy * xz - zz * xy, yy = zz * xx - zx * xz, yz = zx * xy - zy * xx;
  return new Float32Array([xx, yx, zx, 0, xy, yy, zy, 0, xz, yz, zz, 0, -(xx * e[0] + xy * e[1] + xz * e[2]), -(yx * e[0] + yy * e[1] + yz * e[2]), -(zx * e[0] + zy * e[1] + zz * e[2]), 1]);
}
export function mat4Mul(a, b) {
  const o = new Float32Array(16);
  for (let c = 0; c < 4; c++) for (let r = 0; r < 4; r++) o[c * 4 + r] = a[r] * b[c * 4] + a[4 + r] * b[c * 4 + 1] + a[8 + r] * b[c * 4 + 2] + a[12 + r] * b[c * 4 + 3];
  return o;
}
const SUN_DIR = (() => { const x = 0.54, y = 0.78, z = 0.31, l = Math.hypot(x, y, z); return new Float32Array([x / l, y / l, z / l]); })();

export async function createGlRenderer(canvas) {
  const gl = canvas.getContext("webgl2", { antialias: false, depth: true, alpha: false, preserveDrawingBuffer: true, powerPreference: "high-performance" });
  if (!gl) return null;
  const dbg = gl.getExtension("WEBGL_debug_renderer_info");
  const isSoftware = /SwiftShader|llvmpipe|Software|Basic Render/i.test(String((dbg ? gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER)) || ""));
  const hHf = Boolean(gl.getExtension("EXT_color_buffer_half_float"));
  const anisoExt = gl.getExtension("EXT_texture_filter_anisotropic") || gl.getExtension("WEBKIT_EXT_texture_filter_anisotropic");
  const maxAniso = anisoExt ? Math.min(8, gl.getParameter(anisoExt.MAX_TEXTURE_MAX_ANISOTROPY_EXT) || 1) : 1;
  const maxTex = gl.getParameter(gl.MAX_TEXTURE_SIZE) || 2048;
  const sSz = isSoftware ? 1024 : (maxTex >= 8192 ? 2048 : 2048);

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
  const okGl = () => !ctxLost && !gl.isContextLost();
  const tClamp = (f) => {
    gl.texParameteri(T, gl.TEXTURE_MIN_FILTER, f); gl.texParameteri(T, gl.TEXTURE_MAG_FILTER, f);
    gl.texParameteri(T, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE); gl.texParameteri(T, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  };
  const uLocs = (prog, names) => Object.fromEntries(names.map((n) => [n, gl.getUniformLocation(prog, `u_${n}`)]));

  const iDcl = `layout(location=3) in vec4 i_c0;layout(location=4) in vec4 i_c1;layout(location=5) in vec4 i_c2;layout(location=6) in vec4 i_c3;uniform float u_inst;`;
  const iPos = `vec3 wp=u_inst>0.5?(i_c0.xyz*a_pos.x+i_c1.xyz*a_pos.y+i_c2.xyz*a_pos.z+i_c3.xyz):a_pos;`;
  const B = gl.ARRAY_BUFFER, E = gl.ELEMENT_ARRAY_BUFFER, T = gl.TEXTURE_2D, F = gl.FRAMEBUFFER, R = gl.RENDERBUFFER;
  let sPrg, kPrg, pPrg, oPrg, bPrg, sGpu, dGpu, iGpu, qVao, sShd, shd, hTex, hDep, hFbo, bTex, aTex, mrTex, nmTex, emTex, eTex, uSh, uSk, uPb, uPo;
  let ctxLost = false, cKey = "", hdrW = 0, hdrH = 0, lastDrawnW = 0, lastDrawnH = 0, rScale = 1, lSB = 0, lDB = 0, fence = null, aEp = 0, hTx = 0, hMr = 0, hNm = 0, hEm = 0, hEv = 0;
  const bPrgAll = () => {
    sPrg = link(
      `#version 300 es\nlayout(location=0) in vec3 a_pos;${iDcl}uniform mat4 u_lmvp;void main(){${iPos}gl_Position=u_lmvp*vec4(wp,1.0);}`,
      `#version 300 es\nprecision mediump float;void main(){}`,
    );
    kPrg = link(
      `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 vn;void main(){vn=a_uv*2.0-1.0;gl_Position=vec4(vn,0.9999,1.0);}`,
      `#version 300 es\nprecision highp float;in vec2 vn;uniform vec3 u_fwd,u_right,u_up,u_sky,u_fog,u_sun;uniform float u_aspect,u_th,u_he;uniform sampler2D u_ev;out vec4 o_hdr;void main(){vec3 rd=normalize(u_fwd+vn.x*u_aspect*u_th*u_right+vn.y*u_th*u_up),c;if(u_he>0.5){c=pow(texture(u_ev,vec2(atan(rd.z,rd.x)*0.159155+0.5,0.5-asin(clamp(rd.y,-1.0,1.0))*0.31831)).rgb,vec3(2.2))*1.15;}else{float h=clamp(rd.y,-0.25,1.0);vec3 z=pow(max(u_sky,vec3(0.03)),vec3(1.5))*vec3(0.85,1.05,1.38)+vec3(0.015,0.04,0.09),hz=pow(max(u_fog,vec3(0.04)),vec3(1.5))*vec3(1.22,1.18,1.12)+vec3(0.05,0.055,0.05);c=mix(hz,z,pow(clamp(h,0.0,1.0),0.52));if(h<0.0)c=mix(hz,z*0.35,clamp(-h*3.2,0.0,1.0));float mu=max(dot(rd,u_sun),0.0);c+=vec3(1.05,0.8,0.46)*pow(mu,9.0)*0.32+vec3(2.2,1.9,1.45)*smoothstep(0.9984,0.9997,mu);}o_hdr=vec4(max(c,vec3(0.0)),1.0);}`,
    );
    pPrg = link(
      `#version 300 es\nlayout(location=0) in vec3 a_pos;layout(location=1) in vec3 a_norm;layout(location=2) in vec3 a_col;layout(location=7) in vec2 a_uv;${iDcl}uniform mat4 u_mvp,u_lmvp;out vec3 v_pos,v_norm,v_col;out vec2 v_uv;flat out float v_m;out vec4 v_lp;void main(){${iPos}vec3 wn=u_inst>0.5?normalize(i_c0.xyz*a_norm.x+i_c1.xyz*a_norm.y+i_c2.xyz*a_norm.z):a_norm;v_m=floor(a_col.r);vec3 ac=vec3(a_col.r-v_m,a_col.gb);v_pos=wp;v_norm=wn;v_uv=a_uv;v_col=u_inst>0.5?ac*vec3(i_c0.w,i_c1.w,i_c2.w):ac;v_lp=u_lmvp*vec4(wp+wn*0.04,1.0);gl_Position=u_mvp*vec4(wp,1.0);}`,
      `#version 300 es\nprecision highp float;precision highp sampler2DShadow;in vec3 v_pos,v_norm,v_col;in vec2 v_uv;flat in float v_m;in vec4 v_lp;uniform vec3 u_eye,u_fog,u_sky,u_sun;uniform float u_time,u_sh,u_is,u_q,u_ht,u_hm,u_hn,u_hm2,u_he;uniform sampler2DShadow u_smap;uniform sampler2D u_at,u_mr,u_nm,u_em,u_ev;out vec4 o_hdr;float h3(vec3 p){p=fract(p*vec3(0.1031,0.103,0.0973));p+=dot(p,p.yxz+33.33);return fract((p.x+p.y)*p.z);}float tG(vec3 p){vec3 i=floor(p),f=fract(p);f=f*f*(3.0-2.0*f);return mix(mix(h3(i),h3(i+vec3(1,0,0)),f.x),mix(h3(i+vec3(0,0,1)),h3(i+vec3(1,0,1)),f.x),f.z);}float sS(vec4 lp,float ndl){if(u_sh<0.5)return 1.0;vec3 p=lp.xyz/max(lp.w,1e-4)*0.5+0.5;vec2 e=smoothstep(vec2(0.0),vec2(0.07),p.xy)*(1.0-smoothstep(vec2(0.93),vec2(1.0),p.xy));float m=e.x*e.y*step(0.0,p.z)*step(p.z,1.0);if(m<=0.001)return 1.0;float b=max(0.0016*(1.0-ndl),0.00045),a=0.0;if(u_q<1.5)return mix(1.0,texture(u_smap,vec3(p.xy,p.z-b)),m);for(int y=-1;y<=1;y++)for(int x=-1;x<=1;x++)a+=texture(u_smap,vec3(p.xy+vec2(float(x),float(y))*u_is,p.z-b));return mix(1.0,a/9.0,m);}vec3 pertN(vec3 N,vec3 p,vec2 uv){vec3 dp1=dFdx(p),dp2=dFdy(p);vec2 duv1=dFdx(uv),duv2=dFdy(uv);vec3 dp2p=cross(dp2,N),dp1p=cross(N,dp1);vec3 T=dp2p*duv1.x+dp1p*duv2.x,B=dp2p*duv1.y+dp1p*duv2.y;float im=inversesqrt(max(dot(T,T),dot(B,B))+1e-5);vec3 mn=texture(u_nm,uv).xyz*2.0-1.0;return normalize(mat3(T*im,B*im,N)*mn);}void main(){vec3 te=u_eye-v_pos;float d=length(te);vec3 n=normalize(v_norm),v=te/max(d,1e-4),l=u_sun;bool iW=v_m==1.0,iM=v_m==2.0,iV=v_m==3.0,tx=u_ht>0.5&&v_uv.x>-0.5;if(tx&&u_hn>0.5){n=pertN(n,v_pos,v_uv);}else{float fd=(iW||tx)?0.0:step(1.5,u_q)*(1.0-smoothstep(22.0,30.0,d)),g1=mix(0.5,tG(v_pos*5.2),fd),g2=mix(0.5,tG(v_pos*18.5),fd),mc=(g1-0.5)*0.055+(g2-0.5)*0.028;vec3 c1=cross(dFdx(v_pos),n),c2=cross(n,dFdy(v_pos));n=normalize(n+((c1/max(length(c1),1e-4))*dFdx(mc)+(c2/max(length(c2),1e-4))*dFdy(mc))*(iM?0.55:1.15));}if(iW){float w1=sin(v_pos.x*3.1+u_time*2.3)*cos(v_pos.z*2.7-u_time*1.8),w2=sin((v_pos.x+v_pos.z)*5.5-u_time*3.0)*0.5;n=normalize(n+vec3((w1+w2)*0.042,0.0,(w1-w2)*0.042));}vec3 ab=tx?pow(max(texture(u_at,v_uv).rgb,vec3(0.02)),vec3(2.2)):pow(max(v_col,vec3(0.02)),vec3(2.0));float fd0=(iW||tx)?0.0:step(1.5,u_q)*(1.0-smoothstep(22.0,30.0,d)),g1a=mix(0.5,tG(v_pos*5.2),fd0),g2a=mix(0.5,tG(v_pos*18.5),fd0);if(!iW&&!iV&&!tx)ab*=0.93+0.14*g1a;float mt=iW?0.04:(iM?0.72:0.05),rg=iW?0.07:(iM?0.28:clamp(0.66-0.15*g2a,0.24,0.88));if(tx&&u_hm>0.5){vec3 mrs=texture(u_mr,v_uv).rgb;rg=clamp(mrs.g,0.08,0.98);mt=clamp(mrs.b,0.0,1.0);}float ndl=max(dot(n,l),0.0),ndv=max(dot(n,v),1e-3);vec3 h=normalize(l+v);float ndh=max(dot(n,h),0.0),vdh=max(dot(v,h),0.0),sw=sS(v_lp,ndl),a=rg*rg,a2=a*a,dn=ndh*ndh*(a2-1.0)+1.0,dg=a2/max(3.14159265*dn*dn,1e-4),k=(rg+1.0)*(rg+1.0)*0.125,gs=(ndv/(ndv*(1.0-k)+k))*(ndl/(ndl*(1.0-k)+k));vec3 f0=mix(vec3(0.04),ab,mt),fr=f0+(1.0-f0)*pow(1.0-vdh,5.0),sp=(dg*gs*fr)/max(4.0*ndv*ndl,1e-3),kd=(1.0-fr)*(1.0-mt),sr=vec3(2.65,2.42,2.02)*sw,dr=(kd*ab/3.14159265+sp)*sr*ndl;float ao=u_q>=1.5?clamp(1.0-length(fwidth(n))*0.38,0.56,1.0):0.92;vec3 sl=pow(max(u_sky,vec3(0.04)),vec3(1.5))*1.35+vec3(0.08,0.12,0.18),fl=pow(max(u_fog,vec3(0.04)),vec3(1.5))*1.15+vec3(0.05),hm=mix(vec3(0.09,0.08,0.07),sl,0.5+0.5*n.y),rD=reflect(-v,n);vec3 envRef=u_he>0.5?pow(texture(u_ev,vec2(atan(rD.z,rD.x)*0.159155+0.5,0.5-asin(clamp(rD.y,-1.0,1.0))*0.31831)).rgb,vec3(2.2))*1.15:mix(fl,sl,clamp(rD.y*0.5+0.5,0.0,1.0));vec3 ib=envRef*(1.0-rg*0.72),fE=f0+(max(vec3(1.0-rg),f0)-f0)*pow(1.0-ndv,5.0),lt=dr+(kd*ab*hm+ib*fE)*ao;if(tx&&u_hm2>0.5)lt+=pow(texture(u_em,v_uv).rgb,vec3(2.2))*1.6;if(iV)lt+=ab*0.38;if(iW)lt=mix(ab*(hm+0.4*sw),sl+sr*pow(ndh,96.0),clamp(0.18+0.78*pow(1.0-ndv,4.0),0.0,0.95));o_hdr=vec4(max(mix(lt,fl,tx?0.0:clamp(1.0-exp(-d*0.0115),0.0,0.74)),vec3(0.0)),1.0);}`,
    );
    oPrg = link(
      `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 v_uv;void main(){v_uv=a_uv;gl_Position=vec4(a_uv*2.0-1.0,0.0,1.0);}`,
      `#version 300 es\nprecision highp float;in vec2 v_uv;uniform sampler2D u_hdr;uniform vec2 u_ir;uniform float u_cas;out vec4 o_col;vec3 ac(vec3 x){return clamp((x*(2.51*x+0.03))/(x*(2.43*x+0.59)+0.14),0.0,1.0);}void main(){vec3 c=texture(u_hdr,v_uv).rgb,n=texture(u_hdr,v_uv+vec2(0.0,u_ir.y)).rgb,s=texture(u_hdr,v_uv-vec2(0.0,u_ir.y)).rgb,e=texture(u_hdr,v_uv+vec2(u_ir.x,0.0)).rgb,w=texture(u_hdr,v_uv-vec2(u_ir.x,0.0)).rgb,av=(n+s+e+w)*0.25;vec3 mn=min(c,min(min(n,s),min(e,w))),mx=max(c,max(max(n,s),max(e,w)));vec3 sharp=clamp(c+(c-av)*u_cas,mn,mx);vec3 res=u_cas>0.01?sharp:mix(c,(c+av)*0.5,clamp(length(c-av)*2.2,0.0,0.45));o_col=vec4(pow(ac(res*1.06),vec3(1.0/2.2)),1.0);}`,
    );
    bPrg = link(
      `#version 300 es\nlayout(location=0) in vec2 a_uv;out vec2 v_uv;void main(){v_uv=vec2(a_uv.x,1.0-a_uv.y);gl_Position=vec4(a_uv*2.0-1.0,0.0,1.0);}`,
      `#version 300 es\nprecision mediump float;in vec2 v_uv;uniform sampler2D u_tex;out vec4 o_col;void main(){o_col=texture(u_tex,v_uv);}`,
    );
    return Boolean(sPrg && kPrg && pPrg && oPrg);
  };

  const mkVao = (instanced = false) => {
    const vao = gl.createVertexArray(), pos = gl.createBuffer(), norm = gl.createBuffer(), col = gl.createBuffer(), uv = gl.createBuffer(), idx = gl.createBuffer();
    const inst = instanced ? gl.createBuffer() : null;
    gl.bindVertexArray(vao);
    [pos, norm, col].forEach((b, loc) => { gl.bindBuffer(B, b); gl.enableVertexAttribArray(loc); gl.vertexAttribPointer(loc, 3, gl.FLOAT, false, 0, 0); });
    gl.vertexAttrib2f(7, -2, -2);
    if (inst) {
      gl.bindBuffer(B, inst);
      for (let k = 0; k < 4; k++) { gl.enableVertexAttribArray(3 + k); gl.vertexAttribPointer(3 + k, 4, gl.FLOAT, false, 64, k * 16); gl.vertexAttribDivisor(3 + k, 1); }
    }
    gl.bindBuffer(E, idx); gl.bindVertexArray(null);
    return { vao, pos, norm, col, uv, idx, inst, count: 0, instCount: 0, capPos: 0, capNorm: 0, capCol: 0, capIdx: 0, epoch: 0 };
  };
  const mkShd = () => {
    const tex = gl.createTexture(), fbo = gl.createFramebuffer();
    gl.bindTexture(T, tex); gl.texImage2D(T, 0, gl.DEPTH_COMPONENT24, sSz, sSz, 0, gl.DEPTH_COMPONENT, gl.UNSIGNED_INT, null);
    tClamp(gl.LINEAR); gl.texParameteri(T, gl.TEXTURE_COMPARE_MODE, gl.COMPARE_REF_TO_TEXTURE); gl.texParameteri(T, gl.TEXTURE_COMPARE_FUNC, gl.LEQUAL);
    gl.bindFramebuffer(F, fbo); gl.framebufferTexture2D(F, gl.DEPTH_ATTACHMENT, T, tex, 0); gl.drawBuffers([gl.NONE]); gl.readBuffer(gl.NONE);
    return { tex, fbo };
  };
  const mk1 = (c, f = gl.LINEAR) => { const t = gl.createTexture(); gl.bindTexture(T, t); tClamp(f); gl.texImage2D(T, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array(c)); return t; };
  const bGpu = () => {
    gl.getExtension("EXT_color_buffer_half_float");
    if (!bPrgAll()) return false;
    sGpu = mkVao(false); dGpu = mkVao(false); iGpu = mkVao(true);
    qVao = gl.createVertexArray(); const qBuf = gl.createBuffer();
    gl.bindVertexArray(qVao); gl.bindBuffer(B, qBuf);
    gl.bufferData(B, new Float32Array([0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1]), gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0); gl.bindVertexArray(null);
    sShd = mkShd(); shd = mkShd();
    hTex = gl.createTexture(); hDep = gl.createRenderbuffer(); hFbo = gl.createFramebuffer();
    bTex = mk1([0, 0, 0, 255], gl.NEAREST);
    aTex = mk1([255, 255, 255, 255]);
    mrTex = mk1([255, 160, 0, 255]);
    nmTex = mk1([128, 128, 255, 255]);
    emTex = mk1([0, 0, 0, 255]);
    eTex = mk1([20, 24, 32, 255]);
    gl.bindFramebuffer(F, null);
    uSh = uLocs(sPrg, ["lmvp", "inst"]);
    uSk = uLocs(kPrg, ["fwd", "right", "up", "sky", "fog", "sun", "aspect", "th", "he", "ev"]);
    uPb = uLocs(pPrg, ["mvp", "lmvp", "eye", "fog", "sky", "sun", "time", "sh", "is", "smap", "inst", "q", "ht", "hm", "hn", "hm2", "he", "at", "mr", "nm", "em", "ev"]);
    uPo = uLocs(oPrg, ["hdr", "ir", "cas"]);
    hdrW = hdrH = lastDrawnW = lastDrawnH = 0; cKey = ""; fence = null; aEp = 0; hTx = hMr = hNm = hEm = hEv = 0;
    return true;
  };
  if (!bGpu()) return null;
  canvas.addEventListener?.("webglcontextlost", (e) => { e.preventDefault?.(); ctxLost = true; fence = null; });
  canvas.addEventListener?.("webglcontextrestored", () => { ctxLost = false; bGpu(); });
  const dQuad = () => { gl.bindVertexArray(qVao); gl.drawArrays(gl.TRIANGLES, 0, 6); };
  const eHdr = (w, h) => {
    if (w === hdrW && h === hdrH) return true;
    hdrW = w; hdrH = h;
    gl.bindTexture(T, hTex); gl.texImage2D(T, 0, hHf ? gl.RGBA16F : gl.RGBA8, w, h, 0, gl.RGBA, hHf ? gl.HALF_FLOAT : gl.UNSIGNED_BYTE, null); tClamp(gl.LINEAR);
    gl.bindRenderbuffer(R, hDep); gl.renderbufferStorage(R, gl.DEPTH_COMPONENT24, w, h);
    gl.bindFramebuffer(F, hFbo); gl.framebufferTexture2D(F, gl.COLOR_ATTACHMENT0, T, hTex, 0); gl.framebufferRenderbuffer(F, gl.DEPTH_ATTACHMENT, R, hDep);
    const ok = gl.checkFramebufferStatus(F) === gl.FRAMEBUFFER_COMPLETE; gl.bindFramebuffer(F, null); return ok;
  };
  const lUp = [0, 1, 0];
  let lRx = lUp[1] * SUN_DIR[2] - lUp[2] * SUN_DIR[1], lRy = lUp[2] * SUN_DIR[0] - lUp[0] * SUN_DIR[2], lRz = lUp[0] * SUN_DIR[1] - lUp[1] * SUN_DIR[0];
  const lRl = Math.hypot(lRx, lRy, lRz) || 1; lRx /= lRl; lRy /= lRl; lRz /= lRl;
  const lUx = SUN_DIR[1] * lRz - SUN_DIR[2] * lRy, lUy = SUN_DIR[2] * lRx - SUN_DIR[0] * lRz, lUz = SUN_DIR[0] * lRy - SUN_DIR[1] * lRx;
  let csmFar = 28.2;
  const sUp = (t, b, v, k) => {
    gl.bindBuffer(t, b);
    if (v.byteLength > dGpu[k]) { dGpu[k] = Math.max(v.byteLength * 2, 16384); gl.bufferData(t, dGpu[k], gl.STREAM_DRAW); }
    if (v.byteLength > 0) gl.bufferSubData(t, 0, v);
  };

  const upPbrTex = (tex, s, w, h) => {
    gl.bindTexture(T, tex); gl.texImage2D(T, 0, gl.RGBA, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, s);
    gl.texParameteri(T, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
    gl.texParameteri(T, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(T, gl.TEXTURE_WRAP_S, gl.REPEAT); gl.texParameteri(T, gl.TEXTURE_WRAP_T, gl.REPEAT);
    if (anisoExt && !isSoftware && maxAniso > 1) gl.texParameterf(T, anisoExt.TEXTURE_MAX_ANISOTROPY_EXT, maxAniso);
    gl.generateMipmap(T);
  };
  const uBs = (arr, sum) => { for (const [t, b, d] of arr) { gl.bindBuffer(t, b); gl.bufferData(t, d, gl.STATIC_DRAW); if (sum) lSB += d.byteLength; } };
  return {
    backend: "webgl2", isSoftware, maxTextureSize: maxTex, maxAnisotropy: isSoftware ? 1 : maxAniso,
    setRenderScale(pm) { rScale = Math.max(0.65, Math.min(1, (pm || 1000) / 1000)); },
    setCsmSplits(s) { const s1 = Number(s?.[1]); if (Number.isFinite(s1) && s1 > 8) { csmFar = Math.max(22, Math.min(48, s1)); cKey = ""; } },
    getUploadTelemetry() { return { staticBytes: lSB, dynBytes: lDB, staticEpoch: sGpu.epoch, instCount: iGpu.instCount, ctxLost, maxTex, maxAniso: isSoftware ? 1 : maxAniso }; },
    setTex(k, s, w, h, ep) {
      if (!okGl()) return;
      if (k === 2) {
        if (!s) { hEv = 0; return; }
        gl.bindTexture(T, eTex); gl.texImage2D(T, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, s); tClamp(gl.LINEAR); hEv = 1;
        return;
      }
      if (k === 3) {
        if (!s || w <= 0 || h <= 0) { hMr = 0; return; }
        upPbrTex(mrTex, s, w, h); hMr = 1; return;
      }
      if (k === 4) {
        if (!s || w <= 0 || h <= 0) { hNm = 0; return; }
        upPbrTex(nmTex, s, w, h); hNm = 1; return;
      }
      if (k === 5) {
        if (!s || w <= 0 || h <= 0) { hEm = 0; return; }
        upPbrTex(emTex, s, w, h); hEm = 1; return;
      }
      if (!s || w <= 0 || h <= 0) { hTx = hMr = hNm = hEm = 0; return; }
      if (ep && ep === aEp && hTx) return;
      upPbrTex(aTex, s, w, h);
      aEp = ep; hTx = 1;
    },
    uploadSplitBuffers(sv, dv, iv = null) {
      if (!okGl()) return;
      lSB = lDB = 0;
      if (sv && sv.epoch !== sGpu.epoch) {
        uBs([[B, sGpu.pos, sv.pos], [B, sGpu.norm, sv.norm], [B, sGpu.col, sv.col], [E, sGpu.idx, sv.idx]], 1);
        gl.bindVertexArray(sGpu.vao);
        if (sv.uv && sv.uv.length >= (sv.pos.length / 3) * 2) {
          gl.bindBuffer(B, sGpu.uv); gl.bufferData(B, sv.uv, gl.STATIC_DRAW); gl.enableVertexAttribArray(7); gl.vertexAttribPointer(7, 2, gl.FLOAT, false, 0, 0);
        } else { gl.disableVertexAttribArray(7); gl.vertexAttrib2f(7, -2, -2); }
        gl.bindVertexArray(null); sGpu.count = sv.idx.length; sGpu.epoch = sv.epoch; cKey = "";
      }
      if (dv && dv.idx.length > 0) {
        sUp(B, dGpu.pos, dv.pos, "capPos"); sUp(B, dGpu.norm, dv.norm, "capNorm"); sUp(B, dGpu.col, dv.col, "capCol"); sUp(E, dGpu.idx, dv.idx, "capIdx");
        dGpu.count = dv.idx.length; lDB = dv.pos.byteLength * 3 + dv.idx.byteLength;
      } else dGpu.count = 0;
      if (iv && iv.instCount > 0) {
        if (iv.epoch !== iGpu.epoch) {
          uBs([[B, iGpu.pos, iv.pos], [B, iGpu.norm, iv.norm], [B, iGpu.col, iv.col], [E, iGpu.idx, iv.idx], [B, iGpu.inst, iv.data]]);
          iGpu.count = iv.idx.length; iGpu.instCount = iv.instCount; iGpu.epoch = iv.epoch;
        }
      } else iGpu.instCount = 0;
    },
    uploadPackedBuffers(p, n, c, ix) {
      if (!okGl()) return;
      uBs([[B, sGpu.pos, p], [B, sGpu.norm, n], [B, sGpu.col, c], [E, sGpu.idx, ix]]);
      sGpu.count = ix.length; sGpu.epoch = (sGpu.epoch + 1) >>> 0; dGpu.count = 0; cKey = "";
    },
    updateStaticIndices(ix) {
      if (!okGl() || !ix || ix.length === 0) return;
      gl.bindBuffer(E, sGpu.idx);
      gl.bufferData(E, ix, gl.DYNAMIC_DRAW);
      sGpu.count = ix.length;
    },
    render(mvp, eye, lodSpec, timeSec, lTgt = [0, 1.8, 0], fovRad = Math.PI / 3.4, forceSync = false) {
      const w = canvas.width | 0, h = canvas.height | 0;
      if (ctxLost || w <= 0 || h <= 0 || gl.isContextLost()) return;
      const resized = w !== lastDrawnW || h !== lastDrawnH;
      if (fence) {
        if (!forceSync && !resized && gl.getSyncParameter(fence, gl.SYNC_STATUS) === gl.UNSIGNALED && isSoftware) return;
        gl.deleteSync(fence); fence = null;
      }
      lastDrawnW = w; lastDrawnH = h;
      const rank = lodSpec?.rank ?? 3, quality = isSoftware ? Math.min(1, rank) : (rank >= 2 ? 2 : 1);
      const sky = lodSpec?.sky ?? [0.18, 0.3, 0.38], fog = lodSpec?.fog ?? [0.3, 0.4, 0.44];
      const tIdx = sGpu.count + dGpu.count + (iGpu.instCount > 0 ? iGpu.count : 0);
      const shOn = rank >= 1 && tIdx > 0 ? 1 : 0;
      const fx = lTgt[0] ?? 0, fy = lTgt[1] ?? 1.5, fz = lTgt[2] ?? 0;
      const texel = (2 * csmFar) / sSz;
      const tu = Math.round((fx * lRx + fy * lRy + fz * lRz) / texel);
      const tv = Math.round((fx * lUx + fy * lUy + fz * lUz) / texel);
      const tw = Math.round((fx * SUN_DIR[0] + fy * SUN_DIR[1] + fz * SUN_DIR[2]) * 0.5) * 2;
      const su = tu * texel, sv = tv * texel;
      const sfx = su * lRx + sv * lUx + tw * SUN_DIR[0];
      const sfy = su * lRy + sv * lUy + tw * SUN_DIR[1];
      const sfz = su * lRz + sv * lUz + tw * SUN_DIR[2];
      const lOrt = mat4Ortho(-csmFar, csmFar, -csmFar, csmFar, -48, 64);
      const lmvp = mat4Mul(lOrt, mat4LookAt([sfx + SUN_DIR[0] * 14, sfy + SUN_DIR[1] * 14, sfz + SUN_DIR[2] * 14], [sfx, sfy, sfz], [0, 1, 0]));

      const bU = (i, t, u) => { gl.activeTexture(gl.TEXTURE0 + i); gl.bindTexture(T, t); gl.uniform1i(u, i); };
      const dE = (u, g, inst = 0) => {
        if (!g.count || (inst && !g.instCount)) return;
        gl.uniform1f(u, inst); gl.bindVertexArray(g.vao);
        inst ? gl.drawElementsInstanced(gl.TRIANGLES, g.count, gl.UNSIGNED_INT, 0, g.instCount) : gl.drawElements(gl.TRIANGLES, g.count, gl.UNSIGNED_INT, 0);
      };
      if (shOn) {
        const key = `${sGpu.epoch}:${tu},${tv},${tw}:${csmFar}`;
        gl.colorMask(false, false, false, false); gl.enable(gl.DEPTH_TEST); gl.depthMask(true);
        gl.enable(gl.POLYGON_OFFSET_FILL); gl.polygonOffset(1.8, 3);
        gl.useProgram(sPrg); gl.uniformMatrix4fv(uSh.lmvp, false, lmvp);
        if (key !== cKey) {
          gl.bindFramebuffer(F, sShd.fbo); gl.viewport(0, 0, sSz, sSz); gl.clear(gl.DEPTH_BUFFER_BIT);
          dE(uSh.inst, sGpu); cKey = key;
        }
        gl.bindFramebuffer(gl.READ_FRAMEBUFFER, sShd.fbo); gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, shd.fbo);
        gl.blitFramebuffer(0, 0, sSz, sSz, 0, 0, sSz, sSz, gl.DEPTH_BUFFER_BIT, gl.NEAREST);
        gl.bindFramebuffer(F, shd.fbo); gl.viewport(0, 0, sSz, sSz);
        dE(uSh.inst, dGpu); if (!isSoftware) dE(uSh.inst, iGpu, 1);
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
      gl.uniform1f(uSk.aspect, w / h); gl.uniform1f(uSk.th, Math.tan(fovRad * 0.5)); gl.uniform1f(uSk.he, hEv);
      bU(2, eTex, uSk.ev); dQuad();

      gl.depthMask(true); gl.enable(gl.DEPTH_TEST);
      if (tIdx > 0 && mvp) {
        gl.useProgram(pPrg);
        gl.uniformMatrix4fv(uPb.mvp, false, mvp); gl.uniformMatrix4fv(uPb.lmvp, false, lmvp);
        gl.uniform3fv(uPb.eye, eye); gl.uniform3fv(uPb.fog, fog); gl.uniform3fv(uPb.sky, sky); gl.uniform3fv(uPb.sun, SUN_DIR);
        gl.uniform1f(uPb.time, timeSec); gl.uniform1f(uPb.sh, shOn); gl.uniform1f(uPb.is, 1 / sSz); gl.uniform1f(uPb.q, quality);
        gl.uniform1f(uPb.ht, hTx); gl.uniform1f(uPb.hm, hMr); gl.uniform1f(uPb.hn, hNm); gl.uniform1f(uPb.hm2, hEm); gl.uniform1f(uPb.he, hEv);
        bU(0, shd.tex, uPb.smap); bU(1, aTex, uPb.at); bU(2, eTex, uPb.ev);
        bU(3, mrTex, uPb.mr); bU(4, nmTex, uPb.nm); bU(5, emTex, uPb.em);
        dE(uPb.inst, sGpu); dE(uPb.inst, dGpu); dE(uPb.inst, iGpu, 1);
      }
      if (useHdr) {
        gl.bindFramebuffer(F, null); gl.viewport(0, 0, w, h); gl.disable(gl.DEPTH_TEST); gl.useProgram(oPrg);
        bU(0, hTex, uPo.hdr); gl.uniform2f(uPo.ir, 1 / rw, 1 / rh);
        gl.uniform1f(uPo.cas, sc < 0.98 ? 0.42 : 0.14); dQuad();
      }
      gl.bindVertexArray(null);
      if (typeof gl.fenceSync === "function") { fence = gl.fenceSync(gl.SYNC_GPU_COMMANDS_COMPLETE, 0); gl.flush(); }
    },
    blitTensorRgba(rb, w, h) {
      if (!okGl() || !bPrg || !rb || w <= 0 || h <= 0) return;
      gl.bindFramebuffer(F, null); gl.viewport(0, 0, canvas.width, canvas.height); gl.disable(gl.DEPTH_TEST); gl.useProgram(bPrg);
      gl.activeTexture(gl.TEXTURE0); gl.bindTexture(T, bTex); gl.texImage2D(T, 0, gl.RGBA, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, rb);
      dQuad(); gl.bindVertexArray(null);
    },
  };
}
export function createCanvas2dTensorRenderer(cv) {
  const cx = cv.getContext("2d");
  if (!cx) return null;
  const noop = () => {};
  return {
    backend: "tensor-2d", isSoftware: true, setRenderScale: noop, setCsmSplits: noop, uploadSplitBuffers: noop, uploadPackedBuffers: noop,
    getUploadTelemetry() { return { staticBytes: 0, dynBytes: 0, staticEpoch: 0, instCount: 0 }; },
    render(_m, _e, ls) {
      if (cv.width <= 0 || cv.height <= 0) return;
      const [r = 0.18, g = 0.3, b = 0.38] = ls?.sky ?? [];
      cx.fillStyle = `rgb(${Math.round(r * 255)},${Math.round(g * 255)},${Math.round(b * 255)})`;
      cx.fillRect(0, 0, cv.width, cv.height);
    },
    blitTensorRgba(rb, w, h) {
      if (rb && w > 0 && h > 0 && typeof ImageData === "function") cx.putImageData(new ImageData(new Uint8ClampedArray(rb.slice().buffer), w, h), 0, 0);
    },
  };
}
export async function createBestRenderer(cv) { return (await createGlRenderer(cv)) || createCanvas2dTensorRenderer(cv); }
