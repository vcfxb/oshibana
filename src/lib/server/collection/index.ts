import { eq, and, sql, asc, desc, inArray } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '../db/schema';
import { getCardById, getCardsBatch, type ScryfallCard } from '$lib/scryfall';
import type { CollectionSortBy, SortDir } from '$lib/collection';
import { relations } from '$lib/server/db/relations';

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
		sortBy = 'date-updated',
		sortDir = 'desc'
	} = options;

	const ddb = drizzle(db, { relations });

	const collection = await ddb.query.physicalCards.findMany({
		where: {
			userId,
			storageLocationId
		},
		with: {}
	});

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
		case 'total-value':
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
					) * ${schema.physicalCards.quantity}`
				)
			];
			break;
		case 'purchase-price':
			orderBy = [sortOrder(schema.physicalCards.purchasePrice)];
			break;
		case 'set':
			orderBy = [sortOrder(sql`${schema.cardCache.set} || ${schema.cardCache.collectorNumber}`)];
			break;
		case 'quantity':
			orderBy = [sortOrder(schema.physicalCards.quantity)];
			break;
		case 'date-added':
			orderBy = [sortOrder(schema.physicalCards.createdAt)];
			break;
		case 'date-updated':
		default:
			orderBy = [sortOrder(schema.physicalCards.updatedAt)];
			break;
	}

	const totalResult = await ddb
		.select({ count: sql<number>`CAST(SUM(${schema.physicalCards.quantity}) AS INTEGER)` })
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

export type CollectionData = Awaited<ReturnType<typeof getCollection>>;

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
		quantity?: number;
		notes?: string;
	} = {}
) {
	try {
		const ddb = drizzle(db);

		// Ensure card is cached
		await getCachedCard(db, scryfallId);

		const quantity = options.quantity || 1;
		const condition = options.condition || 'NM';
		const isFoil = options.isFoil || false;
		const isAlter = options.isAlter || false;
		const isProxy = options.isProxy || false;
		const language = options.language || 'en';
		const storageLocationId = options.storageLocationId || null;
		const notes = options.notes?.slice(0, 250) || null;
		const purchasePrice =
			options.purchasePrice === undefined ||
			options.purchasePrice === null ||
			isNaN(options.purchasePrice)
				? null
				: Math.round(options.purchasePrice * 100);

		// Try to find an existing identical card to increment quantity
		const [existing] = await ddb
			.select()
			.from(schema.physicalCards)
			.where(
				and(
					eq(schema.physicalCards.userId, userId),
					eq(schema.physicalCards.scryfallId, scryfallId),
					eq(schema.physicalCards.condition, condition),
					eq(schema.physicalCards.isFoil, isFoil),
					eq(schema.physicalCards.isAlter, isAlter),
					eq(schema.physicalCards.isProxy, isProxy),
					eq(schema.physicalCards.language, language),
					storageLocationId
						? eq(schema.physicalCards.storageLocationId, storageLocationId)
						: sql`${schema.physicalCards.storageLocationId} IS NULL`,
					purchasePrice !== null
						? eq(schema.physicalCards.purchasePrice, purchasePrice)
						: sql`${schema.physicalCards.purchasePrice} IS NULL`,
					sql`${schema.physicalCards.currentDeckId} IS NULL`, // Only group unassigned cards
					notes !== null
						? eq(schema.physicalCards.notes, notes)
						: sql`${schema.physicalCards.notes} IS NULL` // Only group cards without specific notes
				)
			)
			.limit(1);

		if (existing) {
			await ddb
				.update(schema.physicalCards)
				.set({
					quantity: existing.quantity + quantity,
					updatedAt: new Date()
				})
				.where(eq(schema.physicalCards.id, existing.id));
			return existing.id;
		}

		const id = crypto.randomUUID();
		await ddb.insert(schema.physicalCards).values({
			id,
			userId,
			scryfallId,
			condition,
			isFoil,
			storageLocationId,
			purchasePrice,
			isAlter,
			isProxy,
			language,
			quantity,
			notes
		});

		return id;
	} catch (e) {
		console.error('Error in addCardToCollection:', e);
		throw e;
	}
}

export async function updateCardInCollection(
	db: D1Database,
	userId: string,
	physicalCardId: string,
	options: {
		condition?: 'NM' | 'LP' | 'MP' | 'HP' | 'DMG';
		isFoil?: boolean;
		storageLocationId?: string | null;
		purchasePrice?: number | null;
		isAlter?: boolean;
		isProxy?: boolean;
		language?: string;
		quantity?: number;
		notes?: string | null;
	} = {}
) {
	try {
		const ddb = drizzle(db);

		const updates: any = {
			updatedAt: new Date()
		};

		if (options.condition !== undefined) updates.condition = options.condition;
		if (options.isFoil !== undefined) updates.isFoil = options.isFoil;
		if (options.storageLocationId !== undefined) {
			updates.storageLocationId =
				options.storageLocationId === 'none' ? null : options.storageLocationId;
		}
		if (options.purchasePrice !== undefined) {
			updates.purchasePrice =
				options.purchasePrice === null || isNaN(options.purchasePrice)
					? null
					: Math.round(options.purchasePrice * 100);
		}
		if (options.isAlter !== undefined) updates.isAlter = options.isAlter;
		if (options.isProxy !== undefined) updates.isProxy = options.isProxy;
		if (options.language !== undefined) updates.language = options.language;
		if (options.quantity !== undefined) updates.quantity = Math.max(1, options.quantity);
		if (options.notes !== undefined) updates.notes = options.notes?.slice(0, 250) || null;

		await ddb
			.update(schema.physicalCards)
			.set(updates)
			.where(
				and(eq(schema.physicalCards.id, physicalCardId), eq(schema.physicalCards.userId, userId))
			);
	} catch (e) {
		console.error('Error in updateCardInCollection:', e);
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

export async function updateStorageLocation(
	db: D1Database,
	userId: string,
	id: string,
	data: {
		name?: string;
		type?: 'binder' | 'box' | 'shelf' | 'physical_deck' | 'other';
		description?: string | null;
	}
) {
	const ddb = drizzle(db);
	await ddb
		.update(schema.storageLocations)
		.set({
			...data,
			updatedAt: new Date()
		})
		.where(and(eq(schema.storageLocations.id, id), eq(schema.storageLocations.userId, userId)));
}
