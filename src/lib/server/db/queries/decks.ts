import { eq, and, sql } from 'drizzle-orm';
import { drizzle } from 'drizzle-orm/d1';
import * as schema from '../schema';

export async function getDeck(db: D1Database, deckId: string) {
	const ddb = drizzle(db);
	const [deck] = await ddb.select().from(schema.decks).where(eq(schema.decks.id, deckId)).limit(1);
	return deck;
}

export async function getDeckSlots(db: D1Database, deckId: string) {
	const ddb = drizzle(db);
	return await ddb
		.select({
			slot: schema.deckSlots,
			card: schema.cardCache
		})
		.from(schema.deckSlots)
		.leftJoin(schema.cardCache, eq(schema.deckSlots.scryfallId, schema.cardCache.scryfallId))
		.where(eq(schema.deckSlots.deckId, deckId));
}

export async function createDeck(
	db: D1Database,
	userId: string,
	data: { name: string; description?: string; format?: string; physicalLocationId?: string }
) {
	const ddb = drizzle(db);
	const id = crypto.randomUUID();
	await ddb.insert(schema.decks).values({
		id,
		userId,
		name: data.name,
		description: data.description || null,
		format: (data.format as any) || 'commander',
		physicalLocationId: data.physicalLocationId === 'none' ? null : data.physicalLocationId || null
	});
	return id;
}

export async function updateDeck(
	db: D1Database,
	userId: string,
	deckId: string,
	data: { name?: string; description?: string | null }
) {
	const ddb = drizzle(db);
	await ddb
		.update(schema.decks)
		.set({
			...data,
			updatedAt: new Date()
		})
		.where(and(eq(schema.decks.id, deckId), eq(schema.decks.userId, userId)));
}

export async function deleteDeck(db: D1Database, userId: string, deckId: string) {
	const ddb = drizzle(db);
	await ddb
		.delete(schema.decks)
		.where(and(eq(schema.decks.id, deckId), eq(schema.decks.userId, userId)));
}

export async function addCardToDeck(
	db: D1Database,
	deckId: string,
	scryfallId: string,
	quantity: number = 1,
	board: 'main' | 'side' | 'maybe' | 'commander' = 'main'
) {
	const ddb = drizzle(db);

	// Check if slot already exists
	const [existing] = await ddb
		.select()
		.from(schema.deckSlots)
		.where(
			and(
				eq(schema.deckSlots.deckId, deckId),
				eq(schema.deckSlots.scryfallId, scryfallId),
				eq(schema.deckSlots.board, board)
			)
		)
		.limit(1);

	if (existing) {
		await ddb
			.update(schema.deckSlots)
			.set({ quantity: existing.quantity + quantity })
			.where(eq(schema.deckSlots.id, existing.id));

		await ddb.insert(schema.deckChanges).values({
			deckId,
			scryfallId,
			changeType: 'update_quantity',
			quantityChange: quantity,
			board
		});

		return existing.id;
	}

	const id = crypto.randomUUID();
	await ddb.insert(schema.deckSlots).values({
		id,
		deckId,
		scryfallId,
		quantity,
		board
	});

	await ddb.insert(schema.deckChanges).values({
		deckId,
		scryfallId,
		changeType: 'add',
		quantityChange: quantity,
		board
	});

	return id;
}

export async function updateDeckSlot(db: D1Database, slotId: string, quantity: number) {
	const ddb = drizzle(db);
	const [slot] = await ddb
		.select()
		.from(schema.deckSlots)
		.where(eq(schema.deckSlots.id, slotId))
		.limit(1);

	if (!slot) return;

	const quantityChange = quantity - slot.quantity;
	if (quantityChange === 0) return;

	if (quantity <= 0) {
		await ddb.delete(schema.deckSlots).where(eq(schema.deckSlots.id, slotId));
		await ddb.insert(schema.deckChanges).values({
			deckId: slot.deckId,
			scryfallId: slot.scryfallId,
			changeType: 'remove',
			quantityChange: -slot.quantity,
			board: slot.board
		});
	} else {
		await ddb.update(schema.deckSlots).set({ quantity }).where(eq(schema.deckSlots.id, slotId));

		await ddb.insert(schema.deckChanges).values({
			deckId: slot.deckId,
			scryfallId: slot.scryfallId,
			changeType: 'update_quantity',
			quantityChange: quantityChange,
			board: slot.board
		});
	}
}

export async function removeDeckSlot(db: D1Database, slotId: string) {
	const ddb = drizzle(db);
	const [slot] = await ddb
		.select()
		.from(schema.deckSlots)
		.where(eq(schema.deckSlots.id, slotId))
		.limit(1);

	if (!slot) return;

	await ddb.delete(schema.deckSlots).where(eq(schema.deckSlots.id, slotId));
	await ddb.insert(schema.deckChanges).values({
		deckId: slot.deckId,
		scryfallId: slot.scryfallId,
		changeType: 'remove',
		quantityChange: -slot.quantity,
		board: slot.board
	});
}

export async function getDeckHistory(db: D1Database, deckId: string) {
	const ddb = drizzle(db);
	return await ddb
		.select({
			change: schema.deckChanges,
			card: schema.cardCache
		})
		.from(schema.deckChanges)
		.leftJoin(schema.cardCache, eq(schema.deckChanges.scryfallId, schema.cardCache.scryfallId))
		.where(eq(schema.deckChanges.deckId, deckId))
		.orderBy(schema.deckChanges.createdAt);
}
