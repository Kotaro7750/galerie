import { useEffect, useState } from 'react';
import { Check, Hash, Plus, Tag as TagIcon, X } from 'lucide-react';
import type { SearchTerm, Tag } from '../api/generated';
import { IconButton } from './IconAction';

type Props = {
  onAdd?: (term: SearchTerm) => void;
  termToEdit?: SearchTerm;
  onAddTag?: (tag: Tag) => void;
  tagToEdit?: Tag;
  existingKeys?: string[];
};

type TagType = Tag['type'];
const tagTypes: { value: TagType; label: string }[] = [
  { value: 'keyOnly', label: 'キーのみ' }, { value: 'text', label: 'テキスト' },
  { value: 'integer', label: '整数' }, { value: 'real', label: '実数' },
  { value: 'textSet', label: 'テキスト集合' }, { value: 'integerSet', label: '整数集合' },
  { value: 'realSet', label: '実数集合' },
];

function parseTag(type: TagType, key: string, values: string[]): Tag | string {
  if (type === 'keyOnly') return { type, key };
  if (!values.length || values.some((value) => value === '')) return '値を入力してください。';
  const isSet = type.endsWith('Set');
  if (!isSet && values.length !== 1) return '単一値のタグには値を1つ入力してください。';
  if (type === 'text' || type === 'textSet') {
    const normalized = values.map((value) => value.normalize('NFC'));
    if (normalized.some((value) => [...value].length > 255)) return 'テキストの値は255文字以内にしてください。';
    if (new Set(normalized).size !== normalized.length) return '集合に同じ値は追加できません。';
    return type === 'text' ? { type, key, value: normalized[0] } : { type, key, values: new Set(normalized) };
  }
  const numbers = values.map(Number);
  if (values.some((value) => !value.trim()) || numbers.some((value) => !Number.isFinite(value))) return '有効な数値を入力してください。';
  if (type === 'integer' || type === 'integerSet') {
    if (numbers.some((value) => !Number.isSafeInteger(value))) return '安全に扱える整数を入力してください。';
    if (new Set(numbers).size !== numbers.length) return '集合に同じ値は追加できません。';
    return type === 'integer' ? { type, key, value: numbers[0] } : { type, key, values: new Set(numbers) };
  }
  if (new Set(numbers).size !== numbers.length) return '集合に同じ値は追加できません。';
  return type === 'real' ? { type, key, value: numbers[0] } : { type, key, values: new Set(numbers) };
}

export function TermEditor({ onAdd, termToEdit, onAddTag, tagToEdit, existingKeys = [] }: Props) {
  const registering = Boolean(onAddTag);
  const [key, setKey] = useState('');
  const [values, setValues] = useState<string[]>([]);
  const [type, setType] = useState<TagType>('keyOnly');
  const [error, setError] = useState('');
  useEffect(() => {
    if (!termToEdit || termToEdit.kind === 'mediaTypeMatch') return;
    setKey(termToEdit.key);
    setValues(termToEdit.kind === 'tagMatch' ? [...termToEdit.values] : []);
    setError('');
  }, [termToEdit]);
  useEffect(() => {
    if (!tagToEdit) return;
    setKey(tagToEdit.key);
    setType(tagToEdit.type);
    setValues(tagToEdit.type === 'keyOnly' ? [] : 'value' in tagToEdit ? [String(tagToEdit.value)] : [...tagToEdit.values].map(String));
    setError('');
  }, [tagToEdit]);
  function add() {
    const normalizedKey = key.normalize('NFKC');
    if (!/^\p{XID_Start}\p{XID_Continue}*$/u.test(normalizedKey) || [...normalizedKey].length > 64) {
      setError('タグ名は64文字以内で、文字から始まる有効な名前を入力してください。');
      return;
    }
    if (registering) {
      if (existingKeys.includes(normalizedKey)) { setError('同じタグ名は登録できません。'); return; }
      const tag = parseTag(type, normalizedKey, values);
      if (typeof tag === 'string') { setError(tag); return; }
      onAddTag?.(tag);
      setType('keyOnly');
    } else {
      if (values.some((value) => value.length === 0)) { setError('空の値は追加できません。'); return; }
      if (values.length === 0) onAdd?.({ kind: 'tagExists', key: normalizedKey });
      else onAdd?.({ kind: 'tagMatch', key: normalizedKey, values: values.map((value) => value.normalize('NFC')) });
    }
    setKey(''); setValues([]); setError('');
  }
  const isSet = type.endsWith('Set');
  return <form className="space-y-3" aria-label={registering ? 'タグエディタ' : 'Termエディタ'} onSubmit={(event) => { event.preventDefault(); add(); }}>
    <div className="flex items-start gap-2">
      <label className="input min-w-0 flex-1"><span className="label"><Hash className="size-4" aria-hidden="true" /></span><input aria-label="タグ名" value={key} onChange={(event) => setKey(event.target.value)} /></label>
      {!registering && <IconButton className="btn-ghost rounded-full" icon={Plus} label="値を追加" onClick={() => setValues([...values, ''])} />}
      <IconButton className="btn-outline btn-success rounded-full" icon={Check} label={registering ? 'タグを追加' : '条件に追加'} type="submit" />
    </div>
    {registering && <label className="select w-full"><span className="label">タグの型</span><select aria-label="タグの型" value={type} onChange={(event) => { const next = event.target.value as TagType; setType(next); setValues(next === 'keyOnly' ? [] : values.length ? (next.endsWith('Set') ? values : values.slice(0, 1)) : ['']); setError(''); }}>{tagTypes.map(({ value, label }) => <option key={value} value={value}>{label}</option>)}</select></label>}
    {(!registering || type !== 'keyOnly') && <fieldset className="space-y-2" aria-label="タグの値">
      {values.map((value, index) => <div key={index} className="flex items-start gap-2">
        <label className="input min-w-0 flex-1"><span className="label"><TagIcon className="size-4" aria-hidden="true" /></span><input aria-label={`値 ${index + 1}`} value={value} onChange={(event) => setValues(values.map((item, i) => i === index ? event.target.value : item))} /></label>
        {(!registering || isSet) && <IconButton icon={X} label={`値 ${index + 1} を削除`} onClick={() => setValues(values.filter((_, i) => i !== index))} />}
      </div>)}
      {registering && isSet && <IconButton className="btn-ghost rounded-full" icon={Plus} label="値を追加" onClick={() => setValues([...values, ''])} />}
    </fieldset>}
    {error && <p role="alert" className="text-error">{error}</p>}
  </form>;
}
