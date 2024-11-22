import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port: 5173,
		strictPort: true,
		watch: {
			// Watch for changes in these directories
			ignored: ['**/node_modules/**', '**/dist/**'],
		},
		hmr: {
			protocol: 'ws',
			host: 'localhost',
		},
	},
	// Vite options tailored for Tauri development
	clearScreen: false,
	// to make use of `TAURI_DEBUG` and other env variables
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		// Tauri uses Chromium on Windows and WebKit on macOS and Linux
		target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
		// don't minify for debug builds
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		// produce sourcemaps for debug builds
		sourcemap: !!process.env.TAURI_DEBUG
	}
});
