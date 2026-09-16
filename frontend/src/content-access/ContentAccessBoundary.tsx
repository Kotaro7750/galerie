import { useEffect, useState, type PropsWithChildren } from 'react';
import { useAuth } from 'react-oidc-context';
import { authEnabled } from '../auth/config';
import { ErrorMessage, Loading } from '../components/Feedback';
import { configureContentAccess } from './api';

const MAX_TIMEOUT_MS = 2_147_483_647;

function ContentAccessGate({ children }: PropsWithChildren) {
  const [attempt, setAttempt] = useState(0);
  const [state, setState] = useState<{ ready: boolean; error?: Error }>({ ready: false });

  useEffect(() => {
    let active = true;
    let refreshing = false;
    let refreshAt: number | undefined;
    let timer: ReturnType<typeof setTimeout> | undefined;

    const schedule = (invalidAfter?: Date) => {
      refreshAt = undefined;
      timer = undefined;
      if (!invalidAfter) return;
      const invalidAfterMs = invalidAfter.getTime();
      const now = Date.now();
      if (!Number.isFinite(invalidAfterMs) || invalidAfterMs <= now) {
        throw new Error('コンテンツアクセスの有効期限が無効です。');
      }
      refreshAt = now + Math.floor((invalidAfterMs - now) / 2);
      timer = setTimeout(() => { void refresh(); }, Math.min(refreshAt - now, MAX_TIMEOUT_MS));
    };

    const refresh = async () => {
      if (refreshing) return;
      refreshing = true;
      if (timer) clearTimeout(timer);
      try {
        const configuration = await configureContentAccess();
        if (!active) return;
        schedule(configuration.invalidAfter);
        setState({ ready: true });
      } catch (error) {
        if (active) setState({ ready: false, error: error instanceof Error ? error : new Error(String(error)) });
      } finally {
        refreshing = false;
      }
    };

    const refreshWhenDue = () => {
      if (document.visibilityState === 'visible' && refreshAt !== undefined && Date.now() >= refreshAt) void refresh();
    };

    void refresh();
    document.addEventListener('visibilitychange', refreshWhenDue);
    return () => {
      active = false;
      if (timer) clearTimeout(timer);
      document.removeEventListener('visibilitychange', refreshWhenDue);
    };
  }, [attempt]);

  if (state.error) return <ErrorMessage error={state.error} onRetry={() => { setState({ ready: false }); setAttempt((value) => value + 1); }} />;
  if (!state.ready) return <Loading label="コンテンツアクセスを設定中…" />;
  return children;
}

function AuthenticatedContentAccessBoundary({ children }: PropsWithChildren) {
  const auth = useAuth();
  if (!auth.isAuthenticated) return children;
  return <ContentAccessGate>{children}</ContentAccessGate>;
}

export function ContentAccessBoundary({ children }: PropsWithChildren) {
  return authEnabled
    ? <AuthenticatedContentAccessBoundary>{children}</AuthenticatedContentAccessBoundary>
    : <ContentAccessGate>{children}</ContentAccessGate>;
}
