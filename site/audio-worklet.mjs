// Verdant Engine — AudioWorklet Modal Bar & Room Reverberation Spatial Processor.
// Synthesizes 128-frame stereo quanta driven by Prism-verified Euler-Bernoulli modal
// bar ratios ([9, 25, 49, 81, 121] / 9), Sabine RT60 reverberant decay, and Woodworth
// spherical-head binaural panning streamed from engine-worker.mjs.

class VerdantModalProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    const opts = options?.processorOptions ?? {};
    const numers = Array.isArray(opts.modeNumers) && opts.modeNumers.length
      ? opts.modeNumers
      : [9, 25, 49, 81, 121];
    const denom = Number(opts.modeDenom) || 9;
    this.modeRatios = numers.map((n) => n / denom);
    this.fundamentalHz = Number(opts.fundamentalHz) || 110.0;
    const rt60Sec = Math.max(0.4, (Number(opts.rt60Ms) || 3000) / 1000.0);
    // -60 dB decay per sample: 10^(-3 / (rt60Sec * sampleRate))
    this.decayPerSample = Math.pow(10, -3.0 / (rt60Sec * sampleRate));
    this.pan = 0.0;
    this.listenerGain = 0.22;
    this.voices = [];
    this.airPhase = 0.0;
    this.rngState = 0x56444e54;

    this.port.onmessage = ({ data }) => {
      if (!data) return;
      if (data.kind === "spatial") {
        this.pan = Math.max(-1.0, Math.min(1.0, Number(data.pan) || 0.0));
        this.listenerGain = Math.max(0.03, Math.min(0.35, Number(data.gain) || 0.18));
      } else if (data.kind === "strike") {
        const modeIdx = (data.modeIndex | 0) % this.modeRatios.length;
        const freq = Math.min(1760.0, this.fundamentalHz * this.modeRatios[Math.abs(modeIdx)]);
        this.voices.push({
          phase: 0.0,
          phaseStep: (2.0 * Math.PI * freq) / sampleRate,
          env: 0.16,
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
    return (x / 2147483648.0) * 0.004;
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
      const ambientAirNoise = this.nextNoise() * (0.5 + 0.5 * Math.sin(this.airPhase));
      const mixed = sample + ambientAirNoise;
      left[i] = mixed * gainL;
      if (right !== left) right[i] = mixed * gainR;
    }
    return true;
  }
}

registerProcessor("verdant-modal-processor", VerdantModalProcessor);
