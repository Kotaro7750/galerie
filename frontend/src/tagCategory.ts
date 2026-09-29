import type { TagSchema } from './api/generated';

export type TagCategory = 'required' | 'optional' | 'additional' | 'unknown';

export function tagCategory(schema: TagSchema | undefined, key: string): TagCategory {
  if (!schema || !key) return 'unknown';
  if (schema.required.some((definition) => definition.key === key)) return 'required';
  if (schema.optional.some((definition) => definition.key === key)) return 'optional';
  return 'additional';
}

export const tagCategoryStyle: Record<TagCategory, { label: string; badge: string; icon: string; hover: string }> = {
  required: { label: '必須タグ', badge: 'badge-primary', icon: 'text-primary', hover: 'group-hover/tag:bg-primary group-hover/tag:text-primary-content group-focus-visible/tag:bg-primary group-focus-visible/tag:text-primary-content' },
  optional: { label: '任意タグ', badge: 'badge-secondary', icon: 'text-secondary', hover: 'group-hover/tag:bg-secondary group-hover/tag:text-secondary-content group-focus-visible/tag:bg-secondary group-focus-visible/tag:text-secondary-content' },
  additional: { label: '定義外タグ', badge: 'badge-accent', icon: 'text-accent', hover: 'group-hover/tag:bg-accent group-hover/tag:text-accent-content group-focus-visible/tag:bg-accent group-focus-visible/tag:text-accent-content' },
  unknown: { label: 'タグの分類を確認中', badge: 'badge-ghost', icon: 'text-base-content/60', hover: 'group-hover/tag:bg-neutral group-hover/tag:text-neutral-content group-focus-visible/tag:bg-neutral group-focus-visible/tag:text-neutral-content' },
};
