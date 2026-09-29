import { Link } from 'react-router-dom';
import type { Content, TagSchema } from '../api/generated';
import { ContentImage } from './ContentImage';
import { ContentTags } from './ContentTags';
import { useSearchStore } from '../store';
import { TriangleAlert } from 'lucide-react';
import { diagnosticReasons } from '../tagPresentation';
import { useState } from 'react';
import type { ContentNavigationItem } from '../contentNavigation';
import { tagSearchTerm } from '../tagSearchTerm';

export function ContentCard({ item, index, navigationItems, schema }: { item: Content; index: number; navigationItems: ContentNavigationItem[]; schema?: TagSchema }) {
  const { draft, applyTerms } = useSearchStore();
  const [diagnosticHovered, setDiagnosticHovered] = useState(false);
  const [diagnosticFocused, setDiagnosticFocused] = useState(false);
  const diagnosticSummary = item.diagnostics.map(({ key, kind }) => `${key}: ${diagnosticReasons[kind]}`);
  return <div className="group hover-3d min-w-0">
    <div className="card image-full relative aspect-square min-w-0 overflow-clip bg-base-200">
      <div className="aspect-square overflow-hidden"><ContentImage key={item.thumbnailUrl} src={item.thumbnailUrl} alt={`画像 ${index + 1} のサムネイル`} thumbnail /></div>
      <Link className="absolute inset-0" to={`/contents/${item.id}`} state={{ navigationItems }} aria-label={`画像 ${index + 1} を開く`} />
      {(item.tags.length > 0 || item.diagnostics.length > 0) && <div className="card-body absolute inset-x-0 top-0 z-10 flex h-2/3 min-h-0 flex-col justify-start gap-1 overflow-visible p-2 pointer-events-none [@media(hover:hover)]:invisible group-hover:visible group-focus-within:visible" role="region" aria-label={`画像 ${index + 1} のタグ`}>
        <div className="min-h-0 overflow-y-auto overscroll-contain pointer-events-auto" aria-label="タグをスクロール">
          <ContentTags tags={item.tags} diagnostics={item.diagnostics} schema={schema} showDiagnostics={false} leadingItem={item.diagnostics.length > 0 &&
            <span className="badge badge-error pointer-events-auto size-7 p-0" aria-label={`診断情報 ${item.diagnostics.length} 件: ${diagnosticSummary.join('、')}`} tabIndex={0}
              onMouseEnter={() => setDiagnosticHovered(true)} onMouseLeave={() => setDiagnosticHovered(false)}
              onFocus={() => setDiagnosticFocused(true)} onBlur={() => setDiagnosticFocused(false)}>
              <TriangleAlert aria-hidden="true" className="size-4" strokeWidth={1.75} />
            </span>} onTagClick={(tag) => applyTerms([...draft, tagSearchTerm(tag)])} />
        </div>
        {item.diagnostics.length > 0 && <div role="tooltip" className={`pointer-events-none absolute inset-x-2 top-11 z-20 max-h-[calc(150%-3.25rem)] overflow-hidden rounded-box bg-base-100 p-2 text-xs text-base-content shadow-lg ${diagnosticHovered || diagnosticFocused ? 'block' : 'hidden'}`}>
          <ul className="space-y-1" aria-label="診断の一覧">{diagnosticSummary.map((summary, diagnosticIndex) => <li key={diagnosticIndex}>{summary}</li>)}</ul>
        </div>}
      </div>}
    </div>
  </div>;
}
