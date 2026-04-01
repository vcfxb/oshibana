import type { URI, UUID } from "./card";

export interface RelatedCard {
    id: UUID;
    object: 'related_card';
    component: string;
    name: string;
    type_line: string;
    uri: URI;
}
