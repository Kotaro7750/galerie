import { ApiError } from '../api/client';
import { RefreshCw } from 'lucide-react';
import { IconButton } from './IconAction';

export function Loading({ label = '読み込み中…' }: { label?: string }) {
  return <div className="flex items-center justify-center gap-2 py-8" role="status"><span className="loading loading-spinner loading-sm" />{label}</div>;
}

export function ErrorMessage({ error, onRetry }: { error: Error; onRetry?: () => void }) {
  return <div className="alert alert-error my-4 flex items-start gap-2 break-words" role="alert">
    <span className="min-w-0 flex-1">{error instanceof ApiError ? error.message : '通信できませんでした。接続を確認して再度お試しください。'}</span>
    {onRetry && <IconButton className="btn-sm shrink-0" icon={RefreshCw} label="再試行" onClick={onRetry} />}
  </div>;
}
