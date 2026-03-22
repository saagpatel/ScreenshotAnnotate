export type AppTheme = "light" | "dark" | "system";

export interface AppPreferences {
  defaultColor: string;
  defaultThickness: number;
  theme: AppTheme;
}

const STORAGE_KEY = "screenshot-annotate.preferences";

export const DEFAULT_PREFERENCES: AppPreferences = {
  defaultColor: "#FF0000",
  defaultThickness: 3,
  theme: "system",
};

export function loadAppPreferences(): AppPreferences {
  if (typeof window === "undefined") {
    return DEFAULT_PREFERENCES;
  }

  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return DEFAULT_PREFERENCES;
    }

    const parsed = JSON.parse(raw) as Partial<AppPreferences>;

    return {
      defaultColor: isHexColor(parsed.defaultColor)
        ? parsed.defaultColor
        : DEFAULT_PREFERENCES.defaultColor,
      defaultThickness: clampThickness(parsed.defaultThickness),
      theme: isTheme(parsed.theme) ? parsed.theme : DEFAULT_PREFERENCES.theme,
    };
  } catch {
    return DEFAULT_PREFERENCES;
  }
}

export function saveAppPreferences(preferences: AppPreferences) {
  if (typeof window === "undefined") {
    return;
  }

  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(preferences));
}

export function applyTheme(theme: AppTheme) {
  if (typeof document === "undefined") {
    return;
  }

  if (theme === "system") {
    document.documentElement.removeAttribute("data-theme");
    return;
  }

  document.documentElement.setAttribute("data-theme", theme);
}

function clampThickness(value: unknown): number {
  if (typeof value !== "number" || Number.isNaN(value)) {
    return DEFAULT_PREFERENCES.defaultThickness;
  }

  return Math.max(1, Math.min(8, Math.round(value)));
}

function isHexColor(value: unknown): value is string {
  return typeof value === "string" && /^#[0-9A-F]{6}$/i.test(value);
}

function isTheme(value: unknown): value is AppTheme {
  return value === "light" || value === "dark" || value === "system";
}
