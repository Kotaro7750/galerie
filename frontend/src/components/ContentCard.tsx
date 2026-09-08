import { Link } from 'react-router-dom';
import type { Content } from '../api/generated';
import { ContentImage } from './ContentImage';
import { ContentTags } from './ContentTags';

export function ContentCard({ item, index }: { item: Content; index: number }) {
  return <Link className="group hover-3d min-w-0" to={`/contents/${item.id}`} aria-label={`画像 ${index + 1} を開く`}>
    <div className="card image-full aspect-square min-w-0 overflow-clip bg-base-200">
      <div className="aspect-square overflow-hidden"><ContentImage key={item.thumbnailUrl} src={item.thumbnailUrl} alt={`画像 ${index + 1} のサムネイル`} thumbnail /></div>
      {(item.tags.length > 0 || item.invalidTags.length > 0) && <div className="card-body min-h-0 overflow-hidden p-3 [@media(hover:hover)]:invisible group-hover:visible group-focus-within:visible" role="region" aria-label={`画像 ${index + 1} のタグ`}>
        <ContentTags tags={item.tags} invalidTags={item.invalidTags} />
      </div>}
    </div>
  </Link>;
}
