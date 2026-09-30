import { useEffect, useRef, useState } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { Trash2 } from 'lucide-react';
import type { Content } from '../api/generated';
import { ApiError, apiRequest, contentsApi } from '../api/client';
import { ContentPreviewCarousel } from './ContentPreviewCarousel';

export function DeleteContentsDialog({ items, onClose, onDeleted }: { items: Content[]; onClose: () => void; onDeleted: (ids: string[]) => void }) {
  const [targets, setTargets] = useState(items);
  const [busy, setBusy] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [progress, setProgress] = useState<{ done: number; total: number }>();
  const dialog = useRef<HTMLDialogElement>(null);
  const cancelButton = useRef<HTMLButtonElement>(null);
  const client = useQueryClient();
  useEffect(() => {
    const previous = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    const element = dialog.current;
    element?.showModal();
    cancelButton.current?.focus();
    return () => { element?.close(); previous?.focus(); };
  }, []);
  async function remove() {
    if (busy || !targets.length) return;
    const pending = [...targets];
    const failed: Record<string, string> = {};
    const deleted: string[] = [];
    setBusy(true);
    setErrors({});
    setProgress({ done: 0, total: pending.length });
    for (const [index, item] of pending.entries()) {
      try {
        await apiRequest(contentsApi.deleteContent({ contentId: item.id }), 'コンテンツを削除できませんでした。');
        deleted.push(item.id);
      } catch (error) {
        failed[item.id] = error instanceof ApiError ? error.message : '通信できませんでした。接続を確認して再度お試しください。';
      }
      setProgress({ done: index + 1, total: pending.length });
    }
    if (deleted.length) {
      for (const id of deleted) client.removeQueries({ queryKey: ['content', id], exact: true });
      await client.invalidateQueries({ queryKey: ['contents'] });
      onDeleted(deleted);
    }
    setBusy(false);
    if (!Object.keys(failed).length) onClose();
    else { setTargets(pending.filter((item) => failed[item.id])); setErrors(failed); }
  }
  return <dialog ref={dialog} className="modal" aria-labelledby="delete-title" onCancel={(event) => { event.preventDefault(); if (!busy) onClose(); }}>
    <div className="modal-box max-h-[90vh] max-w-xl space-y-4">
      <h2 id="delete-title" className="flex items-center gap-2 text-xl font-bold"><Trash2 className="size-5" />コンテンツを削除</h2>
        <p>{targets.length} 件のコンテンツを削除します。この操作は取り消せません。</p>
        <ContentPreviewCarousel label="削除するコンテンツのプレビュー" items={targets.map((item, index) => ({ id: item.id, image: item.thumbnailUrl, alt: `${index + 1} 件目のサムネイル`, removeLabel: `${index + 1} 件目を対象から取り除く` }))} onRemove={(id) => { const next = targets.filter((item) => item.id !== id); setTargets(next); if (!next.length) onClose(); }} busy={busy} />
        {Object.keys(errors).length > 0 && <div role="alert" className="alert alert-error"><ul>{targets.filter((item) => errors[item.id]).map((item, index) => <li key={item.id}>{index + 1} 件目: {errors[item.id]}</li>)}</ul></div>}
        {busy && progress && <div role="status">削除中… {progress.done} / {progress.total}</div>}
        <div className="modal-action"><button ref={cancelButton} className="btn" disabled={busy} onClick={onClose}>キャンセル</button><button className="btn btn-error" disabled={busy || !targets.length} onClick={() => void remove()}><Trash2 className="size-4" />{busy ? '削除中…' : `${targets.length} 件を削除`}</button></div>
    </div>
    <div className="modal-backdrop"><button disabled={busy} aria-label="削除確認を閉じる" onClick={onClose}>閉じる</button></div>
  </dialog>;
}
