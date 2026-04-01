import { error, fail } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { decks, physicalCards, follows } from '$lib/server/db/schema';
import { drizzle } from 'drizzle-orm/d1';
import { eq, and } from 'drizzle-orm';
import { relations } from '$lib/server/db/relations';

export const load: PageServerLoad = async ({ params, platform, locals }) => {
	const { username } = params;

	if (!platform?.env?.DB) {
		throw error(500, 'Database connection not available');
	}

	const db = drizzle(platform.env.DB, { relations });

	const user = await db.query.users.findFirst({
		where: {
			username: username,
		}
	});

	if (!user) {
		throw error(404, 'User not found');
	}

	const getFollowingRelation = async () => {
		if (locals.user) {
			return db.query.follows.findFirst({ 
				where: {
					followerId: locals.user.id,
					followingId: user.id,
				}
			});
		} else {
			return null;
		}
	};

	const [userDecks, cardCount, followerCount, followingCount, isFollowing] = await Promise.all([
		db.select().from(decks).where(eq(decks.userId, user.id)).all(),
		db.$count(physicalCards, eq(physicalCards.userId, user.id)),
		db.$count(follows, eq(follows.followingId, user.id)),
		db.$count(follows, eq(follows.followerId, user.id)),
		getFollowingRelation().then((res) => !!res), // confirm truthy value
	]);

	return {
		profile: user,
		decks: userDecks,
		stats: {
			deckCount: userDecks.length,
			cardCount,
			followerCount,
			followingCount
		},
		isFollowing
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
		const db = drizzle(platform.env.DB, { relations });

		const targetUser = await db.query.users.findFirst({ where: { username } });

		if (!targetUser || targetUser.id === locals.user.id) {
			return fail(400, { message: 'Invalid user' });
		}

		try {
			const existing = await db.query.follows.findFirst({
				where: { followerId: locals.user.id, followingId: targetUser.id }
			});

			if (existing) {
				await db
					.delete(follows)
					.where(
						and(eq(follows.followerId, locals.user.id), eq(follows.followingId, targetUser.id))
					)
					.limit(1);
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
