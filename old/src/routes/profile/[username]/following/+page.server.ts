import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { users, follows } from '$lib/server/db/schema';
import { drizzle } from 'drizzle-orm/d1';
import { eq, sql, desc } from 'drizzle-orm';

export const load: PageServerLoad = async ({ params, platform, locals }) => {
	const { username } = params;

	if (!platform?.env?.DB) {
		throw error(500, 'Database connection not available');
	}

	const db = drizzle(platform.env.DB);

	const profileUser = await db
		.select({
			id: users.id,
			username: users.username
		})
		.from(users)
		.where(eq(users.username, username))
		.get();

	if (!profileUser) {
		throw error(404, 'User not found');
	}

	try {
		const followingList = await db
			.select({
				id: users.id,
				username: users.username,
				createdAt: users.createdAt,
				deckCount: sql<number>`(SELECT COUNT(*) FROM decks WHERE decks.user_id = ${users.id})`,
				cardCount: sql<number>`(SELECT COUNT(*) FROM physical_cards WHERE physical_cards.user_id = ${users.id})`,
				followerCount: sql<number>`(SELECT COUNT(*) FROM follows f2 WHERE f2.following_id = ${users.id})`,
				isFollowing: locals.user
					? sql<boolean>`EXISTS(SELECT 1 FROM follows WHERE follows.follower_id = ${locals.user.id} AND follows.following_id = ${users.id})`
					: sql<boolean>`false`
			})
			.from(follows)
			.innerJoin(users, eq(follows.followingId, users.id))
			.where(eq(follows.followerId, profileUser.id))
			.orderBy(desc(follows.createdAt))
			.all();

		return {
			profileUser,
			following: followingList
		};
	} catch (e) {
		console.error(e);
		throw error(500, 'Failed to load following');
	}
};
