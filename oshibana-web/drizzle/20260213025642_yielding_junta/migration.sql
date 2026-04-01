PRAGMA foreign_keys=OFF;--> statement-breakpoint
CREATE TABLE `__new_physical_cards` (
	`id` text PRIMARY KEY NOT NULL,
	`user_id` text NOT NULL,
	`scryfall_id` text NOT NULL,
	`condition` text DEFAULT 'NM' NOT NULL,
	`is_foil` integer DEFAULT false NOT NULL,
	`purchase_price` integer,
	`is_alter` integer DEFAULT false NOT NULL,
	`is_proxy` integer DEFAULT false NOT NULL,
	`language` text DEFAULT 'en' NOT NULL,
	`storage_location_id` text,
	`current_deck_id` text,
	`notes` text,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	`updated_at` integer DEFAULT (unixepoch()) NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`storage_location_id`) REFERENCES `storage_locations`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`current_deck_id`) REFERENCES `decks`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
INSERT INTO `__new_physical_cards`("id", "user_id", "scryfall_id", "condition", "is_foil", "purchase_price", "is_alter", "is_proxy", "language", "storage_location_id", "current_deck_id", "notes", "created_at", "updated_at") SELECT "id", "user_id", "scryfall_id", "condition", "is_foil", "purchase_price", "is_alter", "is_proxy", "language", "storage_location_id", "current_deck_id", "notes", "created_at", "updated_at" FROM `physical_cards`;--> statement-breakpoint
DROP TABLE `physical_cards`;--> statement-breakpoint
ALTER TABLE `__new_physical_cards` RENAME TO `physical_cards`;--> statement-breakpoint
PRAGMA foreign_keys=ON;--> statement-breakpoint
ALTER TABLE `card_cache` ADD `price_usd` integer;--> statement-breakpoint
ALTER TABLE `card_cache` ADD `price_usd_foil` integer;--> statement-breakpoint
ALTER TABLE `card_cache` ADD `price_eur` integer;--> statement-breakpoint
ALTER TABLE `card_cache` ADD `price_tix` integer;