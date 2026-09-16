import { LogIn, LogOut } from 'lucide-react';
import { useState } from 'react';
import { useAuth } from 'react-oidc-context';
import { authEnabled, signoutRedirectArgs } from './config';
import { clearContentAccess } from '../content-access/api';

function EnabledAuthAction() {
  const auth = useAuth();
  const [signingOut, setSigningOut] = useState(false);
  const [signoutError, setSignoutError] = useState(false);
  if (auth.isLoading) return null;
  if (auth.isAuthenticated) return <div className="flex items-center gap-2">
    {signoutError && <span className="text-error text-sm" role="alert">サインアウトできませんでした。</span>}
    <button className="btn btn-ghost btn-sm" type="button" disabled={signingOut} onClick={() => {
      setSigningOut(true);
      setSignoutError(false);
      void clearContentAccess()
        .then(() => auth.signoutRedirect(signoutRedirectArgs))
        .catch(() => { setSignoutError(true); setSigningOut(false); });
    }}>
      {signingOut ? <span className="loading loading-spinner loading-xs" /> : <LogOut aria-hidden="true" className="size-4" />}ログアウト
    </button>
  </div>;
  return <button className="btn btn-ghost btn-sm" type="button" onClick={() => { void auth.signinRedirect({ state: { returnTo: '/' } }); }}>
    <LogIn aria-hidden="true" className="size-4" />ログイン
  </button>;
}

export function AuthAction() {
  return authEnabled ? <EnabledAuthAction /> : null;
}
