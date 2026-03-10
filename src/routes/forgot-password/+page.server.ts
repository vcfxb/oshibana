import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';
import { drizzle } from 'drizzle-orm/d1';
import { users } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { createPasswordResetToken } from '$lib/server/auth';

export const actions: Actions = {
	default: async ({ request, platform, url }) => {
		const formData = await request.formData();
		const email = formData.get('email');

		if (typeof email !== 'string' || !email) {
			return fail(400, { message: 'Email is required' });
		}

		const db = drizzle(platform!.env.DB);
		const user = await db.select().from(users).where(eq(users.email, email)).get();

		if (!user) {
			// Don't reveal if user exists or not
			return {
				success: true,
				message: "If an account exists with that email, you'll receive a reset link shortly."
			};
		}

		try {
			const token = await createPasswordResetToken(platform!.env.DB, user.id);
			const resetLink = `${url.origin}/reset-password/${token}`;

			// In a real app, send this via email. For now, we'll log it.
			console.log(`Password reset link for ${user.email}: ${resetLink}`);

			return {
				success: true,
				message: "If an account exists with that email, you'll receive a reset link shortly."
			};
		} catch (e) {
			console.error(e);
			return fail(500, { message: 'An error occurred while processing your request.' });
		}
	}
};
