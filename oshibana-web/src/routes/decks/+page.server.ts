import { error, redirect } from '@sveltejs/kit';
import { eq, sql, desc } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '$lib/server/db/schema';
import type { PageServerLoad, Actions } from './$types';
import { deckActions } from '$lib/server/deckActions';

export const load: PageServerLoad = async ({ platform, locals }) => {
	const db = platform?.env.DB;
	if (!db) throw error(500, 'Database not found');

	const ddb = drizzle(db);

	// If logged in, show my decks. If not, show all public decks?
	// For now, let's just show all decks with a filter for "mine".

	let query = ddb.select().from(schema.decks).orderBy(desc(schema.decks.updatedAt)).limit(50);

	if (locals.user) {
		// Maybe prioritizing my decks? Or just showing all for now.
		// Let's just show all for now.
	}

	const allDecks = await query;

	// Get card counts for each deck
	const decksWithCounts = await Promise.all(
		allDecks.map(async (deck) => {
			const [countResult] = await ddb
				.select({ count: sql<number>`CAST(sum(${schema.deckSlots.quantity}) AS INTEGER)` })
				.from(schema.deckSlots)
				.where(eq(schema.deckSlots.deckId, deck.id));

			const [physicalCountResult] = await ddb
				.select({ count: sql<number>`count(*)` })
				.from(schema.physicalCards)
				.innerJoin(
					schema.storageLocations,
					eq(schema.physicalCards.storageLocationId, schema.storageLocations.id)
				)
				.where(eq(schema.storageLocations.trackedDeckId, deck.id));

			const [author] = await ddb
				.select({ username: schema.users.username })
				.from(schema.users)
				.where(eq(schema.users.id, deck.userId))
				.limit(1);

			return {
				...deck,
				virtualCount: countResult?.count || 0,
				physicalCount: physicalCountResult?.count || 0,
				author: author?.username || 'Unknown'
			};
		})
	);

	return {
		decks: decksWithCounts
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
