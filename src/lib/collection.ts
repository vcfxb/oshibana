export type StorageLocationType = 'binder' | 'box' | 'shelf' | 'physical_deck' | 'other';

export type CollectionSortBy = 'name' | 'date-added' | 'value' | 'purchase-price' | 'set';
export type SortDir = 'asc' | 'desc';

function parsePriceToCents(price: string | null | undefined): number | null {
	if (!price) return null;
	const parsed = parseFloat(price);
	return isNaN(parsed) ? null : Math.round(parsed * 100);
}

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
		if (useEur) {
			priceCents = parsePriceToCents(prices.eur);
		}

		if (priceCents == null) {
			if (physicalCard?.isFoil) {
				priceCents =
					parsePriceToCents(prices.usd_foil) ??
					parsePriceToCents(prices.usd_etched) ??
					parsePriceToCents(prices.usd) ??
					parsePriceToCents(prices.eur) ??
					parsePriceToCents(prices.tix);
			} else {
				priceCents =
					parsePriceToCents(prices.usd) ??
					parsePriceToCents(prices.usd_foil) ??
					parsePriceToCents(prices.usd_etched) ??
					parsePriceToCents(prices.eur) ??
					parsePriceToCents(prices.tix);
			}
		}
	} else {
		if (useEur) {
			priceCents = cardData.priceEur;
		}

		if (priceCents == null) {
			if (physicalCard?.isFoil) {
				priceCents =
					cardData.priceUsdFoil ??
					cardData.priceUsdEtched ??
					cardData.priceUsd ??
					cardData.priceEur ??
					cardData.priceTix;
			} else {
				priceCents =
					cardData.priceUsd ??
					cardData.priceUsdFoil ??
					cardData.priceUsdEtched ??
					cardData.priceEur ??
					cardData.priceTix;
			}
		}
	}

	if (priceCents == null) return '—';

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
