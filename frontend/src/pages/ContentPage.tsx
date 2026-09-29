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
import { SlideshowControls } from '../components/SlideshowControls';
import { ErrorMessage, Loading } from '../components/Feedback';
import { useSearchStore } from '../store';
import { useTagSchema } from '../hooks/useTagSchema';
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
  const navigationKey = navigationItems.map((item) => item.id).join('\n');
  const [excludedIds, setExcludedIds] = useState<string[]>([]);
  const [intervalSeconds, setIntervalSeconds] = useState(5);
  const [playing, setPlaying] = useState(false);
  const [shuffled, setShuffled] = useState(false);
  const shuffleQueue = useRef<string[]>([]);
  useEffect(() => { setExcludedIds([]); setPlaying(false); setShuffled(false); }, [navigationKey]);
  useEffect(() => { shuffleQueue.current = []; }, [navigationKey, excludedIds, shuffled]);
  const targetCount = navigationItems.length - excludedIds.filter((id) => navigationItems.some((item) => item.id === id)).length;
  const swiperRef = useRef<SwiperInstance | null>(null);
  useEffect(() => {
    if (canSwipe && swiperRef.current && swiperRef.current.realIndex !== currentIndex) {
      swiperRef.current.slideToLoop(currentIndex, 0, false);
    }
  }, [canSwipe, currentIndex]);
  useEffect(() => {
    if (!playing || currentIndex < 0 || targetCount < 2) return;
    const timer = window.setTimeout(() => {
      let nextIndex: number;
      if (shuffled) {
        if (shuffleQueue.current.length === 0) {
          const ids = navigationItems.map((item) => item.id).filter((id) => id !== contentId && !excludedIds.includes(id));
          for (let index = ids.length - 1; index > 0; index--) {
            const swapIndex = Math.floor(Math.random() * (index + 1));
            [ids[index], ids[swapIndex]] = [ids[swapIndex], ids[index]];
          }
          shuffleQueue.current = ids;
        }
        const nextId = shuffleQueue.current.shift();
        nextIndex = navigationItems.findIndex((item) => item.id === nextId);
      } else {
        const next = navigationItems.findIndex((item, index) => index > currentIndex && !excludedIds.includes(item.id));
        nextIndex = next >= 0 ? next : navigationItems.findIndex((item) => !excludedIds.includes(item.id));
      }
      if (nextIndex >= 0) swiperRef.current?.slideToLoop(nextIndex, 0);
    }, intervalSeconds * 1000);
    return () => window.clearTimeout(timer);
  }, [playing, shuffled, contentId, currentIndex, navigationKey, excludedIds, intervalSeconds, targetCount]);
  const slideshowControls = {
    playing, shuffled, intervalSeconds, targetCount, totalCount: navigationItems.length, excludedCount: excludedIds.length, currentExcluded: excludedIds.includes(contentId),
    onToggle: () => setPlaying((value) => !value),
    onShuffleToggle: () => setShuffled((value) => !value),
    onIntervalChange: setIntervalSeconds,
    onToggleExcluded: () => {
      if (currentIndex < 0) return;
      if (excludedIds.includes(contentId)) {
        setExcludedIds((ids) => ids.filter((id) => id !== contentId));
        return;
      }
      setExcludedIds((ids) => [...ids, contentId]);
      if (targetCount <= 2) setPlaying(false);
    },
    onReset: () => setExcludedIds([]),
  };
  const { draft, applyTerms } = useSearchStore();
  const { imageContainer, fullscreen, fullscreenError, toggleFullscreen, supported } = useFullscreen();
  const fullscreenLabel = fullscreen ? '全画面表示を解除' : '全画面表示';
  const FullscreenIcon = fullscreen ? Minimize : Fullscreen;
  const query = useContent(contentId);
  const schema = useTagSchema().data;
  const [imageResolutions, setImageResolutions] = useState<Record<string, ImageResolutionValue>>({});
  const updateResolution = (src: string, value?: ImageResolutionValue) => setImageResolutions((current) => {
    if (value) return { ...current, [src]: value };
    const next = { ...current };
    delete next[src];
    return next;
  });
  return <section className="space-y-4">
    {canSwipe && !fullscreen && <SlideshowControls {...slideshowControls} className="ml-auto w-fit max-w-full" />}
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
      <section aria-label="タグ情報"><ContentTags tags={query.data.tags} diagnostics={query.data.diagnostics} schema={schema} onTagClick={(tag) => {
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
      {fullscreen && canSwipe && <SlideshowControls {...slideshowControls} className="absolute bottom-4 left-1/2 z-20 max-w-[calc(100%-2rem)] -translate-x-1/2" />}
      {fullscreenError && <p className="alert alert-error absolute bottom-4 left-4 right-4" role="alert">{fullscreenError}</p>}
    </div>}
  </section>;
}
