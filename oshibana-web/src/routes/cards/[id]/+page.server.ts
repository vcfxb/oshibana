import { getCardById, getPrints, getRulings, getLanguages } from '$lib/scryfall/index';
import { error } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { drizzle } from 'drizzle-orm/d1';
import { relations } from '$lib/server/db/relations';
import type { UUID } from '$lib/scryfall/card';
import type { DbStorageLocation } from '$lib/server/db/types';

export const load: PageServerLoad = async ({ params, platform, locals }) => {
	const ddb = drizzle(platform?.env.DB, { relations });

	try {
		const card = await getCardById(params.id);
		const [prints, rulings, languages] = await Promise.all([
			getPrints(card.oracle_id as UUID),
			getRulings(card.id),
			getLanguages(card.set, card.collector_number)
		]);

		let locations: DbStorageLocation[] = [];
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
	} catch (e: any) {
		throw error(404, e.details || e.message || 'Card not found');
	}
};

export const actions: Actions = {};
