/**
 * Orbital Beacon Arena — Playable 3D WebGL2 Starter Game on Verdant Engine.
 *
 * Move the Kinetic Sphere with WASD / Arrow keys across the procedural basin heightfield
 * to activate all three Crystalline Relay Beacons while the autonomous Patrol Drone orbits the
 * arena perimeter. Drag horizontally to orbit the camera.
 */

const canvas = document.getElementById("arena");
const scoreEl = document.getElementById("hud-score");
const hintEl = document.getElementById("hud-hint");
const audioBtn = document.getElementById("audio-btn");
const resetBtn = document.getElementById("reset-btn");

const BEACONS = [
  { name: "North Spire", x: 0.0, z: -6.5, baseColor: [0.92, 0.66, 0.28], freq: 220, active: false },
  { name: "East Spire", x: 6.0, z: 3.4, baseColor: [0.34, 0.76, 0.92], freq: 275, active: false },
  { name: "West Spire", x: -6.0, z: 3.4, baseColor: [0.78, 0.52, 0.94], freq: 330, active: false },
];

const MODAL_BAR_RATIOS = [9 / 9, 25 / 9, 49 / 9, 81 / 9];

function terrainHeight(x, z) {
  const r2 = x * x + z * z;
  const bowl = Math.min(2.6, r2 * 0.012);
  const ripple = 0.22 * Math.sin(x * 0.35) * Math.cos(z * 0.35);
  return bowl + ripple - 0.5;
}

let audioCtx = null;
let audioEnabled = false;

function playBeaconChord(baseHz, panValue) {
  if (!audioEnabled) return;
  if (!audioCtx) {
    const Ctx = window.AudioContext || window.webkitAudioContext;
    if (!Ctx) return;
    audioCtx = new Ctx();
  }
  if (audioCtx.state === "suspended") audioCtx.resume();
  const now = audioCtx.currentTime;
  const panner = audioCtx.createStereoPanner ? audioCtx.createStereoPanner() : null;
  if (panner) {
    panner.pan.setValueAtTime(Math.max(-1, Math.min(1, panValue)), now);
    panner.connect(audioCtx.destination);
  }
  MODAL_BAR_RATIOS.forEach((ratio, idx) => {
    const osc = audioCtx.createOscillator();
    const gain = audioCtx.createGain();
    osc.type = "sine";
    osc.frequency.setValueAtTime(baseHz * Math.sqrt(ratio), now);
    const peak = 0.09 / (idx + 1);
    gain.gain.setValueAtTime(0.0001, now);
    gain.gain.exponentialRampToValueAtTime(peak, now + 0.02);
    gain.gain.exponentialRampToValueAtTime(0.0001, now + 1.4 - idx * 0.18);
    osc.connect(gain);
    gain.connect(panner || audioCtx.destination);
    osc.start(now);
    osc.stop(now + 1.45);
  });
}

audioBtn?.addEventListener("click", () => {
  audioEnabled = !audioEnabled;
  audioBtn.setAttribute("aria-pressed", String(audioEnabled));
  audioBtn.textContent = audioEnabled ? "Audio: On" : "Audio: Off";
  if (audioEnabled) playBeaconChord(264, 0);
});

const player = { x: 0, z: 7.5, vx: 0, vz: 0 };
const drone = { angle: 0, radius: 9.8 };
let camYaw = 0.0;
const keys = new Set();

function updateHud(droneBump = false) {
  const count = BEACONS.filter((b) => b.active).length;
  if (scoreEl) {
    scoreEl.textContent =
      count === BEACONS.length
        ? `Beacons Activated: ${count} / ${BEACONS.length} · Arena Complete!`
        : `Beacons Activated: ${count} / ${BEACONS.length}`;
  }
  if (hintEl) {
    if (droneBump) {
      hintEl.textContent = "Patrol Drone deflected your sphere! Steer clear of its orbit track.";
    } else if (count === BEACONS.length) {
      hintEl.textContent = "All three relay beacons are active. Press Reset Arena to play again.";
    }
  }
}

resetBtn?.addEventListener("click", () => {
  player.x = 0;
  player.z = 7.5;
  player.vx = 0;
  player.vz = 0;
  BEACONS.forEach((b) => { b.active = false; });
  if (hintEl) {
    hintEl.textContent = "Move with WASD or Arrow keys (drag to orbit camera) to activate all three relay beacons.";
  }
  updateHud();
});

window.addEventListener("keydown", (e) => keys.add(e.code));
window.addEventListener("keyup", (e) => keys.delete(e.code));

let dragging = false;
let lastX = 0;
canvas?.addEventListener("pointerdown", (e) => {
  dragging = true;
  lastX = e.clientX;
});
window.addEventListener("pointerup", () => { dragging = false; });
window.addEventListener("pointermove", (e) => {
  if (dragging) {
    camYaw += (e.clientX - lastX) * 0.007;
    lastX = e.clientX;
  }
});

// Build procedural meshes (interleaved pos[3], normal[3], color[3]):
function pushTri(out, a, b, c, col) {
  const ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2];
  const vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
  let nx = uy * vz - uz * vy;
  let ny = uz * vx - ux * vz;
  let nz = ux * vy - uy * vx;
  const len = Math.hypot(nx, ny, nz) || 1;
  nx /= len; ny /= len; nz /= len;
  for (const p of [a, b, c]) {
    out.push(p[0], p[1], p[2], nx, ny, nz, col[0], col[1], col[2]);
  }
}

function buildTerrainVerts() {
  const out = [];
  const N = 24;
  const half = 14;
  for (let iz = 0; iz < N; iz++) {
    for (let ix = 0; ix < N; ix++) {
      const x0 = -half + (ix / N) * half * 2;
      const x1 = -half + ((ix + 1) / N) * half * 2;
      const z0 = -half + (iz / N) * half * 2;
      const z1 = -half + ((iz + 1) / N) * half * 2;
      const y00 = terrainHeight(x0, z0);
      const y10 = terrainHeight(x1, z0);
      const y01 = terrainHeight(x0, z1);
      const y11 = terrainHeight(x1, z1);
      const hAvg = (y00 + y11) * 0.5;
      const col = hAvg < -0.22
        ? [0.16, 0.42, 0.48]
        : [0.22 + hAvg * 0.08, 0.36 + hAvg * 0.06, 0.27 + hAvg * 0.04];
      pushTri(out, [x0, y00, z0], [x0, y01, z1], [x1, y10, z0], col);
      pushTri(out, [x1, y10, z0], [x0, y01, z1], [x1, y11, z1], col);
    }
  }
  return new Float32Array(out);
}

function buildOctahedronVerts(radius, heightScale, col) {
  const out = [];
  const top = [0, radius * heightScale, 0];
  const bot = [0, -radius * heightScale, 0];
  const ring = [
    [radius, 0, 0],
    [0, 0, radius],
    [-radius, 0, 0],
    [0, 0, -radius],
  ];
  for (let i = 0; i < 4; i++) {
    const a = ring[i];
    const b = ring[(i + 1) % 4];
    pushTri(out, top, b, a, col);
    pushTri(out, bot, a, b, col);
  }
  return new Float32Array(out);
}

function mat4Perspective(fovRad, aspect, near, far) {
  const f = 1 / Math.tan(fovRad * 0.5);
  const nf = 1 / (near - far);
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
  for (let c = 0; c < 4; c++) {
    for (let r = 0; r < 4; r++) {
      out[c * 4 + r] =
        a[0 * 4 + r] * b[c * 4 + 0] +
        a[1 * 4 + r] * b[c * 4 + 1] +
        a[2 * 4 + r] * b[c * 4 + 2] +
        a[3 * 4 + r] * b[c * 4 + 3];
    }
  }
  return out;
}

const gl = canvas?.getContext("webgl2", { antialias: true, depth: true });
if (gl) {
  const vs = gl.createShader(gl.VERTEX_SHADER);
  gl.shaderSource(vs, `#version 300 es
    layout(location=0) in vec3 a_pos;
    layout(location=1) in vec3 a_nrm;
    layout(location=2) in vec3 a_col;
    uniform mat4 u_vp;
    uniform vec3 u_offset;
    uniform float u_spin;
    uniform vec3 u_tint;
    out vec3 v_col;
    void main() {
      float c = cos(u_spin), s = sin(u_spin);
      vec3 p = vec3(a_pos.x * c - a_pos.z * s, a_pos.y, a_pos.x * s + a_pos.z * c) + u_offset;
      vec3 n = vec3(a_nrm.x * c - a_nrm.z * s, a_nrm.y, a_nrm.x * s + a_nrm.z * c);
      vec3 lightDir = normalize(vec3(0.45, 0.82, 0.35));
      float diff = max(dot(normalize(n), lightDir), 0.0);
      v_col = (a_col * u_tint) * (0.34 + 0.72 * diff);
      gl_Position = u_vp * vec4(p, 1.0);
    }`);
  gl.compileShader(vs);

  const fs = gl.createShader(gl.FRAGMENT_SHADER);
  gl.shaderSource(fs, `#version 300 es
    precision highp float;
    in vec3 v_col;
    out vec4 o_col;
    void main() {
      o_col = vec4(pow(max(v_col, vec3(0.0)), vec3(1.0 / 2.2)), 1.0);
    }`);
  gl.compileShader(fs);

  const prog = gl.createProgram();
  gl.attachShader(prog, vs);
  gl.attachShader(prog, fs);
  gl.linkProgram(prog);

  function uploadMesh(verts) {
    const vao = gl.createVertexArray();
    gl.bindVertexArray(vao);
    const buf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.bufferData(gl.ARRAY_BUFFER, verts, gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0);
    gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 36, 0);
    gl.enableVertexAttribArray(1);
    gl.vertexAttribPointer(1, 3, gl.FLOAT, false, 36, 12);
    gl.enableVertexAttribArray(2);
    gl.vertexAttribPointer(2, 3, gl.FLOAT, false, 36, 24);
    return { vao, count: verts.length / 9 };
  }

  const terrainGpu = uploadMesh(buildTerrainVerts());
  const crystalGpu = uploadMesh(buildOctahedronVerts(0.65, 2.1, [1, 1, 1]));
  const orbGpu = uploadMesh(buildOctahedronVerts(0.55, 1.15, [0.95, 0.88, 0.62]));
  const droneGpu = uploadMesh(buildOctahedronVerts(0.75, 0.7, [0.92, 0.38, 0.32]));

  const uVp = gl.getUniformLocation(prog, "u_vp");
  const uOffset = gl.getUniformLocation(prog, "u_offset");
  const uSpin = gl.getUniformLocation(prog, "u_spin");
  const uTint = gl.getUniformLocation(prog, "u_tint");

  let lastTime = performance.now();

  function frame(now) {
    const dt = Math.min(0.05, (now - lastTime) * 0.001);
    lastTime = now;

    // Step player movement relative to camera yaw:
    let ix = 0, iz = 0;
    if (keys.has("KeyW") || keys.has("ArrowUp")) iz -= 1;
    if (keys.has("KeyS") || keys.has("ArrowDown")) iz += 1;
    if (keys.has("KeyA") || keys.has("ArrowLeft")) ix -= 1;
    if (keys.has("KeyD") || keys.has("ArrowRight")) ix += 1;
    const il = Math.hypot(ix, iz);
    if (il > 0) {
      ix /= il; iz /= il;
      const cy = Math.cos(camYaw), sy = Math.sin(camYaw);
      const wx = ix * cy + iz * sy;
      const wz = -ix * sy + iz * cy;
      player.vx += wx * 18 * dt;
      player.vz += wz * 18 * dt;
    }
    player.vx *= Math.pow(0.08, dt);
    player.vz *= Math.pow(0.08, dt);
    player.x = Math.max(-12.5, Math.min(12.5, player.x + player.vx * dt));
    player.z = Math.max(-12.5, Math.min(12.5, player.z + player.vz * dt));

    // Step autonomous Patrol Drone along perimeter circle:
    drone.angle += dt * 0.65;
    const dx = Math.cos(drone.angle) * drone.radius;
    const dz = Math.sin(drone.angle) * drone.radius;
    const distDrone = Math.hypot(player.x - dx, player.z - dz);
    if (distDrone < 1.35) {
      const nx = (player.x - dx) / (distDrone || 1);
      const nz = (player.z - dz) / (distDrone || 1);
      player.vx = nx * 9;
      player.vz = nz * 9;
      playBeaconChord(146.8, nx);
      updateHud(true);
    }

    // Check GJK-style proximity activation with each Relay Beacon:
    for (const b of BEACONS) {
      if (!b.active && Math.hypot(player.x - b.x, player.z - b.z) < 1.45) {
        b.active = true;
        playBeaconChord(b.freq, Math.max(-1, Math.min(1, (b.x - player.x) * 0.25)));
        updateHud(false);
      }
    }

    const dpr = Math.min(2, window.devicePixelRatio || 1);
    const w = Math.max(1, Math.floor(canvas.clientWidth * dpr));
    const h = Math.max(1, Math.floor(canvas.clientHeight * dpr));
    if (canvas.width !== w || canvas.height !== h) {
      canvas.width = w;
      canvas.height = h;
    }

    gl.viewport(0, 0, w, h);
    gl.clearColor(0.06, 0.08, 0.1, 1);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.enable(gl.DEPTH_TEST);

    const camDist = 16.5;
    const eye = [
      player.x * 0.35 + Math.sin(camYaw) * camDist,
      9.2,
      player.z * 0.35 + Math.cos(camYaw) * camDist,
    ];
    const vp = mat4Mul(
      mat4Perspective(Math.PI / 3.6, w / h, 0.2, 80.0),
      mat4LookAt(eye, [player.x * 0.35, 0.8, player.z * 0.35], [0, 1, 0]),
    );

    gl.useProgram(prog);
    gl.uniformMatrix4fv(uVp, false, vp);

    // Draw valley terrain:
    gl.bindVertexArray(terrainGpu.vao);
    gl.uniform3f(uOffset, 0, 0, 0);
    gl.uniform1f(uSpin, 0);
    gl.uniform3f(uTint, 1, 1, 1);
    gl.drawArrays(gl.TRIANGLES, 0, terrainGpu.count);

    // Draw 3 Crystalline Relay Beacons:
    gl.bindVertexArray(crystalGpu.vao);
    const tSec = now * 0.001;
    BEACONS.forEach((b, idx) => {
      const ry = terrainHeight(b.x, b.z) + 1.55 + Math.sin(tSec * 2.2 + idx) * 0.18;
      gl.uniform3f(uOffset, b.x, ry, b.z);
      gl.uniform1f(uSpin, tSec * (b.active ? 2.4 : 0.7) + idx);
      const col = b.active ? [0.42, 0.96, 0.72] : b.baseColor;
      gl.uniform3f(uTint, col[0], col[1], col[2]);
      gl.drawArrays(gl.TRIANGLES, 0, crystalGpu.count);
    });

    // Draw player Kinetic Sphere:
    gl.bindVertexArray(orbGpu.vao);
    gl.uniform3f(uOffset, player.x, terrainHeight(player.x, player.z) + 0.7, player.z);
    gl.uniform1f(uSpin, tSec * 1.8);
    gl.uniform3f(uTint, 1, 1, 1);
    gl.drawArrays(gl.TRIANGLES, 0, orbGpu.count);

    // Draw patrolling Drone:
    gl.bindVertexArray(droneGpu.vao);
    gl.uniform3f(uOffset, dx, terrainHeight(dx, dz) + 1.15, dz);
    gl.uniform1f(uSpin, -tSec * 3.0);
    gl.uniform3f(uTint, 1, 1, 1);
    gl.drawArrays(gl.TRIANGLES, 0, droneGpu.count);

    requestAnimationFrame(frame);
  }
  requestAnimationFrame(frame);
}
