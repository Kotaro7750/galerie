import { ContentSearch } from '../components/ContentSearch';
import { ContentCard } from '../components/ContentCard';
import { useContents } from '../hooks/useContents';
import { GalleryToolbar } from '../components/GalleryToolbar';
import { ErrorMessage, Loading } from '../components/Feedback';

export function ContentsPage() {
  const { query, density, setDensity, sentinel, items, refresh, retry } = useContents();
  return <section className="space-y-4">
    <ContentSearch />
    <GalleryToolbar density={density} setDensity={setDensity} refresh={refresh} refreshing={query.isFetching} />
    {query.isPending && <Loading />}
    {query.isError && <ErrorMessage error={query.error} onRetry={retry} />}
    {query.isSuccess && items.length === 0 && <div className="space-y-4 py-12 text-center"><h2>コンテンツが見つかりません</h2><p>検索条件を変更するか、画像を追加してください。</p></div>}
    {items.length > 0 &&
      <div className={`grid gap-4 ${density === 'compact' ? 'grid-cols-3 md:grid-cols-5' : 'grid-cols-2 md:grid-cols-3'}`}>
        {items.map((item, index) => <ContentCard key={item.id} item={item} index={index} />)}
      </div>
    }
    <div ref={sentinel} className="h-px" />
    {query.isFetchingNextPage && <Loading label="続きを読み込み中…" />}
    {query.hasNextPage && !query.isFetchNextPageError && <div className="py-4 text-center"><button className="btn btn-ghost" disabled={query.isFetching} onClick={() => { void query.fetchNextPage(); }}>続きを読み込む</button></div>}
    {query.isSuccess && !query.hasNextPage && items.length > 0 && <p className="py-4 text-center">すべての画像を表示しました</p>}
  </section>;
}
