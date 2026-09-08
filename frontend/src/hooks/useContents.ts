import { useEffect, useRef } from 'react';
import { useInfiniteQuery, useQueryClient } from '@tanstack/react-query';
import { apiRequest, contentsApi } from '../api/client';
import { useGalleryStore } from '../store';

export function useContents() {
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
    // 失敗時は自動取得を止め、ユーザー操作で同じカーソルを再試行する。
    if (!target || !hasNextPage || isFetching || isError) return;
    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) void fetchNextPage();
    }, { rootMargin: '400px' });
    observer.observe(target);
    return () => observer.disconnect();
  }, [hasNextPage, isFetching, isError, fetchNextPage]);

  // ページ間で同じIDが返っても、一覧には一度だけ表示する。
  const items = [...new Map(query.data?.pages.flatMap((page) => page.items).map((item) => [item.id, item])).values()];
  return { query, density, setDensity, sentinel, items,
    refresh: () => { void client.resetQueries({ queryKey: ['contents'], exact: true }); },
    retry: () => { void (query.isFetchNextPageError ? query.fetchNextPage() : query.refetch()); },
  };
}
