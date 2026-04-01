import type { URI } from "./card";

export interface Prices {
    usd?: string;
    usd_foil?: string;
    usd_etched?: string;
    eur?: string;
    eur_foil?: string;
    eur_etched?: string;
    tix?: string;   
}

export interface PurchaseUris {
    tcgplayer?: URI;
    cardmarket?: URI;
    cardhoarder?: URI;
}