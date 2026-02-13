import fs from 'node:fs';
import path from 'node:path';

const DRIZZLE_DIR = path.resolve('drizzle');
const TARGET_DIR = path.resolve('migrations');

console.log(`Refreshing flattened migrations in ${TARGET_DIR}...`);

// Ensure target directory exists and is empty
if (fs.existsSync(TARGET_DIR)) {
	fs.rmSync(TARGET_DIR, { recursive: true, force: true });
}
fs.mkdirSync(TARGET_DIR, { recursive: true });

// Read drizzle directory
const entries = fs.readdirSync(DRIZZLE_DIR, { withFileTypes: true });

// Filter for migration directories (usually start with a timestamp like 2024...)
const migrationDirs = entries
	.filter((entry) => entry.isDirectory() && /^\d{14}_/.test(entry.name))
	.sort((a, b) => a.name.localeCompare(b.name));

for (const entry of migrationDirs) {
	const sourceFile = path.join(DRIZZLE_DIR, entry.name, 'migration.sql');
	const targetLink = path.join(TARGET_DIR, `${entry.name}.sql`);

	if (fs.existsSync(sourceFile)) {
		// Use relative path for the symlink to keep it portable
		const relativePath = path.relative(TARGET_DIR, sourceFile);

		try {
			// On Windows, 'file' type is important, and might require developer mode or admin
			fs.symlinkSync(relativePath, targetLink, 'file');
			console.log(`Created symlink: ${entry.name}.sql -> ${entry.name}/migration.sql`);
		} catch (err) {
			console.error(`Failed to create symlink for ${entry.name}:`, err);
			console.log('Attempting to copy instead...');
			fs.copyFileSync(sourceFile, targetLink);
		}
	}
}

console.log('Done!');
