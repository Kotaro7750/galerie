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
