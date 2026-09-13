import { Moon, Sun } from 'lucide-react';
import { Link } from 'react-router-dom';
import { AuthAction } from '../auth/AuthAction';

export function AppHeader() {
  return <header className="navbar flex-wrap bg-base-200 px-4">
      <Link to="/" className="btn btn-ghost text-xl" aria-label="Galerie トップページ">Galerie</Link>
      <div className="ml-auto"><AuthAction /></div>
      <label className="group flex cursor-pointer items-center gap-2" title="ダークモード">
        <Sun aria-hidden="true" className="size-5 group-has-checked:hidden" strokeWidth={1.75} />
        <Moon aria-hidden="true" className="hidden size-5 group-has-checked:block" strokeWidth={1.75} />
        <input type="checkbox" className="toggle toggle-sm theme-controller" value="dark" aria-label="ダークモード" defaultChecked={window.matchMedia('(prefers-color-scheme: dark)').matches} />
      </label>
    </header>;
}
