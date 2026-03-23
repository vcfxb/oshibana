import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { invalidateSession } from '$lib/server/auth';
import { getRandomCard } from '$lib/scryfall';

export const load: PageServerLoad = async () => {
	// Fetch a few random cards for the landing page
	// We use Promise.all to fetch them in parallel
	try {
		const featuredCards = await Promise.all([
			getRandomCard(),
			getRandomCard(),
			getRandomCard(),
			getRandomCard(),
			getRandomCard()
		]);
		return {
			featuredCards
		};
	} catch (e) {
		console.error('Failed to fetch featured cards:', e);
		return {
			featuredCards: []
		};
	}
};

export const actions: Actions = {
	logout: async ({ cookies, platform, locals }) => {
		if (!locals.session) {
			return fail(401);
		}
		await invalidateSession(platform!.env.DB, locals.session.id);
		cookies.delete('session_id', { path: '/' });
		throw redirect(302, '/login');
	}
};
