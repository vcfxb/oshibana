import { error, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { deckActions } from '$lib/server/deckActions';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(302, '/login?redirectTo=/decks/new');
	}
	return {};
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
