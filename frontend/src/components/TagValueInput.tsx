import { useId, useState } from 'react';
import { Tag as TagIcon } from 'lucide-react';
import type { Tag, TagDefinition } from '../api/generated';
import { valueError } from '../tagSchema';

type Props = {
  definition?: TagDefinition;
  type: Tag['type'];
  index: number;
  value: string;
  duplicate?: boolean;
  formError: string;
  onChange: (value: string) => void;
};

export function TagValueInput({ definition, type, index, value, duplicate = false, formError, onChange }: Props) {
  const hintId = useId();
  const [touched, setTouched] = useState(false);
  const numeric = ['integer', 'integerSet', 'real', 'realSet'].includes(type);
  const allowedValues = definition?.type !== 'keyOnly' ? definition?.allowedValues : undefined;
  let liveError: string | undefined;
  if (duplicate) liveError = '集合に同じ値は追加できません。';
  else if (touched || value) {
    if (!value) liveError = allowedValues ? '値を選択してください。' : '値を入力してください。';
    else if (definition) liveError = valueError(definition, value);
    else if ([...value.normalize('NFC')].length > 255) liveError = 'テキストの値は255文字以内にしてください。';
  }
  const showHint = Boolean(liveError && !formError.endsWith(liveError));
  function change(next: string) { setTouched(true); onChange(next); }

  return <div className="min-w-0 flex-1">
    {allowedValues
      ? <label className="select validator w-full"><span className="label"><TagIcon className="size-4" aria-hidden="true" /></span><select aria-label={`値 ${index + 1}`} aria-invalid={Boolean(liveError)} aria-describedby={showHint ? hintId : undefined} value={value} onChange={(event) => change(event.target.value)} onBlur={() => setTouched(true)}><option value="">値を選択</option>{[...allowedValues].map((allowed) => <option key={allowed} value={String(allowed)}>{allowed}</option>)}</select></label>
      : <label className="input validator w-full"><span className="label"><TagIcon className="size-4" aria-hidden="true" /></span><input aria-label={`値 ${index + 1}`} aria-invalid={Boolean(liveError)} aria-describedby={showHint ? hintId : undefined} type={numeric ? 'number' : 'text'} step={numeric ? type === 'integer' || type === 'integerSet' ? '1' : 'any' : undefined} min={numeric && definition && 'min' in definition ? definition.min : undefined} max={numeric && definition && 'max' in definition ? definition.max : undefined} value={value} onChange={(event) => change(event.target.value)} onBlur={() => setTouched(true)} /></label>}
    {showHint && <p id={hintId} className="validator-hint hidden" aria-live="polite">{liveError}</p>}
  </div>;
}
