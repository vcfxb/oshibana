import { fail } from '@sveltejs/kit';
import {
	removeCardFromCollection,
	addCardToCollection,
	updateCardInCollection
} from '$lib/server/collection';
import type { Actions } from '@sveltejs/kit';

export const collectionActions = {
	removeCard: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const physicalCardId = formData.get('physicalCardId') as string;

		if (!physicalCardId) return fail(400);

		await removeCardFromCollection(db, locals.user.id, physicalCardId);
		return { success: true };
	},

	addCard: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const scryfallId = formData.get('scryfallId') as string;
		if (!scryfallId) return fail(400, { message: 'Missing card ID' });

		const condition = (formData.get('condition') as any) || 'NM';
		const isFoil = formData.get('isFoil') === 'true';
		const storageLocationId = (formData.get('storageLocationId') as string) || null;
		const purchasePriceRaw = formData.get('purchasePrice') as string;
		const purchasePrice =
			purchasePriceRaw && !isNaN(parseFloat(purchasePriceRaw))
				? parseFloat(purchasePriceRaw)
				: undefined;
		const isAlter = formData.get('isAlter') === 'true';
		const isProxy = formData.get('isProxy') === 'true';
		const language = (formData.get('language') as string) || 'en';
		const quantityRaw = formData.get('quantity') as string;
		const quantity = Math.max(1, Math.min(parseInt(quantityRaw) || 1, 100));
		const notes = formData.get('notes') as string;

		try {
			await addCardToCollection(db, locals.user.id, scryfallId, {
				condition,
				isFoil,
				storageLocationId:
					storageLocationId === 'none' ? undefined : storageLocationId || undefined,
				purchasePrice,
				isAlter,
				isProxy,
				language,
				quantity,
				notes
			});
			return { success: true };
		} catch (e) {
			return fail(500, { message: e instanceof Error ? e.message : 'Failed to add card' });
		}
	},

	updateCard: async ({ request, platform, locals }) => {
		if (!locals.user) return fail(401);

		const db = platform?.env.DB;
		if (!db) return fail(500);

		const formData = await request.formData();
		const physicalCardId = formData.get('physicalCardId') as string;
		if (!physicalCardId) return fail(400, { message: 'Missing card ID' });

		const condition = (formData.get('condition') as any) || 'NM';
		const isFoil = formData.get('isFoil') === 'true';
		const storageLocationId = (formData.get('storageLocationId') as string) || null;
		const purchasePriceRaw = formData.get('purchasePrice') as string;
		const purchasePrice =
			purchasePriceRaw && !isNaN(parseFloat(purchasePriceRaw))
				? parseFloat(purchasePriceRaw)
				: null;
		const isAlter = formData.get('isAlter') === 'true';
		const isProxy = formData.get('isProxy') === 'true';
		const language = (formData.get('language') as string) || 'en';
		const quantityRaw = formData.get('quantity') as string;
		const quantity = Math.max(1, Math.min(parseInt(quantityRaw) || 1, 100));
		const notes = formData.get('notes') as string;

		try {
			await updateCardInCollection(db, locals.user.id, physicalCardId, {
				condition,
				isFoil,
				storageLocationId: storageLocationId === 'none' ? null : storageLocationId || undefined,
				purchasePrice,
				isAlter,
				isProxy,
				language,
				quantity,
				notes
			});
			return { success: true };
		} catch (e) {
			return fail(500, { message: e instanceof Error ? e.message : 'Failed to update card' });
		}
	}
} satisfies Actions;
