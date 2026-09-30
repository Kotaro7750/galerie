import type { Content, Tag, TagSchema } from './api/generated';
import { tagError } from './tagSchema';

export function validTags(content: Content, schema: TagSchema): Tag[] {
  return content.tags.filter((tag) => !tagError(schema, tag));
}

function sameTag(left: Tag, right: Tag): boolean {
  if (left.key !== right.key || left.type !== right.type) return false;
  if ('value' in left && 'value' in right) return left.value === right.value;
  if ('values' in left && 'values' in right) {
    const rightValues = right.values as ReadonlySet<string | number>;
    return left.values.size === right.values.size && [...left.values].every((value) => rightValues.has(value));
  }
  return left.type === 'keyOnly' && right.type === 'keyOnly';
}

export function commonTags(contents: Content[], schema: TagSchema): Tag[] {
  if (!contents.length) return [];
  const [first, ...rest] = contents;
  return validTags(first, schema).filter((tag) => rest.every((content) => validTags(content, schema).some((other) => sameTag(tag, other))));
}

export function mergedTags(content: Content, edited: Tag[], commonKeys: string[], schema: TagSchema): Tag[] {
  const replacedKeys = new Set([...commonKeys, ...edited.map((tag) => tag.key)]);
  return [...validTags(content, schema).filter((tag) => !replacedKeys.has(tag.key)), ...edited];
}
