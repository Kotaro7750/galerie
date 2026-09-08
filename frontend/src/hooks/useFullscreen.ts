import { useEffect, useRef, useState } from 'react';

export function useFullscreen() {
  const imageContainer = useRef<HTMLDivElement>(null);
  const [fullscreen, setFullscreen] = useState(false);
  const [fullscreenError, setFullscreenError] = useState('');
  useEffect(() => {
    // Escなどブラウザ側の操作も反映するため、イベントを状態の基準にする。
    const update = () => setFullscreen(document.fullscreenElement === imageContainer.current && document.fullscreenElement !== null);
    document.addEventListener('fullscreenchange', update);
    return () => document.removeEventListener('fullscreenchange', update);
  }, []);
  const toggleFullscreen = async () => {
    setFullscreenError('');
    try {
      if (document.fullscreenElement === imageContainer.current) await document.exitFullscreen();
      else await imageContainer.current?.requestFullscreen();
    } catch {
      setFullscreenError('全画面表示を切り替えられませんでした。');
    }
  };
  return { imageContainer, fullscreen, fullscreenError, toggleFullscreen, supported: !!document.fullscreenEnabled };
}
