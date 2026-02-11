import { error, fail } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '$lib/server/db/schema';
import {
	getCollection,
	getStorageLocations,
	removeCardFromCollection
} from '$lib/server/collection';
import type { PageServerLoad, Actions } from './$types';

export const load: PageServerLoad = async ({ params, platform, url }) => {
	const db = platform?.env.DB;
	if (!db) throw error(500, 'Database not found');

	const ddb = drizzle(db);
	const [profile] = await ddb
		.select()
		.from(schema.users)
		.where(eq(schema.users.username, params.username))
		.limit(1);

	if (!profile) throw error(404, 'User not found');

	const page = Number(url.searchParams.get('page')) || 1;
	const limit = 50;
	const offset = (page - 1) * limit;

	const [collectionData, locations] = await Promise.all([
		getCollection(db, profile.id, { limit, offset }),
		getStorageLocations(db, profile.id)
	]);

	return {
		profile,
		collection: collectionData.items,
		total: collectionData.total,
		page,
		limit,
		locations
	};
};

export const actions: Actions = {
	removeCard: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const physicalCardId = formData.get('physicalCardId') as string;

		if (!physicalCardId) return fail(400);

		await removeCardFromCollection(db, locals.user.id, physicalCardId);
		return { success: true };
	}
};
