import { Pencil, Search, Trash2, X } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useQueryClient } from '@tanstack/react-query';
import { useSearchStore } from '../store';
import { TermEditor } from './TermEditor';
import { IconButton } from './IconAction';
import { MediaTypeIcon } from './MediaTypeIcon';
import { TagBadge } from './TagBadge';
import { MediaType, type SearchTerm, type Tag } from '../api/generated';

function tagFor(term: Extract<SearchTerm, { kind: 'tagMatch' | 'tagExists' }>): Tag {
  if (term.kind === 'tagExists') return { type: 'keyOnly', key: term.key };
  return term.values.length === 1
    ? { type: 'text', key: term.key, value: term.values[0] }
    : { type: 'textSet', key: term.key, values: new Set(term.values) };
}

export function ContentSearch() {
  const { draft, setDraft, apply } = useSearchStore();
  const [termToEdit, setTermToEdit] = useState<SearchTerm>();
  const navigate = useNavigate();
  const client = useQueryClient();
  const allMediaTypes = Object.values(MediaType);
  const mediaTypeTerm = draft.find((term) => term.kind === 'mediaTypeMatch');
  const selectedMediaTypes = mediaTypeTerm?.values ?? allMediaTypes;
  const tagTerms = draft.filter((term): term is Extract<SearchTerm, { kind: 'tagMatch' | 'tagExists' }> => term.kind !== 'mediaTypeMatch');
  function setMediaTypes(values: MediaType[]) {
    const terms = draft.filter((term) => term.kind !== 'mediaTypeMatch');
    setDraft(values.length === allMediaTypes.length ? terms : [...terms, { kind: 'mediaTypeMatch', values }]);
  }
  return <section aria-label="コンテンツ検索" className="card w-full bg-base-200 text-left">
    <div className="card-body gap-4">
      <fieldset aria-label="ファイル形式" className="flex flex-wrap gap-2">{allMediaTypes.map((type) => {
        const selected = selectedMediaTypes.includes(type);
        return <label key={type} className={`btn btn-square relative has-focus-visible:outline-2 has-focus-visible:outline-offset-2 ${selected ? 'btn-soft' : 'btn-ghost'}`}>
          <MediaTypeIcon mediaType={type} />
          <input type="checkbox" className="absolute inset-0 z-10 size-full cursor-pointer opacity-0" aria-label={type} checked={selected} disabled={selected && selectedMediaTypes.length === 1} onChange={(event) => setMediaTypes(event.target.checked ? [...selectedMediaTypes, type] : selectedMediaTypes.filter((value) => value !== type))} />
        </label>;
      })}</fieldset>
      <TermEditor termToEdit={termToEdit} onAdd={(term) => { setDraft([...draft, term]); setTermToEdit(undefined); }} />
      {tagTerms.length > 0 && <ul className="flex flex-wrap gap-3" aria-label="検索条件（すべてに一致）">{tagTerms.map((term, index) => <li key={index} className="group/term relative inline-flex max-w-full">
          <TagBadge tag={tagFor(term)} />
          <span className="pointer-events-none absolute start-full top-1/2 z-10 flex -translate-y-1/2 opacity-0 group-hover/term:pointer-events-auto group-hover/term:opacity-100 group-focus-within/term:pointer-events-auto group-focus-within/term:opacity-100">
            <IconButton className="btn-xs rounded-full" icon={Pencil} label={`条件 ${index + 1} を編集`} onClick={() => { setDraft(draft.filter((candidate) => candidate !== term)); setTermToEdit(term); }} />
            <IconButton className="btn-xs rounded-full" icon={X} label={`条件 ${index + 1} を削除`} onClick={() => setDraft(draft.filter((candidate) => candidate !== term))} />
          </span>
        </li>)}</ul>}
      <div className="flex flex-wrap justify-center gap-2">
        <IconButton className="btn-circle btn-outline btn-primary" icon={Search} label="検索" onClick={() => {
          apply();
          void client.resetQueries({ queryKey: ['contents', draft], exact: true });
          navigate('/contents');
        }} />
        <IconButton className="btn-circle btn-outline btn-secondary" icon={Trash2} label="条件をクリア" disabled={!draft.length} onClick={() => setDraft([])} />
      </div>
    </div>
  </section>;
}
