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
    -(xx * eye[0] + xy * eye[1] + xz * eye[2]),
    -(yx * eye[0] + yy * eye[1] + yz * eye[2]),
    -(zx * eye[0] + zy * eye[1] + zz * eye[2]), 1,
  ]);
}

export function mat4Mul(a, b) {
  const out = new Float32Array(16);
  for (let c = 0; c < 4; c++) {
    for (let r = 0; r < 4; r++) {
      out[c * 4 + r] = a[r] * b[c * 4] + a[4 + r] * b[c * 4 + 1] + a[8 + r] * b[c * 4 + 2] + a[12 + r] * b[c * 4 + 3];
    }
  }
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
  const isSoftware = /SwiftShader|llvmpipe|Software|Basic Render/i.test(
    String((dbg ? gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER)) || ""),
  );
  const hasHalfFloat = Boolean(gl.getExtension("EXT_color_buffer_half_float"));
  const shadowSize = isSoftware ? 1024 : 2048;

  const compile = (t, src) => {
    const s = gl.createShader(t);
    gl.shaderSource(s, src);
    gl.compileShader(s);
    return gl.getShaderParameter(s, gl.COMPILE_STATUS) ? s : null;
  };
  const link = (vsSrc, fsSrc) => {
    const vs = compile(gl.VERTEX_SHADER, vsSrc), fs = compile(gl.FRAGMENT_SHADER, fsSrc);
    if (!vs || !fs) return null;
    const p = gl.createProgram();
    gl.attachShader(p, vs);
    gl.attachShader(p, fs);
    gl.linkProgram(p);
    return gl.getProgramParameter(p, gl.LINK_STATUS) ? p : null;
  };

  // Pass 1: Dynamic Directional Shadow Map (2048x2048 Depth Texture FBO)
  const shadowProg = link(
    `#version 300 es
     layout(location=0) in vec3 a_pos; uniform mat4 u_lmvp;
     void main() { gl_Position = u_lmvp * vec4(a_pos, 1.0); }`,
    `#version 300 es
     precision mediump float; void main() {}`,
  );

  // Pass 2A: Atmospheric Rayleigh/Mie Sky Dome
  const skyProg = link(
    `#version 300 es
     layout(location=0) in vec2 a_uv; out vec2 v_ndc;
     void main() { v_ndc = a_uv * 2.0 - 1.0; gl_Position = vec4(v_ndc, 0.9999, 1.0); }`,
    `#version 300 es
     precision highp float; in vec2 v_ndc;
     uniform vec3 u_fwd, u_right, u_up, u_sky, u_fog, u_sun;
     uniform float u_aspect, u_tanHalf;
     out vec4 o_hdr;
     void main() {
       vec3 rd = normalize(u_fwd + v_ndc.x * u_aspect * u_tanHalf * u_right + v_ndc.y * u_tanHalf * u_up);
       float h = clamp(rd.y, -0.25, 1.0);
       vec3 zenith = pow(max(u_sky, vec3(0.03)), vec3(1.5)) * vec3(0.85, 1.05, 1.38) + vec3(0.015, 0.04, 0.09);
       vec3 horiz = pow(max(u_fog, vec3(0.04)), vec3(1.5)) * vec3(1.22, 1.18, 1.12) + vec3(0.05, 0.055, 0.05);
       vec3 col = mix(horiz, zenith, pow(clamp(h, 0.0, 1.0), 0.52));
       if (h < 0.0) col = mix(horiz, zenith * 0.35, clamp(-h * 3.2, 0.0, 1.0));
       float mu = max(dot(rd, u_sun), 0.0);
       col += vec3(1.05, 0.80, 0.46) * pow(mu, 9.0) * 0.32;
       col += vec3(2.2, 1.9, 1.45) * smoothstep(0.9984, 0.9997, mu);
       o_hdr = vec4(max(col, vec3(0.0)), 1.0);
     }`,
  );

  // Pass 2B: Cook-Torrance GGX + Derivative Micro-Normals + 3x3 PCF Soft Shadows + Water
  const pbrProg = link(
    `#version 300 es
     layout(location=0) in vec3 a_pos; layout(location=1) in vec3 a_norm; layout(location=2) in vec3 a_col;
     uniform mat4 u_mvp, u_lmvp;
     out vec3 v_pos, v_norm, v_col; out vec4 v_lpos;
     void main() {
       v_pos = a_pos; v_norm = a_norm; v_col = a_col;
       v_lpos = u_lmvp * vec4(a_pos + a_norm * 0.04, 1.0);
       gl_Position = u_mvp * vec4(a_pos, 1.0);
     }`,
    `#version 300 es
     precision highp float;
     precision highp sampler2DShadow;
     in vec3 v_pos, v_norm, v_col; in vec4 v_lpos;
     uniform vec3 u_eye, u_fog, u_sky, u_sun;
     uniform float u_time, u_shadows, u_invSmap;
     uniform sampler2DShadow u_smap;
     out vec4 o_hdr;

     float hash3(vec3 p) {
       p = fract(p * vec3(0.1031, 0.1030, 0.0973));
       p += dot(p, p.yxz + 33.33);
       return fract((p.x + p.y) * p.z);
     }
     float triGrain(vec3 p) {
       vec3 i = floor(p), f = fract(p); f = f * f * (3.0 - 2.0 * f);
       float a = mix(hash3(i), hash3(i + vec3(1,0,0)), f.x);
       float b = mix(hash3(i + vec3(0,0,1)), hash3(i + vec3(1,0,1)), f.x);
       return mix(a, b, f.z);
     }
     float sampleShadow(vec4 lp, float ndl) {
       if (u_shadows < 0.5) return 1.0;
       vec3 p = lp.xyz / max(lp.w, 1e-4) * 0.5 + 0.5;
       vec2 edge = smoothstep(vec2(0.0), vec2(0.07), p.xy) * (1.0 - smoothstep(vec2(0.93), vec2(1.0), p.xy));
       float mask = edge.x * edge.y * step(0.0, p.z) * step(p.z, 1.0);
       if (mask <= 0.001) return 1.0;
       float bias = max(0.0016 * (1.0 - ndl), 0.00045), acc = 0.0;
       for (int y = -1; y <= 1; y++) {
         for (int x = -1; x <= 1; x++) {
           acc += texture(u_smap, vec3(p.xy + vec2(float(x), float(y)) * u_invSmap, p.z - bias));
         }
       }
       return mix(1.0, acc / 9.0, mask);
     }
     void main() {
       vec3 n = normalize(v_norm), v = normalize(u_eye - v_pos), l = u_sun;
       bool isWater = (v_col.b > v_col.r * 1.35 && abs(n.y) > 0.92);
       bool isVisor = (v_col.b > 0.85 && v_col.g > 0.72 && v_col.r < 0.35);
       bool isMetal = (v_col.r > v_col.b * 1.25 && v_col.g > v_col.b * 1.05);

       float g1 = triGrain(v_pos * 5.2), g2 = triGrain(v_pos * 18.5);
       float micro = (g1 - 0.5) * 0.055 + (g2 - 0.5) * 0.028;
       if (isWater) {
         float w1 = sin(v_pos.x * 3.1 + u_time * 2.3) * cos(v_pos.z * 2.7 - u_time * 1.8);
         float w2 = sin((v_pos.x + v_pos.z) * 5.5 - u_time * 3.0) * 0.5;
         n = normalize(n + vec3((w1 + w2) * 0.042, 0.0, (w1 - w2) * 0.042));
       } else {
         vec3 dpx = dFdx(v_pos), dpy = dFdy(v_pos);
         vec3 c1 = cross(dpx, n), c2 = cross(n, dpy);
         float l1 = max(length(c1), 1e-4), l2 = max(length(c2), 1e-4);
         vec3 bump = (c1 / l1) * dFdx(micro) + (c2 / l2) * dFdy(micro);
         n = normalize(n + bump * (isMetal ? 0.55 : 1.15));
       }

       vec3 albedo = pow(max(v_col, vec3(0.02)), vec3(2.0));
       if (!isWater && !isVisor) albedo *= 0.93 + 0.14 * g1;
       float metallic = isWater ? 0.04 : (isMetal ? 0.72 : 0.05);
       float rough = isWater ? 0.07 : (isMetal ? 0.28 : clamp(0.66 - 0.15 * g2, 0.24, 0.88));

       float ndl = max(dot(n, l), 0.0), ndv = max(dot(n, v), 1e-3);
       vec3 h = normalize(l + v);
       float ndh = max(dot(n, h), 0.0), vdh = max(dot(v, h), 0.0);
       float shadow = sampleShadow(v_lpos, ndl);

       float a = rough * rough, a2 = a * a;
       float denom = ndh * ndh * (a2 - 1.0) + 1.0;
       float dGgx = a2 / max(3.14159265 * denom * denom, 1e-4);
       float k = (rough + 1.0) * (rough + 1.0) * 0.125;
       float gSmith = (ndv / (ndv * (1.0 - k) + k)) * (ndl / (ndl * (1.0 - k) + k));
       vec3 f0 = mix(vec3(0.04), albedo, metallic);
       vec3 fres = f0 + (1.0 - f0) * pow(1.0 - vdh, 5.0);
       vec3 spec = (dGgx * gSmith * fres) / max(4.0 * ndv * ndl, 1e-3);
       vec3 kd = (1.0 - fres) * (1.0 - metallic);

       vec3 sunRad = vec3(2.65, 2.42, 2.02) * shadow;
       vec3 direct = (kd * albedo / 3.14159265 + spec) * sunRad * ndl;

       float ao = clamp(1.0 - length(fwidth(n)) * 0.38, 0.56, 1.0);
       vec3 skyLin = pow(max(u_sky, vec3(0.04)), vec3(1.5)) * 1.35 + vec3(0.08, 0.12, 0.18);
       vec3 fogLin = pow(max(u_fog, vec3(0.04)), vec3(1.5)) * 1.15 + vec3(0.05, 0.05, 0.05);
       vec3 hemi = mix(vec3(0.09, 0.08, 0.07), skyLin, 0.5 + 0.5 * n.y);
       vec3 rDir = reflect(-v, n);
       vec3 iblSpec = mix(fogLin, skyLin, clamp(rDir.y * 0.5 + 0.5, 0.0, 1.0)) * (1.0 - rough * 0.72);
       vec3 fEnv = f0 + (max(vec3(1.0 - rough), f0) - f0) * pow(1.0 - ndv, 5.0);
       vec3 indirect = (kd * albedo * hemi + iblSpec * fEnv) * ao;

       vec3 lit = direct + indirect;
       if (isVisor) lit += albedo * 0.38;
       if (isWater) lit = mix(albedo * (hemi + 0.4 * shadow), skyLin + sunRad * pow(ndh, 96.0), clamp(0.18 + 0.78 * pow(1.0 - ndv, 4.0), 0.0, 0.95));

       float dist = length(u_eye - v_pos);
       float fogAmt = clamp(1.0 - exp(-dist * 0.0115), 0.0, 0.74);
       o_hdr = vec4(max(mix(lit, fogLin, fogAmt), vec3(0.0)), 1.0);
     }`,
  );

  // Pass 3: ACES Narkowicz Filmic Tonemapping + Subpixel Contrast Anti-Aliasing
  const postProg = link(
    `#version 300 es
     layout(location=0) in vec2 a_uv; out vec2 v_uv;
     void main() { v_uv = a_uv; gl_Position = vec4(a_uv * 2.0 - 1.0, 0.0, 1.0); }`,
    `#version 300 es
     precision highp float; in vec2 v_uv;
     uniform sampler2D u_hdr; uniform vec2 u_invRes;
     out vec4 o_col;
     vec3 aces(vec3 x) {
       return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), 0.0, 1.0);
     }
     void main() {
       vec3 c = texture(u_hdr, v_uv).rgb;
       vec3 n = texture(u_hdr, v_uv + vec2(0.0, u_invRes.y)).rgb;
       vec3 s = texture(u_hdr, v_uv - vec2(0.0, u_invRes.y)).rgb;
       vec3 e = texture(u_hdr, v_uv + vec2(u_invRes.x, 0.0)).rgb;
       vec3 w = texture(u_hdr, v_uv - vec2(u_invRes.x, 0.0)).rgb;
       vec3 avg = (n + s + e + w) * 0.25;
       float edge = clamp(length(c - avg) * 2.2, 0.0, 0.45);
       vec3 aa = mix(c, (c + avg) * 0.5, edge);
       o_col = vec4(pow(aces(aa * 1.06), vec3(1.0 / 2.2)), 1.0);
     }`,
  );

  const blitProg = link(
    `#version 300 es
     layout(location=0) in vec2 a_uv; out vec2 v_uv;
     void main() { v_uv = vec2(a_uv.x, 1.0 - a_uv.y); gl_Position = vec4(a_uv * 2.0 - 1.0, 0.0, 1.0); }`,
    `#version 300 es
     precision mediump float; in vec2 v_uv; uniform sampler2D u_tex; out vec4 o_col;
     void main() { o_col = texture(u_tex, v_uv); }`,
  );
  if (!shadowProg || !skyProg || !pbrProg || !postProg) return null;

  const gpu = { pos: gl.createBuffer(), norm: gl.createBuffer(), col: gl.createBuffer(), idx: gl.createBuffer(), count: 0 };
  const quadBuf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, quadBuf);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1]), gl.STATIC_DRAW);

  // Shadow Map FBO (`DEPTH_COMPONENT24`)
  const shadowTex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, shadowTex);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.DEPTH_COMPONENT24, shadowSize, shadowSize, 0, gl.DEPTH_COMPONENT, gl.UNSIGNED_INT, null);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_COMPARE_MODE, gl.COMPARE_REF_TO_TEXTURE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_COMPARE_FUNC, gl.LEQUAL);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  const shadowFbo = gl.createFramebuffer();
  gl.bindFramebuffer(gl.FRAMEBUFFER, shadowFbo);
  gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, shadowTex, 0);
  gl.drawBuffers([gl.NONE]);
  gl.readBuffer(gl.NONE);

  // Offscreen HDR Scene FBO (resized dynamically with canvas)
  let hdrW = 0, hdrH = 0;
  const hdrTex = gl.createTexture();
  const hdrDepth = gl.createRenderbuffer();
  const hdrFbo = gl.createFramebuffer();
  const ensureHdrFbo = (w, h) => {
    if (w === hdrW && h === hdrH) return true;
    hdrW = w; hdrH = h;
    gl.bindTexture(gl.TEXTURE_2D, hdrTex);
    if (hasHalfFloat) {
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA16F, w, h, 0, gl.RGBA, gl.HALF_FLOAT, null);
    } else {
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
    }
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.bindRenderbuffer(gl.RENDERBUFFER, hdrDepth);
    gl.renderbufferStorage(gl.RENDERBUFFER, gl.DEPTH_COMPONENT24, w, h);
    gl.bindFramebuffer(gl.FRAMEBUFFER, hdrFbo);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, hdrTex, 0);
    gl.framebufferRenderbuffer(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.RENDERBUFFER, hdrDepth);
    const ok = gl.checkFramebufferStatus(gl.FRAMEBUFFER) === gl.FRAMEBUFFER_COMPLETE;
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    return ok;
  };

  const blitTex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, blitTex);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  gl.bindFramebuffer(gl.FRAMEBUFFER, null);

  const uShadowLmvp = gl.getUniformLocation(shadowProg, "u_lmvp");
  const uSky = {
    fwd: gl.getUniformLocation(skyProg, "u_fwd"),
    right: gl.getUniformLocation(skyProg, "u_right"),
    up: gl.getUniformLocation(skyProg, "u_up"),
    sky: gl.getUniformLocation(skyProg, "u_sky"),
    fog: gl.getUniformLocation(skyProg, "u_fog"),
    sun: gl.getUniformLocation(skyProg, "u_sun"),
    aspect: gl.getUniformLocation(skyProg, "u_aspect"),
    tanHalf: gl.getUniformLocation(skyProg, "u_tanHalf"),
  };
  const uPbr = {
    mvp: gl.getUniformLocation(pbrProg, "u_mvp"),
    lmvp: gl.getUniformLocation(pbrProg, "u_lmvp"),
    eye: gl.getUniformLocation(pbrProg, "u_eye"),
    fog: gl.getUniformLocation(pbrProg, "u_fog"),
    sky: gl.getUniformLocation(pbrProg, "u_sky"),
    sun: gl.getUniformLocation(pbrProg, "u_sun"),
    time: gl.getUniformLocation(pbrProg, "u_time"),
    shadows: gl.getUniformLocation(pbrProg, "u_shadows"),
    invSmap: gl.getUniformLocation(pbrProg, "u_invSmap"),
    smap: gl.getUniformLocation(pbrProg, "u_smap"),
  };
  const uPost = {
    hdr: gl.getUniformLocation(postProg, "u_hdr"),
    invRes: gl.getUniformLocation(postProg, "u_invRes"),
  };

  const lightOrtho = mat4Ortho(-34, 34, -34, 34, -48, 64);

  return {
    backend: "webgl2",
    isSoftware,
    uploadPackedBuffers(posF32, normF32, colF32, idxU32) {
      gl.bindBuffer(gl.ARRAY_BUFFER, gpu.pos); gl.bufferData(gl.ARRAY_BUFFER, posF32, gl.DYNAMIC_DRAW);
      gl.bindBuffer(gl.ARRAY_BUFFER, gpu.norm); gl.bufferData(gl.ARRAY_BUFFER, normF32, gl.DYNAMIC_DRAW);
      gl.bindBuffer(gl.ARRAY_BUFFER, gpu.col); gl.bufferData(gl.ARRAY_BUFFER, colF32, gl.DYNAMIC_DRAW);
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, gpu.idx); gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, idxU32, gl.DYNAMIC_DRAW);
      gpu.count = idxU32.length;
    },
    render(mvp, eye, lodSpec, timeSec, lookTarget = [0, 1.8, 0], fovRad = Math.PI / 3.4) {
      const w = canvas.width | 0, h = canvas.height | 0;
      if (w <= 0 || h <= 0) return;
      const sky = lodSpec?.sky ?? [0.18, 0.30, 0.38];
      const fog = lodSpec?.fog ?? [0.30, 0.40, 0.44];
      const shadowsOn = (lodSpec?.rank ?? 3) >= 1 && gpu.count > 0 ? 1 : 0;

      const fx = lookTarget[0] ?? 0, fy = lookTarget[1] ?? 1.5, fz = lookTarget[2] ?? 0;
      const lightView = mat4LookAt(
        [fx + SUN_DIR[0] * 14, fy + SUN_DIR[1] * 14, fz + SUN_DIR[2] * 14],
        [fx, fy, fz],
        [0, 1, 0],
      );
      const lmvp = mat4Mul(lightOrtho, lightView);

      const bindPosOnly = () => {
        gl.bindBuffer(gl.ARRAY_BUFFER, gpu.pos);
        gl.enableVertexAttribArray(0);
        gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 0, 0);
        gl.disableVertexAttribArray(1);
        gl.disableVertexAttribArray(2);
      };
      const bindMeshAttribs = () => {
        const b3 = (buf, loc) => {
          gl.bindBuffer(gl.ARRAY_BUFFER, buf);
          gl.enableVertexAttribArray(loc);
          gl.vertexAttribPointer(loc, 3, gl.FLOAT, false, 0, 0);
        };
        b3(gpu.pos, 0); b3(gpu.norm, 1); b3(gpu.col, 2);
      };

      // Pass 1: Directional Shadow Map FBO
      if (shadowsOn) {
        gl.bindFramebuffer(gl.FRAMEBUFFER, shadowFbo);
        gl.viewport(0, 0, shadowSize, shadowSize);
        gl.colorMask(false, false, false, false);
        gl.enable(gl.DEPTH_TEST);
        gl.depthMask(true);
        gl.clear(gl.DEPTH_BUFFER_BIT);
        gl.enable(gl.POLYGON_OFFSET_FILL);
        gl.polygonOffset(1.8, 3.0);
        gl.useProgram(shadowProg);
        gl.uniformMatrix4fv(uShadowLmvp, false, lmvp);
        bindPosOnly();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, gpu.idx);
        gl.drawElements(gl.TRIANGLES, gpu.count, gl.UNSIGNED_INT, 0);
        gl.disable(gl.POLYGON_OFFSET_FILL);
        gl.colorMask(true, true, true, true);
      }

      // Pass 2: HDR Atmospheric Sky + GGX Lit Scene
      const useHdr = ensureHdrFbo(w, h);
      gl.bindFramebuffer(gl.FRAMEBUFFER, useHdr ? hdrFbo : null);
      gl.viewport(0, 0, w, h);
      gl.clearColor(sky[0], sky[1], sky[2], 1.0);
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

      // Camera basis for Rayleigh/Mie sky dome:
      let fdx = fx - eye[0], fdy = fy - eye[1], fdz = fz - eye[2];
      const fl = Math.hypot(fdx, fdy, fdz) || 1;
      fdx /= fl; fdy /= fl; fdz /= fl;
      let rdx = fdy * 0 - fdz * 1, rdy = fdz * 0 - fdx * 0, rdz = fdx * 1 - fdy * 0;
      // Right = normalize(cross(fwd, [0,1,0])) = [-fdz, 0, fdx]:
      rdx = -fdz; rdy = 0; rdz = fdx;
      const rl = Math.hypot(rdx, rdy, rdz) || 1;
      rdx /= rl; rdz /= rl;
      const udx = rdy * fdz - rdz * fdy, udy = rdz * fdx - rdx * fdz, udz = rdx * fdy - rdy * fdx;

      gl.disable(gl.DEPTH_TEST);
      gl.depthMask(false);
      gl.useProgram(skyProg);
      gl.uniform3f(uSky.fwd, fdx, fdy, fdz);
      gl.uniform3f(uSky.right, rdx, rdy, rdz);
      gl.uniform3f(uSky.up, udx, udy, udz);
      gl.uniform3fv(uSky.sky, sky);
      gl.uniform3fv(uSky.fog, fog);
      gl.uniform3fv(uSky.sun, SUN_DIR);
      gl.uniform1f(uSky.aspect, w / h);
      gl.uniform1f(uSky.tanHalf, Math.tan(fovRad * 0.5));
      gl.bindBuffer(gl.ARRAY_BUFFER, quadBuf);
      gl.enableVertexAttribArray(0);
      gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
      gl.disableVertexAttribArray(1);
      gl.disableVertexAttribArray(2);
      gl.drawArrays(gl.TRIANGLES, 0, 6);

      gl.depthMask(true);
      gl.enable(gl.DEPTH_TEST);
      if (gpu.count > 0 && mvp) {
        gl.useProgram(pbrProg);
        gl.uniformMatrix4fv(uPbr.mvp, false, mvp);
        gl.uniformMatrix4fv(uPbr.lmvp, false, lmvp);
        gl.uniform3fv(uPbr.eye, eye);
        gl.uniform3fv(uPbr.fog, fog);
        gl.uniform3fv(uPbr.sky, sky);
        gl.uniform3fv(uPbr.sun, SUN_DIR);
        gl.uniform1f(uPbr.time, timeSec);
        gl.uniform1f(uPbr.shadows, shadowsOn);
        gl.uniform1f(uPbr.invSmap, 1.0 / shadowSize);
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, shadowTex);
        gl.uniform1i(uPbr.smap, 0);
        bindMeshAttribs();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, gpu.idx);
        gl.drawElements(gl.TRIANGLES, gpu.count, gl.UNSIGNED_INT, 0);
      }

      // Pass 3: ACES Filmic Tonemapping + Contrast AA onto default backbuffer
      if (useHdr) {
        gl.bindFramebuffer(gl.FRAMEBUFFER, null);
        gl.viewport(0, 0, w, h);
        gl.disable(gl.DEPTH_TEST);
        gl.useProgram(postProg);
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, hdrTex);
        gl.uniform1i(uPost.hdr, 0);
        gl.uniform2f(uPost.invRes, 1.0 / w, 1.0 / h);
        gl.bindBuffer(gl.ARRAY_BUFFER, quadBuf);
        gl.enableVertexAttribArray(0);
        gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
        gl.disableVertexAttribArray(1);
        gl.disableVertexAttribArray(2);
        gl.drawArrays(gl.TRIANGLES, 0, 6);
      }
      gl.finish();
    },
    blitTensorRgba(rgbaBytes, width, height) {
      if (!blitProg || !rgbaBytes || width <= 0 || height <= 0) return;
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.viewport(0, 0, canvas.width, canvas.height);
      gl.disable(gl.DEPTH_TEST);
      gl.useProgram(blitProg);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, blitTex);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, rgbaBytes);
      gl.bindBuffer(gl.ARRAY_BUFFER, quadBuf);
      gl.enableVertexAttribArray(0);
      gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
      gl.disableVertexAttribArray(1);
      gl.disableVertexAttribArray(2);
      gl.drawArrays(gl.TRIANGLES, 0, 6);
    },
  };
}

export function createCanvas2dTensorRenderer(canvas) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return null;
  return {
    backend: "tensor-2d",
    isSoftware: true,
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
