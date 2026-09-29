import { useCallback, useEffect, useRef, useState } from 'react';
import { ChevronLeft, ChevronRight, FileUp, HardDrive, Link, Eye, Pencil, Plus, Save, Tags, Trash2, Upload, X } from 'lucide-react';
import type { Swiper as SwiperInstance } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/react';
import 'swiper/css';
import { useQueryClient } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { MediaType, type Tag } from '../api/generated';
import { ApiError, apiRequest, contentsApi } from '../api/client';
import { Loading } from '../components/Feedback';
import { IconButton } from '../components/IconAction';
import { TagBadge } from '../components/TagBadge';
import { TermEditor } from '../components/TermEditor';
import { MediaTypeIcon } from '../components/MediaTypeIcon';
import { ImageResolution, type ImageResolutionValue } from '../components/ImageResolution';
import { convertToAvif } from '../image/convert';

const maxFileSize = 100 * 1024 * 1024;
type Draft = { id: number; file: File };

function DraftPreview({ file, index, total, active, busy, onDelete, onPrevious, onNext, onLayoutChange }: { file: File; index: number; total: number; active: boolean; busy: boolean; onDelete: () => void; onPrevious: () => void; onNext: () => void; onLayoutChange: () => void }) {
  const [previewUrl, setPreviewUrl] = useState<string>();
  const [resolution, setResolution] = useState<ImageResolutionValue>();
  useEffect(() => {
    const url = URL.createObjectURL(file);
    setPreviewUrl(url);
    return () => URL.revokeObjectURL(url);
  }, [file]);
  useEffect(() => {
    const frame = requestAnimationFrame(onLayoutChange);
    return () => cancelAnimationFrame(frame);
  }, [previewUrl, resolution, onLayoutChange]);
  return previewUrl && <figure className="flex w-fit max-w-full flex-col items-center gap-2">
    <figcaption className="flex w-full items-center justify-between gap-2 text-sm">
      <span aria-label={active ? '表示中の画像' : undefined}>{index + 1} / {total}</span>
      <span className="flex items-center justify-end gap-2 sm:gap-4">
        <span className="tooltip inline-flex items-center gap-1" data-tip="ファイルサイズ"><HardDrive role="img" aria-label="ファイルサイズ" className="size-5 shrink-0" strokeWidth={1.75} /><span>{(file.size / 1024 / 1024).toFixed(2)} MiB</span></span>
        <ImageResolution resolution={resolution} />
      </span>
    </figcaption>
    <div className="relative max-w-full">
      <img src={previewUrl} alt="登録するコンテンツのプレビュー" className="block max-h-[60vh] max-w-full rounded-lg object-contain" onLoad={(event) => setResolution({ width: event.currentTarget.naturalWidth, height: event.currentTarget.naturalHeight })} onError={() => setResolution(undefined)} />
      <IconButton icon={X} label={`${index + 1} 件目のコンテンツを削除`} className="btn-circle btn-error btn-sm absolute top-2 right-2 z-10 shadow-md" disabled={busy} onClick={onDelete} />
      {active && total > 1 && <>
        <IconButton icon={ChevronLeft} label="前の画像" className="btn-ghost absolute left-2 top-1/2 z-10 min-h-11 min-w-11 -translate-y-1/2 bg-base-100/80" onClick={onPrevious} />
        <IconButton icon={ChevronRight} label="次の画像" className="btn-ghost absolute right-2 top-1/2 z-10 min-h-11 min-w-11 -translate-y-1/2 bg-base-100/80" onClick={onNext} />
      </>}
    </div>
  </figure>;
}

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
  const [drafts, setDrafts] = useState<Draft[]>([]);
  const [activeIndex, setActiveIndex] = useState(0);
  const nextDraftId = useRef(0);
  const swiper = useRef<SwiperInstance | null>(null);
  const updateCarouselHeight = useCallback(() => swiper.current?.updateAutoHeight(0), []);
  const [sourceUrl, setSourceUrl] = useState('');
  const [sourceError, setSourceError] = useState('');
  const [preparing, setPreparing] = useState(false);
  const [busy, setBusy] = useState(false);
  const [progress, setProgress] = useState<{ done: number; total: number }>();
  const [failures, setFailures] = useState<Record<number, string>>({});
  const [tags, setTags] = useState<Tag[]>([]);
  const [tagToEdit, setTagToEdit] = useState<Tag>();
  const request = useRef<AbortController>();
  const selection = useRef(0);
  const navigate = useNavigate();
  const client = useQueryClient();
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
    if (busy || !drafts.length) return;
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
        <Swiper key={drafts.map((draft) => draft.id).join('-')} autoHeight loop={drafts.length > 1} initialSlide={Math.min(activeIndex, drafts.length - 1)} onRealIndexChange={(instance) => setActiveIndex(instance.realIndex)} onSwiper={(instance) => { swiper.current = instance; }} onBeforeDestroy={(instance) => { if (swiper.current === instance) swiper.current = null; }} className="w-full">
          {drafts.map((draft, index) => <SwiperSlide key={draft.id} className="relative flex! flex-col items-center px-2 sm:px-12">
            <DraftPreview file={draft.file} index={index} total={drafts.length} active={index === Math.min(activeIndex, drafts.length - 1)} busy={busy} onLayoutChange={updateCarouselHeight} onPrevious={() => swiper.current?.slideToLoop((swiper.current.realIndex - 1 + drafts.length) % drafts.length, 0)} onNext={() => swiper.current?.slideToLoop((swiper.current.realIndex + 1) % drafts.length, 0)} onDelete={() => { setDrafts((current) => current.filter((item) => item.id !== draft.id)); setFailures((current) => { const remaining = { ...current }; delete remaining[draft.id]; return remaining; }); }} />
          </SwiperSlide>)}
        </Swiper>
      </div>}
    </div></section>
    <section className="card bg-base-200" aria-label="登録するタグ"><div className="card-body gap-4">
      <h2 className="card-title"><Tags className="size-5" aria-hidden="true" />タグ</h2>
      <TermEditor onAddTag={(tag) => { setTags([...tags, tag]); setTagToEdit(undefined); }} tagToEdit={tagToEdit} existingKeys={tags.map((tag) => tag.key)} />
      {tags.length > 0 && <ul className="flex flex-wrap gap-3" aria-label="登録するタグ一覧">{tags.map((tag, index) => <li key={`${tag.key}-${index}`} className="flex items-center gap-1"><TagBadge tag={tag} /><IconButton className="btn-ghost btn-xs" icon={Pencil} label={`${tag.key} を編集`} disabled={busy} onClick={() => { setTags(tags.filter((item) => item !== tag)); setTagToEdit(tag); }} /><IconButton className="btn-ghost btn-xs" icon={Trash2} label={`${tag.key} を削除`} disabled={busy} onClick={() => setTags(tags.filter((item) => item !== tag))} /></li>)}</ul>}
    </div></section>
    {Object.keys(failures).length > 0 && <div role="alert" className="alert alert-error my-4 break-words"><ul>{drafts.filter((draft) => failures[draft.id]).map((draft) => <li key={draft.id}>{Object.keys(failures).length > 1 ? `${draft.file.name}: ` : ''}{failures[draft.id]}</li>)}</ul></div>}
    {progress && busy && <div role="status" className="flex items-center gap-3"><progress className="progress progress-primary flex-1" value={progress.done} max={progress.total} /><span>{progress.done} / {progress.total}</span></div>}
    <button className="btn btn-primary w-full" disabled={!drafts.length || busy || preparing || Boolean(tagToEdit)} onClick={() => void register()}><Save className="size-5" aria-hidden="true" />{busy ? '登録中…' : '登録する'}</button>
  </section>;
}
