import { getCardById, getPrints, getRulings } from '$lib/scryfall';
import { getStorageLocations } from '$lib/server/collection';
import { error } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';

export const load: PageServerLoad = async ({ params, platform, locals }) => {
	try {
		const card = await getCardById(params.id);
		const [prints, rulings] = await Promise.all([getPrints(card.oracle_id), getRulings(card.id)]);

		let locations: any[] = [];
		if (locals.user && platform?.env.DB) {
			locations = await getStorageLocations(platform.env.DB, locals.user.id);
		}

		return {
			card,
			prints,
			rulings,
			locations
		};
	} catch (e) {
		throw error(404, e instanceof Error ? e.message : 'Card not found');
	}
};

export const actions: Actions = {};
