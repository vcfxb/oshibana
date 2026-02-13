import { error, fail } from '@sveltejs/kit';
import { eq, sql, and } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '$lib/server/db/schema';
import { createStorageLocation } from '$lib/server/collection';
import type { PageServerLoad, Actions } from './$types';
import { relations } from '$lib/server/db/relations';

export const load: PageServerLoad = async ({ params, platform }) => {
	const db = platform?.env.DB;
	if (!db) throw error(500, 'Database not found');

	const ddb = drizzle(db, { relations });
	const [profile] = await ddb
		.select()
		.from(schema.users)
		.where(eq(schema.users.username, params.username))
		.limit(1);

	if (!profile) throw error(404, 'User not found');

	const locations = await ddb.query.storageLocations.findMany({ where: { userId: profile.id }});

	// Get card counts for each location
	const locationsWithCounts = await Promise.all(
		locations.map(async (loc) => {
			const [countResult] = await ddb
				.select({ count: sql<number>`count(*)` })
				.from(schema.physicalCards)
				.where(eq(schema.physicalCards.storageLocationId, loc.id));
			return {
				...loc,
				cardCount: countResult?.count || 0
			};
		})
	);

	return {
		profile,
		locations: locationsWithCounts
	};
};

export const actions: Actions = {
	createLocation: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const name = formData.get('locationName') as string;
		const type = formData.get('type') as 'binder' | 'box' | 'shelf' | 'physical_deck' | 'other';
		const description = formData.get('description') as string;

		if (!name || !type) return fail(400, { message: 'Name and type are required' });

		try {
			await createStorageLocation(db, locals.user.id, {
				name,
				type,
				description: description || undefined
			});
			return { success: true };
		} catch (e) {
			return fail(500, { message: e instanceof Error ? e.message : 'Failed to create location' });
		}
	},
	deleteLocation: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const id = formData.get('id') as string;

		if (!id) return fail(400);

		const ddb = drizzle(db);
		await ddb
			.delete(schema.storageLocations)
			.where(
				and(eq(schema.storageLocations.id, id), eq(schema.storageLocations.userId, locals.user.id))
			);

		return { success: true };
	}
};
