// Verdant Engine — Interactive 3D WebGL2 Browser Studio & Verified Blueprint Inspector.
// Pure ES module, zero inline style mutations (strict hash-locked CSP), 60fps procedural
// sanctuary world with cosmic expansion scrubbing, Autonomous Perimeter Sentinel Rover,
// Orbit + First-Person WASD camera, spatial WebAudio, and binary glTF 2.0 (.glb) export.

const EPOCHS = [
  {
    rank: 0,
    key: "epoch:0",
    short: "t_0",
    label: "t_0 · Cosmogenic (a = 0.125)",
    scale: 0.125,
    terrainAmp: 0.35,
    water: false,
    flora: false,
    builders: false,
    sky: [0.09, 0.08, 0.12],
    fog: [0.14, 0.11, 0.16],
  },
  {
    rank: 1,
    key: "epoch:1",
    short: "Lithic",
    label: "Lithic-Hydrologic (a = 0.25)",
    scale: 0.25,
    terrainAmp: 0.72,
    water: true,
    flora: false,
    builders: false,
    sky: [0.12, 0.17, 0.24],
    fog: [0.18, 0.24, 0.31],
  },
  {
    rank: 2,
    key: "epoch:2",
    short: "Biotic",
    label: "Biotic-Ecological (a = 0.625)",
    scale: 0.625,
    terrainAmp: 0.92,
    water: true,
    flora: true,
    builders: false,
    sky: [0.16, 0.27, 0.34],
    fog: [0.24, 0.35, 0.39],
  },
  {
    rank: 3,
    key: "epoch:3",
    short: "t_obs",
    label: "t_obs · Builders (a = 1.00)",
    scale: 1.0,
    terrainAmp: 1.0,
    water: true,
    flora: true,
    builders: true,
    sky: [0.18, 0.30, 0.38],
    fog: [0.30, 0.40, 0.44],
  },
];

const BLUEPRINTS = [
  { id: 0, query: "blueprint:0", name: "Harmonic Rotunda", guild: "Masonry", target: [0, 2.2, 0], dist: 16 },
  { id: 1, query: "blueprint:1", name: "Meridian Sundial", guild: "Gnomonics", target: [9.5, 1.2, -3.5], dist: 9 },
  { id: 2, query: "blueprint:2", name: "Modal Chime Pergola", guild: "Acoustics", target: [-9.0, 1.8, -4.0], dist: 10 },
  { id: 3, query: "blueprint:3", name: "Quartz Torsion Chamber", guild: "Masonry", target: [-6.5, 1.8, 9.0], dist: 10 },
  { id: 4, query: "blueprint:4", name: "Lagoon Pier Causeway", guild: "Hydrology", target: [5.5, 0.6, 8.5], dist: 11 },
  { id: 5, query: "blueprint:5", name: "Travertine Path Network", guild: "Masonry", target: [0, 0.8, 5.0], dist: 18 },
  { id: 6, query: "blueprint:6", name: "Perimeter Sentinel Rover", guild: "Automation", target: [18, 1.5, 0], dist: 11, followRover: true },
];

function terrainHeight(x, z, amp = 1.0) {
  const r = Math.hypot(x, z);
  const basin = -1.35 * Math.exp(-((x - 5.5) ** 2 + (z - 9.0) ** 2) / 52.0);
  const acropolis = 0.55 * Math.exp(-(r * r) / 70.0);
  const rim = 2.6 * Math.max(0, (r - 17.5) / 14.0) ** 1.45;
  const ridge =
    0.42 * Math.sin(x * 0.19 + 0.4) * Math.cos(z * 0.17 - 0.2) +
    0.21 * Math.sin((x + z) * 0.33);
  return (basin + acropolis + rim + ridge) * amp;
}

class MeshBuilder {
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

function buildStaticWorld(epoch) {
  const mb = new MeshBuilder();
  const amp = epoch.terrainAmp;
  const gridN = 44;
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
      if (!epoch.flora) {
        col = y < -0.25 ? [0.24, 0.26, 0.30] : y > 1.8 ? [0.56, 0.52, 0.46] : [0.40, 0.38, 0.35];
      } else if (y < -0.22) {
        col = [0.62, 0.57, 0.46];
      } else if (y > 2.1) {
        col = [0.64, 0.61, 0.55];
      } else {
        const lush = 0.5 + 0.5 * Math.sin(x * 0.25 + z * 0.2);
        col = [0.20 + 0.08 * lush, 0.44 + 0.12 * lush, 0.22 + 0.06 * lush];
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

  // Water mirror lagoon (Epoch >= 1)
  if (epoch.water) {
    const wSegs = 28;
    const wx = 5.5, wz = 9.0, wr = 7.4, wy = -0.32 * amp;
    const center = mb.addVertex(wx, wy, wz, 0, 1, 0, 0.14, 0.46, 0.58);
    for (let i = 0; i <= wSegs; i++) {
      const a = (i / wSegs) * Math.PI * 2;
      mb.addVertex(wx + Math.cos(a) * wr, wy, wz + Math.sin(a) * wr, 0, 1, 0, 0.18, 0.54, 0.66);
    }
    for (let i = 0; i < wSegs; i++) {
      mb.idx.push(center, center + 1 + i + 1, center + 1 + i);
    }

    // Quartz outcrops
    const crystals = [[-11, 6, 1.3], [12, 5, 1.1], [-13, -8, 1.4], [10, -11, 1.2]];
    for (const [qx, qz, qh] of crystals) {
      const qy = terrainHeight(qx, qz, amp);
      mb.addCylinder(qx, qy - 0.1, qz, 0.45, 0.08, qh, 6, [0.76, 0.84, 0.92]);
      mb.addCylinder(qx + 0.35, qy - 0.1, qz + 0.2, 0.28, 0.05, qh * 0.72, 6, [0.82, 0.76, 0.90]);
    }
  }

  // Phyllotactic botanical flora (Epoch >= 2)
  if (epoch.flora) {
    const golden = 2.39996323;
    for (let i = 0; i < 18; i++) {
      const ang = i * golden;
      const rad = 11.8 + (i % 5) * 2.1;
      const tx = Math.cos(ang) * rad;
      const tz = Math.sin(ang) * rad;
      if (Math.hypot(tx - 5.5, tz - 9.0) < 7.8) continue;
      const ty = terrainHeight(tx, tz, amp);
      const h = 2.1 + (i % 4) * 0.45;
      mb.addCylinder(tx, ty - 0.1, tz, 0.22, 0.13, h, 7, [0.36, 0.25, 0.16]);
      mb.addCylinder(tx, ty + h * 0.65, tz, 1.25, 0.18, h * 0.85, 8, [0.16, 0.46, 0.24]);
      mb.addCylinder(tx, ty + h * 1.05, tz, 0.92, 0.10, h * 0.65, 8, [0.22, 0.54, 0.28]);
    }
  }

  // Differentiated Builder Guild Monuments (Epoch == 3: t_obs)
  if (epoch.builders) {
    const travertine = [0.87, 0.84, 0.77];
    const bronze = [0.76, 0.58, 0.28];
    const basalt = [0.27, 0.29, 0.33];
    const y0 = terrainHeight(0, 0, amp);

    // 0. Harmonic Rotunda: stepped stylobate + 12 Doric columns + ring entablature
    mb.addCylinder(0, y0 - 0.15, 0, 5.4, 5.2, 0.35, 24, travertine);
    mb.addCylinder(0, y0 + 0.20, 0, 4.8, 4.7, 0.28, 24, [0.91, 0.88, 0.82]);
    const colCount = 12;
    for (let i = 0; i < colCount; i++) {
      const a = (i / colCount) * Math.PI * 2;
      mb.addCylinder(Math.cos(a) * 4.1, y0 + 0.48, Math.sin(a) * 4.1, 0.26, 0.21, 3.1, 10, travertine);
    }
    mb.addTorusRing(0, y0 + 3.68, 0, 4.1, 0.28, 24, 8, travertine, Math.PI * 0.5, 0);

    // 1. Equatorial Meridian Sundial
    const sy = terrainHeight(9.5, -3.5, amp);
    mb.addCylinder(9.5, sy - 0.1, -3.5, 1.35, 1.15, 0.85, 12, travertine);
    mb.addBox(9.5, sy + 1.15, -3.5, 0.14, 1.1, 1.2, bronze, 0.45);

    // 2. Wind-Driven Modal Chime Pergola
    const py = terrainHeight(-9.0, -4.0, amp);
    for (const [dx, dz] of [[-1.4, -1.4], [1.4, -1.4], [-1.4, 1.4], [1.4, 1.4]]) {
      mb.addCylinder(-9.0 + dx, py - 0.1, -4.0 + dz, 0.16, 0.14, 2.7, 8, basalt);
    }
    mb.addBox(-9.0, py + 2.65, -4.0, 3.3, 0.18, 0.22, bronze, 0);
    mb.addBox(-9.0, py + 2.65, -4.0, 0.22, 0.18, 3.3, bronze, 0);
    for (let c = 0; c < 6; c++) {
      const cx = -9.9 + c * 0.36;
      const clen = 0.75 + c * 0.14;
      mb.addCylinder(cx, py + 2.55 - clen, -4.0, 0.05, 0.05, clen, 6, [0.86, 0.70, 0.36]);
    }

    // 3. Piezoelectric Quartz Torsion Chamber
    const qcy = terrainHeight(-6.5, 9.0, amp);
    mb.addCylinder(-6.5, qcy - 0.1, 9.0, 1.9, 1.75, 0.55, 8, basalt);
    mb.addCylinder(-6.5, qcy + 0.45, 9.0, 0.45, 0.08, 2.25, 6, [0.84, 0.90, 0.98]);

    // 4. Mirror Lagoon Stone Pier Causeway
    for (let s = 0; s < 5; s++) {
      const pz = 2.8 + s * 1.45;
      const px = 2.4 + s * 0.75;
      mb.addBox(px, 0.05, pz, 1.35, 0.18, 1.35, travertine, 0.48);
      mb.addCylinder(px, -0.9, pz, 0.18, 0.22, 0.95, 8, basalt);
    }

    // 5. Geodesic Travertine Path Network
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

function buildDynamicActors(epoch, timeSec) {
  const mb = new MeshBuilder();
  if (!epoch.builders) return { mb, roverPos: [16.5, 0, 0] };

  const amp = epoch.terrainAmp;
  const y0 = terrainHeight(0, 0, amp);
  const gold = [0.85, 0.68, 0.32];

  // Rotating Cardan Astrolabe Rings inside the Harmonic Rotunda
  mb.addTorusRing(0, y0 + 2.25, 0, 1.55, 0.06, 20, 6, gold, timeSec * 0.55, 0.35);
  mb.addTorusRing(0, y0 + 2.25, 0, 1.18, 0.05, 18, 6, [0.72, 0.82, 0.88], -timeSec * 0.8, 1.05);

  // 6. Autonomous Perimeter Sentinel Rover orbiting the 16.5m geodesic spline
  const theta = timeSec * 0.28;
  const rx = Math.cos(theta) * 16.5;
  const rz = Math.sin(theta) * 16.5;
  const ry = terrainHeight(rx, rz, amp) + 0.36;
  const heading = -theta;

  mb.addBox(rx, ry, rz, 0.95, 0.34, 1.45, [0.80, 0.62, 0.30], heading);
  mb.addBox(rx, ry + 0.28, rz - 0.1, 0.58, 0.24, 0.68, [0.90, 0.88, 0.82], heading);
  mb.addCylinder(rx, ry + 0.38, rz, 0.08, 0.06, 0.52, 8, [0.30, 0.34, 0.38]);
  mb.addBox(rx, ry + 0.92, rz, 0.32, 0.16, 0.22, [0.24, 0.78, 0.86], heading + Math.sin(timeSec * 2.2) * 0.6);

  const c = Math.cos(heading), s = Math.sin(heading);
  for (const [wx, wz] of [[-0.55, -0.52], [0.55, -0.52], [-0.58, 0], [0.58, 0], [-0.55, 0.52], [0.55, 0.52]]) {
    const px = rx + wx * c - wz * s;
    const pz = rz + wx * s + wz * c;
    mb.addCylinder(px, ry - 0.34, pz, 0.20, 0.20, 0.22, 8, [0.20, 0.22, 0.25]);
  }

  return { mb, roverPos: [rx, ry, rz] };
}

function mat4Perspective(fovRad, aspect, near, far) {
  const f = 1.0 / Math.tan(fovRad * 0.5);
  const nf = 1.0 / (near - far);
  return new Float32Array([
    f / aspect, 0, 0, 0,
    0, f, 0, 0,
    0, 0, (far + near) * nf, -1,
    0, 0, 2 * far * near * nf, 0,
  ]);
}

function mat4LookAt(eye, target, up) {
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

function mat4Mul(a, b) {
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

function createGlRenderer(canvas) {
  const gl = canvas.getContext("webgl2", { antialias: true, depth: true, alpha: false })
    || canvas.getContext("webgl", { antialias: true, depth: true, alpha: false });
  if (!gl) return null;

  const isWebGL2 = typeof WebGL2RenderingContext !== "undefined" && gl instanceof WebGL2RenderingContext;
  if (!isWebGL2) gl.getExtension("OES_element_index_uint");

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
         vec3 amb = mix(vec3(0.16, 0.15, 0.14), vec3(0.38, 0.48, 0.56), hemi);
         float spec = pow(max(dot(n, h), 0.0), 36.0) * 0.28;
         vec3 base = v_col;
         if (v_col.b > v_col.r * 1.6 && abs(n.y) > 0.9) {
           float ripple = 0.04 * sin(v_pos.x * 2.8 + u_time * 2.4) * cos(v_pos.z * 2.6 - u_time * 1.9);
           base += ripple;
           spec += pow(1.0 - max(dot(n, v), 0.0), 3.0) * 0.35;
         }
         vec3 lit = base * (amb + vec3(1.02, 0.95, 0.82) * ndl * 0.82) + spec;
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
    uploadStatic: (mb) => upload(staticGpu, mb, gl.STATIC_DRAW),
    uploadDynamic: (mb) => upload(dynamicGpu, mb, gl.DYNAMIC_DRAW),
    render(mvp, eye, epoch, timeSec) {
      gl.viewport(0, 0, canvas.width, canvas.height);
      gl.clearColor(epoch.sky[0], epoch.sky[1], epoch.sky[2], 1.0);
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
      gl.enable(gl.DEPTH_TEST);
      gl.useProgram(prog);
      gl.uniformMatrix4fv(uMvp, false, mvp);
      gl.uniform3fv(uEye, eye);
      gl.uniform3fv(uFog, epoch.fog);
      gl.uniform1f(uTime, timeSec);
      drawBatch(staticGpu);
      drawBatch(dynamicGpu);
    },
  };
}

function buildGlbBinary(staticMb, dynMb) {
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
    scenes: [{ name: "VerdantSanctuary", nodes: [0] }],
    nodes: [{ name: "SanctuaryWorld", mesh: 0 }],
    meshes: [{
      name: "SanctuaryMesh",
      primitives: [{ attributes: { POSITION: 0, NORMAL: 1, COLOR_0: 2 }, indices: 3, material: 0 }],
    }],
    materials: [{ name: "SanctuaryVertexPbr", pbrMetallicRoughness: { metallicFactor: 0.1, roughnessFactor: 0.78 } }],
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
  u8.set(rawJson, 20);
  for (let i = rawJson.byteLength; i < jsonLen; i++) u8[20 + i] = 0x20;

  const binStart = 20 + jsonLen;
  dv.setUint32(binStart, binLen, true);
  dv.setUint32(binStart + 4, 0x004e4942, true);
  const dataStart = binStart + 8;
  u8.set(new Uint8Array(pos.buffer), dataStart);
  u8.set(new Uint8Array(norm.buffer), dataStart + pBytes);
  u8.set(new Uint8Array(col.buffer), dataStart + pBytes + nBytes);
  u8.set(new Uint8Array(idx.buffer), dataStart + pBytes + nBytes + cBytes);
  return new Blob([buf], { type: "model/gltf-binary" });
}

export default async function start({ app, reason, $, chip, show, parse }) {
  const canvas = $("ve-canvas");
  const statusEl = $("ve-status");
  const epochBtn = $("ve-epoch-btn");
  const camBtn = $("ve-cam-btn");
  const audioBtn = $("ve-audio-btn");
  const glbBtn = $("ve-glb-btn");
  const hudEpoch = $("ve-hud-epoch");
  const hudStats = $("ve-hud-stats");
  const telemetryList = $("ve-telemetry");

  let epochIdx = 3;
  let camMode = "orbit";
  let followRover = false;
  let orbitYaw = 0.68;
  let orbitPitch = 0.42;
  let orbitDist = 24.0;
  let target = [0, 1.8, 1.5];
  let walkPos = [0, 2.2, 13.5];
  let walkYaw = Math.PI;
  let walkPitch = -0.08;
  const keys = new Set();

  let staticMesh = buildStaticWorld(EPOCHS[epochIdx]);
  let lastDynMesh = buildDynamicActors(EPOCHS[epochIdx], 0);
  const renderer = canvas ? createGlRenderer(canvas) : null;
  if (renderer) renderer.uploadStatic(staticMesh);

  async function queryVerifier(reqText) {
    if (!app) return null;
    try {
      return show(await app.dispatch(parse(reqText)));
    } catch {
      return null;
    }
  }

  async function syncVerifierBanner(query = EPOCHS[epochIdx].key) {
    if (!app) {
      chip(statusEl, reason ? "Verifier unavailable" : "Engine offline", "bad");
      return;
    }
    const res = await queryVerifier(query);
    if (!res) return;
    if (res.startsWith("conformant:")) {
      const ep = EPOCHS[epochIdx];
      chip(statusEl, `${ep.short} · ${ep.builders ? "7/7 Equilibrium" : "Causal Epoch OK"}`, "good");
    } else if (res.startsWith("refused:")) {
      chip(statusEl, res.replace("refused:", "Refused: "), "warn");
    } else {
      chip(statusEl, res, "bad");
    }
  }

  async function populateBlueprints() {
    if (!telemetryList) return;
    telemetryList.replaceChildren();
    for (const bp of BLUEPRINTS) {
      const li = document.createElement("li");
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "again";
      const label = document.createElement("span");
      label.textContent = bp.name;
      const badge = document.createElement("span");
      const verdict = await queryVerifier(bp.query);
      const ok = verdict && verdict.startsWith("conformant:");
      badge.className = ok ? "tag good" : "tag";
      badge.textContent = ok ? bp.guild.replace(/([a-z])([A-Z])/g, "$1 $2") : "Pending";
      btn.append(label, badge);
      btn.addEventListener("click", async () => {
        if (epochIdx !== 3) setEpoch(3);
        camMode = "orbit";
        camBtn.textContent = "Cam: Orbit";
        followRover = Boolean(bp.followRover);
        target = [...bp.target];
        orbitDist = bp.dist;
        await syncVerifierBanner(bp.query);
      });
      li.append(btn);
      telemetryList.append(li);
    }
  }

  function setEpoch(nextIdx) {
    epochIdx = ((nextIdx % EPOCHS.length) + EPOCHS.length) % EPOCHS.length;
    const ep = EPOCHS[epochIdx];
    epochBtn.textContent = `Epoch: ${ep.short}`;
    hudEpoch.textContent = ep.label;
    if (!ep.builders) followRover = false;
    staticMesh = buildStaticWorld(ep);
    if (renderer) renderer.uploadStatic(staticMesh);
    syncVerifierBanner(ep.key);
  }

  epochBtn?.addEventListener("click", () => setEpoch(epochIdx + 1));

  camBtn?.addEventListener("click", () => {
    camMode = camMode === "orbit" ? "walk" : "orbit";
    camBtn.textContent = camMode === "orbit" ? "Cam: Orbit" : "Cam: Walk";
  });

  // Spatial WebAudio synthesis (Euler-Bernoulli modal chimes + lagoon shimmer)
  let audioCtx = null;
  let audioTimer = 0;
  let audioOn = false;

  function triggerModalChime(eye) {
    if (!audioCtx || !audioOn) return;
    const dist = Math.hypot(eye[0] + 9.0, eye[1] - 2.0, eye[2] + 4.0);
    const atten = Math.min(0.14, 0.9 / (1.0 + 0.18 * dist));
    const modes = [293.66, 440.0, 587.33, 659.25, 880.0];
    const baseFreq = modes[Math.floor(Math.random() * modes.length)];
    const now = audioCtx.currentTime;
    const osc = audioCtx.createOscillator();
    const gain = audioCtx.createGain();
    osc.type = "sine";
    osc.frequency.setValueAtTime(baseFreq, now);
    gain.gain.setValueAtTime(0.0001, now);
    gain.gain.exponentialRampToValueAtTime(atten, now + 0.02);
    gain.gain.exponentialRampToValueAtTime(0.0001, now + 1.8);
    osc.connect(gain).connect(audioCtx.destination);
    osc.start(now);
    osc.stop(now + 1.85);
  }

  audioBtn?.addEventListener("click", () => {
    const Ctx = window.AudioContext || window.webkitAudioContext;
    if (!Ctx) return;
    audioOn = !audioOn;
    audioBtn.setAttribute("aria-pressed", String(audioOn));
    audioBtn.textContent = audioOn ? "Audio: On" : "Audio: Off";
    if (audioOn) {
      audioCtx ??= new Ctx();
      audioCtx.resume?.();
      clearInterval(audioTimer);
      audioTimer = window.setInterval(() => triggerModalChime(currentEye), 1350);
      triggerModalChime(currentEye);
    } else {
      clearInterval(audioTimer);
    }
  });

  glbBtn?.addEventListener("click", () => {
    const blob = buildGlbBinary(staticMesh, lastDynMesh.mb);
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `verdant-engine-${EPOCHS[epochIdx].short}.glb`;
    a.click();
    setTimeout(() => URL.revokeObjectURL(url), 1500);
  });

  // Pointer & keyboard navigation
  let dragging = false;
  let lastX = 0, lastY = 0;
  canvas?.addEventListener("pointerdown", (e) => {
    dragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
    canvas.setPointerCapture?.(e.pointerId);
  });
  canvas?.addEventListener("pointermove", (e) => {
    if (!dragging) return;
    const dx = e.clientX - lastX;
    const dy = e.clientY - lastY;
    lastX = e.clientX;
    lastY = e.clientY;
    if (camMode === "orbit") {
      orbitYaw -= dx * 0.0075;
      orbitPitch = Math.max(0.08, Math.min(1.38, orbitPitch + dy * 0.0065));
    } else {
      walkYaw -= dx * 0.0055;
      walkPitch = Math.max(-1.1, Math.min(1.1, walkPitch - dy * 0.0055));
    }
  });
  const endDrag = () => { dragging = false; };
  canvas?.addEventListener("pointerup", endDrag);
  canvas?.addEventListener("pointercancel", endDrag);
  canvas?.addEventListener("wheel", (e) => {
    e.preventDefault();
    orbitDist = Math.max(5.0, Math.min(42.0, orbitDist + e.deltaY * 0.02));
  }, { passive: false });

  addEventListener("keydown", (e) => {
    if (/^(INPUT|TEXTAREA|SELECT)$/.test(document.activeElement?.tagName ?? "")) return;
    keys.add(e.code);
  });
  addEventListener("keyup", (e) => keys.delete(e.code));

  let currentEye = [16, 11, 18];
  let frames = 0;
  let lastFpsStamp = performance.now();
  let shownFps = 60;
  let prevNow = performance.now();

  function frame(now) {
    const dt = Math.min(0.05, Math.max(0.001, (now - prevNow) * 0.001));
    prevNow = now;
    const timeSec = now * 0.001;
    const ep = EPOCHS[epochIdx];

    if (canvas) {
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const w = Math.max(1, Math.floor(canvas.clientWidth * dpr));
      const h = Math.max(1, Math.floor(canvas.clientHeight * dpr));
      if (canvas.width !== w || canvas.height !== h) {
        canvas.width = w;
        canvas.height = h;
      }
    }

    lastDynMesh = buildDynamicActors(ep, timeSec);
    if (followRover && ep.builders) {
      target = [...lastDynMesh.roverPos];
    }

    let lookTarget;
    if (camMode === "orbit") {
      if (!dragging && !followRover) orbitYaw += dt * 0.08;
      const cp = Math.cos(orbitPitch), sp = Math.sin(orbitPitch);
      currentEye = [
        target[0] + Math.sin(orbitYaw) * cp * orbitDist,
        Math.max(terrainHeight(target[0], target[2], ep.terrainAmp) + 0.8, target[1] + sp * orbitDist),
        target[2] + Math.cos(orbitYaw) * cp * orbitDist,
      ];
      lookTarget = target;
    } else {
      const fwd = (keys.has("KeyW") || keys.has("ArrowUp") ? 1 : 0) - (keys.has("KeyS") || keys.has("ArrowDown") ? 1 : 0);
      const strafe = (keys.has("KeyD") || keys.has("ArrowRight") ? 1 : 0) - (keys.has("KeyA") || keys.has("ArrowLeft") ? 1 : 0);
      const speed = 7.5 * dt;
      walkPos[0] += (Math.sin(walkYaw) * fwd - Math.cos(walkYaw) * strafe) * speed;
      walkPos[2] += (Math.cos(walkYaw) * fwd + Math.sin(walkYaw) * strafe) * speed;
      walkPos[0] = Math.max(-25, Math.min(25, walkPos[0]));
      walkPos[2] = Math.max(-25, Math.min(25, walkPos[2]));
      walkPos[1] = Math.max(ep.water ? -0.1 : -2.0, terrainHeight(walkPos[0], walkPos[2], ep.terrainAmp)) + 1.72;
      currentEye = [...walkPos];
      lookTarget = [
        walkPos[0] + Math.sin(walkYaw) * Math.cos(walkPitch),
        walkPos[1] + Math.sin(walkPitch),
        walkPos[2] + Math.cos(walkYaw) * Math.cos(walkPitch),
      ];
    }

    if (renderer && canvas.width > 0 && canvas.height > 0) {
      renderer.uploadDynamic(lastDynMesh.mb);
      const proj = mat4Perspective(Math.PI / 3.4, canvas.width / canvas.height, 0.25, 140.0);
      const view = mat4LookAt(currentEye, lookTarget, [0, 1, 0]);
      renderer.render(mat4Mul(proj, view), currentEye, ep, timeSec);
    }

    frames++;
    if (now - lastFpsStamp >= 600) {
      shownFps = Math.max(1, Math.round((frames * 1000) / (now - lastFpsStamp)));
      frames = 0;
      lastFpsStamp = now;
      if (hudStats) {
        const tris = Math.round((staticMesh.idx.length + lastDynMesh.mb.idx.length) / 3);
        hudStats.textContent = `${shownFps} fps · ${tris} tris · ${ep.builders ? "7 blueprints + rover" : "evolving"}`;
      }
    }

    requestAnimationFrame(frame);
  }

  await syncVerifierBanner("summary");
  await populateBlueprints();
  requestAnimationFrame(frame);
}
