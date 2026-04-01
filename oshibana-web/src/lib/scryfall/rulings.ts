import type { UUID } from "./card";

export interface ScryfallRuling {
    object: 'ruling';
    oracle_id: UUID;
    source: string;
    published_at: string;
    comment: string;
}
