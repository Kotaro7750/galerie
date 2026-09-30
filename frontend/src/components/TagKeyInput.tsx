import { useId, useState } from 'react';
import { Hash } from 'lucide-react';
import type { TagSchema } from '../api/generated';
import { tagCategory, tagCategoryStyle } from '../tagCategory';

type KeyOption = { key: string; required: boolean };

type Props = {
  value: string;
  options: KeyOption[];
  allowAdditionalTags: boolean;
  schema: TagSchema;
  onChange: (value: string) => void;
};

export function TagKeyInput({ value, options, allowAdditionalTags, schema, onChange }: Props) {
  const listId = useId();
  const [open, setOpen] = useState(false);
  const [activeIndex, setActiveIndex] = useState(-1);
  const query = value.normalize('NFKC').toLocaleLowerCase();
  const matches = options.filter((option) => option.key.toLocaleLowerCase().includes(query));
  const required = matches.filter((option) => option.required);
  const optional = matches.filter((option) => !option.required);
  const showList = open && (matches.length > 0 || value.length > 0);
  const style = tagCategoryStyle[tagCategory(schema, value.normalize('NFKC'))];

  function choose(key: string) {
    onChange(key);
    setOpen(false);
    setActiveIndex(-1);
  }

  return <div className="relative min-w-0 w-full sm:w-auto sm:flex-1" onBlur={(event) => {
    if (!event.currentTarget.contains(event.relatedTarget)) setOpen(false);
  }}>
    <label className="input w-full"><span className="label"><Hash className={`size-4 ${style.icon}`} aria-hidden="true" /></span>
      <input aria-label="タグ名" aria-description={value ? style.label : undefined} role="combobox" aria-autocomplete="list" aria-controls={listId} aria-expanded={showList}
        aria-activedescendant={showList && activeIndex >= 0 ? `${listId}-${activeIndex}` : undefined}
        autoComplete="off" placeholder="タグ名を入力" value={value} onFocus={() => setOpen(true)}
        onChange={(event) => { onChange(event.target.value); setOpen(true); setActiveIndex(-1); }}
        onKeyDown={(event) => {
          if (event.key === 'Escape') { setOpen(false); setActiveIndex(-1); return; }
          if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
            if (!matches.length) return;
            event.preventDefault();
            setOpen(true);
            setActiveIndex((current) => event.key === 'ArrowDown'
              ? (current + 1) % matches.length
              : current < 0 ? matches.length - 1 : (current - 1 + matches.length) % matches.length);
          }
          if (event.key === 'Enter' && showList && activeIndex >= 0) {
            event.preventDefault();
            choose(matches[activeIndex].key);
          }
        }} />
    </label>
    {showList && <div id={listId} role="listbox" aria-label="タグ名の候補" className="absolute z-50 mt-2 max-h-64 w-full overflow-y-auto rounded-box bg-base-100 p-2 shadow-lg ring-1 ring-base-content/10">
      {([['Required', required], ['Optional', optional]] as const).map(([label, group]) => group.length > 0 && <div key={label} role="group" aria-label={label} className="mt-2 first:mt-0">
        <div className="rounded-field bg-base-200 px-3 py-1.5 text-sm font-semibold text-base-content/80">{label}</div>
        {group.map((option) => {
          const index = matches.indexOf(option);
          return <button key={option.key} id={`${listId}-${index}`} type="button" role="option" aria-selected={activeIndex === index}
            className={`block w-full rounded-field px-3 py-2 text-left ${activeIndex === index ? 'bg-base-200' : 'hover:bg-base-200'}`}
            onMouseDown={(event) => event.preventDefault()} onClick={() => choose(option.key)}
            onMouseEnter={() => setActiveIndex(index)}>{option.key}</button>;
        })}
      </div>)}
      {!matches.length && <p className="px-3 py-2 text-sm text-base-content/70">{allowAdditionalTags ? '候補にないタグ名も入力できます' : '該当するタグ名がありません'}</p>}
    </div>}
  </div>;
}
