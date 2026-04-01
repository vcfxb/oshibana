import type { CardFace } from './card_face';
import type { Color } from './colors';
import type { CardImagery, ImageStatus } from './imagery';
import type { Legality } from './legality';
import type { Prices, PurchaseUris } from './prices';
import type { Rarity } from './rarity';
import type { RelatedCard } from './related_card';
import type { RelatedUris } from './related_uris';
import type { SecurityStamp } from './security_stamp';

export type UUID = string & { readonly brand: 'UUID' };
export type URI = string & { readonly brand: 'URI' };

export interface ScryfallCardCore {
	arena_id?: number;
	id: UUID;
	lang: string;
	mtgo_id?: number;
	mtgo_foil_id?: number;
	multiverse_ids?: number[];
	resource_id?: string;
	tcgplayer_id?: number;
	tcgplayer_etched_id?: number;
	cardmarket_id?: number;
	object: 'card';
	layout: string;

	/**
	 * A unique ID for this card’s oracle identity.
	 * This value is consistent across reprinted card
	 * editions, and unique among different cards with
	 * the same name (tokens, Unstable variants, etc).
	 * Always present except for the reversible_card layout
	 * where it will be absent; oracle_id will be found
	 * on each face instead.
	 */
	oracle_id?: UUID;

	prints_search_uri: URI;
	rulings_uri: URI;
	scryfall_uri: URI;
	uri: URI;
}

export interface ScryfallCardGameplay {
	all_parts?: RelatedCard[];
	card_faces?: CardFace[];
	cmc: number;
	color_identity: Color[];
	color_indicator?: Color[];
	colors?: Color[];
	defense?: string;
	edhrec_rank?: number;
	game_changer?: boolean;
	hand_modifier?: string;
	keywords: string[];
	legalities: Record<string, Legality>;
	life_modifier?: string;
	loyalty?: string;
	mana_cost?: string;
	name: string;
	oracle_text?: string;
	penny_rank?: number;
	power?: string;
	produced_mana?: Color[];
	reserved: boolean;
	toughness?: string;
	type_line: string;
}

export interface ScryfallCardPrint {
	artist?: string;
	artist_ids?: UUID[];
	attraction_lights?: number[];
	booster: boolean;
	border_color: string;
	card_back_id: UUID;
	collector_number: string;
	content_warning?: boolean;
	digital: boolean;
	finishes: string[];
	flavor_name?: string;
	flavor_text?: string;
	frame_effects?: string[];
	frame: string;
	full_art: boolean;
	games: string[];
	highres_image: boolean;
	illustration_id?: UUID;
	image_status: ImageStatus;
	image_uris?: CardImagery;
	oversized: boolean;
	prices: Prices;
	printed_name?: string;
	printed_text?: string;
	printed_type_line?: string;
	promo: boolean;
	promo_types?: string[];
	purchase_uris?: PurchaseUris;
	rarity: Rarity;
	related_uris: RelatedUris;
	released_at: string;
	reprint: boolean;
	scryfall_set_uri: URI;
	set: string;
	set_name: string;
	set_search_uri: URI;
	set_type: string;
	set_uri: URI;
	set_id: UUID;
	story_spotlight: boolean;
	textless: boolean;
	variation: boolean;
	variation_of?: UUID;
	security_stamp?: SecurityStamp;
	watermark?: string;
	preview?: {
		previewed_at?: string;
		source_url?: URI;
		souce?: string;
	};
}

export type ScryfallCard = ScryfallCardCore & ScryfallCardGameplay & ScryfallCardPrint;

export interface ScryfallSearchParams {
	q: string;
	unique?: 'cards' | 'art' | 'prints';
	order?: 
		| 'name'
		| 'set'
		| 'released'
		| 'rarity'
		| 'color'
		| 'tix'
		| 'eur'
		| 'usd'
		| 'cmc'
		| 'power'
		| 'toughness'
		| 'edhrec'
		| 'penny'
		| 'artist'
		| 'review';
	dir?: 'auto' | 'asc' | 'desc';
	include_extras?: 'true' | 'false';
	include_multilingual?: 'true' | 'false';
	include_variations?: 'true' | 'false';
	page?: number;
}
