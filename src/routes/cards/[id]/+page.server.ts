import { getCardById, getPrints, getRulings } from '$lib/scryfall';
import { addCardToCollection, getStorageLocations } from '$lib/server/collection';
import { fail } from '@sveltejs/kit';
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
		return {
			error: e instanceof Error ? e.message : 'Unknown error'
		};
	}
};

export const actions: Actions = {
	addToCollection: async ({ params, request, platform, locals }) => {
		if (!locals.user) {
			return fail(401, { message: 'You must be logged in to add cards to your collection' });
		}

		const db = platform?.env.DB;
		if (!db) return fail(500, { message: 'Database not found' });

		const formData = await request.formData();
		const condition = (formData.get('condition') as any) || 'NM';
		const isFoil = formData.get('isFoil') === 'true';
		const storageLocationId = (formData.get('storageLocationId') as string) || null;

		try {
			await addCardToCollection(db, locals.user.id, params.id, {
				condition,
				isFoil,
				storageLocationId: storageLocationId === 'none' ? undefined : storageLocationId || undefined
			});
			return { success: true };
		} catch (e) {
			return fail(500, { message: e instanceof Error ? e.message : 'Failed to add card' });
		}
	}
};
