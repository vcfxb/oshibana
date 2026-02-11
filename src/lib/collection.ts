export type StorageLocationType = 'binder' | 'box' | 'shelf' | 'physical_deck' | 'other';

export function getLocationTypeLabel(type: string): string {
	switch (type) {
		case 'binder':
			return 'Binder';
		case 'box':
			return 'Box';
		case 'shelf':
			return 'Shelf';
		case 'physical_deck':
			return 'Deck';
		default:
			return 'Other';
	}
}
