import type { PropsWithChildren } from 'react';
import { LogIn } from 'lucide-react';
import { useLocation } from 'react-router-dom';
import { useAuth } from 'react-oidc-context';
import { authEnabled } from './config';
import { Loading } from '../components/Feedback';

function AuthenticationRequired({ children }: PropsWithChildren) {
  const auth = useAuth();
  const location = useLocation();
  if (auth.isLoading || auth.activeNavigator) return <Loading label="認証状態を確認中…" />;
  if (auth.error) return <div className="alert alert-error my-4" role="alert">認証に失敗しました。再度ログインしてください。</div>;
  if (auth.isAuthenticated) return children;
  return <section className="hero py-12 text-center">
    <div className="hero-content flex-col">
      <h1 className="text-2xl font-bold">ログインが必要です</h1>
      <button className="btn btn-primary" type="button" onClick={() => { void auth.signinRedirect({ state: { returnTo: `${location.pathname}${location.search}` } }); }}>
        <LogIn aria-hidden="true" className="size-5" />ログイン
      </button>
    </div>
  </section>;
}

export function ProtectedRoute({ children }: PropsWithChildren) {
  return authEnabled ? <AuthenticationRequired>{children}</AuthenticationRequired> : children;
}
