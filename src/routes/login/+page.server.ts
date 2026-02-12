import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { verifyPassword, createSession } from '$lib/server/auth';
import { users } from '$lib/server/db/schema';
import { drizzle } from 'drizzle-orm/d1';
import { eq, or } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
	if (locals.user) {
		throw redirect(302, '/');
	}
	return {};
};

export const actions: Actions = {
	default: async ({ request, platform, cookies }) => {
		const formData = await request.formData();
		const identifier = formData.get('identifier'); // Email or username
		const password = formData.get('password');

		if (
			typeof identifier !== 'string' ||
			typeof password !== 'string' ||
			!identifier ||
			!password
		) {
			return fail(400, { message: 'All fields are required' });
		}

		const db = drizzle(platform!.env.DB);

		const user = await db
			.select()
			.from(users)
			.where(or(eq(users.email, identifier), eq(users.username, identifier)))
			.get();

		if (!user || !user.passwordHash) {
			return fail(400, { message: 'Invalid credentials' });
		}

		const valid = await verifyPassword(password, user.passwordHash);

		if (!valid) {
			return fail(400, { message: 'Invalid credentials' });
		}

		const sessionId = await createSession(platform!.env.DB, user.id);
		cookies.set('session_id', sessionId, {
			path: '/',
			httpOnly: true,
			sameSite: 'lax',
			secure: import.meta.env.PROD,
			maxAge: 60 * 60 * 24 * 30
		});

		throw redirect(303, '/');
	}
};
