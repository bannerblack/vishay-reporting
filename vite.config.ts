import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	
	// Optimize for Tauri
	clearScreen: false,
	server: {
		port: 5173,
		strictPort: true,
		host: '0.0.0.0',
		// Tauri expects a fixed port, fail if that port is not available
		hmr: {
			protocol: 'ws',
			host: 'localhost',
			port: 5173,
		},
		watch: {
			ignored: ['**/src-tauri/**']
		}
	},
	// Prevent vite from obscuring rust errors
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		// Tauri uses Chromium on Windows and WebKit on macOS and Linux
		target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
		// Don't minify for debug builds
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		// Produce sourcemaps for debug builds
		sourcemap: !!process.env.TAURI_DEBUG,
	},
});
