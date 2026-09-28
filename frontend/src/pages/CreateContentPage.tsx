import { useEffect, useRef, useState } from 'react';
import { FileUp, Link, Eye, Pencil, Plus, Tags, Trash2, Upload } from 'lucide-react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { MediaType, type Tag } from '../api/generated';
import { apiRequest, contentsApi } from '../api/client';
import { ErrorMessage } from '../components/Feedback';
import { IconButton } from '../components/IconAction';
import { TagBadge } from '../components/TagBadge';
import { TermEditor } from '../components/TermEditor';
import { MediaTypeIcon } from '../components/MediaTypeIcon';
import { convertToAvif } from '../image/convert';

const maxFileSize = 100 * 1024 * 1024;

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
  const [mediaType, setMediaType] = useState<MediaType>(MediaType.ImageAvif);
  const [sourceMode, setSourceMode] = useState<'file' | 'url'>('file');
  const [file, setFile] = useState<File>();
  const [previewUrl, setPreviewUrl] = useState<string>();
  const [sourceUrl, setSourceUrl] = useState('');
  const [sourceError, setSourceError] = useState('');
  const [preparing, setPreparing] = useState(false);
  const [tags, setTags] = useState<Tag[]>([]);
  const [tagToEdit, setTagToEdit] = useState<Tag>();
  const request = useRef<AbortController>();
  const selection = useRef(0);
  const navigate = useNavigate();
  const client = useQueryClient();
  const mutation = useMutation({
    mutationFn: () => apiRequest(contentsApi.createContent({ content: file!, metadata: { tags } }), 'コンテンツを登録できませんでした。'),
    onSuccess: async (content) => {
      await client.invalidateQueries({ queryKey: ['contents'] });
      navigate(`/contents/${content.id}`);
    },
  });
  useEffect(() => {
    if (!file) { setPreviewUrl(undefined); return; }
    const url = URL.createObjectURL(file);
    setPreviewUrl(url);
    return () => URL.revokeObjectURL(url);
  }, [file]);
  useEffect(() => () => request.current?.abort(), []);

  function clearSelectedFile() {
    ++selection.current;
    request.current?.abort();
    setPreparing(false);
    setSourceError('');
    setFile(undefined);
  }

  function switchSource(mode: 'file' | 'url') {
    if (mode === sourceMode) return;
    clearSelectedFile();
    setSourceMode(mode);
  }

  function switchMediaType(type: MediaType) {
    if (type === mediaType) return;
    clearSelectedFile();
    setMediaType(type);
  }

  async function chooseFile(candidate?: File) {
    const currentSelection = ++selection.current;
    request.current?.abort();
    setPreparing(false);
    setSourceError('');
    if (!candidate) return;
    setFile(undefined);
    const controller = new AbortController();
    request.current = controller;
    setPreparing(true);
    try {
      const selected = await prepareContent(candidate, candidate.name, mediaType, controller.signal);
      if (currentSelection === selection.current) setFile(selected);
    } catch (error) { if (currentSelection === selection.current && !controller.signal.aborted) setSourceError((error as Error).message); }
    finally { if (request.current === controller) { request.current = undefined; setPreparing(false); } }
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
    setFile(undefined);
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
      if (currentSelection === selection.current && !controller.signal.aborted) setFile(selected);
    } catch (error) {
      if (!controller.signal.aborted) setSourceError(error instanceof TypeError ? 'URL から取得できませんでした。公開 URL と CORS 設定を確認してください。' : (error as Error).message);
    } finally {
      if (request.current === controller) { request.current = undefined; setPreparing(false); }
    }
  }
  const busy = mutation.isPending;
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
          <span>ファイルを選択</span>
          <input type="file" accept={mediaType === MediaType.ImageAvif ? 'image/*' : mediaType} aria-label="アップロードするファイル" className="sr-only" disabled={busy} onChange={(event) => { void chooseFile(event.target.files?.[0]); event.target.value = ''; }} />
        </label> :
        <form className="flex flex-wrap gap-2" onSubmit={(event) => { event.preventDefault(); void fetchUrl(); }}>
          <label className="input min-w-0 flex-1"><Link className="size-4 shrink-0" aria-hidden="true" /><input type="url" aria-label="コンテンツ URL" placeholder="https://example.com/image.jpg" value={sourceUrl} onChange={(event) => setSourceUrl(event.target.value)} /></label>
          <IconButton icon={Eye} label="URL をプレビュー" className="btn-secondary" type="submit" disabled={busy || preparing || !sourceUrl.trim()} />
        </form>}
      {preparing && <p role="status">コンテンツを準備中…</p>}
      {sourceError && <p role="alert" className="text-error">{sourceError}</p>}
      {previewUrl && <figure className="flex flex-col gap-2"><img src={previewUrl} alt="登録するコンテンツのプレビュー" className="max-h-[60vh] max-w-full rounded-lg object-contain" /><figcaption className="break-all text-sm">{file?.name} · {((file?.size || 0) / 1024 / 1024).toFixed(2)} MiB</figcaption></figure>}
    </div></section>
    <section className="card bg-base-200" aria-label="登録するタグ"><div className="card-body gap-4">
      <h2 className="card-title"><Tags className="size-5" aria-hidden="true" />タグ</h2>
      <TermEditor onAddTag={(tag) => { setTags([...tags, tag]); setTagToEdit(undefined); }} tagToEdit={tagToEdit} existingKeys={tags.map((tag) => tag.key)} />
      {tags.length > 0 && <ul className="flex flex-wrap gap-3" aria-label="登録するタグ一覧">{tags.map((tag, index) => <li key={`${tag.key}-${index}`} className="flex items-center gap-1"><TagBadge tag={tag} /><IconButton className="btn-ghost btn-xs" icon={Pencil} label={`${tag.key} を編集`} disabled={busy} onClick={() => { setTags(tags.filter((item) => item !== tag)); setTagToEdit(tag); }} /><IconButton className="btn-ghost btn-xs" icon={Trash2} label={`${tag.key} を削除`} disabled={busy} onClick={() => setTags(tags.filter((item) => item !== tag))} /></li>)}</ul>}
    </div></section>
    {mutation.isError && <ErrorMessage error={mutation.error} />}
    <button className="btn btn-primary w-full" disabled={!file || busy || preparing || Boolean(tagToEdit)} onClick={() => mutation.mutate()}><Upload className="size-5" aria-hidden="true" />{busy ? '登録中…' : '登録する'}</button>
  </section>;
}
