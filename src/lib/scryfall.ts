export interface ScryfallImageUris {
	small: string;
	normal: string;
	large: string;
	png: string;
	art_crop: string;
	border_crop: string;
}

export interface ScryfallCardFace {
	name: string;
	mana_cost: string;
	type_line: string;
	oracle_text?: string;
	colors?: string[];
	image_uris?: ScryfallImageUris;
}

export interface ScryfallCard {
	id: string;
	oracle_id: string;
	multiverse_ids?: number[];
	mtgo_id?: number;
	tcgplayer_id?: number;
	cardmarket_id?: number;
	name: string;
	lang: string;
	released_at: string;
	uri: string;
	scryfall_uri: string;
	layout: string;
	highres_image: boolean;
	image_status: string;
	image_uris?: ScryfallImageUris;
	mana_cost?: string;
	cmc: number;
	type_line: string;
	oracle_text?: string;
	colors?: string[];
	color_identity: string[];
	keywords: string[];
	card_faces?: ScryfallCardFace[];
	legalities: Record<string, string>;
	games: string[];
	reserved: boolean;
	foil: boolean;
	nonfoil: boolean;
	finishes: string[];
	oversized: boolean;
	promo: boolean;
	reprint: boolean;
	variation: boolean;
	set_id: string;
	set: string;
	set_name: string;
	set_type: string;
	set_uri: string;
	set_search_uri: string;
	scryfall_set_uri: string;
	rulings_uri: string;
	prints_search_uri: string;
	collector_number: string;
	digital: boolean;
	rarity: string;
	flavor_text?: string;
	card_back_id: string;
	artist: string;
	artist_ids: string[];
	illustration_id?: string;
	border_color: string;
	frame: string;
	full_art: boolean;
	textless: boolean;
	booster: boolean;
	story_spotlight: boolean;
	edhrec_rank?: number;
	penny_rank?: number;
	game_changer?: boolean;
	prices: Record<string, string | null>;
	related_uris: Record<string, string>;
}

export interface ScryfallList<T> {
	object: 'list';
	total_cards?: number;
	has_more: boolean;
	next_page?: string;
	data: T[];
}

export interface ScryfallSymbol {
	symbol: string;
	loose_variant: string | null;
	english: string;
	transposable: boolean;
	represents_mana: boolean;
	cmc: number | null;
	appears_in_mana_costs: boolean;
	funny: boolean;
	colors: string[];
	gatherer_alternates?: string[];
	svg_uri?: string;
}

export interface ScryfallRuling {
	object: 'ruling';
	oracle_id: string;
	source: string;
	published_at: string;
	comment: string;
}

const SCRYFALL_API_BASE = 'https://api.scryfall.com';

async function scryfallFetch<T>(endpoint: string): Promise<T> {
	const response = await fetch(`${SCRYFALL_API_BASE}${endpoint}`, {
		headers: {
			'User-Agent': `Oshibana/${__APP_VERSION__}`,
			Accept: 'application/json'
		}
	});

	if (!response.ok) {
		const error = (await response.json()) as { details?: string };
		throw new Error(error.details || `Failed to fetch from Scryfall: ${response.statusText}`);
	}

	return response.json() as Promise<T>;
}

export async function getSymbology(): Promise<ScryfallList<ScryfallSymbol>> {
	return scryfallFetch('/symbology');
}

export async function getRulings(id: string): Promise<ScryfallList<ScryfallRuling>> {
	return scryfallFetch(`/cards/${id}/rulings`);
}

export async function searchCards(query: string): Promise<ScryfallList<ScryfallCard>> {
	return scryfallFetch(`/cards/search?q=${encodeURIComponent(query)}`);
}

export async function getCardById(id: string): Promise<ScryfallCard> {
	return scryfallFetch(`/cards/${id}`);
}

export async function getCardByNamed(name: string, fuzzy = true): Promise<ScryfallCard> {
	const type = fuzzy ? 'fuzzy' : 'exact';
	return scryfallFetch(`/cards/named?${type}=${encodeURIComponent(name)}`);
}

export async function getPrints(oracleId: string): Promise<ScryfallList<ScryfallCard>> {
	return scryfallFetch(`/cards/search?order=released&q=oracleid%3A${oracleId}&unique=prints`);
}

export async function getRandomCard(): Promise<ScryfallCard> {
	return scryfallFetch('/cards/random');
}
