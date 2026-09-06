import { useEffect } from 'react';
import { Link, NavLink, Route, Routes, useLocation } from 'react-router-dom';
import { HomePage } from './pages/HomePage';
import { ContentsPage } from './pages/ContentsPage';
import { ContentPage } from './pages/ContentPage';

export function App() {
  const { pathname } = useLocation();
  useEffect(() => {
    window.scrollTo(0, 0);
    document.title = pathname === '/' ? 'Galerie' : pathname === '/contents' ? 'ギャラリー | Galerie' : 'コンテンツ | Galerie';
    document.getElementById('main')?.focus({ preventScroll: true });
  }, [pathname]);
  return <div className="flex min-h-screen flex-col">
    <a className="btn sr-only focus:not-sr-only" href="#main" onClick={(event) => { event.preventDefault(); document.getElementById('main')?.focus(); }}>本文へ移動</a>
    <header className="navbar flex-wrap bg-base-200 px-4">
      <Link to="/" className="btn btn-ghost text-xl" aria-label="Galerie トップページ">Galerie</Link>
      <nav className="ml-auto" aria-label="メインナビゲーション"><ul className="menu menu-horizontal"><li><NavLink to="/" end className={({ isActive }) => isActive ? 'menu-active' : ''}>ホーム</NavLink></li><li><NavLink to="/contents" className={({ isActive }) => isActive ? 'menu-active' : ''}>ギャラリー</NavLink></li></ul></nav>
    </header>
    <main id="main" tabIndex={-1} className="container mx-auto flex-1 p-4 focus:outline-none">
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/contents" element={<ContentsPage />} />
        <Route path="/contents/:contentId" element={<ContentPage />} />
        <Route path="*" element={<section className="space-y-4 py-12 text-center"><h1>ページが見つかりません</h1><Link className="btn btn-primary" to="/">ホームへ戻る</Link></section>} />
      </Routes>
    </main>
    <footer className="footer bg-base-200 p-4"><span>galerie.</span><span>あなたの好きなものを、あなたのペースで。</span></footer>
  </div>;
}
