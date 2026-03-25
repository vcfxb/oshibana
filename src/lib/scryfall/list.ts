import type { URI } from "./card";

export interface ScryfallList<T> {
    object: 'list';
    data: T[];
    has_more: boolean;
    next_page?: URI;
    total_cards?: number;
    warnings?: string[];
}
