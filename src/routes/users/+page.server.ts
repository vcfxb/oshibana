import { error, fail } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { users, follows } from '$lib/server/db/schema';
import { drizzle } from 'drizzle-orm/d1';
import { sql, eq, and, desc } from 'drizzle-orm';

export const load: PageServerLoad = async ({ platform, url, locals }) => {
	if (!platform?.env?.DB) {
		throw error(500, 'Database connection not available');
	}

	const db = drizzle(platform.env.DB);

	const page = Math.max(1, parseInt(url.searchParams.get('page') || '1'));
	const sort = url.searchParams.get('sort') || 'newest';
	const limit = 20;
	const offset = (page - 1) * limit;

	try {
		const followerCountSql = sql<number>`(SELECT COUNT(*) FROM follows WHERE follows.following_id = ${users.id})`;

		let orderBy;
		if (sort === 'followers') {
			orderBy = desc(followerCountSql);
		} else {
			orderBy = desc(users.createdAt);
		}

		const [allUsers, totalCount] = await Promise.all([
			db
				.select({
					id: users.id,
					username: users.username,
					createdAt: users.createdAt,
					deckCount: sql<number>`(SELECT COUNT(*) FROM decks WHERE decks.user_id = ${users.id})`,
					cardCount: sql<number>`(SELECT COUNT(*) FROM physical_cards WHERE physical_cards.user_id = ${users.id})`,
					followerCount: followerCountSql,
					isFollowing: locals.user
						? sql<boolean>`EXISTS(SELECT 1 FROM follows WHERE follows.follower_id = ${locals.user.id} AND follows.following_id = ${users.id})`
						: sql<boolean>`false`
				})
				.from(users)
				.orderBy(orderBy)
				.limit(limit)
				.offset(offset)
				.all(),
			db
				.select({ count: sql<number>`COUNT(*)` })
				.from(users)
				.get()
		]);

		return {
			users: allUsers,
			pagination: {
				page,
				limit,
				total: totalCount?.count ?? 0,
				totalPages: Math.ceil((totalCount?.count ?? 0) / limit)
			},
			sort
		};
	} catch (e) {
		console.error(e);
		throw error(500, 'Failed to load users');
	}
};

export const actions: Actions = {
	toggleFollow: async ({ request, locals, platform }) => {
		if (!locals.user) {
			return fail(401, { message: 'Unauthorized' });
		}

		if (!platform?.env?.DB) {
			return fail(500, { message: 'Database connection not available' });
		}

		const db = drizzle(platform.env.DB);
		const formData = await request.formData();
		const followingId = formData.get('userId') as string;

		if (!followingId || followingId === locals.user.id) {
			return fail(400, { message: 'Invalid user ID' });
		}

		try {
			const existing = await db
				.select()
				.from(follows)
				.where(and(eq(follows.followerId, locals.user.id), eq(follows.followingId, followingId)))
				.get();

			if (existing) {
				await db
					.delete(follows)
					.where(and(eq(follows.followerId, locals.user.id), eq(follows.followingId, followingId)));
			} else {
				await db.insert(follows).values({
					followerId: locals.user.id,
					followingId: followingId
				});
			}

			return { success: true };
		} catch (e) {
			console.error(e);
			return fail(500, { message: 'Failed to toggle follow' });
		}
	}
};
