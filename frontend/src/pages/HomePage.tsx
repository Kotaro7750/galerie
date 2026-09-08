import { Link } from 'react-router-dom';

export function HomePage() {
  return <section className="hero bg-base-200 py-12">
    <div className="hero-content flex-col text-center">
      <h1 className="text-3xl font-bold">Galerie</h1>
      <Link className="btn btn-primary" to="/contents">ギャラリーを開く</Link>
    </div>
  </section>;
}
