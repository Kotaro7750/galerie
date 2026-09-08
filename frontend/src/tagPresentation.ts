import type { InvalidTag } from './api/generated';

export const invalidReasons: Record<InvalidTag['reason'], string> = {
  INVALID_KEY: 'タグ名が無効です',
  UNSUPPORTED_VALUE_TYPE: '未対応の型です',
  INVALID_VALUE: '値が無効です',
};

