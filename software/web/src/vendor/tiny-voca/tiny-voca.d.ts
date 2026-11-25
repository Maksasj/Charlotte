export interface GGUFFile {
  metadata: Record<string, unknown>;
  tensors: Record<string, unknown>;
}

export function readGGUF(buffer: ArrayBuffer): GGUFFile;

export class TinyVoca {
  voices: string[];
  config: { sample_rate: number; max_tokens: number };
  constructor(file: GGUFFile);
  synthesize(
    ids: number[],
    stress: number[],
    opts?: {
      voice?: string;
      speed?: number;
      pitch_shift?: number;
      pitch_range?: number;
      seed?: number;
    }
  ): { audio: Float32Array; durations?: Float32Array };
}
