import type { GGUFFile } from '../tiny-voca.js';

export interface Phonemizer {
  phonemize(text: string): { ids: number[]; stress: number[]; phonemes: string };
}

export function fromGGUF(file: GGUFFile): Promise<Phonemizer>;
