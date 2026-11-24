/**
 * The WebGPU bridge (step S5d): a host runs a WGSL kernel that LexLean's `Kernel` module emitted, on WebGPU, and reads
 * back its u32 buffers, so the same integers can be compared with the CPU's word for word.
 *
 * A guest never calls the GPU. It answers a plan: the WGSL text, its BLAKE3 address, the number of invocations and the
 * buffers as u32 words. The host admits the plan, dispatches it, admits each readback and compares. The three
 * decisions are the rules of `model/webgpu-bridge` (WebGpuBridge.lex.tex), one for one, where Lean proves what they
 * guarantee; test/webgpu-bridge-model.test.mjs holds this file to the model's text.
 *
 * LexLean's `Kernel` model fixes buffers as a list of u32 lists, invocation i = global index, reads past the end as 0
 * and stores past the end as dropped. The rest is this bridge's proposal, which the WGSL printer adopts when it is
 * written: buffer i is `array<u32>` at `@group(0) @binding(i)`; every `read_write` binding (any number of them) is an
 * output and is read back whole; the entry point is `main`; the workgroup size is the kernel's own
 * `@workgroup_size(n)` along x; the invocation index is `global_invocation_id.x`; stores are guarded by
 * `arrayLength`; a rendered kernel declares `override invocations: u32;` and returns past it, and the bridge sets it to
 * the plan's count, so the spare invocations of the last workgroup write nothing.
 */
import { blake3Hex } from './blake3.mjs';
import { PrismCodeError } from './errors.mjs';

/** The verdicts, by the model's numbers. */
export const VERDICT = Object.freeze(['agree', 'differ', 'not-run']);

/** The BLAKE3 address of a kernel: the hash of its WGSL text as UTF-8. */
export const kernelAddress = (wgsl) => blake3Hex(new TextEncoder().encode(wgsl));

// The rules of the model, in its own field names.
export const kernelBound = (p) => p.kernelNamed === p.kernelDerived && p.bindings === p.buffers;
export const buffersFit = (p) => p.emptyBuffers === 0 && 1 <= p.outputs && p.largestBuffer <= p.bufferLimit;
export const dispatchCovers = (p) =>
  1 <= p.invocations && 1 <= p.workgroupSize && p.invocations <= p.dispatched && p.dispatched < p.invocations + p.workgroupSize && p.workgroups <= p.workgroupLimit;
export const planAdmitted = (p) => p.adapter === 1 && kernelBound(p) && buffersFit(p) && dispatchCovers(p);
export const readbackAdmitted = (r) => r.readWords === r.expectedWords && r.trailingBytes === 0;
export const agrees = (c) => c.ran === 1 && c.planOk === 1 && c.readbackOk === 1 && c.cpuWords === c.gpuWords && c.differing === 0;
export const verdict = (c) => (agrees(c) ? 0 : c.ran === 1 && c.planOk === 1 ? 1 : 2);

/**
 * The storage bindings a kernel declares, in binding order, and its workgroup size along x. A binding outside
 * group 0, a gap in the numbering or a workgroup that is not `(n)` or `(n, 1, 1)` leaves nothing to match, so the
 * plan is refused by `kernelBound` or `dispatchCovers`.
 */
export function kernelShape(wgsl) {
  const declared = [];
  let regular = true;
  for (const m of wgsl.matchAll(/@group\((\d+)\)\s*@binding\((\d+)\)\s*var<storage\s*(?:,\s*(read|read_write)\s*)?>/g)) {
    if (m[1] !== '0') regular = false;
    declared.push({ binding: Number(m[2]), access: m[3] ?? 'read' });
  }
  declared.sort((a, b) => a.binding - b.binding);
  if (!declared.every((d, i) => d.binding === i)) regular = false;
  const size = /@workgroup_size\(\s*(\d+)\s*(?:,\s*1\s*(?:,\s*1\s*)?)?\)/.exec(wgsl);
  // A rendered kernel declares `override invocations: u32;` and returns early past it, so the padding invocations of
  // the last workgroup write nothing; the host sets it when it creates the pipeline.
  const overridesInvocations = /\boverride\s+invocations\s*:\s*u32\s*;/.test(wgsl);
  return { bindings: regular ? declared : null, declared: declared.length, workgroupSize: size ? Number(size[1]) : 0, overridesInvocations };
}

/** The facts `planAdmitted` decides on, from a plan and the device's limits. */
export function planFacts(plan, { adapter = true, limits = {} } = {}) {
  const shape = kernelShape(plan.wgsl);
  const buffers = plan.buffers.map((b) => Uint32Array.from(b));
  const workgroupSize = shape.workgroupSize;
  const workgroups = workgroupSize > 0 ? Math.ceil(plan.invocations / workgroupSize) : 0;
  return {
    adapter: adapter ? 1 : 0,
    kernelNamed: plan.kernel,
    kernelDerived: kernelAddress(plan.wgsl),
    // An irregular kernel can never match a buffer list, whatever its length.
    bindings: shape.bindings ? shape.bindings.length : buffers.length + shape.declared + 1,
    buffers: buffers.length,
    outputs: (shape.bindings ?? []).filter((b) => b.access === 'read_write').length,
    emptyBuffers: buffers.filter((b) => b.length === 0).length,
    largestBuffer: Math.max(0, ...buffers.map((b) => b.byteLength)),
    bufferLimit: limits.maxStorageBufferBindingSize ?? 134217728,
    invocations: plan.invocations,
    workgroupSize,
    dispatched: workgroups * workgroupSize,
    workgroups,
    workgroupLimit: limits.maxComputeWorkgroupsPerDimension ?? 65535,
  };
}

/** The facts `readbackAdmitted` decides on, for a buffer of `words` words read back as `bytes`. */
export const readbackFacts = (words, bytes) => ({ expectedWords: words, readWords: Math.floor(bytes.byteLength / 4), trailingBytes: bytes.byteLength % 4 });

/** The facts `verdict` decides on: what ran, and the CPU's and the GPU's words. */
export function comparisonFacts({ ran, planOk, readbackOk, cpu, gpu }) {
  const a = cpu ?? [];
  const b = gpu ?? [];
  let differing = 0;
  for (let i = 0; i < Math.min(a.length, b.length); i++) if (a[i] >>> 0 !== b[i] >>> 0) differing += 1;
  return { ran: ran ? 1 : 0, planOk: planOk ? 1 : 0, readbackOk: readbackOk ? 1 : 0, cpuWords: a.length, gpuWords: b.length, differing };
}

/** Compare the CPU's words with the GPU's: `agree`, `differ` or `not-run`, and the facts behind it. */
export function compare(facts) {
  const c = comparisonFacts(facts);
  return { verdict: VERDICT[verdict(c)], facts: c };
}

/**
 * Run an admitted plan on WebGPU and read back every `read_write` buffer.
 *
 * `plan` is `{ wgsl, kernel, invocations, buffers }`: `kernel` the BLAKE3 address the guest names for the WGSL, and
 * `buffers` one array of u32 words per binding. `gpu` is `navigator.gpu` by default. Returns
 * `{ outputs, adapter, facts }`, where `outputs` maps each output binding to its words.
 *
 * Throws `webgpu.unavailable` when there is no WebGPU or no adapter, `webgpu.plan-refused` when the plan breaks a
 * rule of the model (nothing is dispatched), and `webgpu.readback-refused` when a buffer comes back short.
 */
export async function runPlan(plan, { gpu = globalThis.navigator?.gpu } = {}) {
  const adapter = gpu ? await gpu.requestAdapter() : null;
  if (!adapter) throw new PrismCodeError('webgpu.unavailable', 'This host has no WebGPU adapter, so the kernel did not run; the CPU result stands alone.', { facts: planFacts(plan, { adapter: false }) });
  const device = await adapter.requestDevice();
  try {
    const facts = planFacts(plan, { adapter: true, limits: device.limits });
    if (!planAdmitted(facts)) throw new PrismCodeError('webgpu.plan-refused', `The plan for kernel ${plan.kernel} breaks a rule of model/webgpu-bridge, so nothing was dispatched.`, { facts });
    const shape = kernelShape(plan.wgsl);
    const { bindings } = shape;
    const words = plan.buffers.map((b) => Uint32Array.from(b));
    const S = globalThis.GPUBufferUsage;
    const layout = device.createBindGroupLayout({
      entries: bindings.map((b) => ({ binding: b.binding, visibility: globalThis.GPUShaderStage.COMPUTE, buffer: { type: b.access === 'read_write' ? 'storage' : 'read-only-storage' } })),
    });
    device.pushErrorScope('validation');
    const module = device.createShaderModule({ code: plan.wgsl });
    const pipeline = device.createComputePipeline({ layout: device.createPipelineLayout({ bindGroupLayouts: [layout] }), compute: { module, entryPoint: 'main', ...(shape.overridesInvocations ? { constants: { invocations: plan.invocations } } : {}) } });
    const storage = words.map((w) => {
      const buffer = device.createBuffer({ size: w.byteLength, usage: S.STORAGE | S.COPY_SRC | S.COPY_DST });
      device.queue.writeBuffer(buffer, 0, w);
      return buffer;
    });
    const outputs = bindings.filter((b) => b.access === 'read_write');
    const staging = outputs.map((b) => device.createBuffer({ size: words[b.binding].byteLength, usage: S.MAP_READ | S.COPY_DST }));
    const encoder = device.createCommandEncoder();
    const pass = encoder.beginComputePass();
    pass.setPipeline(pipeline);
    pass.setBindGroup(0, device.createBindGroup({ layout, entries: storage.map((buffer, binding) => ({ binding, resource: { buffer } })) }));
    pass.dispatchWorkgroups(facts.workgroups);
    pass.end();
    outputs.forEach((b, i) => encoder.copyBufferToBuffer(storage[b.binding], 0, staging[i], 0, words[b.binding].byteLength));
    device.queue.submit([encoder.finish()]);
    const invalid = await device.popErrorScope();
    if (invalid) throw new PrismCodeError('webgpu.plan-refused', `WebGPU refused kernel ${plan.kernel}: ${invalid.message}`, { facts });
    const read = new Map();
    for (const [i, b] of outputs.entries()) {
      await staging[i].mapAsync(globalThis.GPUMapMode.READ);
      const bytes = staging[i].getMappedRange().slice(0);
      staging[i].unmap();
      const r = readbackFacts(words[b.binding].length, bytes);
      if (!readbackAdmitted(r)) throw new PrismCodeError('webgpu.readback-refused', `Binding ${b.binding} came back as ${bytes.byteLength} bytes, not ${words[b.binding].length} whole words.`, { facts: r });
      read.set(b.binding, new Uint32Array(bytes));
    }
    const info = adapter.info ?? {};
    return { outputs: read, adapter: { vendor: info.vendor ?? '', architecture: info.architecture ?? '' }, facts };
  } finally {
    device.destroy();
  }
}
