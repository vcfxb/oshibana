ALTER TABLE `physical_cards` ADD `purchase_price` real;--> statement-breakpoint
ALTER TABLE `physical_cards` ADD `is_alter` integer DEFAULT false NOT NULL;--> statement-breakpoint
ALTER TABLE `physical_cards` ADD `is_proxy` integer DEFAULT false NOT NULL;--> statement-breakpoint
ALTER TABLE `physical_cards` ADD `language` text DEFAULT 'en' NOT NULL;