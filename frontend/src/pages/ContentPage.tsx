import { useEffect, useMemo, useRef, useState } from 'react';
import { IconAnchor, IconButton, IconLink } from '../components/IconAction';
import { ArrowLeft, ChevronLeft, ChevronRight, ExternalLink, Fullscreen, Minimize } from 'lucide-react';
import type { Swiper as SwiperInstance } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/react';
import 'swiper/css';
import { useFullscreen } from '../hooks/useFullscreen';
import { useContent } from '../hooks/useContent';
import { useLocation, useNavigate, useParams } from 'react-router-dom';
import { MediaTypeIcon } from '../components/MediaTypeIcon';
import { ContentImage } from '../components/ContentImage';
import { ImageResolution, type ImageResolutionValue } from '../components/ImageResolution';
import { ContentTags } from '../components/ContentTags';
import { ErrorMessage, Loading } from '../components/Feedback';
import { useSearchStore } from '../store';
import type { ContentNavigationItem } from '../contentNavigation';

export function ContentPage() {
  const { contentId = '' } = useParams();
  const location = useLocation();
  const navigate = useNavigate();
  const navigationItems = useMemo(() => Array.isArray(location.state?.navigationItems)
    ? (location.state.navigationItems as unknown[]).filter((item): item is ContentNavigationItem =>
      typeof item === 'object' && item !== null &&
      typeof (item as ContentNavigationItem).id === 'string' && typeof (item as ContentNavigationItem).contentUrl === 'string')
    : [], [location.state]);
  const currentIndex = navigationItems.findIndex((item) => item.id === contentId);
  const canSwipe = currentIndex >= 0 && navigationItems.length > 1;
  const swiperRef = useRef<SwiperInstance | null>(null);
  useEffect(() => {
    if (canSwipe && swiperRef.current && swiperRef.current.realIndex !== currentIndex) {
      swiperRef.current.slideToLoop(currentIndex, 0, false);
    }
  }, [canSwipe, currentIndex]);
  const { draft, applyTerms } = useSearchStore();
  const { imageContainer, fullscreen, fullscreenError, toggleFullscreen, supported } = useFullscreen();
  const fullscreenLabel = fullscreen ? '全画面表示を解除' : '全画面表示';
  const FullscreenIcon = fullscreen ? Minimize : Fullscreen;
  const query = useContent(contentId);
  const [imageResolutions, setImageResolutions] = useState<Record<string, ImageResolutionValue>>({});
  const updateResolution = (src: string, value?: ImageResolutionValue) => setImageResolutions((current) => {
    if (value) return { ...current, [src]: value };
    const next = { ...current };
    delete next[src];
    return next;
  });
  return <section className="space-y-4">
    <div className="flex items-center gap-3">
      <IconLink to="/contents" label="ギャラリーに戻る" icon={ArrowLeft} />
      {query.data && <div className="ml-auto flex flex-wrap items-center justify-end gap-x-6 gap-y-2">
        <MediaTypeIcon mediaType={query.data.mediaType} />
        <ImageResolution resolution={imageResolutions[query.data.contentUrl]} />
        <div className="flex items-center gap-1">
          <IconButton disabled={!supported} label={fullscreenLabel} icon={FullscreenIcon} onClick={() => { void toggleFullscreen(); }} />
          <IconAnchor href={query.data.contentUrl} target="_blank" rel="noreferrer" label="元の画像を開く" icon={ExternalLink} />
        </div>
      </div>}
    </div>
    {query.isPending && <Loading />}
    {query.isError && <ErrorMessage error={query.error} onRetry={() => { void query.refetch(); }} />}
    {query.data && <>
      <section aria-label="タグ情報"><ContentTags tags={query.data.tags} diagnostics={query.data.diagnostics} onTagClick={(tag) => {
        applyTerms([...draft, { kind: 'tagExists', key: tag.key }]);
        navigate('/contents');
      }} /></section>
    </>}
    {(query.data || canSwipe) && <div ref={imageContainer} className="relative flex min-w-0 items-center justify-center rounded-box bg-base-200 p-4 [&:fullscreen]:rounded-none [&:fullscreen]:p-0">
      {canSwipe ? <Swiper className="h-full w-full [&_.swiper-slide]:!flex [&_.swiper-slide]:items-center [&_.swiper-slide]:justify-center"
        loop loopPreventsSliding={false} initialSlide={currentIndex} runCallbacksOnInit={false}
        onSwiper={(instance) => { swiperRef.current = instance; if (instance.realIndex !== currentIndex) instance.slideToLoop(currentIndex, 0, false); }}
        onBeforeDestroy={(instance) => { if (swiperRef.current === instance) swiperRef.current = null; }}
        onSlideChange={(instance) => {
          const item = navigationItems[instance.realIndex];
          if (item && item.id !== contentId) navigate(`/contents/${item.id}`, { state: { navigationItems } });
        }}>
        {navigationItems.map((item, index) => {
          const src = query.data?.id === item.id ? query.data.contentUrl : item.contentUrl;
          return <SwiperSlide key={item.id}>
            {({ isActive }) => <div aria-hidden={!isActive} className="flex h-full w-full items-center justify-center">
              <ContentImage key={src} src={src} alt={`コンテンツ ${item.id}`} loading={index === currentIndex ? 'eager' : 'lazy'}
                onResolutionChange={(value) => updateResolution(src, value)} />
            </div>}
          </SwiperSlide>;
        })}
      </Swiper> : query.data && <ContentImage key={query.data.contentUrl} src={query.data.contentUrl} alt={`コンテンツ ${query.data.id}`}
        onResolutionChange={(value) => updateResolution(query.data.contentUrl, value)} />}
      {canSwipe && <>
        <IconButton className="btn-ghost absolute left-4 top-1/2 z-10 min-h-11 min-w-11 -translate-y-1/2 bg-base-100/80" label="前のコンテンツ" icon={ChevronLeft} onClick={() => swiperRef.current?.slidePrev()} />
        <IconButton className="btn-ghost absolute right-4 top-1/2 z-10 min-h-11 min-w-11 -translate-y-1/2 bg-base-100/80" label="次のコンテンツ" icon={ChevronRight} onClick={() => swiperRef.current?.slideNext()} />
      </>}
      {fullscreen && <IconButton className="absolute right-4 top-4 z-20" label={fullscreenLabel} icon={FullscreenIcon} onClick={() => { void toggleFullscreen(); }} />}
      {fullscreenError && <p className="alert alert-error absolute bottom-4 left-4 right-4" role="alert">{fullscreenError}</p>}
    </div>}
  </section>;
}
