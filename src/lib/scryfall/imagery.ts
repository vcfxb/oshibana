import type { URI } from "./card";

export interface CardImagery {
	small: URI;
	normal: URI;
	large: URI;
	png: URI;
	art_crop: URI;
	border_crop: URI;
}

export type ImageStatus = 'missing' | 'placeholder' | 'lowres' | 'highres_scan';
