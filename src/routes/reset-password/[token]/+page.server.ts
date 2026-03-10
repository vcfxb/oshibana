import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { drizzle } from 'drizzle-orm/d1';
import { users } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import {
	validatePasswordResetToken,
	deletePasswordResetToken,
	hashPassword
} from '$lib/server/auth';

export const load: PageServerLoad = async ({ params, platform }) => {
	const userId = await validatePasswordResetToken(platform!.env.DB, params.token);
	if (!userId) {
		return { valid: false };
	}
	return { valid: true };
};

export const actions: Actions = {
	default: async ({ request, platform, params }) => {
		const formData = await request.formData();
		const password = formData.get('password');
		const confirm = formData.get('confirm');

		if (typeof password !== 'string' || typeof confirm !== 'string' || !password || !confirm) {
			return fail(400, { message: 'All fields are required' });
		}

		if (password !== confirm) {
			return fail(400, { message: 'Passwords do not match' });
		}

		if (password.length < 8) {
			return fail(400, { message: 'Password must be at least 8 characters' });
		}

		const db = drizzle(platform!.env.DB);
		const userId = await validatePasswordResetToken(platform!.env.DB, params.token);

		if (!userId) {
			return fail(400, { message: 'Invalid or expired token' });
		}

		try {
			const passwordHash = await hashPassword(password);
			await db.update(users).set({ passwordHash }).where(eq(users.id, userId));

			// Clean up token
			await deletePasswordResetToken(platform!.env.DB, params.token);
		} catch (e) {
			console.error(e);
			return fail(500, { message: 'An error occurred while resetting your password.' });
		}

		throw redirect(303, '/login?resetSuccess=true');
	}
};
