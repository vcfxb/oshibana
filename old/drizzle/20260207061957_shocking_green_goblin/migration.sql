CREATE TABLE `card_assignments` (
	`deck_slot_id` text NOT NULL,
	`physical_card_id` text NOT NULL,
	PRIMARY KEY(`deck_slot_id`, `physical_card_id`),
	FOREIGN KEY (`deck_slot_id`) REFERENCES `deck_slots`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`physical_card_id`) REFERENCES `physical_cards`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `deck_slots` (
	`id` text PRIMARY KEY NOT NULL,
	`deck_id` text NOT NULL,
	`scryfall_id` text NOT NULL,
	`quantity` integer DEFAULT 1 NOT NULL,
	`board` text DEFAULT 'main' NOT NULL,
	FOREIGN KEY (`deck_id`) REFERENCES `decks`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `decks` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`description` text,
	`created_at` integer NOT NULL,
	`updated_at` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `physical_cards` (
	`id` text PRIMARY KEY NOT NULL,
	`scryfall_id` text NOT NULL,
	`condition` text DEFAULT 'NM' NOT NULL,
	`is_foil` integer DEFAULT false NOT NULL,
	`location` text DEFAULT 'collection' NOT NULL,
	`notes` text
);
