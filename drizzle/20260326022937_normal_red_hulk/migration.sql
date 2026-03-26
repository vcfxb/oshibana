CREATE TABLE `deck_changes` (
	`id` integer PRIMARY KEY AUTOINCREMENT,
	`deck_id` text NOT NULL,
	`scryfall_id` text NOT NULL,
	`change_type` text NOT NULL,
	`quantity_change` integer NOT NULL,
	`board` text NOT NULL,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	CONSTRAINT `fk_deck_changes_deck_id_decks_id_fk` FOREIGN KEY (`deck_id`) REFERENCES `decks`(`id`) ON DELETE CASCADE
);
--> statement-breakpoint
ALTER TABLE `decks` ADD `format` text DEFAULT 'commander' NOT NULL;--> statement-breakpoint
ALTER TABLE `decks` ADD `physical_location_id` text REFERENCES storage_locations(id);--> statement-breakpoint
PRAGMA foreign_keys=OFF;--> statement-breakpoint
CREATE TABLE `__new_card_assignments` (
	`deck_slot_id` text NOT NULL,
	`physical_card_id` text NOT NULL,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	CONSTRAINT `card_assignments_deck_slot_id_physical_card_id_pk` PRIMARY KEY(`deck_slot_id`, `physical_card_id`),
	CONSTRAINT `fk_card_assignments_deck_slot_id_deck_slots_id_fk` FOREIGN KEY (`deck_slot_id`) REFERENCES `deck_slots`(`id`) ON DELETE CASCADE,
	CONSTRAINT `fk_card_assignments_physical_card_id_physical_cards_id_fk` FOREIGN KEY (`physical_card_id`) REFERENCES `physical_cards`(`id`) ON DELETE CASCADE
);
--> statement-breakpoint
INSERT INTO `__new_card_assignments`(`deck_slot_id`, `physical_card_id`, `created_at`) SELECT `deck_slot_id`, `physical_card_id`, `created_at` FROM `card_assignments`;--> statement-breakpoint
DROP TABLE `card_assignments`;--> statement-breakpoint
ALTER TABLE `__new_card_assignments` RENAME TO `card_assignments`;--> statement-breakpoint
PRAGMA foreign_keys=ON;--> statement-breakpoint
PRAGMA foreign_keys=OFF;--> statement-breakpoint
CREATE TABLE `__new_deck_slots` (
	`id` text PRIMARY KEY,
	`deck_id` text NOT NULL,
	`scryfall_id` text NOT NULL,
	`quantity` integer DEFAULT 1 NOT NULL,
	`board` text DEFAULT 'main' NOT NULL,
	CONSTRAINT `fk_deck_slots_deck_id_decks_id_fk` FOREIGN KEY (`deck_id`) REFERENCES `decks`(`id`) ON DELETE CASCADE
);
--> statement-breakpoint
INSERT INTO `__new_deck_slots`(`id`, `deck_id`, `scryfall_id`, `quantity`, `board`) SELECT `id`, `deck_id`, `scryfall_id`, `quantity`, `board` FROM `deck_slots`;--> statement-breakpoint
DROP TABLE `deck_slots`;--> statement-breakpoint
ALTER TABLE `__new_deck_slots` RENAME TO `deck_slots`;--> statement-breakpoint
PRAGMA foreign_keys=ON;--> statement-breakpoint
PRAGMA foreign_keys=OFF;--> statement-breakpoint
CREATE TABLE `__new_physical_cards` (
	`id` text PRIMARY KEY,
	`user_id` text NOT NULL,
	`scryfall_id` text NOT NULL,
	`condition` text DEFAULT 'NM' NOT NULL,
	`is_foil` integer DEFAULT false NOT NULL,
	`purchase_price` integer,
	`is_alter` integer DEFAULT false NOT NULL,
	`is_proxy` integer DEFAULT false NOT NULL,
	`language` text DEFAULT 'en' NOT NULL,
	`quantity` integer DEFAULT 1 NOT NULL,
	`storage_location_id` text,
	`current_deck_id` text,
	`notes` text,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	`updated_at` integer DEFAULT (unixepoch()) NOT NULL,
	CONSTRAINT `physical_cards_user_id_users_id_fk` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
	CONSTRAINT `fk_physical_cards_scryfall_id_card_cache_scryfall_id_fk` FOREIGN KEY (`scryfall_id`) REFERENCES `card_cache`(`scryfall_id`),
	CONSTRAINT `physical_cards_storage_location_id_storage_locations_id_fk` FOREIGN KEY (`storage_location_id`) REFERENCES `storage_locations`(`id`),
	CONSTRAINT `fk_physical_cards_current_deck_id_decks_id_fk` FOREIGN KEY (`current_deck_id`) REFERENCES `decks`(`id`) ON DELETE SET NULL
);
--> statement-breakpoint
INSERT INTO `__new_physical_cards`(`id`, `user_id`, `scryfall_id`, `condition`, `is_foil`, `purchase_price`, `is_alter`, `is_proxy`, `language`, `quantity`, `storage_location_id`, `current_deck_id`, `notes`, `created_at`, `updated_at`) SELECT `id`, `user_id`, `scryfall_id`, `condition`, `is_foil`, `purchase_price`, `is_alter`, `is_proxy`, `language`, `quantity`, `storage_location_id`, `current_deck_id`, `notes`, `created_at`, `updated_at` FROM `physical_cards`;--> statement-breakpoint
DROP TABLE `physical_cards`;--> statement-breakpoint
ALTER TABLE `__new_physical_cards` RENAME TO `physical_cards`;--> statement-breakpoint
PRAGMA foreign_keys=ON;