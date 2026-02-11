import { searchCards } from '$lib/scryfall';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
	const q = url.searchParams.get('q');

	if (q) {
		try {
			const results = await searchCards(q);
			return {
				results,
				q
			};
		} catch (e) {
			return {
				error: e instanceof Error ? e.message : 'Unknown error',
				q
			};
		}
	}

	return {
		q: ''
	};
};
