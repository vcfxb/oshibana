CREATE TABLE `card_cache` (
	`scryfall_id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`set` text NOT NULL,
	`set_name` text NOT NULL,
	`collector_number` text NOT NULL,
	`image_uri` text,
	`mana_cost` text,
	`type_line` text,
	`oracle_text` text,
	`colors` text,
	`color_identity` text,
	`rarity` text NOT NULL,
	`updated_at` integer DEFAULT (unixepoch()) NOT NULL
);
