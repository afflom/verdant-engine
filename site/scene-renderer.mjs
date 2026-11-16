export function mat4Perspective(fovRad, aspect, near, far) {
  const f = 1 / Math.tan(fovRad * 0.5), nf = 1 / (near - far);
  return new Float32Array([f / aspect, 0, 0, 0, 0, f, 0, 0, 0, 0, (far + near) * nf, -1, 0, 0, 2 * far * near * nf, 0]);
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

export async function createGlRenderer(canvas) {
  const gl = canvas.getContext("webgl2", { antialias: false, depth: true, alpha: false, powerPreference: "high-performance" });
  if (!gl) return null;
  const dbg = gl.getExtension("WEBGL_debug_renderer_info");
  const isSoftware = /SwiftShader|llvmpipe|Software|Basic Render/i.test(
    String((dbg ? gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER)) || ""),
  );

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

  const prog = link(
    `#version 300 es
     layout(location=0) in vec3 a_pos; layout(location=1) in vec3 a_norm; layout(location=2) in vec3 a_col;
     uniform mat4 u_mvp; out vec3 v_pos, v_norm, v_col;
     void main() { v_pos = a_pos; v_norm = a_norm; v_col = a_col; gl_Position = u_mvp * vec4(a_pos, 1.0); }`,
    `#version 300 es
     precision highp float;
     in vec3 v_pos, v_norm, v_col;
     uniform vec3 u_eye, u_fog; uniform float u_time; out vec4 o_col;
     void main() {
       vec3 n = normalize(v_norm), l = normalize(vec3(0.58, 0.76, 0.30)), v = normalize(u_eye - v_pos);
       float ndl = max(dot(n, l), 0.0);
       vec3 amb = mix(vec3(0.22, 0.21, 0.20), vec3(0.44, 0.56, 0.66), 0.5 + 0.5 * n.y);
       float spec = pow(max(dot(n, normalize(l + v)), 0.0), 36.0) * 0.30;
       float rim = pow(1.0 - max(dot(n, v), 0.0), 2.8) * 0.16;
       vec3 base = v_col;
       if (v_col.b > v_col.r * 1.4 && abs(n.y) > 0.9) {
         base += 0.04 * sin(v_pos.x * 2.8 + u_time * 2.4) * cos(v_pos.z * 2.6 - u_time * 1.9);
         spec += pow(1.0 - max(dot(n, v), 0.0), 3.0) * 0.36;
       }
       vec3 lit = base * (amb + vec3(1.04, 0.96, 0.84) * ndl * 0.84) + spec + vec3(0.45, 0.78, 0.92) * rim;
       float fog = clamp(1.0 - exp(-length(u_eye - v_pos) * 0.014), 0.0, 0.76);
       o_col = vec4(pow(max(mix(lit, u_fog, fog), vec3(0.0)), vec3(1.0 / 2.2)), 1.0);
     }`,
  );
  if (!prog) return null;

  const blitProg = link(
    `#version 300 es
     layout(location=0) in vec2 a_uv; out vec2 v_uv;
     void main() { v_uv = vec2(a_uv.x, 1.0 - a_uv.y); gl_Position = vec4(a_uv * 2.0 - 1.0, 0.0, 1.0); }`,
    `#version 300 es
     precision mediump float; in vec2 v_uv; uniform sampler2D u_tex; out vec4 o_col;
     void main() { o_col = texture(u_tex, v_uv); }`,
  );

  const gpu = { pos: gl.createBuffer(), norm: gl.createBuffer(), col: gl.createBuffer(), idx: gl.createBuffer(), count: 0 };
  const quadBuf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, quadBuf);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1]), gl.STATIC_DRAW);
  const blitTex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, blitTex);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

  const uMvp = gl.getUniformLocation(prog, "u_mvp");
  const uEye = gl.getUniformLocation(prog, "u_eye");
  const uFog = gl.getUniformLocation(prog, "u_fog");
  const uTime = gl.getUniformLocation(prog, "u_time");

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
    render(mvp, eye, lodSpec, timeSec) {
      if (canvas.width <= 0 || canvas.height <= 0) return;
      gl.viewport(0, 0, canvas.width, canvas.height);
      gl.clearColor(lodSpec.sky[0], lodSpec.sky[1], lodSpec.sky[2], 1.0);
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
      if (!gpu.count || !mvp) return;
      gl.enable(gl.DEPTH_TEST);
      gl.useProgram(prog);
      gl.uniformMatrix4fv(uMvp, false, mvp);
      gl.uniform3fv(uEye, eye);
      gl.uniform3fv(uFog, lodSpec.fog);
      gl.uniform1f(uTime, timeSec);
      const bind3 = (buf, loc) => {
        gl.bindBuffer(gl.ARRAY_BUFFER, buf);
        gl.enableVertexAttribArray(loc);
        gl.vertexAttribPointer(loc, 3, gl.FLOAT, false, 0, 0);
      };
      bind3(gpu.pos, 0); bind3(gpu.norm, 1); bind3(gpu.col, 2);
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, gpu.idx);
      gl.drawElements(gl.TRIANGLES, gpu.count, gl.UNSIGNED_INT, 0);
    },
    blitTensorRgba(rgbaBytes, width, height) {
      if (!blitProg || !rgbaBytes || width <= 0 || height <= 0) return;
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
