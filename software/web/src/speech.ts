interface SpeechRecognitionAlternativeLike {
  transcript: string;
}

interface SpeechRecognitionResultLike {
  isFinal: boolean;
  length: number;
  [index: number]: SpeechRecognitionAlternativeLike;
}

interface SpeechRecognitionEventLike {
  resultIndex: number;
  results: {
    length: number;
    [index: number]: SpeechRecognitionResultLike;
  };
}

interface SpeechRecognitionErrorEventLike {
  error: string;
}

interface SpeechRecognitionLike {
  lang: string;
  continuous: boolean;
  interimResults: boolean;
  maxAlternatives: number;
  onresult: ((e: SpeechRecognitionEventLike) => void) | null;
  onerror: ((e: SpeechRecognitionErrorEventLike) => void) | null;
  onend: (() => void) | null;
  start(): void;
  stop(): void;
  abort(): void;
}

type SpeechRecognitionCtor = new () => SpeechRecognitionLike;

function getSpeechCtor(): SpeechRecognitionCtor | null {
  const w = window as unknown as {
    SpeechRecognition?: SpeechRecognitionCtor;
    webkitSpeechRecognition?: SpeechRecognitionCtor;
  };
  return w.SpeechRecognition ?? w.webkitSpeechRecognition ?? null;
}

export function isSpeechRecognitionSupported(): boolean {
  return getSpeechCtor() !== null;
}

export interface ListenHandle {
  promise: Promise<string | null>;
  stop(): void;
}

export function listenOnce(opts?: {
  lang?: string;
  onInterim?: (text: string) => void;
}): ListenHandle {
  const Ctor = getSpeechCtor();
  if (!Ctor) {
    return {
      promise: Promise.reject(
        new Error('Speech recognition is not supported in this browser (try Chrome or Safari).')
      ),
      stop() {},
    };
  }

  const rec = new Ctor();
  rec.lang = opts?.lang ?? 'en-US';
  rec.continuous = false;
  rec.interimResults = true;
  rec.maxAlternatives = 1;

  let settled = false;
  let aborted = false;
  let finalText = '';
  let latestText = '';

  const promise = new Promise<string | null>((resolve, reject) => {
    const finish = (val: string | null) => {
      if (settled) return;
      settled = true;
      resolve(val);
    };

    rec.onresult = (e) => {
      let interim = '';
      for (let i = e.resultIndex; i < e.results.length; i++) {
        const res = e.results[i];
        const part = res[0]?.transcript ?? '';
        if (res.isFinal) {
          finalText += part;
        } else {
          interim += part;
        }
      }
      latestText = (finalText + interim).trim();
      opts?.onInterim?.(latestText);
    };

    rec.onerror = (e) => {
      if (settled) return;
      if (aborted || e.error === 'aborted' || e.error === 'no-speech') {
        finish(null);
        return;
      }
      settled = true;
      reject(new Error(`Microphone error: ${e.error}`));
    };

    rec.onend = () => {
      if (aborted) {
        finish(null);
        return;
      }
      const text = (finalText || latestText).trim();
      finish(text || null);
    };

    try {
      rec.start();
    } catch (err) {
      settled = true;
      reject(err instanceof Error ? err : new Error(String(err)));
    }
  });

  return {
    promise,
    stop() {
      aborted = true;
      try {
        rec.abort();
      } catch {
        // already stopped
      }
    },
  };
}
