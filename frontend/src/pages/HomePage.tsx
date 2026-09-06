import { Link } from 'react-router-dom';

export function HomePage() {
  return <>
    <section className="hero bg-base-200 py-12">
      <div className="hero-content flex-col text-center">
        <h1 className="text-3xl font-bold">好きなものを、<br />ゆっくり眺める。</h1>
        <p className="py-4">大切に集めた画像を、ひとつの場所に。<br />Galerie は、あなたのための小さなギャラリーです。</p>
        <Link className="btn btn-primary" to="/contents">ギャラリーを開く <span aria-hidden="true">↗</span></Link>
      </div>
    </section>
    <section className="space-y-4 py-8" aria-labelledby="guide-title">
      <h2 id="guide-title" className="text-xl font-bold">Galerie の楽しみ方</h2>
      <div className="grid gap-4 md:grid-cols-3">
        <article className="card bg-base-200"><div className="card-body"><h3 className="card-title">一覧を眺める</h3><p>サムネイルをスクロールすると、続きの画像が自然に現れます。</p></div></article>
        <article className="card bg-base-200"><div className="card-body"><h3 className="card-title">気になる一枚を開く</h3><p>画像を選ぶと、大きな表示でじっくり楽しめます。</p></div></article>
        <article className="card bg-base-200"><div className="card-body"><h3 className="card-title">また、見つける</h3><p>一覧に戻って、お気に入りのコレクションを巡りましょう。</p></div></article>
      </div>
    </section>
  </>;
}
