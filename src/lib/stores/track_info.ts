import { writable } from "svelte/store";

export const trackInfo = writable<{ [k: string]: any }>({});
