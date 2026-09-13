import { AppHeader } from './components/AppHeader';
import { usePageNavigation } from './hooks/usePageNavigation';
import { version } from '../package.json';
import { Link, Route, Routes } from 'react-router-dom';
import { HomePage } from './pages/HomePage';
import { ContentsPage } from './pages/ContentsPage';
import { ContentPage } from './pages/ContentPage';
import { ProtectedRoute } from './auth/ProtectedRoute';

export function App() {
  usePageNavigation();
  return <div className="flex min-h-screen flex-col">
    <a className="btn sr-only focus:not-sr-only" href="#main" onClick={(event) => { event.preventDefault(); document.getElementById('main')?.focus(); }}>本文へ移動</a>
    <AppHeader />
    <main id="main" tabIndex={-1} className="container mx-auto flex-1 p-4 focus:outline-none">
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/contents" element={<ProtectedRoute><ContentsPage /></ProtectedRoute>} />
        <Route path="/contents/:contentId" element={<ProtectedRoute><ContentPage /></ProtectedRoute>} />
        <Route path="*" element={<section className="space-y-4 py-12 text-center"><h1>ページが見つかりません</h1><Link className="btn btn-primary" to="/">ホームへ戻る</Link></section>} />
      </Routes>
    </main>
    <footer className="footer flex flex-wrap justify-center gap-x-4 gap-y-1 bg-base-200 p-4"><span>v{version}</span><span>MIT License</span></footer>
  </div>;
}
