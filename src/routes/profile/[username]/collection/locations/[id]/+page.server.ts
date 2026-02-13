import { error } from '@sveltejs/kit';
import { eq, and } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '$lib/server/db/schema';
import { getCollection } from '$lib/server/collection';
import { collectionActions } from '$lib/server/collectionActions';
import type { PageServerLoad } from './$types';
import type { CollectionSortBy, SortDir } from '$lib/collection.js';
import { relations } from '$lib/server/db/relations';


export const load: PageServerLoad = async ({ params, platform, url }) => {
	const db = platform?.env.DB;
	if (!db) throw error(500, 'Database not found');

	const ddb = drizzle(db, { relations });
	const [profile] = await ddb
		.select()
		.from(schema.users)
		.where(eq(schema.users.username, params.username))
		.limit(1);

	if (!profile) throw error(404, 'User not found');

	const [location] = await ddb
		.select()
		.from(schema.storageLocations)
		.where(
			and(eq(schema.storageLocations.id, params.id), eq(schema.storageLocations.userId, profile.id))
		)
		.limit(1);

	if (!location) throw error(404, 'Location not found');

	const page = Number(url.searchParams.get('page')) || 1;
	const limit = 50;
	const offset = (page - 1) * limit;

	const sortBy = (url.searchParams.get('sortBy') as CollectionSortBy) || 'date-added';
	const sortDir = (url.searchParams.get('sortDir') as SortDir) || 'desc';

	const [collectionData, locations] = await Promise.all([
		getCollection(db, profile.id, { limit, offset, storageLocationId: params.id, sortBy, sortDir }),
		ddb.query.storageLocations.findMany({ where: { userId: profile.id }})
	]);

	const formattedLocation = {
		...location,
		displayType: location.type.replace('_', ' ')
	};

	return {
		profile,
		location: formattedLocation,
		collection: collectionData.items,
		total: collectionData.total,
		page,
		limit,
		sortBy,
		sortDir,
		locations
	};
};

export const actions = collectionActions;
