import { Container, getContainer, getRandom } from "@cloudflare/containers";
import { Hono } from "hono";

export class ScryfallPullContainer extends Container<Env> {
	// Port the container listens on (default: 8080)
	defaultPort = 8080;
	// Time before container sleeps due to inactivity (default: 30s)
	sleepAfter = "15m";
	// Environment variables passed to the container
	envVars = {
		MESSAGE: "Scryfall Pull Container",
	};

	// Optional lifecycle hooks
	override onStart() {
		console.log("Scryfall Pull Container successfully started");
	}

	override onStop() {
		console.log("Scryfall Pull Container successfully shut down");
	}

	override onError(error: unknown) {
		console.log("Scryfall Pull Container error:", error);
	}
}

// Create Hono app with proper typing for Cloudflare Workers
const app = new Hono<{
	Bindings: Env;
}>();

// Home route with available endpoints
app.get("/", (c) => {
	return c.text(
		"Scryfall Pull Worker\n" +
			"GET /pull - Trigger a pull of Scryfall bulk data\n",
	);
});

// Route requests to a specific container using the container ID
app.get("/pull", async (c) => {
	const containerId = c.env.SCRYFALL_PULL_CONTAINER.idFromName("singleton-puller");
	const container = c.env.SCRYFALL_PULL_CONTAINER.get(containerId);
	return await container.fetch(new Request(new URL("/pull", c.req.url), { method: "POST" }));
});

// Bridge for the container to talk back to D1
app.post("/db/upsert-cards", async (c) => {
	const body = await c.req.json<{ cards: any[] }>();
	const { cards } = body;

	if (!cards || !Array.isArray(cards)) {
		return c.json({ error: "Invalid cards data" }, 400);
	}

	console.log(`Upserting ${cards.length} cards...`);

	// We use raw SQL for efficiency in bulk upserts with D1
	// Scryfall data can be large, so we batch inserts.
	// D1 has a limit on parameters per statement (approx 100 on some platforms, 32k on SQLite, but D1 has its own limits).
	// We'll do batches of 50.

	const batchSize = 50;
	for (let i = 0; i < cards.length; i += batchSize) {
		const batch = cards.slice(i, i + batchSize);
		
		const placeholders = batch.map(() => "(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, unixepoch())").join(", ");
		const query = `
			INSERT INTO cards (
				scryfall_id, oracle_id, name, "set", set_name, collector_number, 
				image_uri, mana_cost, type_line, oracle_text, colors, color_identity, 
				rarity, price_usd, price_usd_foil, price_usd_etched, price_eur, price_tix, updated_at
			) VALUES ${placeholders}
			ON CONFLICT (scryfall_id) DO UPDATE SET
				oracle_id = excluded.oracle_id,
				name = excluded.name,
				"set" = excluded."set",
				set_name = excluded.set_name,
				collector_number = excluded.collector_number,
				image_uri = excluded.image_uri,
				mana_cost = excluded.mana_cost,
				type_line = excluded.type_line,
				oracle_text = excluded.oracle_text,
				colors = excluded.colors,
				color_identity = excluded.color_identity,
				rarity = excluded.rarity,
				price_usd = excluded.price_usd,
				price_usd_foil = excluded.price_usd_foil,
				price_usd_etched = excluded.price_usd_etched,
				price_eur = excluded.price_eur,
				price_tix = excluded.price_tix,
				updated_at = unixepoch()
		`;

		const params: any[] = [];
		batch.forEach(card => {
			params.push(
				card.scryfallId,
				card.oracleId,
				card.name,
				card.set,
				card.setName,
				card.collectorNumber,
				card.imageUri,
				card.manaCost,
				card.typeLine,
				card.oracleText,
				card.colors ? JSON.stringify(card.colors) : null,
				card.colorIdentity ? JSON.stringify(card.colorIdentity) : null,
				card.rarity,
				card.priceUsd ? Math.round(card.priceUsd * 100) : null,
				card.priceUsdFoil ? Math.round(card.priceUsdFoil * 100) : null,
				card.priceUsdEtched ? Math.round(card.priceUsdEtched * 100) : null,
				card.priceEur ? Math.round(card.priceEur * 100) : null,
				card.priceTix ? Math.round(card.priceTix * 100) : null
			);
		});

		try {
			await c.env.DB.prepare(query).bind(...params).run();
		} catch (err: any) {
			console.error("Error inserting batch:", err.message);
			return c.json({ error: err.message }, 500);
		}
	}

	return c.json({ success: true, count: cards.length });
});

export default app;
