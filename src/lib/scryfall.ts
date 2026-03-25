
import type { ScryfallCard } from "./scryfall/card";
import type { ScryfallError } from "./scryfall/error";
import type { ScryfallList } from "./scryfall/list";
import type { ScryfallRuling } from "./scryfall/rulings";
import type { ScryfallSymbol } from "./scryfall/symbology";

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

		this.request_timestamps = this.request_timestamps.filter((timestamp) => {
			timestamp >= now_ms - ONE_SECOND_MS
		});
	}

	private async waitForRatelimit() {
		this.gcTimestamps();

		let count = this.request_timestamps.length;
		if (count >= MAX_REQUESTS_PER_SEC) {
			let ninth_oldest = this.request_timestamps[count - 9];
			let next_available_time = ninth_oldest + ONE_SECOND_MS;
			let delta = next_available_time - Date.now();
			console.warn(`waiting ${delta} ms for scryfall ratelimit`);
			await new Promise((resolve) => setTimeout(resolve, delta));
		}
	}

	protected async scryfallFetch<T>(endpoint: string, options: RequestInit = {}): Promise<T | ScryfallError> {
		await this.waitForRatelimit();

		const response = await fetch(`${SCRYFALL_API_BASE}${endpoint}`, {
			...options,
			headers: {
				'User-Agent': `Oshibana/${__APP_VERSION__}`,
				Accept: 'application/json',
				...(options.headers || {})
			}
		});

		this.request_timestamps.push(Date.now());
		return response.json() as Promise<T | ScryfallError>;
	}

	async getSymbology() {
		return this.scryfallFetch<ScryfallList<ScryfallSymbol>>('/symbology');
	}

	async getRulings(id: string) {
		return this.scryfallFetch<ScryfallList<ScryfallRuling>>(`/cards/${id}/rulings`);
	}

	async searchCards(query: string) {
		return this.scryfallFetch<ScryfallList<ScryfallCard>>(`/cards/search?q=${encodeURIComponent(query)}`);
	}

	async getCardById(id: string) {
		return this.scryfallFetch<ScryfallCard>(`/cards/${id}`);
	}
}

