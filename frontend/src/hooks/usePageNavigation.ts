import { useEffect } from 'react';
import { useLocation } from 'react-router-dom';

export function usePageNavigation() {
  const { pathname } = useLocation();
  useEffect(() => {
    window.scrollTo(0, 0);
    document.title = pathname === '/' ? 'Galerie' : pathname === '/contents' ? 'ギャラリー | Galerie' : 'コンテンツ | Galerie';
    document.getElementById('main')?.focus({ preventScroll: true });
  }, [pathname]);
}
