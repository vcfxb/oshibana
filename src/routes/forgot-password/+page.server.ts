import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';
import { drizzle } from 'drizzle-orm/d1';
import { users } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { createPasswordResetToken } from '$lib/server/auth';
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
			const senderAddr = 'no-reply@oshibana.cards';
			const mimeMessage = createMimeMessage();

			mimeMessage.setSender({ name: 'oshibana.cards', addr: senderAddr });
			mimeMessage.setRecipient(user.email);
			mimeMessage.setSubject('Oshibana password reset request');
			mimeMessage.addMessage({
				contentType: 'text/html',
				data: `\
				<div style="font-family: sans-serif;">
					<h2>Password Reset</h2>
					<p>We received a request to reset your password.</p>
					<p>Click the link below to choose a new password:</p>
					<p><a href="${resetLink}">${resetLink}</a></p>
					<p>If you did not request this, please safely ignore this email.</p>
				</div>
				`.split('\n').map((s) => s.trimStart()).join('\n') // weird hack for making it look less weird
			});

			// Weird hacky workaround for import problems.
			let EmailMessage;
			try {
				const cfEmail = await import('cloudflare:email');
				EmailMessage = cfEmail.EmailMessage;
			} catch {
				EmailMessage = class EmailMessage {
					constructor (public sender: string, public recipient: string, public raw: string) {}
				};
			}


			const emailMessage = new EmailMessage(senderAddr, user.email, mimeMessage.asRaw());

			if (platform?.env.PASSWORD_RESETS?.send) {
				await platform?.env.PASSWORD_RESETS.send(emailMessage);
			} else {
				console.log(`email:\n${mimeMessage.asRaw()}`);
			}

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
