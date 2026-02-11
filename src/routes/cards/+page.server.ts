import { searchCards } from '$lib/scryfall';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
	const q = url.searchParams.get('q');

	if (q) {
		try {
			const results = await searchCards(q);

			if (results.total_cards === 1 && results.data.length === 1) {
				throw redirect(302, `/cards/${results.data[0].id}`);
			}

			return {
				results,
				q
			};
		} catch (e) {
			if (e && typeof e === 'object' && 'status' in e && e.status === 302) {
				throw e;
			}
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
