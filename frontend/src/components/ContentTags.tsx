import { InvalidTagBadge, TagBadge } from './TagBadge';
import type { Content } from '../api/generated';

export function ContentTags({ tags, invalidTags }: Pick<Content, 'tags' | 'invalidTags'>) {
  return (
    <div className="text-sm">
      {tags.length === 0 && invalidTags.length === 0 ? (
        <p>タグはありません</p>
      ) : (
        <ul className="flex flex-wrap gap-2" aria-label="タグ">
          {tags.map((tag, index) => (
            <li key={`valid-${tag.key}-${index}`} className="max-w-full">
              <TagBadge tag={tag} />
            </li>
          ))}
          {invalidTags.map((tag, index) => (
            <li key={`invalid-${tag.key}-${index}`} className="max-w-full">
              <InvalidTagBadge tag={tag} />
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
