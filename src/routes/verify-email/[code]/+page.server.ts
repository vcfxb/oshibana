import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { drizzle } from 'drizzle-orm/d1';
import { users } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { validateEmailVerificationCode, deleteEmailVerificationCode } from '$lib/server/auth';

export const load: PageServerLoad = async ({ params, platform }) => {
	const code = params.code;

	if (!code) {
		return { success: false, message: 'Invalid verification link.' };
	}

	const db = drizzle(platform!.env.DB);

	try {
		const userId = await validateEmailVerificationCode(platform!.env.DB, code);

		if (!userId) {
			return { success: false, message: 'Invalid or expired verification link.' };
		}

		await db.update(users).set({ emailVerified: true }).where(eq(users.id, userId));

		await deleteEmailVerificationCode(platform!.env.DB, userId);

		return {
			success: true,
			message: 'Your email address has been verified successfully!'
		};
	} catch (e) {
		console.error(e);
		return { success: false, message: 'An error occurred during verification.' };
	}
};
