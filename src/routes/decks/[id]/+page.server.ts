import { error } from '@sveltejs/kit';
import { eq, sql } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '$lib/server/db/schema';
import type { PageServerLoad, Actions } from './$types';
import { getDeck, getDeckSlots, getDeckHistory } from '$lib/server/db/queries/decks';
import { deckActions } from '$lib/server/deckActions';

export const load: PageServerLoad = async ({ params, platform, locals }) => {
	const db = platform?.env.DB;
	if (!db) throw error(500, 'Database not found');

	const deckId = params.id;
	const deck = await getDeck(db, deckId);

	if (!deck) throw error(404, 'Deck not found');

	const ddb = drizzle(db);
	const [author] = await ddb
		.select({ username: schema.users.username, id: schema.users.id })
		.from(schema.users)
		.where(eq(schema.users.id, deck.userId))
		.limit(1);

	const slots = await getDeckSlots(db, deckId);
	const history = await getDeckHistory(db, deckId);

	// Fetch author data for the history as well (or at least ensure it's loaded)
	// Actually slots already have card cache data from getDeckSlots.

	const isOwner = locals.user?.id === deck.userId;

	return {
		deck,
		author: author || { username: 'Unknown', id: '' },
		slots,
		history,
		isOwner
	};
};

export const actions: Actions = {
	...deckActions
};
