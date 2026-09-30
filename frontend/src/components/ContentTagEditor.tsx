import { useState } from 'react';
import { Pencil, Tags, Trash2 } from 'lucide-react';
import type { Tag, TagSchema } from '../api/generated';
import { registrationErrors } from '../tagSchema';
import { IconButton } from './IconAction';
import { TagBadge } from './TagBadge';
import { TermEditor } from './TermEditor';

export function ContentTagEditor({ tags, onChange, schema, busy = false, schemaError = false, label = '編集するタグ', onEditingChange, validationErrors }: {
  tags: Tag[]; onChange: (tags: Tag[]) => void; schema?: TagSchema; busy?: boolean; schemaError?: boolean; label?: string; onEditingChange?: (editing: boolean) => void; validationErrors?: string[];
}) {
  const [tagToEdit, setTagToEdit] = useState<Tag>();
  const errors = validationErrors ?? (schema ? registrationErrors(schema, tags) : []);
  return <section className="card bg-base-200" aria-label={label}><div className="card-body gap-4">
    <h2 className="card-title"><Tags className="size-5" aria-hidden="true" />タグ</h2>
    {schema ? <TermEditor schema={schema} onAddTag={(tag) => { onChange([...tags, tag]); setTagToEdit(undefined); onEditingChange?.(false); }} tagToEdit={tagToEdit} existingKeys={tags.map((tag) => tag.key)} />
      : <p role="status">{schemaError ? 'タグスキーマを取得できませんでした。再読み込みしてお試しください。' : 'タグスキーマを読み込み中…'}</p>}
    {errors.length > 0 && <ul className="text-sm text-error" aria-label="タグスキーマの確認結果">{errors.map((error) => <li key={error}>{error}</li>)}</ul>}
    {tags.length > 0 && <ul className="flex flex-wrap gap-3" aria-label={label}>{tags.map((tag, index) => <li key={`${tag.key}-${index}`} className="group/tag-item relative inline-flex max-w-full items-center hover:z-20 focus-within:z-20 [@media(hover:none)]:flex-wrap">
      <TagBadge tag={tag} schema={schema} />
      <span className="pointer-events-none absolute top-full left-0 z-20 inline-flex items-center gap-1 rounded-box bg-base-100 p-1 opacity-0 shadow-md group-hover/tag-item:pointer-events-auto group-hover/tag-item:opacity-100 group-focus-within/tag-item:pointer-events-auto group-focus-within/tag-item:opacity-100 [@media(hover:none)]:pointer-events-auto [@media(hover:none)]:static [@media(hover:none)]:bg-transparent [@media(hover:none)]:p-0 [@media(hover:none)]:opacity-100 [@media(hover:none)]:shadow-none">
        <IconButton className="btn-xs rounded-full [@media(hover:none)]:min-h-11 [@media(hover:none)]:min-w-11" icon={Pencil} label={`${tag.key} を編集`} disabled={busy} onClick={() => { onChange(tags.filter((item) => item !== tag)); setTagToEdit(tag); onEditingChange?.(true); }} />
        <IconButton className="btn-xs rounded-full [@media(hover:none)]:min-h-11 [@media(hover:none)]:min-w-11" icon={Trash2} label={`${tag.key} を削除`} disabled={busy} onClick={() => onChange(tags.filter((item) => item !== tag))} />
      </span>
    </li>)}</ul>}
  </div></section>;
}
