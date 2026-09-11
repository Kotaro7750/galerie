import { Tag as TagIcon, Tags, Hash } from 'lucide-react';
import type { InvalidTag, Tag } from '../api/generated';
import { invalidReasons } from '../tagPresentation';

function tagValue(tag: Tag): string {
  switch (tag.type) {
    case 'keyOnly': return tag.key;
    case 'text':
    case 'integer':
    case 'real': return `${tag.value}`;
    case 'textSet':
    case 'integerSet':
    case 'realSet': return [...tag.values].join(', ');
  }
}

function tagLabel(tag: Tag): string {
  return tag.type === 'keyOnly' ? tag.key : `${tag.key}: ${tagValue(tag)}`;
}

export function InvalidTagBadge({ tag }: { tag: InvalidTag }) {
  const reason = invalidReasons[tag.reason];

  return (
    <span className="tooltip focus:tooltip-open max-w-full" data-tip={reason} tabIndex={0} aria-label={`${tag.key}: ${reason}`}>
      <span className="badge badge-dash badge-error h-auto min-h-7 max-w-full py-1">
        <TagIcon aria-hidden="true" className="size-4 shrink-0" strokeWidth={1.75} />
        <span className="min-w-0 whitespace-pre-wrap [overflow-wrap:anywhere]">{tag.key}</span>
      </span>
    </span>
  );
}

export function TagBadge({ tag, onClick }: { tag: Tag; onClick?: () => void }) {
  const keyOnly = tag.type === 'keyOnly';
  const ValueIcon = tag.type === 'textSet' || tag.type === 'integerSet' || tag.type === 'realSet' ? Tags : TagIcon;

  const Wrapper = onClick ? 'button' : 'span';
  return (
    <Wrapper type={onClick ? 'button' : undefined} className={`group/tag inline-block max-w-full text-left ${onClick ? 'pointer-events-auto cursor-pointer' : ''}`} tabIndex={0} aria-label={tagLabel(tag)} onClick={onClick}>
      <span className={`badge badge-soft badge-primary h-auto min-h-7 max-w-full py-1 ${onClick ? 'transition-colors group-hover/tag:bg-primary group-hover/tag:text-primary-content group-focus-visible/tag:bg-primary group-focus-visible/tag:text-primary-content' : ''}`}>
        {keyOnly ? <Hash aria-hidden="true" className="size-4 shrink-0" strokeWidth={1.75} /> : (
          <span className="grid size-4 shrink-0" aria-hidden="true">
            <ValueIcon className="col-start-1 row-start-1 size-4 group-hover/tag:invisible group-focus/tag:invisible" strokeWidth={1.75} />
            <Hash className="invisible col-start-1 row-start-1 size-4 group-hover/tag:visible group-focus/tag:visible" strokeWidth={1.75} />
          </span>
        )}
        {
          keyOnly ?
            <span className="min-w-0 whitespace-pre-wrap [overflow-wrap:anywhere]">
              {tag.key}
            </span>
            :
            // 両方の文字列を同じセルに重ね、切り替え時のバッジ幅を保つ。
            <span className="grid min-w-0 whitespace-pre-wrap [overflow-wrap:anywhere]">
              <span className="col-start-1 row-start-1 group-hover/tag:invisible group-focus/tag:invisible">{tagValue(tag)}</span>
              <span className="invisible col-start-1 row-start-1 group-hover/tag:visible group-focus/tag:visible">{tag.key}</span>
            </span>
        }
      </span>
    </Wrapper>
  );
}
