import { useState } from 'react';
import type { ImageResolutionValue } from './ImageResolution';

export function ContentImage({ src, alt, thumbnail = false, loading, onResolutionChange }: { src: string; alt: string; thumbnail?: boolean; loading?: 'eager' | 'lazy'; onResolutionChange?: (resolution?: ImageResolutionValue) => void }) {
  const [failed, setFailed] = useState(false);
  const [attempt, setAttempt] = useState(0);
  if (failed) return <div className="flex h-full flex-col items-center justify-center gap-4 p-4 text-center" role="status">
    <span>画像を読み込めませんでした</span>
    {!thumbnail && <button className="btn btn-sm" onClick={() => { setAttempt(attempt + 1); setFailed(false); }}>画像を再読み込み</button>}
  </div>;
  return <img className={thumbnail ? 'h-full w-full object-cover' : 'max-h-screen max-w-full object-contain'} key={attempt} src={src} alt={alt} loading={loading ?? (thumbnail ? 'lazy' : 'eager')}
    decoding="async" onLoad={(event) => onResolutionChange?.({ width: event.currentTarget.naturalWidth, height: event.currentTarget.naturalHeight })}
    onError={() => { onResolutionChange?.(); setFailed(true); }} />;
}
