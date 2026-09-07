import { useEffect, useRef } from 'react';
import { useInfiniteQuery, useQueryClient } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import { apiRequest, contentsApi } from '../api/client';
import { useGalleryStore } from '../store';
import { ContentImage } from '../components/ContentImage';
import { ContentTags } from '../components/ContentTags';
import { ErrorMessage, Loading } from '../components/Feedback';

export function ContentsPage() {
  const client = useQueryClient();
  const density = useGalleryStore((state) => state.density);
  const setDensity = useGalleryStore((state) => state.setDensity);
  const sentinel = useRef<HTMLDivElement>(null);
  const query = useInfiniteQuery({
    queryKey: ['contents'],
    initialPageParam: undefined as string | undefined,
    queryFn: ({ pageParam, signal }) => apiRequest(contentsApi.listContents({ cursor: pageParam, limit: 30 }, { signal })),
    getNextPageParam: (page) => page.nextCursor,
  });
  const { hasNextPage, isFetching, isError, fetchNextPage } = query;

  useEffect(() => {
    const target = sentinel.current;
    if (!target || !hasNextPage || isFetching || isError) return;
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) void fetchNextPage();
    }, { rootMargin: '400px' });
    observer.observe(target);
    return () => observer.disconnect();
  }, [hasNextPage, isFetching, isError, fetchNextPage]);

  const items = [...new Map(query.data?.pages.flatMap((page) => page.items).map((item) => [item.id, item])).values()];
  return <section className="space-y-4">
    <div className="flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
      <div><h1 className="text-2xl font-bold">ギャラリー</h1><p>一枚ずつ、好きなものに出会う。</p></div>
      <div className="flex flex-wrap items-center gap-2">
        <div className="join" role="group" aria-label="サムネイルの大きさ">
          <button className={`btn btn-sm join-item ${density === 'comfortable' ? 'btn-active' : ''}`} aria-pressed={density === 'comfortable'} onClick={() => setDensity('comfortable')}>大きく</button>
          <button className={`btn btn-sm join-item ${density === 'compact' ? 'btn-active' : ''}`} aria-pressed={density === 'compact'} onClick={() => setDensity('compact')}>小さく</button>
        </div>
        <button className="btn btn-ghost btn-sm" disabled={query.isFetching} onClick={() => { void client.resetQueries({ queryKey: ['contents'], exact: true }); }}>一覧を更新</button>
      </div>
    </div>
    {query.isPending && <Loading />}
    {query.isError && <ErrorMessage error={query.error} onRetry={() => { void (query.isFetchNextPageError ? query.fetchNextPage() : query.refetch()); }} />}
    {query.isSuccess && items.length === 0 && <div className="space-y-4 py-12 text-center"><h2>まだコンテンツがありません</h2><p>画像が追加されると、ここに表示されます。</p></div>}
    {items.length > 0 && <>
      <p className="text-sm">{items.length} 枚を表示</p>
      <div className={`grid gap-4 ${density === 'compact' ? 'grid-cols-3 md:grid-cols-5' : 'grid-cols-2 md:grid-cols-3'}`}>
        {items.map((item, index) => <div className="content-card card min-w-0 bg-base-200" key={item.id}>
          <Link to={`/contents/${item.id}`} aria-label={`画像 ${index + 1} を開く`}>
          <div className="aspect-square overflow-hidden"><ContentImage key={item.thumbnailUrl} src={item.thumbnailUrl} alt={`画像 ${index + 1} のサムネイル`} thumbnail /></div>
          <div className="card-body flex-row justify-between p-4"><span>{String(index + 1).padStart(2, '0')}</span><span aria-hidden="true">↗</span></div>
          </Link>
          <div className="content-card-tags rounded-box border border-base-300 bg-base-100 p-4 shadow-lg" tabIndex={0} role="region" aria-label={`画像 ${index + 1} のタグ`}>
            <ContentTags tags={item.tags} invalidTags={item.invalidTags} />
          </div>
        </div>)}
      </div>
    </>}
    <div ref={sentinel} className="h-px" />
    {query.isFetchingNextPage && <Loading label="続きを読み込み中…" />}
    {query.hasNextPage && !query.isFetchNextPageError && <div className="py-4 text-center"><button className="btn btn-ghost" disabled={query.isFetching} onClick={() => { void query.fetchNextPage(); }}>続きを読み込む</button></div>}
    {query.isSuccess && !query.hasNextPage && items.length > 0 && <p className="py-4 text-center">すべての画像を表示しました</p>}
  </section>;
}
