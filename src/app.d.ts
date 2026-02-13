import type { Env } from './worker-configuration';
import type { DbUser, DbSession } from '$lib/server/db/types';


declare global {
	namespace App {
		interface Platform {
			env: Env;
			ctx: ExecutionContext;
			caches: CacheStorage;
			cf?: IncomingRequestCfProperties;
		}
		interface Locals {
			user: DbUser | null;
			session: DbSession | null;
		}
	}
	const __APP_VERSION__: string;
}

// to make weird raw svg imports work ig
declare module '*?raw' {
	const content: string;
	export default content;
}

export {};
