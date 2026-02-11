import type { Env } from './worker-configuration';

declare global {
	namespace App {
		interface Platform {
			env: Env;
			ctx: ExecutionContext;
			caches: CacheStorage;
			cf?: IncomingRequestCfProperties;
		}
		interface Locals {
			user: import('./lib/server/db/schema').users.$inferSelect | null;
			session: import('./lib/server/db/schema').sessions.$inferSelect | null;
		}
	}
	const __APP_VERSION__: string;
}

export {};
