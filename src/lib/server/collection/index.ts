import { eq, and, sql } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '../db/schema';
import { getCardById } from '$lib/scryfall';

export async function getCachedCard(db: D1Database, scryfallId: string) {
	const ddb = drizzle(db);
	const [cached] = await ddb
		.select()
		.from(schema.cardCache)
		.where(eq(schema.cardCache.scryfallId, scryfallId))
		.limit(1);

	if (cached) return cached;

	// Fetch from Scryfall and cache it
	const card = await getCardById(scryfallId);
	const colors = card.colors ? JSON.stringify(card.colors) : null;
	const colorIdentity = card.color_identity ? JSON.stringify(card.color_identity) : null;
	const imageUri = card.image_uris?.normal || card.card_faces?.[0]?.image_uris?.normal || null;

	const newCard = {
		scryfallId: card.id,
		name: card.name,
		set: card.set,
		setName: card.set_name,
		collectorNumber: card.collector_number,
		imageUri,
		manaCost: card.mana_cost || card.card_faces?.[0]?.mana_cost || null,
		typeLine: card.type_line,
		oracleText: card.oracle_text || card.card_faces?.[0]?.oracle_text || null,
		colors,
		colorIdentity,
		rarity: card.rarity,
		updatedAt: new Date()
	};

	await ddb.insert(schema.cardCache).values(newCard).onConflictDoUpdate({
		target: schema.cardCache.scryfallId,
		set: newCard
	});

	return newCard;
}

export async function getCollection(
	db: D1Database,
	userId: string,
	options: {
		limit?: number;
		offset?: number;
		storageLocationId?: string;
	} = {}
) {
	const { limit = 50, offset = 0, storageLocationId } = options;
	const ddb = drizzle(db);

	let query = ddb
		.select({
			physicalCard: schema.physicalCards,
			cardData: schema.cardCache
		})
		.from(schema.physicalCards)
		.leftJoin(schema.cardCache, eq(schema.physicalCards.scryfallId, schema.cardCache.scryfallId))
		.where(
			and(
				eq(schema.physicalCards.userId, userId),
				storageLocationId
					? eq(schema.physicalCards.storageLocationId, storageLocationId)
					: undefined
			)
		);

	const totalResult = await ddb
		.select({ count: sql<number>`count(*)` })
		.from(schema.physicalCards)
		.where(
			and(
				eq(schema.physicalCards.userId, userId),
				storageLocationId
					? eq(schema.physicalCards.storageLocationId, storageLocationId)
					: undefined
			)
		);

	const items = await query.limit(limit).offset(offset);

	return {
		items,
		total: totalResult[0]?.count || 0
	};
}

export async function addCardToCollection(
	db: D1Database,
	userId: string,
	scryfallId: string,
	options: {
		condition?: 'NM' | 'LP' | 'MP' | 'HP' | 'DMG';
		isFoil?: boolean;
		storageLocationId?: string;
		purchasePrice?: number;
		isAlter?: boolean;
		isProxy?: boolean;
		language?: string;
	} = {}
) {
	try {
		const ddb = drizzle(db);

		// Ensure card is cached
		await getCachedCard(db, scryfallId);

		const id = crypto.randomUUID();
		await ddb.insert(schema.physicalCards).values({
			id,
			userId,
			scryfallId,
			condition: options.condition || 'NM',
			isFoil: options.isFoil || false,
			storageLocationId: options.storageLocationId,
			purchasePrice:
				options.purchasePrice === null || isNaN(options.purchasePrice as number)
					? null
					: options.purchasePrice,
			isAlter: options.isAlter || false,
			isProxy: options.isProxy || false,
			language: options.language || 'en'
		});

		return id;
	} catch (e) {
		console.error('Error in addCardToCollection:', e);
		throw e;
	}
}

export async function removeCardFromCollection(
	db: D1Database,
	userId: string,
	physicalCardId: string
) {
	const ddb = drizzle(db);
	await ddb
		.delete(schema.physicalCards)
		.where(
			and(eq(schema.physicalCards.id, physicalCardId), eq(schema.physicalCards.userId, userId))
		);
}

export async function getStorageLocations(db: D1Database, userId: string) {
	const ddb = drizzle(db);
	return ddb
		.select()
		.from(schema.storageLocations)
		.where(eq(schema.storageLocations.userId, userId));
}

export async function createStorageLocation(
	db: D1Database,
	userId: string,
	data: {
		name: string;
		type: 'binder' | 'box' | 'shelf' | 'physical_deck' | 'other';
		description?: string;
	}
) {
	const ddb = drizzle(db);
	const id = crypto.randomUUID();
	await ddb.insert(schema.storageLocations).values({
		id,
		userId,
		name: data.name,
		type: data.type,
		description: data.description
	});
	return id;
}
