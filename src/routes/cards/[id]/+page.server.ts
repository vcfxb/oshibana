import { getCardById, getPrints, getRulings } from '$lib/scryfall';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	try {
		const card = await getCardById(params.id);
		const [prints, rulings] = await Promise.all([getPrints(card.oracle_id), getRulings(card.id)]);
		return {
			card,
			prints,
			rulings
		};
	} catch (e) {
		return {
			error: e instanceof Error ? e.message : 'Unknown error'
		};
	}
};
