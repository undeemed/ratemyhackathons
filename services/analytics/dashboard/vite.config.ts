import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		port: 5174,
		proxy: {
			'/api': 'http://127.0.0.1:8081',
			'/main-api': {
				target: 'http://127.0.0.1:8080',
				rewrite: (path) => path.replace(/^\/main-api/, '/api')
			}
		}
	}
});
