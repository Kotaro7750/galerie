import { EyeOff, FunnelX, Pause, Play, Shuffle, Timer } from 'lucide-react';
import { IconButton } from './IconAction';

type SlideshowControlsProps = {
  playing: boolean;
  intervalSeconds: number;
  targetCount: number;
  totalCount: number;
  excludedCount: number;
  currentExcluded: boolean;
  shuffled: boolean;
  onToggle: () => void;
  onShuffleToggle: () => void;
  onIntervalChange: (seconds: number) => void;
  onToggleExcluded: () => void;
  onReset: () => void;
  className?: string;
};

export function SlideshowControls({ playing, intervalSeconds, targetCount, totalCount, excludedCount, currentExcluded, shuffled, onToggle, onShuffleToggle, onIntervalChange, onToggleExcluded, onReset, className = '' }: SlideshowControlsProps) {
  return <div role="group" aria-label="スライドショー" className={`flex flex-wrap items-center justify-center gap-2 rounded-box bg-base-100/90 p-2 shadow-sm ${className}`}>
    <IconButton label={playing ? 'スライドショーを一時停止' : 'スライドショーを再生'} icon={playing ? Pause : Play} disabled={!playing && targetCount < 2} onClick={onToggle} />
    <IconButton label="シャッフル再生" icon={Shuffle} className={shuffled ? 'btn-soft btn-primary' : 'btn-ghost'} aria-pressed={shuffled} onClick={onShuffleToggle} />
    <label className="flex items-center gap-2 text-sm">
      <Timer className="size-5 shrink-0" strokeWidth={1.75} aria-hidden="true" />
      <span className="whitespace-nowrap">{intervalSeconds}s</span>
      <input type="range" className="range range-primary range-xs w-28" min="1" max="15" step="1" value={intervalSeconds} onChange={(event) => onIntervalChange(Number(event.target.value))} aria-label="スライドショーの表示間隔" />
    </label>
    <span className="text-xs whitespace-nowrap" aria-live="polite" aria-label={`スライドショー対象 ${targetCount} / ${totalCount} 件`}>{targetCount} / {totalCount}</span>
    <span className="tooltip tooltip-top" data-tip={currentExcluded ? '再生対象から除外中（クリックで戻す）' : '再生対象（クリックで除外）'}>
      <IconButton label="表示中の画像をスライドショー対象から除外" icon={EyeOff} className={currentExcluded ? 'btn-soft btn-primary' : 'btn-ghost'} aria-pressed={currentExcluded} disabled={totalCount === 0} onClick={onToggleExcluded} />
    </span>
    <span className="tooltip tooltip-top" data-tip="除外した画像をすべて戻す">
      <IconButton label="スライドショー対象をリセット" icon={FunnelX} disabled={excludedCount === 0} onClick={onReset} />
    </span>
  </div>;
}
