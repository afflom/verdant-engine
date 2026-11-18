// Verdant Engine — AudioWorklet Modal Bar, Woodworth ITD, & Hadamard FDN Reverberator.
// Synthesizes 128-frame stereo quanta driven by Prism-verified Euler-Bernoulli modal
// bar ratios (PRISM_MODAL_BAR_NUMERS / PRISM_MODAL_BAR_DENOM), Woodworth spherical-head
// interaural time difference (itdMicros), and a 4-channel Hadamard Feedback Delay Network
// scaled to Sabine chamber reverberation RT60.

import {
  PRISM_CHAMBER_SABINE_RT60_MS,
  PRISM_MODAL_BAR_DENOM,
  PRISM_MODAL_BAR_NUMERS,
} from "./prism-bridge-meta.mjs";

class VerdantModalProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    const opts = options?.processorOptions ?? {};
    const numers = Array.isArray(opts.modeNumers) && opts.modeNumers.length
      ? opts.modeNumers
      : PRISM_MODAL_BAR_NUMERS;
    const denom = Number(opts.modeDenom) || PRISM_MODAL_BAR_DENOM;
    this.modeRatios = numers.map((n) => n / denom);
    this.fundamentalHz = Number(opts.fundamentalHz) || 110.0;
    const rt60Sec = Math.max(0.4, (Number(opts.rt60Ms) || PRISM_CHAMBER_SABINE_RT60_MS) / 1000.0);
    this.decayPerSample = Math.pow(10, -3.0 / (rt60Sec * sampleRate));
    this.pan = 0.0;
    this.listenerGain = 0.22;
    this.itdMicros = 0.0;
    this.pavilionContact = false;
    this.attendedSlot = 0;
    this.voices = [];
    this.airPhase = 0.0;
    this.rngState = 0x56444e54;

    // Woodworth interaural time-difference sample ring (up to ~650 us -> ~32 samples at 48 kHz)
    this.itdRing = new Float32Array(128);
    this.itdWrite = 0;

    // 4-channel prime-length Hadamard Feedback Delay Network (Sabine chamber early/late tail)
    this.fdnDelays = [149, 211, 263, 307];
    this.fdnBufs = this.fdnDelays.map((len) => new Float32Array(len));
    this.fdnIdx = new Int32Array(4);
    this.fdnFeedback = this.fdnDelays.map((len) =>
      Math.min(0.94, Math.pow(10, (-3.0 * len) / (rt60Sec * sampleRate))),
    );

    this.port.onmessage = ({ data }) => {
      if (!data) return;
      if (data.kind === "spatial") {
        this.pan = Math.max(-1.0, Math.min(1.0, Number(data.pan) || 0.0));
        this.listenerGain = Math.max(0.03, Math.min(0.35, Number(data.gain) || 0.18));
        this.itdMicros = Math.max(-750.0, Math.min(750.0, Number(data.itdMicros) || 0.0));
        this.pavilionContact = Boolean(data.pavilionContact);
        this.attendedSlot = data.attendedSlot >>> 0;
      } else if (data.kind === "strike") {
        const modeIdx = (data.modeIndex | 0) % this.modeRatios.length;
        const detune = 1.0 + (this.attendedSlot % 5) * 0.012;
        const freq = Math.min(1760.0, this.fundamentalHz * this.modeRatios[Math.abs(modeIdx)] * detune);
        this.voices.push({
          phase: 0.0,
          phaseStep: (2.0 * Math.PI * freq) / sampleRate,
          env: this.pavilionContact ? 0.21 : 0.16,
        });
        if (this.voices.length > 6) this.voices.shift();
      }
    };
  }

  nextNoise() {
    let x = this.rngState | 0;
    x ^= x << 13;
    x ^= x >>> 17;
    x ^= x << 5;
    this.rngState = x;
    return (x / 2147483648.0) * 0.0035;
  }

  process(_inputs, outputs) {
    const out = outputs[0];
    if (!out || out.length === 0) return true;
    const left = out[0];
    const right = out.length > 1 ? out[1] : out[0];
    const frames = left.length;

    const angle = (this.pan + 1.0) * (Math.PI * 0.25);
    const gainL = Math.cos(angle) * this.listenerGain;
    const gainR = Math.sin(angle) * this.listenerGain;
    const itdSamples = Math.round((Math.abs(this.itdMicros) * 1e-6) * sampleRate);
    const clampedItd = Math.min(64, Math.max(0, itdSamples));
    const delayLeft = this.itdMicros > 0 ? clampedItd : 0;
    const delayRight = this.itdMicros < 0 ? clampedItd : 0;
    const wetMix = this.pavilionContact ? 0.34 : 0.20;

    for (let i = 0; i < frames; i++) {
      let sample = 0.0;
      for (let v = this.voices.length - 1; v >= 0; v--) {
        const voice = this.voices[v];
        sample += Math.sin(voice.phase) * voice.env;
        voice.phase += voice.phaseStep;
        if (voice.phase > Math.PI * 2.0) voice.phase -= Math.PI * 2.0;
        voice.env *= this.decayPerSample;
        if (voice.env < 0.00015) this.voices.splice(v, 1);
      }
      this.airPhase += (2.0 * Math.PI * 0.35) / sampleRate;
      const dry = sample + this.nextNoise() * (0.5 + 0.5 * Math.sin(this.airPhase));

      // 4x4 orthogonal Hadamard FDN step (0.5 * H_4)
      const d0 = this.fdnBufs[0][this.fdnIdx[0]];
      const d1 = this.fdnBufs[1][this.fdnIdx[1]];
      const d2 = this.fdnBufs[2][this.fdnIdx[2]];
      const d3 = this.fdnBufs[3][this.fdnIdx[3]];
      const s01 = d0 + d1, d01 = d0 - d1;
      const s23 = d2 + d3, d23 = d2 - d3;
      const h0 = 0.5 * (s01 + s23);
      const h1 = 0.5 * (d01 + d23);
      const h2 = 0.5 * (s01 - s23);
      const h3 = 0.5 * (d01 - d23);
      const fb = [h0, h1, h2, h3];
      for (let c = 0; c < 4; c++) {
        this.fdnBufs[c][this.fdnIdx[c]] = dry * 0.25 + fb[c] * this.fdnFeedback[c];
        this.fdnIdx[c] = (this.fdnIdx[c] + 1) % this.fdnDelays[c];
      }

      this.itdRing[this.itdWrite] = dry;
      const dryL = this.itdRing[(this.itdWrite - delayLeft + 128) & 127];
      const dryR = this.itdRing[(this.itdWrite - delayRight + 128) & 127];
      this.itdWrite = (this.itdWrite + 1) & 127;

      const revL = (d0 + d2) * wetMix;
      const revR = (d1 + d3) * wetMix;
      left[i] = dryL * gainL + revL * this.listenerGain;
      if (right !== left) right[i] = dryR * gainR + revR * this.listenerGain;
    }
    return true;
  }
}

registerProcessor("verdant-modal-processor", VerdantModalProcessor);
