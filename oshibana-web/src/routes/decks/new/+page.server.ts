import { error, redirect } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '$lib/server/db/schema';
import type { Actions, PageServerLoad } from './$types';
import { deckActions } from '$lib/server/deckActions';

export const load: PageServerLoad = async ({ locals, platform }) => {
	if (!locals.user) {
		throw redirect(302, '/login?redirectTo=/decks/new');
	}

	const db = platform?.env.DB;
	if (!db) throw error(500, 'Database not found');

	const ddb = drizzle(db);
	const storageLocations = await ddb
		.select()
		.from(schema.storageLocations)
		.where(eq(schema.storageLocations.userId, locals.user.id));

	return {
		storageLocations
	};
};

export const actions: Actions = {
	default: async (event) => {
		const result = await deckActions.createDeck(event);
		if (result && 'id' in result) {
			throw redirect(303, `/decks/${result.id}`);
		}
		return result;
	}
};
