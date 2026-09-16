import { apiRequest, contentAccessApi } from '../api/client';

let configuring: Promise<Awaited<ReturnType<typeof contentAccessApi.configureContentAccess>>> | undefined;

export function configureContentAccess() {
  configuring ??= apiRequest(
    contentAccessApi.configureContentAccess(),
    'コンテンツアクセスを設定できませんでした。時間をおいて再度お試しください。',
  ).finally(() => { configuring = undefined; });
  return configuring;
}

export function clearContentAccess() {
  return apiRequest(
    contentAccessApi.clearContentAccess(),
    'コンテンツアクセス設定を解除できませんでした。再度お試しください。',
  );
}
