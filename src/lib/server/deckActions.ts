import { fail } from '@sveltejs/kit';
import type { Actions } from '@sveltejs/kit';
import {
	createDeck,
	updateDeck,
	deleteDeck,
	addCardToDeck,
	updateDeckSlot,
	removeDeckSlot
} from '$lib/server/db/queries/decks';
import { getCachedCard } from '$lib/server/collection';

export const deckActions = {
	createDeck: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const name = formData.get('name') as string;
		const description = formData.get('description') as string;
		const format = formData.get('format') as string;

		if (!name) return fail(400, { message: 'Missing deck name' });

		try {
			const id = await createDeck(db, locals.user.id, { name, description, format });
			return { success: true, id };
		} catch (e: any) {
			return fail(500, { message: e.message || 'Failed to create deck' });
		}
	},

	updateDeck: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const deckId = formData.get('deckId') as string;
		const name = formData.get('name') as string;
		const description = formData.get('description') as string;

		if (!deckId) return fail(400, { message: 'Missing deck ID' });

		try {
			await updateDeck(db, locals.user.id, deckId, {
				name: name || undefined,
				description: description || null
			});
			return { success: true };
		} catch (e: any) {
			return fail(500, { message: e.message || 'Failed to update deck' });
		}
	},

	deleteDeck: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const deckId = formData.get('deckId') as string;

		if (!deckId) return fail(400, { message: 'Missing deck ID' });

		try {
			await deleteDeck(db, locals.user.id, deckId);
			return { success: true };
		} catch (e: any) {
			return fail(500, { message: e.message || 'Failed to delete deck' });
		}
	},

	addCard: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const deckId = formData.get('deckId') as string;
		const scryfallId = formData.get('scryfallId') as string;
		const quantity = parseInt(formData.get('quantity') as string) || 1;
		const board = (formData.get('board') as any) || 'main';

		if (!deckId || !scryfallId) return fail(400, { message: 'Missing deck ID or card ID' });

		try {
			// Ensure card is in cache
			await getCachedCard(db, scryfallId);
			await addCardToDeck(db, deckId, scryfallId, quantity, board);
			return { success: true };
		} catch (e: any) {
			return fail(500, { message: e.message || 'Failed to add card' });
		}
	},

	updateSlot: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const slotId = formData.get('slotId') as string;
		const quantity = parseInt(formData.get('quantity') as string);

		if (!slotId || isNaN(quantity))
			return fail(400, { message: 'Missing slot ID or invalid quantity' });

		try {
			await updateDeckSlot(db, slotId, quantity);
			return { success: true };
		} catch (e: any) {
			return fail(500, { message: e.message || 'Failed to update slot' });
		}
	},

	removeSlot: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const slotId = formData.get('slotId') as string;

		if (!slotId) return fail(400, { message: 'Missing slot ID' });

		try {
			await removeDeckSlot(db, slotId);
			return { success: true };
		} catch (e: any) {
			return fail(500, { message: e.message || 'Failed to remove slot' });
		}
	}
} satisfies Actions;
