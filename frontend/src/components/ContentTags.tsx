import { DiagnosticBadge, TagBadge } from './TagBadge';
import type { Content, TagSchema } from '../api/generated';
import type { ReactNode } from 'react';

export function ContentTags({ tags, diagnostics, schema, onTagClick, showDiagnostics = true, leadingItem }: Pick<Content, 'tags' | 'diagnostics'> & { schema?: TagSchema; onTagClick?: (tag: Content['tags'][number]) => void; showDiagnostics?: boolean; leadingItem?: ReactNode }) {
  return (
    <div className="text-sm">
      {tags.length === 0 && !leadingItem && (!showDiagnostics || diagnostics.length === 0) ? (
        <p>タグはありません</p>
      ) : (
        <ul className="flex flex-wrap gap-2" aria-label="タグ">
          {leadingItem && <li className="max-w-full">{leadingItem}</li>}
          {tags.map((tag, index) => (
            <li key={`valid-${tag.key}-${index}`} className="max-w-full">
              <TagBadge tag={tag} schema={schema} onClick={onTagClick ? () => onTagClick(tag) : undefined} />
            </li>
          ))}
          {showDiagnostics && diagnostics.map((diagnostic, index) => (
            <li key={`diagnostic-${diagnostic.key}-${index}`} className="max-w-full">
              <DiagnosticBadge diagnostic={diagnostic} absent={diagnostic.kind === 'missingRequiredTag' && !tags.some((tag) => tag.key === diagnostic.key) && !diagnostics.some((other) => other.key === diagnostic.key && other.kind !== 'missingRequiredTag')} />
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
