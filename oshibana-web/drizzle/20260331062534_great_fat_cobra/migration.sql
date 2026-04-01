PRAGMA foreign_keys=OFF;--> statement-breakpoint
CREATE TABLE `__new_email_verification_codes` (
	`id` integer PRIMARY KEY AUTOINCREMENT,
	`code` text NOT NULL UNIQUE,
	`user_id` text NOT NULL,
	`email` text NOT NULL,
	`expires_at` integer NOT NULL,
	CONSTRAINT `email_verification_codes_user_id_users_id_fk` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);
--> statement-breakpoint
INSERT INTO `__new_email_verification_codes`(`id`, `code`, `user_id`, `email`, `expires_at`) SELECT `id`, `code`, `user_id`, `email`, `expires_at` FROM `email_verification_codes`;--> statement-breakpoint
DROP TABLE `email_verification_codes`;--> statement-breakpoint
ALTER TABLE `__new_email_verification_codes` RENAME TO `email_verification_codes`;--> statement-breakpoint
PRAGMA foreign_keys=ON;