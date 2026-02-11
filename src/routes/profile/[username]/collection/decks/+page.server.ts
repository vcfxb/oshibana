import { error } from '@sveltejs/kit';
import { eq, sql } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '$lib/server/db/schema';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, platform }) => {
	const db = platform?.env.DB;
	if (!db) throw error(500, 'Database not found');

	const ddb = drizzle(db);
	const [profile] = await ddb
		.select()
		.from(schema.users)
		.where(eq(schema.users.username, params.username))
		.limit(1);

	if (!profile) throw error(404, 'User not found');

	const userDecks = await ddb
		.select()
		.from(schema.decks)
		.where(eq(schema.decks.userId, profile.id));

	// Get card counts for each deck
	const decksWithCounts = await Promise.all(
		userDecks.map(async (deck) => {
			const [countResult] = await ddb
				.select({ count: sql<number>`count(*)` })
				.from(schema.physicalCards)
				.where(eq(schema.physicalCards.currentDeckId, deck.id));
			return {
				...deck,
				cardCount: countResult?.count || 0
			};
		})
	);

	return {
		profile,
		decks: decksWithCounts
	};
};
