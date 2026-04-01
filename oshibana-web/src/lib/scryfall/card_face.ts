import type { UUID } from "./card";
import type { Color } from "./colors";
import type { CardImagery } from "./imagery";

export interface CardFace {
    artist?: string;
    artist_id?: UUID;
    cmc?: number;
    color_identity: Color[];
    color_indicator?: Color[];
    colors?: Color[];
    defense?: string;
    flavor_text?: string;
    illustration_id?: UUID;
    image_uris?: CardImagery;
    layout?: string;
    loyalty?: string;
    mana_cost: string;
    name: string;
    object: 'card_face';
    oracle_id?: UUID;
    oracle_text?: string;
    power?: string;
    printed_name?: string;
    printed_text?: string;
    printed_type_line?: string;
    toughness?: string;
    type_line?: string;
    watermark?: string;
}
