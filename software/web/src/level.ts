// Live sound levels for the voice waveform: the microphone (what Karin can
// hear) and Karin's own voice output. Each source is an AnalyserNode; `levelOf`
// turns its current samples into a 0..1 loudness.

let micCtx: AudioContext | null = null;
let micStream: MediaStream | null = null;
let micAnalyser: AnalyserNode | null = null;

/** Opens the microphone for metering. Safe to call repeatedly. */
export async function openMic(): Promise<AnalyserNode> {
  if (micAnalyser) return micAnalyser;
  micStream = await navigator.mediaDevices.getUserMedia({
    audio: { echoCancellation: true, noiseSuppression: true, autoGainControl: true },
  });
  micCtx = new AudioContext();
  const source = micCtx.createMediaStreamSource(micStream);
  micAnalyser = micCtx.createAnalyser();
  micAnalyser.fftSize = 1024;
  micAnalyser.smoothingTimeConstant = 0.2;
  source.connect(micAnalyser);
  return micAnalyser;
}

export function closeMic() {
  micStream?.getTracks().forEach((t) => t.stop());
  void micCtx?.close();
  micStream = null;
  micCtx = null;
  micAnalyser = null;
}

export function micAnalyserNode(): AnalyserNode | null {
  return micAnalyser;
}

const buffers = new WeakMap<AnalyserNode, Float32Array<ArrayBuffer>>();

/** Loudness of what the analyser hears right now, 0 (silence) .. 1 (loud). */
export function levelOf(analyser: AnalyserNode | null): number {
  if (!analyser) return 0;
  let buf = buffers.get(analyser);
  if (!buf || buf.length !== analyser.fftSize) {
    buf = new Float32Array(analyser.fftSize);
    buffers.set(analyser, buf);
  }
  analyser.getFloatTimeDomainData(buf);
  let sum = 0;
  for (let i = 0; i < buf.length; i++) sum += buf[i] * buf[i];
  const rms = Math.sqrt(sum / buf.length);
  if (rms <= 0) return 0;
  const db = 20 * Math.log10(rms);
  // -58 dB (room noise) .. -12 dB (close, loud speech)
  return Math.min(1, Math.max(0, (db + 58) / 46));
}
