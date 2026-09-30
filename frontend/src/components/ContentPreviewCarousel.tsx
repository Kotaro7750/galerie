import { useCallback, useEffect, useRef, useState } from 'react';
import { ChevronLeft, ChevronRight, HardDrive, X } from 'lucide-react';
import type { Swiper as SwiperInstance } from 'swiper';
import { Swiper, SwiperSlide } from 'swiper/react';
import 'swiper/css';
import { IconButton } from './IconAction';
import { ImageResolution, type ImageResolutionValue } from './ImageResolution';

export type ContentPreviewItem = { id: string | number; image: File | string; alt: string; removeLabel?: string };

function PreviewImage({ item, onLayoutChange, onResolutionChange }: {
  item: ContentPreviewItem; onLayoutChange: () => void; onResolutionChange: (value?: ImageResolutionValue) => void;
}) {
  const [objectUrl, setObjectUrl] = useState<string>();
  useEffect(() => {
    if (typeof item.image === 'string') return;
    const url = URL.createObjectURL(item.image);
    setObjectUrl(url);
    return () => URL.revokeObjectURL(url);
  }, [item.image]);
  const src = typeof item.image === 'string' ? item.image : objectUrl;
  useEffect(() => {
    const frame = requestAnimationFrame(onLayoutChange);
    return () => cancelAnimationFrame(frame);
  }, [src, onLayoutChange]);
  return src && <img src={src} alt={item.alt} className={`block max-w-full rounded-lg object-contain ${typeof item.image === 'string' ? 'max-h-[40vh]' : 'max-h-[60vh]'}`}
    onLoad={(event) => { onResolutionChange({ width: event.currentTarget.naturalWidth, height: event.currentTarget.naturalHeight }); onLayoutChange(); }}
    onError={() => { onResolutionChange(undefined); onLayoutChange(); }} />;
}

export function ContentPreviewCarousel({ items, onRemove, busy = false, label }: {
  items: ContentPreviewItem[]; onRemove?: (id: string | number) => void; busy?: boolean; label: string;
}) {
  const swiper = useRef<SwiperInstance | null>(null);
  const [active, setActive] = useState(0);
  const [resolutions, setResolutions] = useState<Record<string, ImageResolutionValue>>({});
  const updateCarouselHeight = useCallback(() => swiper.current?.updateAutoHeight(0), []);
  if (!items.length) return null;
  const current = items[Math.min(active, items.length - 1)];
  return <figure className="flex min-w-0 w-full flex-col gap-2" aria-label={label}>
    <figcaption className="flex w-full items-center justify-between gap-2 px-2 text-sm sm:px-12">
      <span aria-label="表示中の画像">{Math.min(active, items.length - 1) + 1} / {items.length}</span>
      {typeof current.image !== 'string' && <span className="flex items-center justify-end gap-2 sm:gap-4">
        <span className="tooltip inline-flex items-center gap-1" data-tip="ファイルサイズ"><HardDrive role="img" aria-label="ファイルサイズ" className="size-5 shrink-0" strokeWidth={1.75} /><span>{(current.image.size / 1024 / 1024).toFixed(2)} MiB</span></span>
        <ImageResolution resolution={resolutions[String(current.id)]} />
      </span>}
    </figcaption>
    <Swiper key={items.map(({ id }) => id).join('-')} autoHeight loop={items.length > 1} initialSlide={Math.min(active, items.length - 1)} onRealIndexChange={(instance) => setActive(instance.realIndex)} onSwiper={(instance) => { swiper.current = instance; }} onBeforeDestroy={(instance) => { if (swiper.current === instance) swiper.current = null; }} className="w-full">
      {items.map((item, index) => <SwiperSlide key={item.id} className="flex! items-center justify-center px-2 sm:px-12">
        <div className="relative w-fit max-w-full">
          <PreviewImage item={item} onLayoutChange={updateCarouselHeight} onResolutionChange={(value) => setResolutions((currentResolutions) => {
            const next = { ...currentResolutions };
            if (value) next[String(item.id)] = value;
            else delete next[String(item.id)];
            return next;
          })} />
          {onRemove && <IconButton icon={X} label={item.removeLabel ?? `${index + 1} 件目を取り除く`} className="btn-circle btn-error btn-sm absolute top-2 right-2 z-10 shadow-md" disabled={busy} onClick={() => onRemove(item.id)} />}
          {index === Math.min(active, items.length - 1) && items.length > 1 && <>
            <IconButton icon={ChevronLeft} label="前の画像" className="btn-ghost absolute left-2 top-1/2 z-10 min-h-11 min-w-11 -translate-y-1/2 bg-base-100/80" onClick={() => swiper.current?.slidePrev()} />
            <IconButton icon={ChevronRight} label="次の画像" className="btn-ghost absolute right-2 top-1/2 z-10 min-h-11 min-w-11 -translate-y-1/2 bg-base-100/80" onClick={() => swiper.current?.slideNext()} />
          </>}
        </div>
      </SwiperSlide>)}
    </Swiper>
  </figure>;
}
