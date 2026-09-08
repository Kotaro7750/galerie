import { IconAnchor, IconButton, IconLink } from '../components/IconAction';
import { ArrowLeft, ExternalLink, Maximize, Minimize } from 'lucide-react';
import { useFullscreen } from '../hooks/useFullscreen';
import { useContent } from '../hooks/useContent';
import { useParams } from 'react-router-dom';
import { MediaTypeIcon } from '../components/MediaTypeIcon';
import { ContentImage } from '../components/ContentImage';
import { ContentTags } from '../components/ContentTags';
import { ErrorMessage, Loading } from '../components/Feedback';

export function ContentPage() {
  const { contentId = '' } = useParams();
  const { imageContainer, fullscreen, fullscreenError, toggleFullscreen, supported } = useFullscreen();
  const fullscreenLabel = fullscreen ? '全画面表示を解除' : '全画面表示';
  const FullscreenIcon = fullscreen ? Minimize : Maximize;
  const query = useContent(contentId);
  return <section className="space-y-4">
    <div className="flex items-center gap-3">
      <IconLink to="/contents" label="ギャラリーに戻る" icon={ArrowLeft} />
      {query.data && <div className="ml-auto flex items-center gap-6">
        <MediaTypeIcon mediaType={query.data.mediaType} />
        <div className="flex items-center gap-1">
          <IconButton disabled={!supported} label={fullscreenLabel} icon={FullscreenIcon} onClick={() => { void toggleFullscreen(); }} />
          <IconAnchor href={query.data.contentUrl} target="_blank" rel="noreferrer" label="元の画像を開く" icon={ExternalLink} />
        </div>
      </div>}
    </div>
    {query.isPending && <Loading />}
    {query.isError && <ErrorMessage error={query.error} onRetry={() => { void query.refetch(); }} />}
    {query.data && <>
      <section aria-label="タグ情報"><ContentTags tags={query.data.tags} invalidTags={query.data.invalidTags} /></section>
      <div ref={imageContainer} className="relative flex items-center justify-center rounded-box bg-base-200 p-4 [&:fullscreen]:rounded-none [&:fullscreen]:p-0">
        <ContentImage key={query.data.contentUrl} src={query.data.contentUrl} alt={`コンテンツ ${query.data.id}`} />
        {fullscreen && <IconButton className="absolute right-4 top-4" label={fullscreenLabel} icon={FullscreenIcon} onClick={() => { void toggleFullscreen(); }} />}
        {fullscreenError && <p className="alert alert-error absolute bottom-4 left-4 right-4" role="alert">{fullscreenError}</p>}
      </div>
    </>}
  </section>;
}
