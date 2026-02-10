import { ref } from 'vue';

type ThemeMode = 'light' | 'dark' | 'system';

const THEME_STORAGE_KEY = 'mytv-theme-preference';

const themeMode = ref<ThemeMode>('system');

export function useTheme() {
  const isDark = ref(false);
  let mediaQuery: MediaQueryList | null = null;

  const applyTheme = () => {
    const root = document.documentElement;
    const body = document.body;
    
    let effectiveDark = false;
    
    if (themeMode.value === 'system') {
      effectiveDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    } else {
      effectiveDark = themeMode.value === 'dark';
    }

    isDark.value = effectiveDark;

    if (effectiveDark) {
      root.classList.add('theme-dark');
      body.classList.add('theme-dark');
      body.setAttribute('arco-theme', 'dark');
      root.classList.remove('theme-light');
      body.classList.remove('theme-light');
      root.style.colorScheme = 'dark';
    } else {
      root.classList.add('theme-light');
      body.classList.add('theme-light');
      body.removeAttribute('arco-theme');
      root.classList.remove('theme-dark');
      body.classList.remove('theme-dark');
      root.style.colorScheme = 'light';
    }
  };

  const setTheme = (mode: ThemeMode) => {
    themeMode.value = mode;
    localStorage.setItem(THEME_STORAGE_KEY, mode);
    applyTheme();
  };

  const handleSystemChange = () => {
    if (themeMode.value === 'system') {
      applyTheme();
    }
  };

  const initTheme = () => {
    const stored = localStorage.getItem(THEME_STORAGE_KEY) as ThemeMode | null;
    if (stored && ['light', 'dark', 'system'].includes(stored)) {
      themeMode.value = stored;
    }
    
    mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', handleSystemChange);
    
    applyTheme();
  };

  return {
    themeMode,
    isDark,
    setTheme,
    initTheme,
    cleanup: () => {
      mediaQuery?.removeEventListener('change', handleSystemChange);
    }
  };
}
