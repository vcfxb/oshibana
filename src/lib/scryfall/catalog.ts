import type { URI } from "./card";

export interface ScryfallCatalog {
    object: 'catalog';
    uri: URI;
    total_values: number;
    data: string[];
}
