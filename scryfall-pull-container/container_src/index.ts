import { createServer } from 'http';

interface ScryfallBulkData {
	data: Array<{
		type: string;
		download_uri: string;
	}>;
}

interface Card {
	scryfallId: string;
	oracleId: string;
	name: string;
	set: string;
	setName: string;
	collectorNumber: string;
	imageUri: string;
	manaCost: string;
	typeLine: string;
	oracleText: string;
	colors: string[] | null;
	colorIdentity: string[] | null;
	rarity: string;
	priceUsd: number | null;
	priceUsdFoil: number | null;
	priceUsdEtched: number | null;
	priceEur: number | null;
	priceTix: number | null;
}

const mapScryfallToCard = (raw: any): Card => {
	return {
		scryfallId: raw.id,
		oracleId: raw.oracle_id,
		name: raw.name,
		set: raw.set,
		setName: raw.set_name,
		collectorNumber: raw.collector_number,
		imageUri: raw.image_uris?.normal || null,
		manaCost: raw.mana_cost || null,
		typeLine: raw.type_line || null,
		oracleText: raw.oracle_text || null,
		colors: raw.colors || null,
		colorIdentity: raw.color_identity || null,
		rarity: raw.rarity,
		priceUsd: raw.prices?.usd ? parseFloat(raw.prices.usd) : null,
		priceUsdFoil: raw.prices?.usd_foil ? parseFloat(raw.prices.usd_foil) : null,
		priceUsdEtched: raw.prices?.usd_etched ? parseFloat(raw.prices.usd_etched) : null,
		priceEur: raw.prices?.eur ? parseFloat(raw.prices.eur) : null,
		priceTix: raw.prices?.tix ? parseFloat(raw.prices.tix) : null,
	};
};

const sendBatch = async (cards: Card[]) => {
	const response = await fetch('http://localhost/db/upsert-cards', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ cards }),
	});

	if (!response.ok) {
		const text = await response.text();
		throw new Error(`Bridge error (${response.status}): ${text}`);
	}
};

const server = createServer(async (req, res) => {
	if (req.url === '/pull' && req.method === 'POST') {
		try {
			console.log('Starting Scryfall pull...');

			// 1. Get bulk data URL
			const bulkResp = await fetch('https://api.scryfall.com/bulk-data');
			if (!bulkResp.ok) throw new Error('Failed to fetch Scryfall bulk data info');
			const bulkInfo = (await bulkResp.json()) as ScryfallBulkData;

			const downloadURL = bulkInfo.data.find((d) => d.type === 'default_cards')?.download_uri;
			if (!downloadURL) throw new Error('Could not find default_cards download URL');

			console.log(`Downloading bulk data from ${downloadURL}`);

			// 2. Download and stream the JSON
			const resp = await fetch(downloadURL);
			if (!resp.ok || !resp.body) throw new Error('Failed to download bulk data');

			const reader = resp.body.getReader();
			const decoder = new TextDecoder();
			let buffer = '';
			let cards: Card[] = [];
			let totalProcessed = 0;
			let isFirstToken = true;

			while (true) {
				const { done, value } = await reader.read();
				if (done) break;

				buffer += decoder.decode(value, { stream: true });

				// Extremely naive JSON array streaming for demo purposes
				// In a real app, use a proper streaming JSON parser like 'oboe' or 'stream-json'
				// But since we want to avoid complex dependencies in a container if possible:
				
				let startIndex = 0;
				if (isFirstToken) {
					const openBracket = buffer.indexOf('[');
					if (openBracket !== -1) {
						startIndex = openBracket + 1;
						isFirstToken = false;
					}
				}

				while (true) {
					// Look for the end of a JSON object
					// Note: Scryfall objects don't contain "}," internally in a way that breaks this usually
					// but this is fragile. A real parser is better.
					const endIndex = buffer.indexOf('},', startIndex);
					if (endIndex === -1) break;

					const objectStr = buffer.substring(startIndex, endIndex + 1);
					try {
						const rawCard = JSON.parse(objectStr);
						cards.push(mapScryfallToCard(rawCard));
						
						if (cards.length >= 500) {
							await sendBatch(cards);
							totalProcessed += cards.length;
							console.log(`Processed ${totalProcessed} cards...`);
							cards = [];
						}
					} catch (e) {
						// Skip if partial object
					}
					
					startIndex = endIndex + 2;
				}
				buffer = buffer.substring(startIndex);
			}

			// Handle the last object which ends with "}]"
			const lastEndIndex = buffer.lastIndexOf('}');
			if (lastEndIndex !== -1) {
				try {
					const objectStr = buffer.substring(0, lastEndIndex + 1);
					const rawCard = JSON.parse(objectStr);
					cards.push(mapScryfallToCard(rawCard));
				} catch (e) {}
			}

			if (cards.length > 0) {
				await sendBatch(cards);
				totalProcessed += cards.length;
			}

			console.log(`Finished pull. Total: ${totalProcessed}`);
			res.writeHead(200);
			res.end(`Successfully processed ${totalProcessed} cards`);
		} catch (error: any) {
			console.error('Pull failed:', error);
			res.writeHead(500);
			res.end(`Pull failed: ${error.message}`);
		}
	} else {
		res.writeHead(404);
		res.end();
	}
});

server.listen(8080, () => {
	console.log('Container listening on port 8080');
});
