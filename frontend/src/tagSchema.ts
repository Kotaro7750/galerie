import type { Tag, TagDefinition, TagSchema } from './api/generated';

export function definitionFor(schema: TagSchema, key: string): TagDefinition | undefined {
  return [...schema.required, ...schema.optional].find((definition) => definition.key === key);
}

export function valueError(definition: TagDefinition, raw: string): string | undefined {
  if (definition.type === 'keyOnly') return 'キーのみのタグに値は指定できません。';
  if (definition.type === 'text' || definition.type === 'textSet') {
    const value = raw.normalize('NFC');
    const length = [...value].length;
    if (!length || length > 255) return 'テキストの値は1～255文字で入力してください。';
    if (definition.minLength !== undefined && length < definition.minLength) return `${definition.minLength}文字以上で入力してください。`;
    if (definition.maxLength !== undefined && length > definition.maxLength) return `${definition.maxLength}文字以下で入力してください。`;
    if (definition.allowedValues && !definition.allowedValues.has(value)) return '許可された値を選んでください。';
    return undefined;
  }
  const integer = definition.type === 'integer' || definition.type === 'integerSet';
  const format = integer ? /^[-+]?\d+$/ : /^[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?$/;
  if (!format.test(raw)) return integer ? '整数を入力してください。' : '実数を入力してください。';
  const value = Number(raw);
  if (!Number.isFinite(value) || (integer && !Number.isSafeInteger(value))) return '有効な数値を入力してください。';
  if (definition.min !== undefined && value < definition.min) return `${definition.min}以上で入力してください。`;
  if (definition.max !== undefined && value > definition.max) return `${definition.max}以下で入力してください。`;
  if (definition.allowedValues && !definition.allowedValues.has(value)) return '許可された値を選んでください。';
  return undefined;
}

export function tagError(schema: TagSchema, tag: Tag): string | undefined {
  const definition = definitionFor(schema, tag.key);
  if (!definition) {
    if (!schema.allowAdditionalTags) return `${tag.key} はスキーマで許可されていません。`;
    if (!['keyOnly', 'text', 'textSet'].includes(tag.type)) return `${tag.key} はキーのみ・テキスト・テキスト集合のいずれかで入力してください。`;
    return undefined;
  }
  if (tag.type !== definition.type) return `${tag.key} は ${definition.type} 型で入力してください。`;
  if (tag.type === 'keyOnly') return undefined;
  const values = 'value' in tag ? [tag.value] : [...tag.values];
  if (!values.length) return `${tag.key} に値を入力してください。`;
  for (const value of values) {
    const error = valueError(definition, String(value));
    if (error) return `${tag.key}: ${error}`;
  }
  return undefined;
}

export function registrationErrors(schema: TagSchema, tags: Tag[]): string[] {
  const errors = schema.required.filter((definition) => !tags.some((tag) => tag.key === definition.key))
    .map((definition) => `必須タグ ${definition.key} を追加してください。`);
  const keys = new Set<string>();
  for (const tag of tags) {
    if (keys.has(tag.key)) errors.push(`${tag.key} は重複しています。`);
    keys.add(tag.key);
    const error = tagError(schema, tag);
    if (error) errors.push(error);
  }
  return errors;
}
