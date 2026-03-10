import { eq } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '../server/db/schema';

// --- Password Hashing (PBKDF2) ---

const PBKDF2_ITERATIONS = 100000;
const SALT_SIZE = 16;
const KEY_SIZE = 64;

export async function hashPassword(password: string): Promise<string> {
	const salt = crypto.getRandomValues(new Uint8Array(SALT_SIZE));
	const passwordBuffer = new TextEncoder().encode(password);

	const baseKey = await crypto.subtle.importKey('raw', passwordBuffer, 'PBKDF2', false, [
		'deriveBits',
		'deriveKey'
	]);

	const derivedKey = await crypto.subtle.deriveBits(
		{
			name: 'PBKDF2',
			salt,
			iterations: PBKDF2_ITERATIONS,
			hash: 'SHA-512'
		},
		baseKey,
		KEY_SIZE * 8
	);

	const combined = new Uint8Array(SALT_SIZE + KEY_SIZE);
	combined.set(salt);
	combined.set(new Uint8Array(derivedKey), SALT_SIZE);

	return btoa(String.fromCharCode(...combined));
}

export async function verifyPassword(password: string, hash: string): Promise<boolean> {
	const combined = new Uint8Array(
		atob(hash)
			.split('')
			.map((c) => c.charCodeAt(0))
	);
	const salt = combined.slice(0, SALT_SIZE);
	const originalKey = combined.slice(SALT_SIZE);

	const passwordBuffer = new TextEncoder().encode(password);
	const baseKey = await crypto.subtle.importKey('raw', passwordBuffer, 'PBKDF2', false, [
		'deriveBits',
		'deriveKey'
	]);

	const derivedKey = await crypto.subtle.deriveBits(
		{
			name: 'PBKDF2',
			salt,
			iterations: PBKDF2_ITERATIONS,
			hash: 'SHA-512'
		},
		baseKey,
		KEY_SIZE * 8
	);

	const newKey = new Uint8Array(derivedKey);
	if (newKey.length !== originalKey.length) return false;

	// Constant-time comparison
	let result = 0;
	for (let i = 0; i < newKey.length; i++) {
		result |= newKey[i] ^ originalKey[i];
	}
	return result === 0;
}

// --- Session Management ---

const SESSION_EXPIRATION_MS = 1000 * 60 * 60 * 24 * 30; // 30 days

export async function createSession(db: D1Database, userId: string): Promise<string> {
	const sessionId = crypto.randomUUID();
	const expiresAt = new Date(Date.now() + SESSION_EXPIRATION_MS);

	const ddb = drizzle(db);
	await ddb.insert(schema.sessions).values({
		id: sessionId,
		userId,
		expiresAt: expiresAt.getTime()
	});

	return sessionId;
}

export async function validateSession(db: D1Database, sessionId: string) {
	const ddb = drizzle(db);
	const [result] = await ddb
		.select({
			user: schema.users,
			session: schema.sessions
		})
		.from(schema.sessions)
		.innerJoin(schema.users, eq(schema.sessions.userId, schema.users.id))
		.where(eq(schema.sessions.id, sessionId))
		.limit(1);

	if (!result) return { user: null, session: null };

	const { user, session } = result;

	if (Date.now() >= session.expiresAt) {
		await ddb.delete(schema.sessions).where(eq(schema.sessions.id, session.id));
		return { user: null, session: null };
	}

	// Extend session if it's close to expiring (optional)
	if (session.expiresAt - Date.now() < SESSION_EXPIRATION_MS / 2) {
		const newExpiresAt = new Date(Date.now() + SESSION_EXPIRATION_MS);
		await ddb
			.update(schema.sessions)
			.set({ expiresAt: newExpiresAt.getTime() })
			.where(eq(schema.sessions.id, session.id));
	}

	return { user, session };
}

export async function invalidateSession(db: D1Database, sessionId: string) {
	const ddb = drizzle(db);
	await ddb.delete(schema.sessions).where(eq(schema.sessions.id, sessionId));
}

// --- Password Reset ---

const RESET_TOKEN_EXPIRATION_MS = 1000 * 60 * 60 * 2; // 2 hours

export async function createPasswordResetToken(db: D1Database, userId: string): Promise<string> {
	const ddb = drizzle(db);
	// Delete any existing tokens for this user
	await ddb.delete(schema.passwordResetTokens).where(eq(schema.passwordResetTokens.userId, userId));

	const token = crypto.randomUUID();
	const tokenHash = await hashToken(token);
	const expiresAt = new Date(Date.now() + RESET_TOKEN_EXPIRATION_MS);

	await ddb.insert(schema.passwordResetTokens).values({
		tokenHash,
		userId,
		expiresAt: expiresAt.getTime()
	});

	return token;
}

export async function validatePasswordResetToken(
	db: D1Database,
	token: string
): Promise<string | null> {
	const ddb = drizzle(db);
	const tokenHash = await hashToken(token);

	const [result] = await ddb
		.select()
		.from(schema.passwordResetTokens)
		.where(eq(schema.passwordResetTokens.tokenHash, tokenHash))
		.limit(1);

	if (!result) return null;

	if (Date.now() >= result.expiresAt) {
		await ddb
			.delete(schema.passwordResetTokens)
			.where(eq(schema.passwordResetTokens.tokenHash, tokenHash));
		return null;
	}

	return result.userId;
}

export async function deletePasswordResetToken(db: D1Database, token: string) {
	const ddb = drizzle(db);
	const tokenHash = await hashToken(token);
	await ddb
		.delete(schema.passwordResetTokens)
		.where(eq(schema.passwordResetTokens.tokenHash, tokenHash));
}

async function hashToken(token: string): Promise<string> {
	const data = new TextEncoder().encode(token);
	const hashBuffer = await crypto.subtle.digest('SHA-256', data);
	return Array.from(new Uint8Array(hashBuffer))
		.map((b) => b.toString(16).padStart(2, '0'))
		.join('');
}
