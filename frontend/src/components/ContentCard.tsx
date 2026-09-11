import { Link } from 'react-router-dom';
import type { Content } from '../api/generated';
import { ContentImage } from './ContentImage';
import { ContentTags } from './ContentTags';
import { useSearchStore } from '../store';

export function ContentCard({ item, index }: { item: Content; index: number }) {
  const { draft, applyTerms } = useSearchStore();
  return <div className="group hover-3d min-w-0">
    <div className="card image-full relative aspect-square min-w-0 overflow-clip bg-base-200">
      <div className="aspect-square overflow-hidden"><ContentImage key={item.thumbnailUrl} src={item.thumbnailUrl} alt={`画像 ${index + 1} のサムネイル`} thumbnail /></div>
      <Link className="absolute inset-0" to={`/contents/${item.id}`} aria-label={`画像 ${index + 1} を開く`} />
      {(item.tags.length > 0 || item.invalidTags.length > 0) && <div className="card-body z-10 min-h-0 overflow-hidden p-3 pointer-events-none [@media(hover:hover)]:invisible group-hover:visible group-focus-within:visible" role="region" aria-label={`画像 ${index + 1} のタグ`}>
        <ContentTags tags={item.tags} invalidTags={item.invalidTags} onTagClick={(tag) => applyTerms([...draft, { kind: 'tagExists', key: tag.key }])} />
      </div>}
    </div>
  </div>;
}
