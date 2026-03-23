import { getCardById, getPrints, getRulings, getLanguages } from '$lib/scryfall';
import { error } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { drizzle } from 'drizzle-orm/d1';
import { relations } from '$lib/server/db/relations';

export const load: PageServerLoad = async ({ params, platform, locals }) => {
	const ddb = drizzle(platform?.env.DB, { relations });

	try {
		const card = await getCardById(params.id);
		const [prints, rulings, languages] = await Promise.all([
			getPrints(card.oracle_id),
			getRulings(card.id),
			getLanguages(card.set, card.collector_number).catch(() => ({ data: [] }))
		]);

		let locations: any[] = [];
		if (locals.user && platform?.env.DB) {
			locations = await ddb.query.storageLocations.findMany({ where: { userId: locals.user.id } });
		}

		return {
			card,
			prints,
			rulings,
			languages,
			locations
		};
	} catch (e) {
		throw error(404, e instanceof Error ? e.message : 'Card not found');
	}
};

export const actions: Actions = {};
