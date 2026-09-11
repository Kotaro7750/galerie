import { ContentSearch } from '../components/ContentSearch';
import { Images } from 'lucide-react';
import { IconLink } from '../components/IconAction';

export function HomePage() {
  return <section className="hero bg-base-200 py-12">
    <div className="hero-content w-full max-w-2xl flex-col text-center">
      <h1 className="text-3xl font-bold">Galerie</h1>
      <ContentSearch />
      <div className="divider">OR</div>
      <IconLink className="btn-circle btn-primary" icon={Images} label="ギャラリーを開く" to="/contents" />
    </div>
  </section>;
}
