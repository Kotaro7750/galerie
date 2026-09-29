export type Theme = 'light' | 'dark';

const THEME_STORAGE_KEY = 'galerie-theme';

export function readTheme(): Theme {
  try {
    const saved = window.localStorage.getItem(THEME_STORAGE_KEY);
    if (saved === 'light' || saved === 'dark') return saved;
  } catch {
    // ストレージを利用できない場合はOS設定を使う。
  }
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

export function applyTheme(theme: Theme) {
  document.documentElement.dataset.theme = theme;
}

export function saveTheme(theme: Theme) {
  try {
    window.localStorage.setItem(THEME_STORAGE_KEY, theme);
  } catch {
    // ストレージを利用できなくても現在のページでは切り替えられる。
  }
}
