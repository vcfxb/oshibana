import { createMimeMessage } from 'mimetext';

export interface EmailOptions {
	senderName?: string;
	senderAddr?: string;
	recipient: string;
	subject: string;
	html: string;
}

export async function sendEmail(platform: App.Platform | undefined, options: EmailOptions) {
	const {
		senderName = 'oshibana.cards',
		senderAddr = 'no-reply@oshibana.cards',
		recipient,
		subject,
		html
	} = options;

	const mimeMessage = createMimeMessage();

	mimeMessage.setSender({ name: senderName, addr: senderAddr });
	mimeMessage.setRecipient(recipient);
	mimeMessage.setSubject(subject);
	mimeMessage.addMessage({
		contentType: 'text/html',
		data: html
	});

	// Cloudflare Email Message hack
	let EmailMessage;
	try {
		const moduleName = 'cloudflare:email';
		const cfEmail = await import(/* @vite-ignore */ moduleName);
		EmailMessage = cfEmail.EmailMessage;
	} catch {
		EmailMessage = class EmailMessage {
			constructor(
				public sender: string,
				public recipient: string,
				public raw: string
			) {}
		};
	}

	const emailMessage = new EmailMessage(senderAddr, recipient, mimeMessage.asRaw());

	if (import.meta.env.DEV) {
		console.log(
			`--- Email Debug ---\nSubject: ${subject}\nTo: ${recipient}\nContent:\n${html}\n------------------`
		);
	}

	if (platform?.env?.NOREPLY_EMAIL?.send) {
		await platform.env.NOREPLY_EMAIL.send(emailMessage);
	} else if (!import.meta.env.DEV) {
		console.log(
			`--- Email Log (No Binding) ---\nSubject: ${subject}\nTo: ${recipient}\n------------------`
		);
	}
}

export async function sendVerificationEmail(
	platform: App.Platform | undefined,
	email: string,
	verifyLink: string
) {
	await sendEmail(platform, {
		recipient: email,
		subject: 'Verify your email address - Oshibana',
		html: `
			<div style="font-family: sans-serif;">
				<h2>Welcome to Oshibana!</h2>
				<p>Please click the link below to verify your email address and complete your registration:</p>
				<p><a href="${verifyLink}">${verifyLink}</a></p>
				<p>If you did not create an account, please ignore this email.</p>
			</div>
		`
			.split('\n')
			.map((s) => s.trimStart())
			.join('\n')
	});
}
