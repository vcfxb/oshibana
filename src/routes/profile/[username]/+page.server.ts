import { error, fail } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { users, decks, physicalCards, follows } from '$lib/server/db/schema';
import { drizzle } from 'drizzle-orm/d1';
import { eq, sql, and } from 'drizzle-orm';

export const load: PageServerLoad = async ({ params, platform, locals }) => {
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

	const [userDecks, cardCount, followerCount, followingCount, isFollowing] = await Promise.all([
		db.select().from(decks).where(eq(decks.userId, user.id)).all(),
		db
			.select({ count: sql<number>`count(*)` })
			.from(physicalCards)
			.where(eq(physicalCards.userId, user.id))
			.get(),
		db
			.select({ count: sql<number>`count(*)` })
			.from(follows)
			.where(eq(follows.followingId, user.id))
			.get(),
		db
			.select({ count: sql<number>`count(*)` })
			.from(follows)
			.where(eq(follows.followerId, user.id))
			.get(),
		locals.user
			? db
					.select()
					.from(follows)
					.where(and(eq(follows.followerId, locals.user.id), eq(follows.followingId, user.id)))
					.get()
			: Promise.resolve(null)
	]);

	return {
		profile: user,
		decks: userDecks,
		stats: {
			deckCount: userDecks.length,
			cardCount: cardCount?.count ?? 0,
			followerCount: followerCount?.count ?? 0,
			followingCount: followingCount?.count ?? 0
		},
		isFollowing: !!isFollowing
	};
};

export const actions: Actions = {
	toggleFollow: async ({ locals, platform, params }) => {
		if (!locals.user) {
			return fail(401, { message: 'Unauthorized' });
		}

		if (!platform?.env?.DB) {
			return fail(500, { message: 'Database connection not available' });
		}

		const { username } = params;
		const db = drizzle(platform.env.DB);

		const targetUser = await db.select().from(users).where(eq(users.username, username)).get();

		if (!targetUser || targetUser.id === locals.user.id) {
			return fail(400, { message: 'Invalid user' });
		}

		try {
			const existing = await db
				.select()
				.from(follows)
				.where(and(eq(follows.followerId, locals.user.id), eq(follows.followingId, targetUser.id)))
				.get();

			if (existing) {
				await db
					.delete(follows)
					.where(
						and(eq(follows.followerId, locals.user.id), eq(follows.followingId, targetUser.id))
					);
			} else {
				await db.insert(follows).values({
					followerId: locals.user.id,
					followingId: targetUser.id
				});
			}

			return { success: true };
		} catch (e) {
			console.error(e);
			return fail(500, { message: 'Failed to toggle follow' });
		}
	}
};
