import { ContentSearch } from '../components/ContentSearch';
import { ContentCard } from '../components/ContentCard';
import { useContents } from '../hooks/useContents';
import { GalleryToolbar } from '../components/GalleryToolbar';
import { ErrorMessage, Loading } from '../components/Feedback';
import { IconLink } from '../components/IconAction';
import { IconButton } from '../components/IconAction';
import { CheckSquare2, Pencil, Plus, Trash2, X } from 'lucide-react';
import { useTagSchema } from '../hooks/useTagSchema';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { DeleteContentsDialog } from '../components/DeleteContentsDialog';

export function ContentsPage() {
  const { query, density, setDensity, sentinel, items, refresh, retry } = useContents();
  const schema = useTagSchema().data;
  const navigate = useNavigate();
  const [selecting, setSelecting] = useState(false);
  const [selectedIds, setSelectedIds] = useState<string[]>([]);
  const [confirmDelete, setConfirmDelete] = useState(false);
  const selected = items.filter((item) => selectedIds.includes(item.id));
  const toggle = (id: string) => setSelectedIds((ids) => ids.includes(id) ? ids.filter((item) => item !== id) : [...ids, id]);
  const navigationItems = items.map(({ id, contentUrl }) => ({ id, contentUrl }));
  return <section className="space-y-4">
    <ContentSearch />
    <GalleryToolbar density={density} setDensity={setDensity} refresh={refresh} refreshing={query.isFetching} />
    <div className="flex flex-wrap items-center justify-end gap-2">
      {selecting && <><span role="status">{selected.length} 件選択中</span><IconButton icon={Pencil} label="選択したコンテンツを編集" disabled={!selected.length} onClick={() => navigate('/contents/edit', { state: { items: selected } })} /><IconButton className="btn-ghost text-error hover:bg-error hover:text-error-content" icon={Trash2} label="選択したコンテンツを削除" disabled={!selected.length} onClick={() => setConfirmDelete(true)} /></>}
      <IconButton icon={selecting ? X : CheckSquare2} label={selecting ? '選択を終了' : 'コンテンツを選択'} aria-pressed={selecting} onClick={() => { setSelecting(!selecting); setSelectedIds([]); }} />
      <IconLink className="btn-primary" icon={Plus} label="コンテンツを登録" to="/contents/new" />
    </div>
    {query.isPending && <Loading />}
    {query.isError && <ErrorMessage error={query.error} onRetry={retry} />}
    {query.isSuccess && items.length === 0 && <div className="space-y-4 py-12 text-center"><h2>コンテンツが見つかりません</h2><p>検索条件を変更するか、画像を追加してください。</p></div>}
    {items.length > 0 &&
      <div className={`grid gap-4 ${density === 'compact' ? 'grid-cols-3 md:grid-cols-5' : 'grid-cols-2 md:grid-cols-3'}`}>
        {items.map((item, index) => <ContentCard key={item.id} item={item} index={index} navigationItems={navigationItems} schema={schema} selecting={selecting} selected={selectedIds.includes(item.id)} onSelect={() => toggle(item.id)} onLongPress={() => { setSelecting(true); setSelectedIds((ids) => ids.includes(item.id) ? ids : [...ids, item.id]); }} />)}
      </div>
    }
    <div ref={sentinel} className="h-px" />
    {query.isFetchingNextPage && <Loading label="続きを読み込み中…" />}
    {query.hasNextPage && !query.isFetchNextPageError && <div className="py-4 text-center"><button className="btn btn-ghost" disabled={query.isFetching} onClick={() => { void query.fetchNextPage(); }}>続きを読み込む</button></div>}
    {query.isSuccess && !query.hasNextPage && items.length > 0 && <p className="py-4 text-center">すべての画像を表示しました</p>}
    {confirmDelete && <DeleteContentsDialog items={selected} onClose={() => setConfirmDelete(false)} onDeleted={(ids) => setSelectedIds((current) => current.filter((id) => !ids.includes(id)))} />}
  </section>;
}
