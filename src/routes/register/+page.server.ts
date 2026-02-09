import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { hashPassword, createSession } from '$lib/server/auth';
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
		const username = formData.get('username');
		const email = formData.get('email');
		const password = formData.get('password');

		if (
			typeof username !== 'string' ||
			typeof email !== 'string' ||
			typeof password !== 'string' ||
			!username ||
			!email ||
			!password
		) {
			return fail(400, { message: 'All fields are required' });
		}

		if (username.length < 3 || username.length > 31) {
			return fail(400, { message: 'Username must be between 3 and 31 characters' });
		}

		if (password.length < 6) {
			return fail(400, { message: 'Password must be at least 6 characters' });
		}

		const db = drizzle(platform!.env.DB);

		// Check if user already exists
		const existingUser = await db
			.select()
			.from(users)
			.where(or(eq(users.email, email), eq(users.username, username)))
			.get();

		if (existingUser) {
			return fail(400, { message: 'Email or username already taken' });
		}

		try {
			const passwordHash = await hashPassword(password);
			const userId = crypto.randomUUID();

			await db.insert(users).values({
				id: userId,
				username,
				email,
				passwordHash
			});

			const sessionId = await createSession(platform!.env.DB, userId);
			cookies.set('session_id', sessionId, {
				path: '/',
				httpOnly: true,
				sameSite: 'lax',
				secure: import.meta.env.PROD,
				maxAge: 60 * 60 * 24 * 30 // 30 days
			});
		} catch (e) {
			console.error(e);
			return fail(500, { message: 'An error occurred during registration' });
		}

		throw redirect(302, '/');
	}
};
