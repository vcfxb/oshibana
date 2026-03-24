import { fail, redirect, error } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { invalidateSession } from '$lib/server/auth';
import { users, decks, physicalCards } from '$lib/server/db/schema';
import { drizzle } from 'drizzle-orm/d1';
import { count, sql } from 'drizzle-orm';

export const load: PageServerLoad = async ({ platform }) => {
	if (!platform?.env?.DB) {
		throw error(500, 'Database connection not available');
	}

	const ddb = drizzle(platform.env.DB);

	try {
		const [userCount, deckCount, cardStats] = await Promise.all([
			ddb
				.select({ count: count() })
				.from(users)
				.get(),
			ddb
				.select({ count: count() })
				.from(decks)
				.get(),
			ddb
				.select({ totalCards: sql<number>`SUM(quantity)` })
				.from(physicalCards)
				.get()
		]);

		return {
			stats: {
				users: userCount?.count ?? 0,
				decks: deckCount?.count ?? 0,
				cards: cardStats?.totalCards ?? 0
			}
		};
	} catch (e) {
		console.error('Failed to fetch landing page stats:', e);
		return {
			stats: {
				users: 0,
				decks: 0,
				cards: 0
			}
		};
	}
};

export const actions: Actions = {
	logout: async ({ cookies, platform, locals }) => {
		if (!locals.session) {
			return fail(401);
		}
		await invalidateSession(platform!.env.DB, locals.session.id);
		cookies.delete('session_id', { path: '/' });
		throw redirect(302, '/login');
	}
};
