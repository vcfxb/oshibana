import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';
import { drizzle } from 'drizzle-orm/d1';
import { users } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { createPasswordResetToken } from '$lib/server/auth';
import { env } from 'cloudflare:workers';
import { EmailMessage } from 'cloudflare:email';
import { createMimeMessage } from 'mimetext';

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
			const senderAddr = "no-reply@oshibana.cards";

			const mimeMessage = createMimeMessage();

			mimeMessage.setSender({ name: "oshibana.cards", addr: senderAddr });
			mimeMessage.setRecipient(user.email);
			mimeMessage.setSubject("Oshibana password reset request");
			mimeMessage.addMessage({
				contentType: 'text/html',
				data: `
				<div style="font-family: sans-serif;">
					<h2>Password Reset</h2>
					<p>We received a request to reset your password.</p>
					<p>Click the link below to choose a new password:</p>
					<p><a href="${resetLink}">${resetLink}</a></p>
					<p>If you did not request this, please safely ignore this email.</p>
				</div>
				`,
			});

			const emailMessage = new EmailMessage(senderAddr, user.email, mimeMessage.asRaw());

			await env.PASSWORD_RESETS.send(emailMessage);

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
