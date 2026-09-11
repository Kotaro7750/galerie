import { useEffect, useState } from 'react';
import { Check, Hash, Plus, Tag, X } from 'lucide-react';
import type { SearchTerm } from '../api/generated';
import { IconButton } from './IconAction';

export function TermEditor({ onAdd, termToEdit }: { onAdd: (term: SearchTerm) => void; termToEdit?: SearchTerm }) {
  const [key, setKey] = useState('');
  const [values, setValues] = useState<string[]>([]);
  const [error, setError] = useState('');
  useEffect(() => {
    if (!termToEdit || termToEdit.kind === 'mediaTypeMatch') return;
    setKey(termToEdit.key);
    setValues(termToEdit.kind === 'tagMatch' ? [...termToEdit.values] : []);
    setError('');
  }, [termToEdit]);
  function add() {
    const normalizedKey = key.normalize('NFKC');
    if (!/^\p{XID_Start}\p{XID_Continue}*$/u.test(normalizedKey) || [...normalizedKey].length > 64) {
      setError('タグ名は64文字以内で、文字から始まる有効な名前を入力してください。');
      return;
    }
    if (values.some((value) => value.length === 0)) {
      setError('空の値は追加できません。');
      return;
    }
    if (values.length === 0) onAdd({ kind: 'tagExists', key: normalizedKey });
    else onAdd({ kind: 'tagMatch', key: normalizedKey, values: values.map((value) => value.normalize('NFC')) });
    setKey(''); setValues([]); setError('');
  }
  return <form className="space-y-3" aria-label="Termエディタ" onSubmit={(event) => { event.preventDefault(); add(); }}>
    <div className="flex items-start gap-2">
      <label className="input min-w-0 flex-1"><span className="label"><Hash className="size-4" aria-hidden="true" /></span><input aria-label="タグ名" value={key} onChange={(event) => setKey(event.target.value)} /></label>
      <IconButton className="btn-ghost rounded-full" icon={Plus} label="値を追加" onClick={() => setValues([...values, ''])} />
      <IconButton className="btn-outline btn-success rounded-full" icon={Check} label="条件に追加" type="submit" />
    </div>
    <fieldset className="space-y-2" aria-label="タグの値">
      {values.map((value, index) => <div key={index} className="flex items-start gap-2">
        <label className="input min-w-0 flex-1"><span className="label"><Tag className="size-4" aria-hidden="true" /></span><input aria-label={`値 ${index + 1}`} value={value} onChange={(event) => setValues(values.map((item, i) => i === index ? event.target.value : item))} /></label>
        <IconButton icon={X} label={`値 ${index + 1} を削除`} onClick={() => setValues(values.filter((_, i) => i !== index))} />
      </div>)}
    </fieldset>
    {error && <p role="alert" className="text-error">{error}</p>}
  </form>;
}
