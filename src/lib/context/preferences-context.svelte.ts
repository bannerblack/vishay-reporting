import { getContext, setContext } from 'svelte';
import { user as userAdapter, type UserPreferences } from '$lib/db/database';
import { setLanguage } from '$lib/i18n/strings';

// Available themes - can be extended
export const THEMES = {
	light: 'light',
	dark: 'dark',
	blue: 'blue',
	green: 'green',
	purple: 'purple',
	system: 'system'
} as const;

export type ThemeName = keyof typeof THEMES;

class PreferencesContext {
	preferences = $state<UserPreferences>({
		theme: 'system',
		language: 'en',
		notifications: true
	});

	isLoading = $state(false);

	async load(userId: number) {
		this.isLoading = true;
		try {
			const prefs = await userAdapter.getUserPreferences(userId);
			this.preferences = prefs;
			this.applyPreferences();
		} catch (error) {
			console.error('Failed to load preferences:', error);
			// Apply defaults if loading fails
			this.applyPreferences();
		} finally {
			this.isLoading = false;
		}
	}

	async update(userId: number, preferences: Partial<UserPreferences>) {
		const updated = { ...this.preferences, ...preferences };

		try {
			await userAdapter.updateUserPreferences(userId, { preferences: updated });
			this.preferences = updated;
			this.applyPreferences();
		} catch (error) {
			console.error('Failed to update preferences:', error);
			throw error;
		}
	}

	async setTheme(userId: number, theme: string) {
		await this.update(userId, { theme });
	}

	async setLanguage(userId: number, language: string) {
		await this.update(userId, { language });
	}

	private applyPreferences() {
		// Apply theme
		this.applyTheme(this.preferences.theme);

		// Apply language (cast to 'en' for now until more languages are added)
		setLanguage(this.preferences.language as 'en');
	}

	private applyTheme(theme: string) {
		const root = document.documentElement;

		// Remove all theme classes
		root.classList.remove('light', 'dark', 'blue', 'green', 'purple');

		if (theme === 'system') {
			// Use system preference
			const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
				? 'dark'
				: 'light';
			root.classList.add(systemTheme);
		} else {
			// Apply specific theme
			root.classList.add(theme);
		}
	}
}

const PREFERENCES_CONTEXT_KEY = Symbol('preferences');

export function createPreferencesContext() {
	return new PreferencesContext();
}

export function setPreferencesContext(context: PreferencesContext) {
	setContext(PREFERENCES_CONTEXT_KEY, context);
}

export function getPreferencesContext(): PreferencesContext {
	return getContext<PreferencesContext>(PREFERENCES_CONTEXT_KEY);
}
