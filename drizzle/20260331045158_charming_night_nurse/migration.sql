ALTER TABLE `card_cache` RENAME TO `cards`;--> statement-breakpoint
ALTER TABLE `cards` ADD `oracle_id` text;