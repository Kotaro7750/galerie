import { DiagnosticBadge, TagBadge } from './TagBadge';
import type { Content } from '../api/generated';

export function ContentTags({ tags, diagnostics, onTagClick }: Pick<Content, 'tags' | 'diagnostics'> & { onTagClick?: (tag: Content['tags'][number]) => void }) {
  return (
    <div className="text-sm">
      {tags.length === 0 && diagnostics.length === 0 ? (
        <p>タグはありません</p>
      ) : (
        <ul className="flex flex-wrap gap-2" aria-label="タグ">
          {tags.map((tag, index) => (
            <li key={`valid-${tag.key}-${index}`} className="max-w-full">
              <TagBadge tag={tag} onClick={onTagClick ? () => onTagClick(tag) : undefined} />
            </li>
          ))}
          {diagnostics.map((diagnostic, index) => (
            <li key={`diagnostic-${diagnostic.key}-${index}`} className="max-w-full">
              <DiagnosticBadge diagnostic={diagnostic} />
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
