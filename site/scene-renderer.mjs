// Verdant Engine — Universal OffscreenCanvas / HTMLCanvasElement Renderer
// (WebGL2/WebGL1 + Prism uor-matmul 16x16 Software Tile Framebuffer Blitter)
// and Khronos Binary glTF 2.0 (.glb) Scene Serializer.

export function terrainHeight(x, z, amp = 1.0) {
  const r = Math.hypot(x, z);
  const basin = -1.35 * Math.exp(-((x - 5.5) ** 2 + (z - 9.0) ** 2) / 52.0);
  const plateau = 0.55 * Math.exp(-(r * r) / 70.0);
  const rim = 2.6 * Math.max(0, (r - 17.5) / 14.0) ** 1.45;
  const ridge =
    0.42 * Math.sin(x * 0.19 + 0.4) * Math.cos(z * 0.17 - 0.2) +
    0.21 * Math.sin((x + z) * 0.33);
  return (basin + plateau + rim + ridge) * amp;
}

export class MeshBuilder {
  constructor() {
    this.pos = [];
    this.norm = [];
    this.col = [];
    this.idx = [];
  }

  addVertex(px, py, pz, nx, ny, nz, r, g, b) {
    const i = this.pos.length / 3;
    this.pos.push(px, py, pz);
    this.norm.push(nx, ny, nz);
    this.col.push(r, g, b);
    return i;
  }

  addQuad(a, b, c, d, col) {
    const ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2];
    const vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
    let nx = uy * vz - uz * vy;
    let ny = uz * vx - ux * vz;
    let nz = ux * vy - uy * vx;
    const len = Math.hypot(nx, ny, nz) || 1;
    nx /= len; ny /= len; nz /= len;
    const i0 = this.addVertex(a[0], a[1], a[2], nx, ny, nz, col[0], col[1], col[2]);
    const i1 = this.addVertex(b[0], b[1], b[2], nx, ny, nz, col[0], col[1], col[2]);
    const i2 = this.addVertex(c[0], c[1], c[2], nx, ny, nz, col[0], col[1], col[2]);
    const i3 = this.addVertex(d[0], d[1], d[2], nx, ny, nz, col[0], col[1], col[2]);
    this.idx.push(i0, i1, i2, i0, i2, i3);
  }

  addBox(cx, cy, cz, sx, sy, sz, col, yaw = 0) {
    const hx = sx * 0.5, hy = sy * 0.5, hz = sz * 0.5;
    const c = Math.cos(yaw), s = Math.sin(yaw);
    const rot = (x, y, z) => [cx + x * c - z * s, cy + y, cz + x * s + z * c];
    const p000 = rot(-hx, -hy, -hz), p100 = rot(hx, -hy, -hz);
    const p110 = rot(hx, hy, -hz), p010 = rot(-hx, hy, -hz);
    const p001 = rot(-hx, -hy, hz), p101 = rot(hx, -hy, hz);
    const p111 = rot(hx, hy, hz), p011 = rot(-hx, hy, hz);
    this.addQuad(p001, p101, p111, p011, col);
    this.addQuad(p100, p000, p010, p110, col);
    this.addQuad(p011, p111, p110, p010, col);
    this.addQuad(p000, p100, p101, p001, col);
    this.addQuad(p101, p100, p110, p111, col);
    this.addQuad(p000, p001, p011, p010, col);
  }

  addCylinder(cx, y0, cz, r0, r1, h, segs, col) {
    const base = this.pos.length / 3;
    for (let i = 0; i <= segs; i++) {
      const a = (i / segs) * Math.PI * 2;
      const ca = Math.cos(a), sa = Math.sin(a);
      this.addVertex(cx + ca * r0, y0, cz + sa * r0, ca, 0.15, sa, col[0], col[1], col[2]);
      this.addVertex(cx + ca * r1, y0 + h, cz + sa * r1, ca, 0.15, sa, col[0], col[1], col[2]);
    }
    for (let i = 0; i < segs; i++) {
      const k = base + i * 2;
      this.idx.push(k, k + 2, k + 3, k, k + 3, k + 1);
    }
    const topCenter = this.addVertex(cx, y0 + h, cz, 0, 1, 0, col[0], col[1], col[2]);
    for (let i = 0; i < segs; i++) {
      const a0 = (i / segs) * Math.PI * 2;
      const a1 = ((i + 1) / segs) * Math.PI * 2;
      const v0 = this.addVertex(cx + Math.cos(a0) * r1, y0 + h, cz + Math.sin(a0) * r1, 0, 1, 0, col[0], col[1], col[2]);
      const v1 = this.addVertex(cx + Math.cos(a1) * r1, y0 + h, cz + Math.sin(a1) * r1, 0, 1, 0, col[0], col[1], col[2]);
      this.idx.push(topCenter, v0, v1);
    }
  }

  addTorusRing(cx, cy, cz, majorR, minorR, segs, sides, col, tiltX = 0, tiltZ = 0) {
    const cxr = Math.cos(tiltX), sxr = Math.sin(tiltX);
    const czr = Math.cos(tiltZ), szr = Math.sin(tiltZ);
    const xf = (x, y, z) => {
      const y1 = y * cxr - z * sxr;
      const z1 = y * sxr + z * cxr;
      const x2 = x * czr - y1 * szr;
      const y2 = x * szr + y1 * czr;
      return [cx + x2, cy + y2, cz + z1];
    };
    const base = this.pos.length / 3;
    for (let i = 0; i <= segs; i++) {
      const u = (i / segs) * Math.PI * 2;
      const cu = Math.cos(u), su = Math.sin(u);
      for (let j = 0; j <= sides; j++) {
        const v = (j / sides) * Math.PI * 2;
        const cv = Math.cos(v), sv = Math.sin(v);
        const r = majorR + minorR * cv;
        const p = xf(r * cu, minorR * sv, r * su);
        const n = xf(cv * cu, sv, cv * su);
        this.addVertex(p[0], p[1], p[2], n[0] - cx, n[1] - cy, n[2] - cz, col[0], col[1], col[2]);
      }
    }
    const stride = sides + 1;
    for (let i = 0; i < segs; i++) {
      for (let j = 0; j < sides; j++) {
        const a = base + i * stride + j;
        const b = a + stride;
        this.idx.push(a, b, b + 1, a, b + 1, a + 1);
      }
    }
  }
}

export function buildStaticWorld(lodSpec, baseGridN = 36) {
  const mb = new MeshBuilder();
  const rank = lodSpec?.rank ?? 3;
  const amp = lodSpec?.density ?? 1.0;
  const gridN =
    rank === 0
      ? Math.max(12, Math.floor(baseGridN * 0.5))
      : rank === 1
        ? Math.max(18, Math.floor(baseGridN * 0.75))
        : baseGridN;
  const half = 28;
  const base = mb.pos.length / 3;

  for (let gz = 0; gz <= gridN; gz++) {
    for (let gx = 0; gx <= gridN; gx++) {
      const x = ((gx / gridN) * 2 - 1) * half;
      const z = ((gz / gridN) * 2 - 1) * half;
      const y = terrainHeight(x, z, amp);
      const hx = terrainHeight(x + 0.5, z, amp) - terrainHeight(x - 0.5, z, amp);
      const hz = terrainHeight(x, z + 0.5, amp) - terrainHeight(x, z - 0.5, amp);
      const len = Math.hypot(-hx, 1.0, -hz);
      let col;
      if (rank === 0) {
        col = y < -0.25 ? [0.24, 0.26, 0.30] : y > 1.8 ? [0.54, 0.52, 0.48] : [0.38, 0.37, 0.35];
      } else if (y < -0.22) {
        col = [0.58, 0.55, 0.48];
      } else if (y > 2.1) {
        col = [0.64, 0.61, 0.56];
      } else {
        const checker = 0.5 + 0.5 * Math.sin(x * 0.25 + z * 0.2);
        col = [0.22 + 0.08 * checker, 0.43 + 0.11 * checker, 0.26 + 0.06 * checker];
      }
      mb.addVertex(x, y, z, -hx / len, 1.0 / len, -hz / len, col[0], col[1], col[2]);
    }
  }
  const stride = gridN + 1;
  for (let gz = 0; gz < gridN; gz++) {
    for (let gx = 0; gx < gridN; gx++) {
      const a = base + gz * stride + gx;
      const b = a + stride;
      mb.idx.push(a, b, a + 1, a + 1, b, b + 1);
    }
  }

  const stone = [0.86, 0.84, 0.78];
  const alloy = [0.76, 0.58, 0.28];
  const slate = [0.27, 0.29, 0.33];
  const y0 = terrainHeight(0, 0, amp);

  if (rank >= 1) {
    // Fluid wave testbed surface
    const wSegs = rank >= 2 ? 28 : 16;
    const wx = 5.5, wz = 9.0, wr = 7.4, wy = -0.32 * amp;
    const center = mb.addVertex(wx, wy, wz, 0, 1, 0, 0.14, 0.46, 0.58);
    for (let i = 0; i <= wSegs; i++) {
      const a = (i / wSegs) * Math.PI * 2;
      mb.addVertex(wx + Math.cos(a) * wr, wy, wz + Math.sin(a) * wr, 0, 1, 0, 0.18, 0.54, 0.66);
    }
    for (let i = 0; i < wSegs; i++) {
      mb.idx.push(center, center + 1 + i + 1, center + 1 + i);
    }

    // Node 0: StaticMeshTransformBatch — Central Arch Pavilion
    const colSegs = rank >= 3 ? 12 : 8;
    mb.addCylinder(0, y0 - 0.15, 0, 5.4, 5.2, 0.35, 20, stone);
    mb.addCylinder(0, y0 + 0.20, 0, 4.8, 4.7, 0.28, 20, [0.91, 0.88, 0.82]);
    const colCount = rank >= 2 ? 12 : 8;
    for (let i = 0; i < colCount; i++) {
      const a = (i / colCount) * Math.PI * 2;
      mb.addCylinder(Math.cos(a) * 4.1, y0 + 0.48, Math.sin(a) * 4.1, 0.26, 0.21, 3.1, colSegs, stone);
    }

    // Node 1: LinearBlendSkinningBatch — Axial Marker Tower
    const sy = terrainHeight(9.5, -3.5, amp);
    mb.addCylinder(9.5, sy - 0.1, -3.5, 1.35, 1.15, 0.85, 12, stone);
    mb.addBox(9.5, sy + 1.15, -3.5, 0.18, 1.2, 1.2, alloy, 0.45);

    // Node 2: TropicalBvhVisibilityCull — Modal Acoustics Frame
    const py = terrainHeight(-9.0, -4.0, amp);
    for (const [dx, dz] of [[-1.4, -1.4], [1.4, -1.4], [-1.4, 1.4], [1.4, 1.4]]) {
      mb.addCylinder(-9.0 + dx, py - 0.1, -4.0 + dz, 0.16, 0.14, 2.7, 8, slate);
    }
    mb.addBox(-9.0, py + 2.65, -4.0, 3.3, 0.18, 0.22, alloy, 0);
    mb.addBox(-9.0, py + 2.65, -4.0, 0.22, 0.18, 3.3, alloy, 0);
    for (let c = 0; c < 5; c++) {
      const cx = -9.8 + c * 0.40;
      const clen = 0.78 + c * 0.15;
      mb.addCylinder(cx, py + 2.55 - clen, -4.0, 0.05, 0.05, clen, 6, [0.86, 0.70, 0.36]);
    }
  }

  if (rank >= 2) {
    // Node 3: Articulated Rig Bay — XPBD joint gantry frame
    const qcy = terrainHeight(-6.5, 9.0, amp);
    mb.addCylinder(-6.5, qcy - 0.1, 9.0, 2.05, 1.85, 0.45, 10, slate);
    mb.addCylinder(-7.75, qcy + 0.35, 9.0, 0.16, 0.14, 2.85, 8, slate);
    mb.addCylinder(-5.25, qcy + 0.35, 9.0, 0.16, 0.14, 2.85, 8, slate);
    mb.addBox(-6.5, qcy + 3.22, 9.0, 2.95, 0.20, 0.26, alloy, 0);

    // Node 4: ShadowCascadeProjector — Elevated Deck Bridge
    for (let s = 0; s < 5; s++) {
      const pz = 2.8 + s * 1.45;
      const px = 2.4 + s * 0.75;
      mb.addBox(px, 0.05, pz, 1.35, 0.18, 1.35, stone, 0.48);
      mb.addCylinder(px, -0.9, pz, 0.18, 0.22, 0.95, 8, slate);
    }

    // Outer calibration pylons for BVH frustum & shadow cascade bounds
    const pylonMarkers = [[-11, 6, 1.4], [12, 5, 1.2], [-13, -8, 1.5], [10, -11, 1.3]];
    for (const [qx, qz, qh] of pylonMarkers) {
      const qy = terrainHeight(qx, qz, amp);
      mb.addCylinder(qx, qy - 0.1, qz, 0.42, 0.10, qh, 6, [0.74, 0.82, 0.90]);
    }
  }

  if (rank >= 3) {
    // Full lossless LOD3 detail: upper entablature torus + Node 5 ModalAudioMixMatrix path grid
    mb.addTorusRing(0, y0 + 3.68, 0, 4.1, 0.28, 24, 8, stone, Math.PI * 0.5, 0);
    for (let s = 0; s < 16; s++) {
      const a = (s / 16) * Math.PI * 2;
      const rx = Math.cos(a) * 16.5;
      const rz = Math.sin(a) * 16.5;
      const ry = terrainHeight(rx, rz, amp);
      mb.addBox(rx, ry + 0.04, rz, 1.3, 0.08, 2.8, [0.83, 0.80, 0.72], -a);
    }
  }

  return mb;
}

export function buildDynamicActors(lodSpec, timeSec, wasmMover = null) {
  const mb = new MeshBuilder();
  const rank = lodSpec?.rank ?? 3;
  if (rank < 1) return { mb, moverPos: [16.5, 0, 0] };

  const amp = lodSpec?.density ?? 1.0;
  const y0 = terrainHeight(0, 0, amp);
  const gold = [0.85, 0.68, 0.32];

  mb.addTorusRing(0, y0 + 2.25, 0, 1.55, 0.06, 20, 6, gold, timeSec * 0.55, 0.35);
  mb.addTorusRing(0, y0 + 2.25, 0, 1.18, 0.05, 18, 6, [0.72, 0.82, 0.88], -timeSec * 0.8, 1.05);

  if (rank < 2) return { mb, moverPos: [16.5, 0.5, 0] };

  // Node 3: XPBD Articulated Hinge + Slider + BallSocket dynamic rig
  const qcy = terrainHeight(-6.5, 9.0, amp);
  const slideX = -6.5 + Math.sin(timeSec * 1.35) * 0.72;
  const swing = Math.sin(timeSec * 2.05) * 0.52;
  mb.addBox(slideX, qcy + 3.22, 9.0, 0.46, 0.32, 0.42, [0.26, 0.80, 0.88], 0);
  const bobX = slideX + Math.sin(swing) * 1.35;
  const bobY = qcy + 3.12 - Math.cos(swing) * 1.35;
  mb.addCylinder((slideX + bobX) * 0.5, Math.min(qcy + 3.12, bobY), 9.0, 0.06, 0.06, Math.max(0.4, qcy + 3.12 - bobY), 6, [0.82, 0.88, 0.94]);
  mb.addTorusRing(bobX, bobY, 9.0, 0.34, 0.10, 14, 6, gold, timeSec * 1.6, swing);

  let rx;
  let rz;
  let heading;
  if (wasmMover) {
    const scale = 16.5 / 26.0;
    rx = wasmMover.x * scale;
    rz = wasmMover.z * scale;
    heading = wasmMover.yaw;
  } else {
    const theta = timeSec * 0.28;
    rx = Math.cos(theta) * 16.5;
    rz = Math.sin(theta) * 16.5;
    heading = -theta;
  }
  const ry = terrainHeight(rx, rz, amp) + 0.42;

  // Node 6: 4-Wheel Raycast Suspension Vehicle + Kinematic Spline Mover
  mb.addBox(rx, ry, rz, 1.04, 0.34, 1.56, [0.84, 0.64, 0.30], heading);
  mb.addBox(rx, ry + 0.28, rz - 0.08, 0.64, 0.24, 0.74, [0.92, 0.90, 0.84], heading);
  mb.addCylinder(rx, ry + 0.38, rz, 0.08, 0.06, 0.52, 8, [0.34, 0.38, 0.44]);
  mb.addBox(rx, ry + 0.92, rz, 0.34, 0.16, 0.24, [0.24, 0.82, 0.90], heading + Math.sin(timeSec * 2.2) * 0.6);

  const c = Math.cos(heading), s = Math.sin(heading);
  for (const [wx, wz] of [[-0.58, -0.56], [0.58, -0.56], [-0.58, 0.56], [0.58, 0.56]]) {
    const px = rx + wx * c - wz * s;
    const pz = rz + wx * s + wz * c;
    const groundY = terrainHeight(px, pz, amp);
    const wheelY = Math.max(groundY + 0.02, ry - 0.38 + Math.sin(timeSec * 4.5 + wx * 2.0 + wz) * 0.04);
    const strutLen = Math.max(0.14, ry - wheelY);
    mb.addCylinder(px, wheelY + 0.12, pz, 0.05, 0.05, strutLen, 6, [0.68, 0.78, 0.86]);
    mb.addCylinder(px, wheelY, pz, 0.22, 0.22, 0.24, 10, [0.22, 0.25, 0.30]);
  }

  return { mb, moverPos: [rx, ry, rz] };
}

export function mat4Perspective(fovRad, aspect, near, far) {
  const f = 1.0 / Math.tan(fovRad * 0.5);
  const nf = 1.0 / (near - far);
  return new Float32Array([
    f / aspect, 0, 0, 0,
    0, f, 0, 0,
    0, 0, (far + near) * nf, -1,
    0, 0, 2 * far * near * nf, 0,
  ]);
}

export function mat4LookAt(eye, target, up) {
  let zx = eye[0] - target[0], zy = eye[1] - target[1], zz = eye[2] - target[2];
  let zl = Math.hypot(zx, zy, zz) || 1;
  zx /= zl; zy /= zl; zz /= zl;
  let xx = up[1] * zz - up[2] * zy;
  let xy = up[2] * zx - up[0] * zz;
  let xz = up[0] * zy - up[1] * zx;
  let xl = Math.hypot(xx, xy, xz) || 1;
  xx /= xl; xy /= xl; xz /= xl;
  const yx = zy * xz - zz * xy;
  const yy = zz * xx - zx * xz;
  const yz = zx * xy - zy * xx;
  return new Float32Array([
    xx, yx, zx, 0,
    xy, yy, zy, 0,
    xz, yz, zz, 0,
    -(xx * eye[0] + xy * eye[1] + xz * eye[2]),
    -(yx * eye[0] + yy * eye[1] + yz * eye[2]),
    -(zx * eye[0] + zy * eye[1] + zz * eye[2]),
    1,
  ]);
}

export function mat4Mul(a, b) {
  const out = new Float32Array(16);
  for (let col = 0; col < 4; col++) {
    for (let row = 0; row < 4; row++) {
      out[col * 4 + row] =
        a[0 * 4 + row] * b[col * 4 + 0] +
        a[1 * 4 + row] * b[col * 4 + 1] +
        a[2 * 4 + row] * b[col * 4 + 2] +
        a[3 * 4 + row] * b[col * 4 + 3];
    }
  }
  return out;
}

export async function createGlRenderer(canvas) {
  const gl =
    canvas.getContext("webgl2", { antialias: false, depth: true, alpha: false, powerPreference: "high-performance" }) ||
    canvas.getContext("webgl", { antialias: false, depth: true, alpha: false });
  if (!gl) return null;

  const isWebGL2 = typeof WebGL2RenderingContext !== "undefined" && gl instanceof WebGL2RenderingContext;
  if (!isWebGL2) gl.getExtension("OES_element_index_uint");
  const dbg = gl.getExtension("WEBGL_debug_renderer_info");
  const rendererStr = String(
    (dbg ? gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER)) || "",
  );
  const isSoftware = /SwiftShader|llvmpipe|Software|Basic Render/i.test(rendererStr);

  const vsSrc = isWebGL2
    ? `#version 300 es
       layout(location=0) in vec3 a_pos;
       layout(location=1) in vec3 a_norm;
       layout(location=2) in vec3 a_col;
       uniform mat4 u_mvp;
       out vec3 v_pos;
       out vec3 v_norm;
       out vec3 v_col;
       void main() {
         v_pos = a_pos;
         v_norm = a_norm;
         v_col = a_col;
         gl_Position = u_mvp * vec4(a_pos, 1.0);
       }`
    : `attribute vec3 a_pos;
       attribute vec3 a_norm;
       attribute vec3 a_col;
       uniform mat4 u_mvp;
       varying vec3 v_pos;
       varying vec3 v_norm;
       varying vec3 v_col;
       void main() {
         v_pos = a_pos;
         v_norm = a_norm;
         v_col = a_col;
         gl_Position = u_mvp * vec4(a_pos, 1.0);
       }`;

  const fsSrc = isWebGL2
    ? `#version 300 es
       precision highp float;
       in vec3 v_pos;
       in vec3 v_norm;
       in vec3 v_col;
       uniform vec3 u_eye;
       uniform vec3 u_fog;
       uniform float u_time;
       out vec4 o_col;
       void main() {
         vec3 n = normalize(v_norm);
         vec3 l = normalize(vec3(0.58, 0.76, 0.30));
         vec3 v = normalize(u_eye - v_pos);
         vec3 h = normalize(l + v);
         float ndl = max(dot(n, l), 0.0);
         float hemi = 0.5 + 0.5 * n.y;
         vec3 amb = mix(vec3(0.22, 0.21, 0.20), vec3(0.44, 0.56, 0.66), hemi);
         float spec = pow(max(dot(n, h), 0.0), 36.0) * 0.30;
         float rim = pow(1.0 - max(dot(n, v), 0.0), 2.8) * 0.16;
         vec3 base = v_col;
         if (v_col.b > v_col.r * 1.6 && abs(n.y) > 0.9) {
           float ripple = 0.04 * sin(v_pos.x * 2.8 + u_time * 2.4) * cos(v_pos.z * 2.6 - u_time * 1.9);
           base += ripple;
           spec += pow(1.0 - max(dot(n, v), 0.0), 3.0) * 0.38;
         }
         vec3 lit = base * (amb + vec3(1.04, 0.96, 0.84) * ndl * 0.84) + spec + vec3(0.45, 0.78, 0.92) * rim;
         float dist = length(u_eye - v_pos);
         float fogFactor = 1.0 - exp(-dist * 0.016);
         vec3 col = mix(lit, u_fog, clamp(fogFactor, 0.0, 0.78));
         o_col = vec4(pow(max(col, vec3(0.0)), vec3(1.0 / 2.2)), 1.0);
       }`
    : `precision highp float;
       varying vec3 v_pos;
       varying vec3 v_norm;
       varying vec3 v_col;
       uniform vec3 u_eye;
       uniform vec3 u_fog;
       uniform float u_time;
       void main() {
         vec3 n = normalize(v_norm);
         vec3 l = normalize(vec3(0.58, 0.76, 0.30));
         float ndl = max(dot(n, l), 0.0);
         vec3 amb = mix(vec3(0.18, 0.17, 0.15), vec3(0.38, 0.48, 0.56), 0.5 + 0.5 * n.y);
         vec3 lit = v_col * (amb + vec3(1.0, 0.94, 0.82) * ndl * 0.82);
         float dist = length(u_eye - v_pos);
         vec3 col = mix(lit, u_fog, clamp(1.0 - exp(-dist * 0.016), 0.0, 0.78));
         gl_FragColor = vec4(pow(max(col, vec3(0.0)), vec3(0.4545)), 1.0);
       }`;

  const compile = (type, src) => {
    const s = gl.createShader(type);
    gl.shaderSource(s, src);
    gl.compileShader(s);
    if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) return null;
    return s;
  };
  const vs = compile(gl.VERTEX_SHADER, vsSrc);
  const fs = compile(gl.FRAGMENT_SHADER, fsSrc);
  if (!vs || !fs) return null;

  const prog = gl.createProgram();
  gl.attachShader(prog, vs);
  gl.attachShader(prog, fs);
  gl.bindAttribLocation(prog, 0, "a_pos");
  gl.bindAttribLocation(prog, 1, "a_norm");
  gl.bindAttribLocation(prog, 2, "a_col");
  gl.linkProgram(prog);
  if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) return null;

  const makeBuffers = () => ({
    pos: gl.createBuffer(),
    norm: gl.createBuffer(),
    col: gl.createBuffer(),
    idx: gl.createBuffer(),
    count: 0,
  });

  const staticGpu = makeBuffers();
  const dynamicGpu = makeBuffers();

  const upload = (gpu, mb, usage) => {
    gl.bindBuffer(gl.ARRAY_BUFFER, gpu.pos);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(mb.pos), usage);
    gl.bindBuffer(gl.ARRAY_BUFFER, gpu.norm);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(mb.norm), usage);
    gl.bindBuffer(gl.ARRAY_BUFFER, gpu.col);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(mb.col), usage);
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, gpu.idx);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, isWebGL2 ? new Uint32Array(mb.idx) : new Uint16Array(mb.idx), usage);
    gpu.count = mb.idx.length;
  };

  const uMvp = gl.getUniformLocation(prog, "u_mvp");
  const uEye = gl.getUniformLocation(prog, "u_eye");
  const uFog = gl.getUniformLocation(prog, "u_fog");
  const uTime = gl.getUniformLocation(prog, "u_time");

  const drawBatch = (gpu) => {
    if (!gpu.count) return;
    gl.bindBuffer(gl.ARRAY_BUFFER, gpu.pos);
    gl.enableVertexAttribArray(0);
    gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 0, 0);
    gl.bindBuffer(gl.ARRAY_BUFFER, gpu.norm);
    gl.enableVertexAttribArray(1);
    gl.vertexAttribPointer(1, 3, gl.FLOAT, false, 0, 0);
    gl.bindBuffer(gl.ARRAY_BUFFER, gpu.col);
    gl.enableVertexAttribArray(2);
    gl.vertexAttribPointer(2, 3, gl.FLOAT, false, 0, 0);
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, gpu.idx);
    gl.drawElements(gl.TRIANGLES, gpu.count, isWebGL2 ? gl.UNSIGNED_INT : gl.UNSIGNED_SHORT, 0);
  };

  return {
    backend: isWebGL2 ? "webgl2" : "webgl1",
    isSoftware,
    uploadStatic: (mb) => upload(staticGpu, mb, gl.STATIC_DRAW),
    uploadDynamic: (mb) => upload(dynamicGpu, mb, gl.DYNAMIC_DRAW),
    render(mvp, eye, lodSpec, timeSec) {
      if (canvas.width <= 0 || canvas.height <= 0) return;
      gl.viewport(0, 0, canvas.width, canvas.height);
      gl.clearColor(lodSpec.sky[0], lodSpec.sky[1], lodSpec.sky[2], 1.0);
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
      gl.enable(gl.DEPTH_TEST);
      gl.useProgram(prog);
      gl.uniformMatrix4fv(uMvp, false, mvp);
      gl.uniform3fv(uEye, eye);
      gl.uniform3fv(uFog, lodSpec.fog);
      gl.uniform1f(uTime, timeSec);
      drawBatch(staticGpu);
      drawBatch(dynamicGpu);
    },
  };
}

export function createCanvas2dTensorRenderer(canvas) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return null;
  return {
    backend: "tensor-2d",
    isSoftware: true,
    uploadStatic() {},
    uploadDynamic() {},
    render(_mvp, _eye, lodSpec) {
      if (canvas.width <= 0 || canvas.height <= 0) return;
      const r = Math.round((lodSpec?.sky?.[0] ?? 0.18) * 255);
      const g = Math.round((lodSpec?.sky?.[1] ?? 0.3) * 255);
      const b = Math.round((lodSpec?.sky?.[2] ?? 0.38) * 255);
      ctx.fillStyle = `rgb(${r},${g},${b})`;
      ctx.fillRect(0, 0, canvas.width, canvas.height);
    },
    blitTensorRgba(rgbaBytes, width, height) {
      if (!rgbaBytes || width <= 0 || height <= 0 || typeof ImageData !== "function") return;
      const clamped = new Uint8ClampedArray(rgbaBytes.buffer, rgbaBytes.byteOffset, width * height * 4);
      const img = new ImageData(clamped, width, height);
      ctx.putImageData(img, 0, 0);
    },
  };
}

export async function createBestRenderer(canvas) {
  return (await createGlRenderer(canvas)) || createCanvas2dTensorRenderer(canvas);
}

export function buildGlbArrayBuffer(staticMb, dynMb) {
  const pos = new Float32Array(staticMb.pos.length + dynMb.pos.length);
  pos.set(staticMb.pos, 0);
  pos.set(dynMb.pos, staticMb.pos.length);

  const norm = new Float32Array(staticMb.norm.length + dynMb.norm.length);
  norm.set(staticMb.norm, 0);
  norm.set(dynMb.norm, staticMb.norm.length);

  const col = new Float32Array(staticMb.col.length + dynMb.col.length);
  col.set(staticMb.col, 0);
  col.set(dynMb.col, staticMb.col.length);

  const vOffset = staticMb.pos.length / 3;
  const idx = new Uint32Array(staticMb.idx.length + dynMb.idx.length);
  idx.set(staticMb.idx, 0);
  for (let i = 0; i < dynMb.idx.length; i++) idx[staticMb.idx.length + i] = dynMb.idx[i] + vOffset;

  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < pos.length; i += 3) {
    for (let k = 0; k < 3; k++) {
      if (pos[i + k] < min[k]) min[k] = pos[i + k];
      if (pos[i + k] > max[k]) max[k] = pos[i + k];
    }
  }

  const pBytes = pos.byteLength, nBytes = norm.byteLength, cBytes = col.byteLength, iBytes = idx.byteLength;
  const binLen = Math.ceil((pBytes + nBytes + cBytes + iBytes) / 4) * 4;
  const vCount = pos.length / 3;

  const jsonObj = {
    asset: { version: "2.0", generator: "Verdant Engine 0.1.0" },
    scene: 0,
    scenes: [{ name: "VerdantEngineScene", nodes: [0] }],
    nodes: [{ name: "StandardWorldNode", mesh: 0 }],
    meshes: [{
      name: "StandardWorldMesh",
      primitives: [{ attributes: { POSITION: 0, NORMAL: 1, COLOR_0: 2 }, indices: 3, material: 0 }],
    }],
    materials: [{ name: "StandardVertexPbr", pbrMetallicRoughness: { metallicFactor: 0.1, roughnessFactor: 0.78 } }],
    buffers: [{ byteLength: binLen }],
    bufferViews: [
      { buffer: 0, byteOffset: 0, byteLength: pBytes, target: 34962 },
      { buffer: 0, byteOffset: pBytes, byteLength: nBytes, target: 34962 },
      { buffer: 0, byteOffset: pBytes + nBytes, byteLength: cBytes, target: 34962 },
      { buffer: 0, byteOffset: pBytes + nBytes + cBytes, byteLength: iBytes, target: 34963 },
    ],
    accessors: [
      { bufferView: 0, componentType: 5126, count: vCount, type: "VEC3", min, max },
      { bufferView: 1, componentType: 5126, count: vCount, type: "VEC3" },
      { bufferView: 2, componentType: 5126, count: vCount, type: "VEC3" },
      { bufferView: 3, componentType: 5125, count: idx.length, type: "SCALAR" },
    ],
  };

  const enc = new TextEncoder();
  const rawJson = enc.encode(JSON.stringify(jsonObj));
  const jsonLen = Math.ceil(rawJson.byteLength / 4) * 4;
  const totalLen = 12 + 8 + jsonLen + 8 + binLen;
  const buf = new ArrayBuffer(totalLen);
  const dv = new DataView(buf);
  const u8 = new Uint8Array(buf);

  dv.setUint32(0, 0x46546c67, true);
  dv.setUint32(4, 2, true);
  dv.setUint32(8, totalLen, true);
  dv.setUint32(12, jsonLen, true);
  dv.setUint32(16, 0x4e4f534a, true);
  u8.fill(0x20, 20, 20 + jsonLen);
  u8.set(rawJson, 20);

  const binStart = 20 + jsonLen;
  dv.setUint32(binStart, binLen, true);
  dv.setUint32(binStart + 4, 0x004e4942, true);
  let off = binStart + 8;
  u8.set(new Uint8Array(pos.buffer), off); off += pBytes;
  u8.set(new Uint8Array(norm.buffer), off); off += nBytes;
  u8.set(new Uint8Array(col.buffer), off); off += cBytes;
  u8.set(new Uint8Array(idx.buffer), off);

  return buf;
}
