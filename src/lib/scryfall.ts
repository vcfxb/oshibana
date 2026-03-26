import type { ScryfallCard } from './scryfall/card';
import type { ScryfallError } from './scryfall/error';
import type { ScryfallList } from './scryfall/list';
import type { ScryfallRuling } from './scryfall/rulings';
import type { ScryfallSymbol } from './scryfall/symbology';

const SCRYFALL_API_BASE = 'https://api.scryfall.com';
const MAX_REQUESTS_PER_SEC = 10;
const ONE_SECOND_MS = 1000;

export class ScryfallClient {
	protected request_timestamps: number[];

	constructor() {
		this.request_timestamps = [];
	}

	private gcTimestamps() {
		let now_ms = Date.now();

		this.request_timestamps = this.request_timestamps.filter(
			(timestamp) => timestamp >= now_ms - ONE_SECOND_MS
		);
	}

	private async waitForRatelimit() {
		this.gcTimestamps();

		let count = this.request_timestamps.length;
		if (count >= MAX_REQUESTS_PER_SEC) {
			let ninth_oldest = this.request_timestamps[count - 9];
			let next_available_time = ninth_oldest + ONE_SECOND_MS;
			let delta = next_available_time - Date.now();
			if (delta > 0) {
				console.warn(`waiting ${delta} ms for scryfall ratelimit`);
				await new Promise((resolve) => setTimeout(resolve, delta));
			}
		}
	}

	protected async scryfallFetch<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
		await this.waitForRatelimit();

		this.request_timestamps.push(Date.now());
		const response = await fetch(`${SCRYFALL_API_BASE}${endpoint}`, {
			...options,
			headers: {
				'User-Agent': `Oshibana/${__APP_VERSION__}`,
				Accept: 'application/json',
				...(options.method === 'POST' ? { 'Content-Type': 'application/json' } : {}),
				...(options.headers || {})
			}
		});

		const data = (await response.json()) as any;
		if (data.object === 'error') {
			throw data as ScryfallError;
		}

		return data as T;
	}

	async getSymbology() {
		return this.scryfallFetch<ScryfallList<ScryfallSymbol>>('/symbology');
	}

	async getRulings(id: string) {
		return this.scryfallFetch<ScryfallList<ScryfallRuling>>(`/cards/${id}/rulings`);
	}

	async searchCards(query: string) {
		return this.scryfallFetch<ScryfallList<ScryfallCard>>(
			`/cards/search?q=${encodeURIComponent(query)}`
		);
	}

	async getCardById(id: string) {
		return this.scryfallFetch<ScryfallCard>(`/cards/${id}`);
	}

	async getPrints(oracleId: string) {
		return this.searchCards(`oracle_id:${oracleId} unique:prints`);
	}

	async getLanguages(set: string, collectorNumber: string) {
		return this.scryfallFetch<ScryfallList<ScryfallCard>>(
			`/cards/search?q=${encodeURIComponent(`s:"${set}" cn:"${collectorNumber}" lang:any`)}&include_multilingual=true`
		);
	}

	async getCardsBatch(
		identifiers: { id?: string; name?: string; set?: string; collector_number?: string }[]
	) {
		return this.scryfallFetch<ScryfallList<ScryfallCard>>('/cards/collection', {
			method: 'POST',
			body: JSON.stringify({ identifiers })
		});
	}
}

const client = new ScryfallClient();

export const getSymbology = client.getSymbology.bind(client);
export const getRulings = client.getRulings.bind(client);
export const searchCards = client.searchCards.bind(client);
export const getCardById = client.getCardById.bind(client);
export const getPrints = client.getPrints.bind(client);
export const getLanguages = client.getLanguages.bind(client);
export const getCardsBatch = client.getCardsBatch.bind(client);

export type { ScryfallCard };
