PRAGMA defer_foreign_keys=ON;--> statement-breakpoint
CREATE TABLE `__new_users` (
	`id` text PRIMARY KEY,
	`username` text NOT NULL UNIQUE,
	`email` text NOT NULL UNIQUE,
	`password_hash` text,
	`email_verified` integer DEFAULT false NOT NULL,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	`updated_at` integer DEFAULT (unixepoch()) NOT NULL
);
--> statement-breakpoint
INSERT INTO `__new_users`(`id`, `username`, `email`, `password_hash`, `email_verified`, `created_at`, `updated_at`) SELECT `id`, `username`, `email`, `password_hash`, `email_verified`, `created_at`, `updated_at` FROM `users`;--> statement-breakpoint
DROP TABLE `users`;--> statement-breakpoint
ALTER TABLE `__new_users` RENAME TO `users`;--> statement-breakpoint
PRAGMA defer_foreign_keys=OFF;--> statement-breakpoint
PRAGMA defer_foreign_keys=ON;--> statement-breakpoint
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
	`storage_location_id` text,
	`current_deck_id` text,
	`notes` text,
	`created_at` integer DEFAULT (unixepoch()) NOT NULL,
	`updated_at` integer DEFAULT (unixepoch()) NOT NULL,
	CONSTRAINT `physical_cards_user_id_users_id_fk` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
	CONSTRAINT `fk_physical_cards_scryfall_id_card_cache_scryfall_id_fk` FOREIGN KEY (`scryfall_id`) REFERENCES `card_cache`(`scryfall_id`),
	CONSTRAINT `physical_cards_storage_location_id_storage_locations_id_fk` FOREIGN KEY (`storage_location_id`) REFERENCES `storage_locations`(`id`),
	CONSTRAINT `physical_cards_current_deck_id_decks_id_fk` FOREIGN KEY (`current_deck_id`) REFERENCES `decks`(`id`)
);
--> statement-breakpoint
INSERT INTO `__new_physical_cards`(`id`, `user_id`, `scryfall_id`, `condition`, `is_foil`, `purchase_price`, `is_alter`, `is_proxy`, `language`, `storage_location_id`, `current_deck_id`, `notes`, `created_at`, `updated_at`) SELECT `id`, `user_id`, `scryfall_id`, `condition`, `is_foil`, `purchase_price`, `is_alter`, `is_proxy`, `language`, `storage_location_id`, `current_deck_id`, `notes`, `created_at`, `updated_at` FROM `physical_cards`;--> statement-breakpoint
DROP TABLE `physical_cards`;--> statement-breakpoint
ALTER TABLE `__new_physical_cards` RENAME TO `physical_cards`;--> statement-breakpoint
PRAGMA defer_foreign_keys=OFF;--> statement-breakpoint
DROP INDEX IF EXISTS `users_username_unique`;--> statement-breakpoint
DROP INDEX IF EXISTS `users_email_unique`;