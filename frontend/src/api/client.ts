import { QueryClient } from '@tanstack/react-query';
import { Configuration, ContentAccessApi, ContentsApi, ResponseError } from './generated';
import { authEnabled, getAccessToken } from '../auth/config';

export class ApiError extends Error {
  constructor(public readonly status: number, message: string) {
    super(message);
    this.name = 'ApiError';
  }
}

const apiConfiguration = new Configuration({
  basePath: (import.meta.env.VITE_API_BASE_URL || '/api/v0').replace(/\/$/, ''),
  middleware: authEnabled ? [{
    pre: async ({ url, init }) => {
      const token = await getAccessToken();
      if (!token) throw new ApiError(401, 'ログインの有効期限が切れました。再度ログインしてください。');
      return { url, init: { ...init, headers: { ...init.headers, Authorization: `Bearer ${token}` } } };
    },
  }] : [],
});

export const contentAccessApi = new ContentAccessApi(apiConfiguration);
export const contentsApi = new ContentsApi(apiConfiguration);

// Keep HTTP error presentation outside the generated client so regeneration is safe.
export async function apiRequest<T>(request: Promise<T>, fallbackMessage?: string): Promise<T> {
  try {
    return await request;
  } catch (error) {
    if (error instanceof ResponseError) {
      const status = error.response.status;
      const problem: unknown = await error.response.json().catch(() => null);
      const detail = problem && typeof problem === 'object' && 'detail' in problem
        && typeof problem.detail === 'string' ? problem.detail : undefined;
      throw new ApiError(status, detail || (status === 404
        ? 'コンテンツが見つかりません。削除された可能性があります。'
        : status === 400 ? 'リクエストが無効です。一覧を更新してお試しください。'
          : fallbackMessage || 'コンテンツを取得できませんでした。時間をおいて再度お試しください。'));
    }
    throw error;
  }
}

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60_000,
      retry: (count, error) => !(error instanceof ApiError && error.status < 500) && count < 1,
      refetchOnWindowFocus: false,
    },
  },
});
