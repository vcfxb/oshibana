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

export function formatCurrentPrice(
	cardData: any,
	physicalCard?: any,
	locale: string = 'en-US'
): string {
	if (!cardData) return '—';

	// Heuristic for EUR vs USD based on locale
	const euroLocales = [
		'at',
		'be',
		'cy',
		'ee',
		'fi',
		'fr',
		'de',
		'gr',
		'ie',
		'it',
		'lv',
		'lt',
		'lu',
		'mt',
		'nl',
		'pt',
		'sk',
		'si',
		'es'
	];
	const region = locale.split('-')[1]?.toLowerCase();
	const lang = locale.split('-')[0].toLowerCase();
	const useEur =
		euroLocales.includes(region) ||
		lang === 'de' ||
		lang === 'fr' ||
		lang === 'it' ||
		lang === 'es';

	let priceCents: number | null = null;

	// Handle both Scryfall API format and our DB cache format
	const isScryfallFormat = !!cardData.prices;

	if (isScryfallFormat) {
		const prices = cardData.prices;
		const rawPrice = useEur
			? prices.eur
			: physicalCard?.isFoil
				? (prices.usd_foil ?? prices.usd)
				: (prices.usd ?? prices.usd_foil);
		if (rawPrice) {
			const parsed = parseFloat(rawPrice);
			if (!isNaN(parsed)) {
				priceCents = Math.round(parsed * 100);
			}
		}
	} else {
		if (useEur) {
			priceCents = cardData.priceEur;
		} else {
			// Prefer foil price if the physical card is foil
			priceCents = physicalCard?.isFoil
				? (cardData.priceUsdFoil ?? cardData.priceUsd)
				: (cardData.priceUsd ?? cardData.priceUsdFoil);
		}
	}

	if (priceCents === null || priceCents === undefined) return '—';

	try {
		return new Intl.NumberFormat(locale, {
			style: 'currency',
			currency: useEur ? 'EUR' : 'USD'
		}).format(priceCents / 100);
	} catch (e) {
		return (useEur ? '€' : '$') + (priceCents / 100).toFixed(2);
	}
}

export function formatPrice(
	price: string | number | null | undefined,
	currency: 'USD' | 'EUR' | 'TIX',
	locale: string = 'en-US'
): string {
	if (price === null || price === undefined) return '—';

	let amount: number;
	if (typeof price === 'string') {
		amount = parseFloat(price);
	} else {
		amount = price / 100;
	}

	if (isNaN(amount)) return '—';

	try {
		return new Intl.NumberFormat(locale, {
			style: 'currency',
			currency: currency
		}).format(amount);
	} catch (e) {
		const symbols = { USD: '$', EUR: '€', TIX: 'TIX ' };
		return (symbols[currency] || '') + amount.toFixed(2);
	}
}
