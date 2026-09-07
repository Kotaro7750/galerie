import type { Content, Tag, InvalidTag } from '../api/generated';

function tagLabel(tag: Tag): string {
  switch (tag.type) {
    case 'keyOnly': return tag.key;
    case 'text':
    case 'integer':
    case 'real': return `${tag.key}: ${tag.value === '' ? '（空文字）' : tag.value}`;
    case 'textSet':
    case 'integerSet':
    case 'realSet': return `${tag.key}: ${tag.values.size === 0 ? '（空集合）' : [...tag.values].map((value) => value === '' ? '（空文字）' : value).join(', ')}`;
  }
}

const invalidReasons: Record<InvalidTag['reason'], string> = {
  INVALID_KEY: 'タグ名が無効です',
  UNSUPPORTED_VALUE_TYPE: '未対応の型です',
  INVALID_VALUE: '値が無効です',
};

export function ContentTags({ tags, invalidTags }: Pick<Content, 'tags' | 'invalidTags'>) {
  return <div className="space-y-3 text-sm">
    <p className="font-semibold">タグ</p>
    {tags.length === 0 ? <p>タグはありません</p> : <ul className="flex flex-wrap gap-2" aria-label="タグ">
      {tags.map((tag, index) => <li className="rounded-box bg-base-200 px-3 py-1 whitespace-pre-wrap [overflow-wrap:anywhere]" key={`${tag.key}-${index}`}>{tagLabel(tag)}</li>)}
    </ul>}
    {invalidTags.length > 0 && <div className="space-y-2">
      <p className="font-semibold">無効なタグ</p>
      <ul className="space-y-1 [overflow-wrap:anywhere]" aria-label="無効なタグ">
        {invalidTags.map((tag, index) => <li key={`${tag.key}-${index}`}>{tag.key}: {invalidReasons[tag.reason]}</li>)}
      </ul>
    </div>}
  </div>;
}
