import type { AuthObject } from 'svelte-clerk/server';

declare global {
	namespace App {
		interface Locals {
			auth: () => AuthObject;
		}
	}
}

export {};
