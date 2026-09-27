import type { ContentDiagnostic } from './api/generated';

export const diagnosticReasons: Record<ContentDiagnostic['kind'], string> = {
  invalidKey: 'タグ名が無効です',
  duplicateKey: 'タグ名が重複しています',
  unsupportedXmpValueType: '未対応のXMP値形式です',
  notAllowedTagKey: 'タグ名がスキーマで許可されていません',
  unparseableTagValue: 'タグの値を解釈できません',
  duplicateSetValue: '集合の値が重複しています',
  notAllowedTagValue: 'タグの値がスキーマの制約を満たしていません',
  missingRequiredTag: '必須タグがありません',
};
