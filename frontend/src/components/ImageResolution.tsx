import { Ruler } from 'lucide-react';

export type ImageResolutionValue = { width: number; height: number };

export function ImageResolution({ resolution }: { resolution?: ImageResolutionValue }) {
  if (!resolution) return null;
  return <span className="tooltip inline-flex items-center gap-1" data-tip="解像度（ピクセル）">
    <Ruler role="img" aria-label="解像度（ピクセル）" className="size-5 shrink-0" strokeWidth={1.75} />
    <span>{resolution.width} × {resolution.height}</span>
  </span>;
}
