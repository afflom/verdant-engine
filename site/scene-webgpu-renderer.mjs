// Verdant Engine — Native W3C WebGPU UHD Graphics Renderer (`site/scene-webgpu-renderer.mjs`).
// Runs Verdant's `naga`-validated W3C WGSL PBR + ACES Filmic shader natively on hardware WebGPU
// adapters (`requestAdapter({ powerPreference: "high-performance" })`) with anisotropic PBR
// texture sampling (`maxAnisotropy: 8`) and depth testing, returning `null` cleanly when WebGPU
// is absent or software-emulated so `engine-worker.mjs` falls back to WebGL2 HD PBR + CAS.

const WGSL_UHD_PBR = `
struct UhdSceneUniforms {
  mvp: mat4x4<f32>,
  eye_time: vec4<f32>,
  sun_cas: vec4<f32>,
  sky_flags: vec4<f32>,
};
struct VsIn {
  @location(0) pos: vec3<f32>,
  @location(1) norm: vec3<f32>,
  @location(2) col: vec3<f32>,
  @location(3) uv: vec2<f32>,
};
struct VsOut {
  @builtin(position) clip_pos: vec4<f32>,
  @location(0) world_pos: vec3<f32>,
  @location(1) world_norm: vec3<f32>,
  @location(2) vert_col: vec3<f32>,
  @location(3) uv: vec2<f32>,
};
@group(0) @binding(0) var<uniform> u_scene: UhdSceneUniforms;
@group(0) @binding(1) var u_sampler: sampler;
@group(0) @binding(2) var u_albedo_tex: texture_2d<f32>;
@group(0) @binding(3) var u_mr_tex: texture_2d<f32>;

@vertex fn vs_uhd_pbr(in: VsIn) -> VsOut {
  var out: VsOut;
  out.clip_pos = u_scene.mvp * vec4<f32>(in.pos, 1.0);
  out.world_pos = in.pos;
  out.world_norm = normalize(in.norm);
  out.vert_col = in.col;
  out.uv = in.uv;
  return out;
}

fn aces_filmic(x: vec3<f32>) -> vec3<f32> {
  return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment fn fs_uhd_pbr(in: VsOut) -> @location(0) vec4<f32> {
  let n = normalize(in.world_norm);
  let v = normalize(u_scene.eye_time.xyz - in.world_pos);
  let l = normalize(u_scene.sun_cas.xyz);
  let h = normalize(l + v);
  let use_tex = u_scene.sky_flags.w > 0.5 && in.uv.x > -0.5;
  let tex_albedo = textureSample(u_albedo_tex, u_sampler, max(in.uv, vec2<f32>(0.0))).rgb;
  let tex_mr = textureSample(u_mr_tex, u_sampler, max(in.uv, vec2<f32>(0.0))).rgb;
  let base_col = select(in.vert_col, tex_albedo, use_tex);
  let albedo = pow(max(base_col, vec3<f32>(0.02)), vec3<f32>(2.2));
  let rough = select(0.55, clamp(tex_mr.g, 0.08, 0.98), use_tex);
  let metal = select(0.08, clamp(tex_mr.b, 0.0, 1.0), use_tex);
  let ndl = max(dot(n, l), 0.0);
  let ndv = max(dot(n, v), 0.001);
  let ndh = max(dot(n, h), 0.0);
  let a = rough * rough;
  let a2 = a * a;
  let denom = ndh * ndh * (a2 - 1.0) + 1.0;
  let d_ggx = a2 / max(3.14159265 * denom * denom, 0.0001);
  let f0 = mix(vec3<f32>(0.04), albedo, metal);
  let fresnel = f0 + (vec3<f32>(1.0) - f0) * pow(1.0 - ndv, 5.0);
  let spec = (d_ggx * fresnel) / max(4.0 * ndv * ndl + 0.001, 0.001);
  let kd = (vec3<f32>(1.0) - fresnel) * (1.0 - metal);
  let direct = (kd * albedo / 3.14159265 + spec) * vec3<f32>(2.6, 2.4, 2.05) * ndl;
  let hemi = mix(vec3<f32>(0.08, 0.08, 0.09), u_scene.sky_flags.xyz + vec3<f32>(0.12), 0.5 + 0.5 * n.y);
  let hdr = direct + kd * albedo * hemi + fresnel * hemi * (1.0 - rough * 0.65);
  let ldr = pow(aces_filmic(hdr * 1.06), vec3<f32>(1.0 / 2.2));
  return vec4<f32>(ldr, 1.0);
}
`;

export async function probeWebGpuHardware() {
  if (typeof navigator === "undefined" || !navigator.gpu) return null;
  try {
    const adapter = await navigator.gpu.requestAdapter({ powerPreference: "high-performance" });
    if (!adapter || adapter.isFallbackAdapter) return null;
    return {
      maxTextureDimension2D: adapter.limits?.maxTextureDimension2D || 8192,
      maxStorageBufferBindingSize: adapter.limits?.maxStorageBufferBindingSize || 134217728,
      adapter,
    };
  } catch {
    return null;
  }
}

export async function createWebGpuRenderer(canvas) {
  const probe = await probeWebGpuHardware();
  if (!probe) return null;
  const ctx = canvas.getContext("webgpu");
  if (!ctx) return null;
  try {
    const device = await probe.adapter.requestDevice();
    const format = navigator.gpu.getPreferredCanvasFormat();
    ctx.configure({ device, format, alphaMode: "opaque" });

    const shader = device.createShaderModule({ code: WGSL_UHD_PBR });
    const pipeline = device.createRenderPipeline({
      layout: "auto",
      vertex: {
        module: shader,
        entryPoint: "vs_uhd_pbr",
        buffers: [
          { arrayStride: 12, attributes: [{ shaderLocation: 0, offset: 0, format: "float32x3" }] },
          { arrayStride: 12, attributes: [{ shaderLocation: 1, offset: 0, format: "float32x3" }] },
          { arrayStride: 12, attributes: [{ shaderLocation: 2, offset: 0, format: "float32x3" }] },
          { arrayStride: 8, attributes: [{ shaderLocation: 3, offset: 0, format: "float32x2" }] },
        ],
      },
      fragment: { module: shader, entryPoint: "fs_uhd_pbr", targets: [{ format }] },
      primitive: { topology: "triangle-list", cullMode: "none" },
      depthStencil: { format: "depth24plus", depthWriteEnabled: true, depthCompare: "less" },
    });

    const uBuf = device.createBuffer({
      size: 112,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    const sampler = device.createSampler({
      magFilter: "linear",
      minFilter: "linear",
      addressModeU: "repeat",
      addressModeV: "repeat",
      maxAnisotropy: 8,
    });
    const makeTex = (w, h, rgba) => {
      const tex = device.createTexture({
        size: [w, h, 1],
        format: "rgba8unorm",
        usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST,
      });
      device.queue.writeTexture({ texture: tex }, rgba, { bytesPerRow: w * 4 }, [w, h, 1]);
      return tex;
    };
    let albedoTex = makeTex(1, 1, new Uint8Array([255, 255, 255, 255]));
    let mrTex = makeTex(1, 1, new Uint8Array([255, 160, 0, 255]));
    let hasTex = 0;
    let bindGroup = null;
    const rebuildBindGroup = () => {
      bindGroup = device.createBindGroup({
        layout: pipeline.getBindGroupLayout(0),
        entries: [
          { binding: 0, resource: { buffer: uBuf } },
          { binding: 1, resource: sampler },
          { binding: 2, resource: albedoTex.createView() },
          { binding: 3, resource: mrTex.createView() },
        ],
      });
    };
    rebuildBindGroup();

    let posBuf = null, normBuf = null, colBuf = null, uvBuf = null, idxBuf = null;
    let posCap = 0, normCap = 0, colCap = 0, uvCap = 0, idxCap = 0;
    let idxCount = 0, staticEpoch = 0, depthTex = null, depthW = 0, depthH = 0;
    const ensureDepth = (w, h) => {
      if (depthTex && depthW === w && depthH === h) return;
      depthTex?.destroy?.();
      depthTex = device.createTexture({ size: [w, h, 1], format: "depth24plus", usage: GPUTextureUsage.RENDER_ATTACHMENT });
      depthW = w; depthH = h;
    };
    const writeOrGrow = (prevBuf, prevCap, data, usage) => {
      const byteLen = Math.max(16, (data.byteLength + 3) & ~3);
      let buf = prevBuf, cap = prevCap;
      if (!buf || byteLen > cap) {
        buf?.destroy?.();
        cap = Math.max(byteLen, cap * 2);
        buf = device.createBuffer({ size: cap, usage: usage | GPUBufferUsage.COPY_DST });
      }
      device.queue.writeBuffer(buf, 0, data.buffer, data.byteOffset, data.byteLength);
      return [buf, cap];
    };
    const uData = new Float32Array(28);
    const noop = () => {};

    return {
      backend: "webgpu",
      isSoftware: false,
      maxTextureSize: probe.maxTextureDimension2D,
      maxAnisotropy: 8,
      setRenderScale: noop,
      setCsmSplits: noop,
      getUploadTelemetry() { return { staticBytes: 0, dynBytes: 0, staticEpoch, instCount: 0, ctxLost: false, maxTex: probe.maxTextureDimension2D, maxAniso: 8 }; },
      setTex(k, s, w, h) {
        if (k === 1) {
          if (!s || w <= 0 || h <= 0) { hasTex = 0; return; }
          albedoTex.destroy?.();
          albedoTex = makeTex(w, h, s);
          hasTex = 1;
          rebuildBindGroup();
        } else if (k === 3 && s && w > 0 && h > 0) {
          mrTex.destroy?.();
          mrTex = makeTex(w, h, s);
          rebuildBindGroup();
        }
      },
      uploadSplitBuffers(sv) {
        if (!sv || sv.epoch === staticEpoch) return;
        [posBuf, posCap] = writeOrGrow(posBuf, posCap, sv.pos, GPUBufferUsage.VERTEX);
        [normBuf, normCap] = writeOrGrow(normBuf, normCap, sv.norm, GPUBufferUsage.VERTEX);
        [colBuf, colCap] = writeOrGrow(colBuf, colCap, sv.col, GPUBufferUsage.VERTEX);
        const uvArr = sv.uv && sv.uv.length >= (sv.pos.length / 3) * 2
          ? sv.uv
          : new Float32Array((sv.pos.length / 3) * 2).fill(-2);
        [uvBuf, uvCap] = writeOrGrow(uvBuf, uvCap, uvArr, GPUBufferUsage.VERTEX);
        [idxBuf, idxCap] = writeOrGrow(idxBuf, idxCap, sv.idx, GPUBufferUsage.INDEX);
        idxCount = sv.idx.length;
        staticEpoch = sv.epoch;
      },
      updateStaticIndices(ix) {
        if (!ix || ix.length === 0) return;
        [idxBuf, idxCap] = writeOrGrow(idxBuf, idxCap, ix, GPUBufferUsage.INDEX);
        idxCount = ix.length;
      },
      uploadPackedBuffers: noop,
      blitTensorRgba: noop,
      render(mvp, eye, lodSpec, timeSec) {
        const w = canvas.width | 0, h = canvas.height | 0;
        if (w <= 0 || h <= 0 || !idxCount || !posBuf) return;
        ensureDepth(w, h);
        const sky = lodSpec?.sky ?? [0.08, 0.1, 0.14];
        uData.set(mvp, 0);
        uData[16] = eye[0]; uData[17] = eye[1]; uData[18] = eye[2]; uData[19] = timeSec || 0;
        uData[20] = 0.54; uData[21] = 0.78; uData[22] = 0.31; uData[23] = 0.25;
        uData[24] = sky[0]; uData[25] = sky[1]; uData[26] = sky[2]; uData[27] = hasTex;
        device.queue.writeBuffer(uBuf, 0, uData);
        const enc = device.createCommandEncoder();
        const pass = enc.beginRenderPass({
          colorAttachments: [{
            view: ctx.getCurrentTexture().createView(),
            clearValue: { r: sky[0], g: sky[1], b: sky[2], a: 1 },
            loadOp: "clear",
            storeOp: "store",
          }],
          depthStencilAttachment: { view: depthTex.createView(), depthClearValue: 1, depthLoadOp: "clear", depthStoreOp: "store" },
        });
        pass.setPipeline(pipeline);
        pass.setBindGroup(0, bindGroup);
        pass.setVertexBuffer(0, posBuf);
        pass.setVertexBuffer(1, normBuf);
        pass.setVertexBuffer(2, colBuf);
        pass.setVertexBuffer(3, uvBuf);
        pass.setIndexBuffer(idxBuf, "uint32");
        pass.drawIndexed(idxCount);
        pass.end();
        device.queue.submit([enc.finish()]);
      },
    };
  } catch {
    return null;
  }
}
