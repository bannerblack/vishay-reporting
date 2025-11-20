import { mdsvex } from 'mdsvex';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: [vitePreprocess(), mdsvex()],
	kit: { 
		adapter: adapter({
			fallback: 'index.html'
		}),
		alias: {
			'$types':"src/types/types",
			'$db': "src/db",
			'$components': "src/components"
		},
		csrf: {
			trustedOrigins: ['tauri://localhost', 'http://tauri.localhost', 'https://tauri.localhost']
		}
	},
	extensions: ['.svelte', '.svx']
};

export default config;
