import { ApiError } from '../api/client';

export function Loading({ label = '読み込み中…' }: { label?: string }) {
  return <div className="flex items-center justify-center gap-2 py-8" role="status"><span className="loading loading-spinner loading-sm" />{label}</div>;
}

export function ErrorMessage({ error, onRetry }: { error: Error; onRetry?: () => void }) {
  return <div className="alert alert-error my-4 break-words" role="alert">
    <span>{error instanceof ApiError ? error.message : '通信できませんでした。接続を確認して再度お試しください。'}</span>
    {onRetry && <button className="btn btn-sm" onClick={onRetry}>再試行</button>}
  </div>;
}
