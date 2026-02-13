import { sql } from 'drizzle-orm';
import { sqliteTable, text, integer, primaryKey } from 'drizzle-orm/sqlite-core';

// --- Authentication & User Management ---

export const users = sqliteTable('users', {
	id: text('id').primaryKey(), // UUID
	username: text('username').notNull().unique(),
	email: text('email').notNull().unique(),
	passwordHash: text('password_hash'),
	emailVerified: integer('email_verified', { mode: 'boolean' }).notNull().default(false),
	createdAt: integer('created_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`),
	updatedAt: integer('updated_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`)
		.$onUpdate(() => new Date())
});

export const sessions = sqliteTable('sessions', {
	id: text('id').primaryKey(),
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	expiresAt: integer('expires_at').notNull()
});

export const emailVerificationCodes = sqliteTable('email_verification_codes', {
	id: integer('id').primaryKey({ autoIncrement: true }),
	code: text('code').notNull(),
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	email: text('email').notNull(),
	expiresAt: integer('expires_at').notNull()
});

export const passwordResetTokens = sqliteTable('password_reset_tokens', {
	tokenHash: text('token_hash').primaryKey(),
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	expiresAt: integer('expires_at').notNull()
});

// --- Card Cache (Scryfall Data) ---

export const cardCache = sqliteTable('card_cache', {
	scryfallId: text('scryfall_id').primaryKey(),
	name: text('name').notNull(),
	set: text('set').notNull(),
	setName: text('set_name').notNull(),
	collectorNumber: text('collector_number').notNull(),
	imageUri: text('image_uri'),
	manaCost: text('mana_cost'),
	typeLine: text('type_line'),
	oracleText: text('oracle_text'),
	colors: text('colors'), // JSON stringified array
	colorIdentity: text('color_identity'), // JSON stringified array
	rarity: text('rarity').notNull(),
	priceUsd: integer('price_usd'),
	priceUsdFoil: integer('price_usd_foil'),
	priceUsdEtched: integer('price_usd_etched'),
	priceEur: integer('price_eur'),
	priceTix: integer('price_tix'),
	updatedAt: integer('updated_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`)
		.$onUpdate(() => new Date())
});

// --- Storage & Organization ---

export const storageLocations = sqliteTable('storage_locations', {
	id: text('id').primaryKey(), // UUID
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	name: text('name').notNull(),
	type: text('type', { enum: ['binder', 'box', 'shelf', 'physical_deck', 'other'] })
		.notNull()
		.default('binder'),
	description: text('description'),
	createdAt: integer('created_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`),
	updatedAt: integer('updated_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`)
		.$onUpdate(() => new Date())
});

// --- Deck & Collection Management ---

export const decks = sqliteTable('decks', {
	id: text('id').primaryKey(), // UUID
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	name: text('name').notNull(),
	description: text('description'),
	createdAt: integer('created_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`),
	updatedAt: integer('updated_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`)
		.$onUpdate(() => new Date())
});

export const deckSlots = sqliteTable('deck_slots', {
	id: text('id').primaryKey(), // UUID
	deckId: text('deck_id')
		.notNull()
		.references(() => decks.id),
	scryfallId: text('scryfall_id').notNull(),
	quantity: integer('quantity').notNull().default(1),
	board: text('board', { enum: ['main', 'side', 'maybe', 'commander'] })
		.notNull()
		.default('main')
});

export const physicalCards = sqliteTable('physical_cards', {
	id: text('id').primaryKey(), // UUID
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	scryfallId: text('scryfall_id').notNull().references(() => cardCache.scryfallId),
	condition: text('condition', { enum: ['NM', 'LP', 'MP', 'HP', 'DMG'] })
		.notNull()
		.default('NM'),
	isFoil: integer('is_foil', { mode: 'boolean' }).notNull().default(false),
	purchasePrice: integer('purchase_price'),
	isAlter: integer('is_alter', { mode: 'boolean' }).notNull().default(false),
	isProxy: integer('is_proxy', { mode: 'boolean' }).notNull().default(false),
	language: text('language').notNull().default('en'),

	// Physical tracking
	storageLocationId: text('storage_location_id').references(() => storageLocations.id, {
		onDelete: 'no action'
	}),
	currentDeckId: text('current_deck_id').references(() => decks.id, { onDelete: 'no action' }),

	notes: text('notes'),
	createdAt: integer('created_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`),
	updatedAt: integer('updated_at', { mode: 'timestamp' })
		.notNull()
		.default(sql`(unixepoch())`)
		.$onUpdate(() => new Date())
});

export const cardAssignments = sqliteTable(
	'card_assignments',
	{
		deckSlotId: text('deck_slot_id')
			.notNull()
			.references(() => deckSlots.id),
		physicalCardId: text('physical_card_id')
			.notNull()
			.references(() => physicalCards.id),
		createdAt: integer('created_at', { mode: 'timestamp' })
			.notNull()
			.default(sql`(unixepoch())`)
	},
	(table) => [primaryKey({ columns: [table.deckSlotId, table.physicalCardId] })]
);

export const follows = sqliteTable(
	'follows',
	{
		followerId: text('follower_id')
			.notNull()
			.references(() => users.id),
		followingId: text('following_id')
			.notNull()
			.references(() => users.id),
		createdAt: integer('created_at', { mode: 'timestamp' })
			.notNull()
			.default(sql`(unixepoch())`)
	},
	(table) => [primaryKey({ columns: [table.followerId, table.followingId] })]
);
