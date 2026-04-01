import { searchCards } from '$lib/scryfall/index';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
	const q = url.searchParams.get('q');
	const page = parseInt(url.searchParams.get('page') || '1');
	const unique = (url.searchParams.get('unique') as any) || undefined;
	const order = (url.searchParams.get('order') as any) || undefined;
	const dir = (url.searchParams.get('dir') as any) || undefined;

	if (q) {
		try {
			const results = await searchCards({ q, page, unique, order, dir });

			if (results.total_cards === 1 && results.data.length === 1 && page === 1) {
				throw redirect(302, `/cards/${results.data[0].id}`);
			}

			return {
				results,
				q,
				page,
				unique,
				order,
				dir
			};
		} catch (e: any) {
			if (e && typeof e === 'object' && 'status' in e && e.status === 302) {
				throw e;
			}

			return {
				error: e.details || e.message || 'Unknown error',
				q,
				page,
				unique,
				order,
				dir
			};
		}
	}

	return {
		q: '',
		page: 1,
		unique: undefined,
		order: undefined,
		dir: undefined
	};
};
