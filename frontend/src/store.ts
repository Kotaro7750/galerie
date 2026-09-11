import type { SearchTerm } from './api/generated';
import { create } from 'zustand';

type GalleryState = {
  density: 'comfortable' | 'compact';
  setDensity: (density: GalleryState['density']) => void;
};

// Local presentation state only; content data belongs to TanStack Query.
export const useGalleryStore = create<GalleryState>((set) => ({
  density: 'comfortable',
  setDensity: (density) => set({ density }),
}));

// Draft terms are separate from the condition used for API requests.
export const useSearchStore = create<{
  draft: SearchTerm[];
  condition: SearchTerm[];
  setDraft: (draft: SearchTerm[]) => void;
  applyTerms: (terms: SearchTerm[]) => void;
  apply: () => void;
}>((set) => ({
  draft: [], condition: [],
  setDraft: (draft) => set({ draft }),
  applyTerms: (terms) => set({ draft: terms, condition: [...terms] }),
  apply: () => set((state) => ({ condition: [...state.draft] })),
}));
