import { useQuery } from '@tanstack/react-query';
import { Link, useParams } from 'react-router-dom';
import { apiRequest, contentsApi } from '../api/client';
import { ContentImage } from '../components/ContentImage';
import { ErrorMessage, Loading } from '../components/Feedback';

export function ContentPage() {
  const { contentId = '' } = useParams();
  const query = useQuery({
    queryKey: ['content', contentId],
    queryFn: ({ signal }) => apiRequest(contentsApi.getContent({ contentId }, { signal })),
  });
  return <section className="space-y-4">
    <div className="flex flex-wrap items-center justify-between gap-2"><Link className="btn btn-ghost" to="/contents">← ギャラリーに戻る</Link><h1>コンテンツ</h1></div>
    {query.isPending && <Loading />}
    {query.isError && <ErrorMessage error={query.error} onRetry={() => { void query.refetch(); }} />}
    {query.data && <>
      <div className="flex justify-center rounded-box bg-base-200 p-4"><ContentImage key={query.data.contentUrl} src={query.data.contentUrl} alt={`コンテンツ ${query.data.id}`} /></div>
      <div className="flex flex-wrap justify-between gap-2 text-sm"><span>{query.data.mediaType}</span><a className="link" href={query.data.contentUrl} target="_blank" rel="noreferrer">元の画像を開く ↗</a></div>
    </>}
  </section>;
}
