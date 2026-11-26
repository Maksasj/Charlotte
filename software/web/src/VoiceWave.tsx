import { useEffect, useRef } from 'react';
import { levelOf } from './level';

export type WavePhase = 'idle' | 'loading' | 'listening' | 'thinking' | 'speaking';

const BAR = 2; // bar width, px
const GAP = 2; // space between bars, px
const STEP_MS = 55; // a new bar this often

/**
 * A scrolling level meter: each bar is how loud one moment was, newest on the
 * right. Listening shows the microphone, speaking shows Karin's voice, thinking
 * breathes gently.
 */
export default function VoiceWave({
  phase,
  mic,
  output,
}: {
  phase: WavePhase;
  mic: () => AnalyserNode | null;
  output: () => AnalyserNode | null;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const phaseRef = useRef(phase);
  phaseRef.current = phase;
  const sources = useRef({ mic, output });
  sources.current = { mic, output };

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let history: { v: number; who: 'mic' | 'karin' | 'none' }[] = [];
    let raf = 0;
    let last = 0;
    let peak = 0;

    const colorOf = (name: string) =>
      getComputedStyle(canvas).getPropertyValue(name).trim() || '#888';

    const draw = (now: number) => {
      raf = requestAnimationFrame(draw);
      const dpr = window.devicePixelRatio || 1;
      const w = canvas.clientWidth;
      const h = canvas.clientHeight;
      if (canvas.width !== Math.round(w * dpr) || canvas.height !== Math.round(h * dpr)) {
        canvas.width = Math.round(w * dpr);
        canvas.height = Math.round(h * dpr);
      }
      const slots = Math.max(1, Math.floor((w + GAP) / (BAR + GAP)));

      const p = phaseRef.current;
      const live =
        p === 'listening'
          ? levelOf(sources.current.mic())
          : p === 'speaking'
            ? levelOf(sources.current.output())
            : 0;
      peak = Math.max(live, peak * 0.85);

      if (now - last >= STEP_MS) {
        last = now;
        let v = 0;
        let who: 'mic' | 'karin' | 'none' = 'none';
        if (p === 'listening') {
          v = peak;
          who = 'mic';
        } else if (p === 'speaking') {
          v = peak;
          who = 'karin';
        } else if (p === 'thinking' || p === 'loading') {
          v = 0.1 + 0.08 * Math.sin(now / 300 + history.length * 0.4);
        }
        peak = live;
        history.push({ v, who });
        if (history.length > slots) history = history.slice(history.length - slots);
        while (history.length < slots) history.unshift({ v: 0, who: 'none' });
      }

      const mid = h / 2;
      const micColor = colorOf('--brand');
      const karinColor = colorOf('--ok');
      const quiet = colorOf('--border-strong') || colorOf('--muted');
      const thinking = colorOf('--warn');

      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      ctx.clearRect(0, 0, w, h);
      const offset = w - history.length * (BAR + GAP) + GAP;
      history.forEach((bar, i) => {
        const x = offset + i * (BAR + GAP);
        const fade = Math.min(1, (i + 1) / (history.length * 0.4));
        const barH = Math.max(BAR, bar.v * (h - 4));
        ctx.globalAlpha = bar.who === 'none' ? 0.6 * fade : 0.9 * fade;
        ctx.fillStyle =
          bar.who === 'mic'
            ? micColor
            : bar.who === 'karin'
              ? karinColor
              : p === 'thinking' || p === 'loading'
                ? thinking
                : quiet;
        ctx.beginPath();
        ctx.roundRect(x, mid - barH / 2, BAR, barH, BAR / 2);
        ctx.fill();
      });
      ctx.globalAlpha = 1;
    };
    raf = requestAnimationFrame(draw);
    return () => cancelAnimationFrame(raf);
  }, []);

  return <canvas ref={canvasRef} className={`voice-wave ${phase}`} aria-hidden="true" />;
}
