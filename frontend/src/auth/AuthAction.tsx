import { LogIn, LogOut } from 'lucide-react';
import { useAuth } from 'react-oidc-context';
import { authEnabled, signoutRedirectArgs } from './config';

function EnabledAuthAction() {
  const auth = useAuth();
  if (auth.isLoading) return null;
  if (auth.isAuthenticated) return <button className="btn btn-ghost btn-sm" type="button" onClick={() => { void auth.signoutRedirect(signoutRedirectArgs); }}>
    <LogOut aria-hidden="true" className="size-4" />ログアウト
  </button>;
  return <button className="btn btn-ghost btn-sm" type="button" onClick={() => { void auth.signinRedirect({ state: { returnTo: '/' } }); }}>
    <LogIn aria-hidden="true" className="size-4" />ログイン
  </button>;
}

export function AuthAction() {
  return authEnabled ? <EnabledAuthAction /> : null;
}
