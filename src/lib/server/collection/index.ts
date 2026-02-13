import { eq, and, sql, asc, desc, inArray } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '../db/schema';
import { getCardById, getCardsBatch, type ScryfallCard } from '$lib/scryfall';
import type { CollectionSortBy, SortDir } from '$lib/collection';

export type { CollectionSortBy, SortDir };

function parsePrice(price: string | null | undefined): number | null {
	if (!price) return null;
	const parsed = parseFloat(price);
	return isNaN(parsed) ? null : Math.round(parsed * 100);
}

function mapScryfallToCache(card: ScryfallCard) {
	const colors = card.colors ? JSON.stringify(card.colors) : null;
	const colorIdentity = card.color_identity ? JSON.stringify(card.color_identity) : null;
	const imageUri = card.image_uris?.normal || card.card_faces?.[0]?.image_uris?.normal || null;

	return {
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
		priceUsd: parsePrice(card.prices.usd),
		priceUsdFoil: parsePrice(card.prices.usd_foil),
		priceUsdEtched: parsePrice(card.prices.usd_etched),
		priceEur: parsePrice(card.prices.eur),
		priceTix: parsePrice(card.prices.tix),
		updatedAt: new Date()
	};
}

export async function getCachedCard(db: D1Database, scryfallId: string) {
	const ddb = drizzle(db);
	const [cached] = await ddb
		.select()
		.from(schema.cardCache)
		.where(eq(schema.cardCache.scryfallId, scryfallId))
		.limit(1);

	const isExpired =
		cached && new Date().getTime() - cached.updatedAt.getTime() > 24 * 60 * 60 * 1000;

	if (cached && !isExpired) return cached;

	// Fetch from Scryfall and cache it
	const card = await getCardById(scryfallId);
	const newCard = mapScryfallToCache(card);

	await ddb
		.insert(schema.cardCache)
		.values(newCard)
		.onConflictDoUpdate({
			target: schema.cardCache.scryfallId,
			set: {
				...newCard,
				updatedAt: new Date() // Ensure it's updated even if values are the same
			}
		});

	return newCard;
}

export async function updateStaleCards(db: D1Database, scryfallIds: string[]) {
	if (scryfallIds.length === 0) return;

	const ddb = drizzle(db);
	const identifiers = scryfallIds.map((id) => ({ id }));

	// Scryfall allows up to 75 identifiers per request
	const CHUNK_SIZE = 75;
	for (let i = 0; i < identifiers.length; i += CHUNK_SIZE) {
		const chunk = identifiers.slice(i, i + CHUNK_SIZE);
		try {
			const results = await getCardsBatch(chunk);
			const cardsToCache = results.data
				.filter((c: any) => c.object === 'card')
				.map((c: any) => mapScryfallToCache(c as ScryfallCard));

			for (const card of cardsToCache) {
				await ddb
					.insert(schema.cardCache)
					.values(card)
					.onConflictDoUpdate({
						target: schema.cardCache.scryfallId,
						set: {
							...card,
							updatedAt: new Date()
						}
					});
			}
		} catch (e) {
			console.error('Error updating batch of cards:', e);
		}
	}
}

export async function getCollection(
	db: D1Database,
	userId: string,
	options: {
		limit?: number;
		offset?: number;
		storageLocationId?: string;
		sortBy?: CollectionSortBy;
		sortDir?: SortDir;
	} = {}
) {
	const {
		limit = 50,
		offset = 0,
		storageLocationId,
		sortBy = 'date-added',
		sortDir = 'desc'
	} = options;
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

	const sortOrder = sortDir === 'asc' ? asc : desc;
	let orderBy;

	switch (sortBy) {
		case 'name':
			orderBy = [sortOrder(schema.cardCache.name)];
			break;
		case 'value':
			// Sort by actual value based on foil status, with fallback to other currencies
			orderBy = [
				sortOrder(
					sql`COALESCE(
						CASE WHEN ${schema.physicalCards.isFoil} THEN ${schema.cardCache.priceUsdFoil} ELSE ${schema.cardCache.priceUsd} END,
						${schema.cardCache.priceUsd},
						${schema.cardCache.priceUsdFoil},
						${schema.cardCache.priceUsdEtched},
						${schema.cardCache.priceEur},
						${schema.cardCache.priceTix},
						0
					)`
				)
			];
			break;
		case 'purchase-price':
			orderBy = [sortOrder(schema.physicalCards.purchasePrice)];
			break;
		case 'set':
			orderBy = [sortOrder(sql`${schema.cardCache.set} || ${schema.cardCache.collectorNumber}`)];
			break;
		case 'date-added':
		default:
			orderBy = [sortOrder(schema.physicalCards.createdAt)];
			break;
	}

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

	const items = await query
		.orderBy(...orderBy)
		.limit(limit)
		.offset(offset);

	// Check for stale cache items or items with missing prices
	const now = new Date().getTime();
	const EXPIRATION = 24 * 60 * 60 * 1000;
	const REFRESH_EMPTY_PRICES = 60 * 60 * 1000; // 1 hour for cards with no prices

	const staleIds = items
		.filter((item) => {
			if (!item.cardData) return true;
			const age = now - item.cardData.updatedAt.getTime();
			if (age > EXPIRATION) return true;

			// If all prices are null, retry more frequently (but not every time)
			const hasNoPrices =
				item.cardData.priceUsd === null &&
				item.cardData.priceUsdFoil === null &&
				item.cardData.priceEur === null;

			if (hasNoPrices && age > REFRESH_EMPTY_PRICES) return true;

			return false;
		})
		.map((item) => item.physicalCard.scryfallId);

	if (staleIds.length > 0) {
		// Update in background or await if we want fresh data now
		// For collection page, fresh data is nice, but we don't want to block too long.
		// However, the user specifically asked to use the batch API for this.
		await updateStaleCards(db, Array.from(new Set(staleIds)));

		// Re-fetch the updated items or just patch them.
		// Since we only updated the cache, we can re-query the cache for these IDs.
		const updatedCache = await ddb
			.select()
			.from(schema.cardCache)
			.where(inArray(schema.cardCache.scryfallId, staleIds));

		const cacheMap = new Map(updatedCache.map((c) => [c.scryfallId, c]));
		for (const item of items) {
			const updated = cacheMap.get(item.physicalCard.scryfallId);
			if (updated) {
				item.cardData = updated;
			}
		}
	}

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
				options.purchasePrice === undefined ||
				options.purchasePrice === null ||
				isNaN(options.purchasePrice)
					? null
					: Math.round(options.purchasePrice * 100),
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
