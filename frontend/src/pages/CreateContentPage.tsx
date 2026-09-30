import { useEffect, useRef, useState } from 'react';
import { FileUp, Link, Eye, Plus, Save, Upload, X } from 'lucide-react';
import { useQueryClient } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { MediaType, type Tag } from '../api/generated';
import { ApiError, apiRequest, contentsApi } from '../api/client';
import { Loading } from '../components/Feedback';
import { IconButton } from '../components/IconAction';
import { ContentTagEditor } from '../components/ContentTagEditor';
import { ContentPreviewCarousel } from '../components/ContentPreviewCarousel';
import { MediaTypeIcon } from '../components/MediaTypeIcon';
import { convertToAvif } from '../image/convert';
import { useTagSchema } from '../hooks/useTagSchema';
import { registrationErrors } from '../tagSchema';

const maxFileSize = 100 * 1024 * 1024;
type Draft = { id: number; file: File };

async function prepareImage(file: Blob, name: string, signal: AbortSignal): Promise<File> {
  if (!file.size || file.size > maxFileSize) throw new Error('100 MiB 以下の画像を選択してください。');
  const header = new Uint8Array(await file.slice(0, 64).arrayBuffer());
  if (signal.aborted) throw new DOMException('Aborted', 'AbortError');
  if (header.length >= 12 && String.fromCharCode(...header.slice(4, 8)) === 'ftyp') {
    const boxSize = new DataView(header.buffer).getUint32(0);
    const brands = [8, ...Array.from({ length: Math.max(0, Math.floor((Math.min(boxSize, header.length) - 16) / 4)) }, (_, index) => 16 + index * 4)]
      .map((offset) => String.fromCharCode(...header.slice(offset, offset + 4)));
    if (brands.some((brand) => brand === 'avif' || brand === 'avis')) return new File([file], name.replace(/\.[^.]+$/, '') + '.avif', { type: 'image/avif' });
  }
  const converted = await convertToAvif(file, signal);
  if (converted.size > maxFileSize) throw new Error('変換後の AVIF ファイルが 100 MiB を超えました。');
  return new File([converted], name.replace(/\.[^.]+$/, '') + '.avif', { type: 'image/avif' });
}

function prepareContent(file: Blob, name: string, mediaType: MediaType, signal: AbortSignal): Promise<File> {
  switch (mediaType) {
    case MediaType.ImageAvif: return prepareImage(file, name, signal);
    default: {
      const unsupported: never = mediaType;
      throw new Error(`未対応のメディアタイプです: ${unsupported}`);
    }
  }
}

export function CreateContentPage() {
  const schemaQuery = useTagSchema();
  const [mediaType, setMediaType] = useState<MediaType>(MediaType.ImageAvif);
  const [sourceMode, setSourceMode] = useState<'file' | 'url'>('file');
  const [drafts, setDrafts] = useState<Draft[]>([]);
  const nextDraftId = useRef(0);
  const [sourceUrl, setSourceUrl] = useState('');
  const [sourceError, setSourceError] = useState('');
  const [preparing, setPreparing] = useState(false);
  const [busy, setBusy] = useState(false);
  const [progress, setProgress] = useState<{ done: number; total: number }>();
  const [failures, setFailures] = useState<Record<number, string>>({});
  const [tags, setTags] = useState<Tag[]>([]);
  const [editingTag, setEditingTag] = useState(false);
  const request = useRef<AbortController>();
  const selection = useRef(0);
  const navigate = useNavigate();
  const client = useQueryClient();
  const tagErrors = schemaQuery.data ? registrationErrors(schemaQuery.data, tags) : [];
  useEffect(() => () => request.current?.abort(), []);

  function cancelPreparation() {
    ++selection.current;
    request.current?.abort();
    setPreparing(false);
    setSourceError('');
  }

  function switchSource(mode: 'file' | 'url') {
    if (mode === sourceMode) return;
    cancelPreparation();
    setSourceMode(mode);
  }

  function switchMediaType(type: MediaType) {
    if (type === mediaType) return;
    cancelPreparation();
    setMediaType(type);
  }

  function clearUrl() {
    setSourceUrl('');
    cancelPreparation();
  }

  function addDraft(file: File) {
    setDrafts((current) => [...current, { id: ++nextDraftId.current, file }]);
  }

  async function chooseFiles(candidates: File[]) {
    const currentSelection = ++selection.current;
    request.current?.abort();
    setPreparing(false);
    setSourceError('');
    if (!candidates.length) return;
    const controller = new AbortController();
    request.current = controller;
    setPreparing(true);
    const errors: string[] = [];
    try {
      for (const candidate of candidates) {
        if (controller.signal.aborted) break;
        try {
          const selected = await prepareContent(candidate, candidate.name, mediaType, controller.signal);
          if (currentSelection === selection.current && !controller.signal.aborted) addDraft(selected);
        } catch (error) {
          if (controller.signal.aborted) break;
          errors.push(`${candidate.name}: ${(error as Error).message}`);
        }
      }
      if (currentSelection === selection.current && errors.length) setSourceError(errors.join(' '));
    } finally { if (request.current === controller) { request.current = undefined; setPreparing(false); } }
  }
  async function fetchUrl() {
    const currentSelection = ++selection.current;
    request.current?.abort();
    setSourceError('');
    let url: URL;
    try {
      url = new URL(sourceUrl);
      if (!['http:', 'https:'].includes(url.protocol)) throw new Error();
    } catch { setSourceError('http または https の URL を入力してください。'); return; }
    const controller = new AbortController();
    request.current = controller;
    setPreparing(true);
    try {
      const response = await fetch(url, { signal: controller.signal });
      if (!response.ok) throw new Error(`URL から取得できませんでした（HTTP ${response.status}）。`);
      if (Number(response.headers.get('Content-Length')) > maxFileSize) throw new Error('100 MiB 以下の画像を指定してください。');
      const blob = await response.blob();
      if (controller.signal.aborted) return;
      const selected = await prepareContent(blob, url.pathname.split('/').pop() || 'content', mediaType, controller.signal);
      if (currentSelection === selection.current && !controller.signal.aborted) addDraft(selected);
    } catch (error) {
      if (!controller.signal.aborted) setSourceError(error instanceof TypeError ? 'URL から取得できませんでした。取得元の CORS 設定を確認するか、画像をコピーして貼り付けてください。' : (error as Error).message);
    } finally {
      if (request.current === controller) { request.current = undefined; setPreparing(false); }
    }
  }
  function pasteImage(event: ClipboardEvent) {
    if (busy || preparing || !event.clipboardData) return;
    const image = Array.from(event.clipboardData.items)
      .find((item) => item.kind === 'file' && item.type.startsWith('image/'))?.getAsFile();
    if (!image) return;
    event.preventDefault();
    setSourceMode('file');
    setSourceUrl('');
    void chooseFiles([image]);
  }
  useEffect(() => {
    document.addEventListener('paste', pasteImage);
    return () => document.removeEventListener('paste', pasteImage);
  });
  async function register() {
    if (busy || !drafts.length || !schemaQuery.data || tagErrors.length || editingTag) return;
    const pending = [...drafts];
    const submittedTags = [...tags];
    setBusy(true);
    setProgress({ done: 0, total: pending.length });
    setFailures({});
    let lastId: string | undefined;
    let failed = 0;
    for (const [index, draft] of pending.entries()) {
      try {
        const content = await apiRequest(contentsApi.createContent({ content: draft.file, metadata: { tags: submittedTags } }), 'コンテンツを登録できませんでした。');
        lastId = content.id;
        setDrafts((current) => current.filter((item) => item.id !== draft.id));
      } catch (error) {
        failed++;
        setFailures((current) => ({ ...current, [draft.id]: error instanceof ApiError ? error.message : '通信できませんでした。接続を確認して再度お試しください。' }));
      }
      setProgress({ done: index + 1, total: pending.length });
    }
    if (lastId) await client.invalidateQueries({ queryKey: ['contents'] });
    setBusy(false);
    if (!failed && lastId) navigate(`/contents/${lastId}`);
  }
  return <section className="mx-auto max-w-3xl space-y-6">
    <h1 className="flex items-center gap-2 text-2xl font-bold"><Plus className="size-6" aria-hidden="true" />コンテンツを登録</h1>
    <section className="card bg-base-200" aria-label="コンテンツファイル"><div className="card-body gap-4">
      <div className="flex items-center justify-between gap-3">
        <h2 className="card-title"><Upload className="size-5" aria-hidden="true" />コンテンツ</h2>
        <div className="flex items-center gap-3">
          <fieldset aria-label="登録するメディアタイプ" className="flex flex-wrap gap-2">{Object.values(MediaType).map((type) =>
            <label key={type} className={`btn btn-square relative has-focus-visible:outline-2 has-focus-visible:outline-offset-2 ${mediaType === type ? 'btn-soft' : 'btn-ghost'}`}>
              <MediaTypeIcon mediaType={type} />
              <input type="radio" name="registrationMediaType" className="absolute inset-0 z-10 size-full cursor-pointer opacity-0" aria-label={type} checked={mediaType === type} disabled={busy} onChange={() => switchMediaType(type)} />
            </label>)}</fieldset>
          <div className="join" role="group" aria-label="入力方法">
            <IconButton icon={FileUp} label="ファイルから追加" className={`join-item ${sourceMode === 'file' ? 'btn-primary' : 'btn-ghost'}`} aria-pressed={sourceMode === 'file'} disabled={busy} onClick={() => switchSource('file')} />
            <IconButton icon={Link} label="URL から追加" className={`join-item ${sourceMode === 'url' ? 'btn-primary' : 'btn-ghost'}`} aria-pressed={sourceMode === 'url'} disabled={busy} onClick={() => switchSource('url')} />
          </div>
        </div>
      </div>
      {sourceMode === 'file' ?
        <label className="flex cursor-pointer flex-col items-center gap-2 rounded-box border border-dashed border-base-content/30 p-6 text-center hover:border-primary focus-within:outline-2 focus-within:outline-primary">
          <FileUp className="size-8" aria-hidden="true" />
          <span>ファイルを選択、または画像を貼り付け</span>
          <input type="file" multiple accept={mediaType === MediaType.ImageAvif ? 'image/*' : mediaType} aria-label="アップロードするファイル" className="sr-only" disabled={busy || preparing} onChange={(event) => { void chooseFiles(Array.from(event.target.files || [])); event.target.value = ''; }} />
        </label> :
        <form className="flex flex-wrap gap-2" onSubmit={(event) => { event.preventDefault(); void fetchUrl(); }}>
          <div className="input min-w-0 flex-1"><Link className="size-4 shrink-0" aria-hidden="true" /><input type="url" aria-label="コンテンツ URL" placeholder="https://example.com/image.jpg" value={sourceUrl} onChange={(event) => setSourceUrl(event.target.value)} />{sourceUrl && <button type="button" className="btn btn-ghost btn-square btn-sm" aria-label="URL をクリア" title="URL をクリア" disabled={busy} onClick={clearUrl}><X className="size-4" aria-hidden="true" /></button>}</div>
          <IconButton icon={Eye} label="URL をプレビュー" className="btn-secondary" type="submit" disabled={busy || preparing || !sourceUrl.trim()} />
        </form>}
      {preparing && <Loading label="コンテンツを準備中…" />}
      {sourceError && <p role="alert" className="text-error">{sourceError}</p>}
      {drafts.length > 0 && <div aria-label="登録するコンテンツ一覧" className="relative min-w-0">
        <ContentPreviewCarousel label="登録するコンテンツのプレビュー" items={drafts.map((draft, index) => ({ id: draft.id, image: draft.file, alt: '登録するコンテンツのプレビュー', removeLabel: `${index + 1} 件目のコンテンツを削除` }))} busy={busy} onRemove={(id) => { setDrafts((current) => current.filter((item) => item.id !== id)); setFailures((current) => { const remaining = { ...current }; delete remaining[Number(id)]; return remaining; }); }} />
      </div>}
    </div></section>
    <ContentTagEditor tags={tags} onChange={setTags} schema={schemaQuery.data} schemaError={schemaQuery.isError} busy={busy} label="登録するタグ" onEditingChange={setEditingTag} />
    {Object.keys(failures).length > 0 && <div role="alert" className="alert alert-error my-4 break-words"><ul>{drafts.filter((draft) => failures[draft.id]).map((draft) => <li key={draft.id}>{Object.keys(failures).length > 1 ? `${draft.file.name}: ` : ''}{failures[draft.id]}</li>)}</ul></div>}
    {progress && busy && <div role="status" className="flex items-center gap-3"><progress className="progress progress-primary flex-1" value={progress.done} max={progress.total} /><span>{progress.done} / {progress.total}</span></div>}
    <button className="btn btn-primary w-full" disabled={!drafts.length || busy || preparing || editingTag || !schemaQuery.data || tagErrors.length > 0} onClick={() => void register()}><Save className="size-5" aria-hidden="true" />{busy ? '登録中…' : '登録する'}</button>
  </section>;
}
