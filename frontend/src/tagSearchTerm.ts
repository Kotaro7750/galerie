import type { SearchTerm, Tag } from './api/generated';

export function tagSearchTerm(tag: Tag): SearchTerm {
  if (tag.type === 'keyOnly') return { kind: 'tagExists', key: tag.key };

  const values = tag.type === 'textSet' || tag.type === 'integerSet' || tag.type === 'realSet'
    ? Array.from(tag.values, String)
    : [String(tag.value)];

  return values.length > 0
    ? { kind: 'tagMatch', key: tag.key, values }
    : { kind: 'tagExists', key: tag.key };
}
