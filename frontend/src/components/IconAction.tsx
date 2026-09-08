import type { ComponentProps } from 'react';
import type { LucideIcon } from 'lucide-react';
import { Link } from 'react-router-dom';

type Appearance = { label: string; icon: LucideIcon; className?: string };

function appearance({ label, icon: Icon, className = 'btn-ghost' }: Appearance) {
  return {
    className: `btn btn-square ${className}`,
    'aria-label': label,
    title: label,
    children: <Icon aria-hidden="true" className="size-5" strokeWidth={1.75} />,
  };
}

export function IconButton({ label, icon, className, ...props }: Appearance & Omit<ComponentProps<'button'>, 'children'>) {
  return <button type="button" {...props} {...appearance({ label, icon, className })} />;
}

export function IconLink({ label, icon, className, ...props }: Appearance & Omit<ComponentProps<typeof Link>, 'children'>) {
  return <Link {...props} {...appearance({ label, icon, className })} />;
}

export function IconAnchor({ label, icon, className, ...props }: Appearance & Omit<ComponentProps<'a'>, 'children'>) {
  return <a {...props} {...appearance({ label, icon, className })} />;
}
