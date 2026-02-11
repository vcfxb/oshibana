import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { users, decks, physicalCards } from '$lib/server/db/schema';
import { drizzle } from 'drizzle-orm/d1';
import { eq, sql } from 'drizzle-orm';

export const load: PageServerLoad = async ({ params, platform }) => {
	const { username } = params;

	if (!platform?.env?.DB) {
		throw error(500, 'Database connection not available');
	}

	const db = drizzle(platform.env.DB);

	const user = await db
		.select({
			id: users.id,
			username: users.username,
			createdAt: users.createdAt
		})
		.from(users)
		.where(eq(users.username, username))
		.get();

	if (!user) {
		throw error(404, 'User not found');
	}

	const [userDecks, cardCount] = await Promise.all([
		db.select().from(decks).where(eq(decks.userId, user.id)).all(),
		db
			.select({ count: sql<number>`count(*)` })
			.from(physicalCards)
			.where(eq(physicalCards.userId, user.id))
			.get()
	]);

	return {
		profile: user,
		decks: userDecks,
		stats: {
			deckCount: userDecks.length,
			cardCount: cardCount?.count ?? 0
		}
	};
};
