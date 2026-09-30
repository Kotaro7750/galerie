import { useEffect, useRef, useState } from 'react';
import { useLocation, useNavigate, useParams } from 'react-router-dom';
import { useQueryClient } from '@tanstack/react-query';
import { ArrowLeft, Pencil, Save } from 'lucide-react';
import type { Content, Tag } from '../api/generated';
import { ApiError, apiRequest, contentsApi } from '../api/client';
import { ContentTagEditor } from '../components/ContentTagEditor';
import { ContentPreviewCarousel } from '../components/ContentPreviewCarousel';
import { ErrorMessage, Loading } from '../components/Feedback';
import { IconLink } from '../components/IconAction';
import { useContent } from '../hooks/useContent';
import { useTagSchema } from '../hooks/useTagSchema';
import { registrationErrors, tagError } from '../tagSchema';
import { commonTags, mergedTags, validTags } from '../editTags';
import { diagnosticReasons } from '../tagPresentation';

export function EditContentPage() {
  const { contentId } = useParams();
  const location = useLocation();
  const navigate = useNavigate();
  const client = useQueryClient();
  const query = useContent(contentId || '', Boolean(contentId));
  const schema = useTagSchema();
  const activeSchema = schema.data;
  const selected = (location.state as { items?: Content[] } | null)?.items;
  const editKey = contentId || selected?.map((item) => item.id).join('|') || '';
  const initializedKey = useRef('');
  const [pending, setPending] = useState<Content[]>(contentId ? [] : selected || []);
  const [tags, setTags] = useState<Tag[]>([]);
  const [commonKeys, setCommonKeys] = useState<string[]>([]);
  const [editingTag, setEditingTag] = useState(false);
  const [busy, setBusy] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [progress, setProgress] = useState<{ done: number; total: number }>();
  useEffect(() => {
    if (!activeSchema || !editKey || initializedKey.current === editKey) return;
    if (contentId && query.data?.id !== contentId) return;
    const targets = contentId ? [query.data!] : selected || [];
    const initialTags = contentId ? validTags(targets[0], activeSchema) : commonTags(targets, activeSchema);
    setPending(targets);
    setTags(initialTags);
    setCommonKeys(initialTags.map((tag) => tag.key));
    initializedKey.current = editKey;
  }, [activeSchema, contentId, editKey, query.data, selected]);
  const validationErrors = activeSchema && initializedKey.current === editKey
    ? pending.flatMap((item, index) => registrationErrors(activeSchema, mergedTags(item, tags, commonKeys, activeSchema))
      .map((error) => pending.length > 1 ? `${index + 1} 件目: ${error}` : error)) : [];
  const invalidNotices = activeSchema ? pending.flatMap((item, index) => [
    ...item.diagnostics.filter((diagnostic) => diagnostic.kind !== 'missingRequiredTag')
      .map((diagnostic) => ({ id: `${item.id}-${diagnostic.key}-${diagnostic.kind}`, message: `${diagnostic.key}: ${diagnosticReasons[diagnostic.kind]}` })),
    ...item.tags.filter((tag) => tagError(activeSchema, tag))
      .map((tag) => ({ id: `${item.id}-${tag.key}`, message: `${tag.key}: ${tagError(activeSchema, tag)}` })),
  ].map((notice) => ({ ...notice, message: pending.length > 1 ? `${index + 1} 件目: ${notice.message}` : notice.message }))) : [];
  async function save() {
    if (busy || !pending.length || !activeSchema || initializedKey.current !== editKey || editingTag || validationErrors.length) return;
    const targets = [...pending];
    const submittedTags = [...tags];
    const controlledKeys = [...commonKeys];
    const failed: Record<string, string> = {};
    setBusy(true);
    setErrors({});
    setProgress({ done: 0, total: targets.length });
    for (const [index, item] of targets.entries()) {
      try {
        const updated = await apiRequest(contentsApi.updateContent({ contentId: item.id, contentMetadataInput: { tags: mergedTags(item, submittedTags, controlledKeys, activeSchema) } }), 'タグを更新できませんでした。');
        client.setQueryData(['content', item.id], updated);
      } catch (error) {
        failed[item.id] = error instanceof ApiError ? error.message : '通信できませんでした。接続を確認して再度お試しください。';
      }
      setProgress({ done: index + 1, total: targets.length });
    }
    await client.invalidateQueries({ queryKey: ['contents'] });
    setBusy(false);
    if (Object.keys(failed).length) { setPending(targets.filter((item) => failed[item.id])); setErrors(failed); }
    else navigate(contentId ? `/contents/${contentId}` : '/contents', { replace: true });
  }
  if (!contentId && !selected?.length) return <section className="space-y-4"><p>編集するコンテンツを一覧で選択してください。</p><IconLink to="/contents" label="一覧に戻る" icon={ArrowLeft} /></section>;
  return <section className="mx-auto max-w-3xl space-y-6">
    <div className="flex items-center gap-3"><IconLink to={contentId ? `/contents/${contentId}` : '/contents'} label="戻る" icon={ArrowLeft} /><h1 className="flex items-center gap-2 text-2xl font-bold"><Pencil className="size-6" />コンテンツのタグを編集</h1></div>
    {contentId && query.isPending && <Loading />}
    {contentId && query.isError && <ErrorMessage error={query.error} onRetry={() => void query.refetch()} />}
    {pending.length > 0 && <>
      <section className="card bg-base-200" aria-label="更新するコンテンツ"><div className="card-body gap-4"><h2 className="card-title">コンテンツ</h2><ContentPreviewCarousel label="更新するコンテンツのプレビュー" items={pending.map((item, index) => ({ id: item.id, image: item.thumbnailUrl, alt: `${index + 1} 件目のサムネイル` }))} /></div></section>
      <ContentTagEditor tags={tags} onChange={setTags} schema={activeSchema} schemaError={schema.isError} busy={busy} onEditingChange={setEditingTag} validationErrors={validationErrors} />
      {invalidNotices.length > 0 && <div role="note" className="alert alert-warning"><div><p>更新すると、次の無効なタグは削除されます。更新リクエストには含めません。</p><ul className="list-disc pl-5" aria-label="更新時に削除される無効なタグ">{invalidNotices.map((notice) => <li key={notice.id}>{notice.message}</li>)}</ul></div></div>}
      {Object.keys(errors).length > 0 && <div role="alert" className="alert alert-error"><ul>{pending.map((item, index) => errors[item.id] && <li key={item.id}>{index + 1} 件目: {errors[item.id]}</li>)}</ul></div>}
      {busy && progress && <div role="status">更新中… {progress.done} / {progress.total}</div>}
      <button className="btn btn-primary w-full" disabled={busy || editingTag || !activeSchema || initializedKey.current !== editKey || validationErrors.length > 0} onClick={() => void save()}><Save className="size-5" />{busy ? '更新中…' : `${pending.length} 件のタグを更新`}</button>
    </>}
  </section>;
}
