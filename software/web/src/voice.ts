import { readGGUF, TinyVoca } from './vendor/tiny-voca/tiny-voca.js';
import { fromGGUF, type Phonemizer } from './vendor/tiny-voca/dizione/engine.js';
import { limited, utterancesOf } from './vendor/tiny-voca/text.js';

const MODEL_URL =
  'https://huggingface.co/n4ze3m/tiny-voca/resolve/main/gguf/tiny-voca-js.gguf';

export type VoiceStatus = 'idle' | 'loading' | 'ready' | 'speaking' | 'error';

const paint = () => new Promise<void>((resolve) => setTimeout(resolve, 16));

class TinyVocaPlayer {
  private voca: TinyVoca | null = null;
  private phonemizer: Phonemizer | null = null;
  private loadPromise: Promise<void> | null = null;
  private audioCtx: AudioContext | null = null;
  private sources: AudioBufferSourceNode[] = [];
  private runId = 0;
  private listeners = new Set<(status: VoiceStatus, detail?: string) => void>();
  private status: VoiceStatus = 'idle';

  subscribe(fn: (status: VoiceStatus, detail?: string) => void): () => void {
    this.listeners.add(fn);
    fn(this.status);
    return () => this.listeners.delete(fn);
  }

  private emit(status: VoiceStatus, detail?: string) {
    this.status = status;
    for (const fn of this.listeners) fn(status, detail);
  }

  private ensureAudioContext(): AudioContext {
    if (!this.audioCtx) {
      this.audioCtx = new AudioContext();
    }
    if (this.audioCtx.state === 'suspended') {
      void this.audioCtx.resume();
    }
    return this.audioCtx;
  }

  async ensureLoaded(): Promise<void> {
    if (this.voca && this.phonemizer) return;
    if (this.loadPromise) return this.loadPromise;

    this.emit('loading');
    this.loadPromise = (async () => {
      try {
        const res = await fetch(MODEL_URL);
        if (!res.ok) throw new Error(`HTTP ${res.status} fetching tiny-voca model`);
        const buf = await res.arrayBuffer();
        await paint();
        const file = readGGUF(buf);
        this.voca = new TinyVoca(file);
        this.phonemizer = await fromGGUF(file);
        this.emit('ready');
      } catch (err) {
        this.loadPromise = null;
        this.emit('error', (err as Error).message);
        throw err;
      }
    })();

    return this.loadPromise;
  }

  stop() {
    this.runId += 1;
    for (const src of this.sources) {
      try {
        src.stop();
      } catch {
        // already stopped
      }
    }
    this.sources = [];
    if (this.voca) {
      this.emit('ready');
    } else if (this.status === 'speaking') {
      this.emit('idle');
    }
  }

  async speak(text: string): Promise<void> {
    const cleaned = text.replace(/[`*_#]/g, '').trim();
    if (!cleaned) return;

    const ctx = this.ensureAudioContext();
    this.stop();
    const mine = ++this.runId;

    await this.ensureLoaded();
    if (mine !== this.runId || !this.voca || !this.phonemizer) return;

    let utterances;
    try {
      utterances = utterancesOf(cleaned, this.phonemizer, this.voca.config.max_tokens - 12);
    } catch (err) {
      this.emit('error', (err as Error).message);
      return;
    }
    if (!utterances.length) return;

    this.emit('speaking');
    await paint();

    const rate = this.voca.config.sample_rate;
    let nextTime = 0;

    for (const [seed, utt] of utterances.entries()) {
      if (mine !== this.runId) return;
      const { audio } = this.voca.synthesize(utt.ids, utt.stress, {
        voice: 'F2',
        seed,
      });
      if (mine !== this.runId) return;

      const heard = limited(audio);
      const buffer = ctx.createBuffer(1, heard.length, rate);
      buffer.copyToChannel(new Float32Array(heard), 0);

      const source = ctx.createBufferSource();
      source.buffer = buffer;
      source.connect(ctx.destination);

      nextTime = Math.max(nextTime, ctx.currentTime + 0.04);
      source.start(nextTime);
      nextTime += heard.length / rate;
      this.sources.push(source);

      if (seed === utterances.length - 1) {
        source.onended = () => {
          if (mine === this.runId) {
            this.sources = [];
            this.emit('ready');
          }
        };
      }
      await paint();
    }
  }
}

export const karinVoice = new TinyVocaPlayer();
