const canvas = document.getElementById("arena");
const gl = canvas.getContext("webgl2", { antialias: true, depth: true });

if (gl) {
  const vs = gl.createShader(gl.VERTEX_SHADER);
  gl.shaderSource(vs, `#version 300 es
    layout(location=0) in vec3 a_pos;
    layout(location=1) in vec3 a_col;
    uniform float u_angle;
    uniform float u_aspect;
    out vec3 v_col;
    void main() {
      float c = cos(u_angle), s = sin(u_angle);
      vec3 p = vec3(a_pos.x * c - a_pos.z * s, a_pos.y, a_pos.x * s + a_pos.z * c);
      p.z -= 5.0;
      float f = 1.732;
      v_col = a_col;
      gl_Position = vec4((p.x * f) / u_aspect, p.y * f, p.z * 1.02 + 0.4, -p.z);
    }`);
  gl.compileShader(vs);

  const fs = gl.createShader(gl.FRAGMENT_SHADER);
  gl.shaderSource(fs, `#version 300 es
    precision highp float;
    in vec3 v_col;
    out vec4 o_col;
    void main() { o_col = vec4(v_col, 1.0); }`);
  gl.compileShader(fs);

  const prog = gl.createProgram();
  gl.attachShader(prog, vs);
  gl.attachShader(prog, fs);
  gl.linkProgram(prog);

  const verts = new Float32Array([
    0, 1.2, 0,   0.85, 0.68, 0.30,
    -1, -0.8, 1, 0.25, 0.62, 0.45,
    1, -0.8, 1,  0.25, 0.62, 0.45,
    0, 1.2, 0,   0.85, 0.68, 0.30,
    1, -0.8, 1,  0.20, 0.52, 0.64,
    0, -0.8, -1.2, 0.20, 0.52, 0.64,
    0, 1.2, 0,   0.85, 0.68, 0.30,
    0, -0.8, -1.2, 0.72, 0.58, 0.38,
    -1, -0.8, 1, 0.72, 0.58, 0.38,
  ]);
  const buf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buf);
  gl.bufferData(gl.ARRAY_BUFFER, verts, gl.STATIC_DRAW);

  let yaw = 0.4;
  let drag = false, lx = 0;
  canvas.addEventListener("pointerdown", (e) => { drag = true; lx = e.clientX; });
  addEventListener("pointerup", () => { drag = false; });
  addEventListener("pointermove", (e) => { if (drag) { yaw += (e.clientX - lx) * 0.01; lx = e.clientX; } });

  const uAngle = gl.getUniformLocation(prog, "u_angle");
  const uAspect = gl.getUniformLocation(prog, "u_aspect");

  function loop() {
    canvas.width = canvas.clientWidth * devicePixelRatio;
    canvas.height = canvas.clientHeight * devicePixelRatio;
    if (!drag) yaw += 0.008;
    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(0.07, 0.09, 0.11, 1);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.enable(gl.DEPTH_TEST);
    gl.useProgram(prog);
    gl.uniform1f(uAngle, yaw);
    gl.uniform1f(uAspect, Math.max(0.2, canvas.width / Math.max(1, canvas.height)));
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.enableVertexAttribArray(0);
    gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 24, 0);
    gl.enableVertexAttribArray(1);
    gl.vertexAttribPointer(1, 3, gl.FLOAT, false, 24, 12);
    gl.drawArrays(gl.TRIANGLES, 0, 9);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
}
