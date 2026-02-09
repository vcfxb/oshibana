import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { invalidateSession } from '$lib/server/auth';

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
