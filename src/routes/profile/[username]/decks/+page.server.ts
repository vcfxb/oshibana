import { error, redirect } from '@sveltejs/kit';
import { eq, sql, desc } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '$lib/server/db/schema';
import type { PageServerLoad, Actions } from './$types';
import { deckActions } from '$lib/server/deckActions';

export const load: PageServerLoad = async ({ params, platform, locals }) => {
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
		.where(eq(schema.decks.userId, profile.id))
		.orderBy(desc(schema.decks.updatedAt));

	// Get card counts for each deck
	const decksWithCounts = await Promise.all(
		userDecks.map(async (deck) => {
			const [countResult] = await ddb
				.select({ count: sql<number>`CAST(sum(${schema.deckSlots.quantity}) AS INTEGER)` })
				.from(schema.deckSlots)
				.where(eq(schema.deckSlots.deckId, deck.id));

			const [physicalCountResult] = await ddb
				.select({ count: sql<number>`count(*)` })
				.from(schema.physicalCards)
				.where(eq(schema.physicalCards.currentDeckId, deck.id));

			return {
				...deck,
				virtualCount: countResult?.count || 0,
				physicalCount: physicalCountResult?.count || 0,
				author: profile.username
			};
		})
	);

	return {
		profile,
		decks: decksWithCounts,
		isOwnProfile: locals.user?.id === profile.id
	};
};

export const actions: Actions = {
	createDeck: async (event) => {
		const result = await deckActions.createDeck(event);
		if (result && 'id' in result) {
			throw redirect(303, `/decks/${result.id}`);
		}
		return result;
	}
};
