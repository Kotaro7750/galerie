import { Grid2X2, Grid3X3, RefreshCw } from 'lucide-react';
import { IconButton } from './IconAction';
import type { useContents } from '../hooks/useContents';

type Props = Pick<ReturnType<typeof useContents>, 'density' | 'setDensity' | 'refresh'> & { refreshing: boolean };
const sizes = [
  { value: 'comfortable', label: '大きく', icon: Grid2X2 },
  { value: 'compact', label: '小さく', icon: Grid3X3 },
] as const;

export function GalleryToolbar({ density, setDensity, refresh, refreshing }: Props) {
  return <div className="flex flex-wrap items-center justify-end gap-2">
    <div className="join" role="group" aria-label="サムネイルの大きさ">
      {sizes.map(({ value, label, icon }) => <IconButton key={value} label={label} icon={icon}
        className={`btn-sm join-item ${density === value ? 'btn-active' : ''}`}
        aria-pressed={density === value} onClick={() => setDensity(value)} />)}
    </div>
    <IconButton label="一覧を更新" icon={RefreshCw} className="btn-ghost btn-sm" disabled={refreshing} onClick={refresh} />
  </div>;
}
