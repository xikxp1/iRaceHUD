import { writable } from "svelte/store";

export const trackSettings = writable<{ [k: string]: any }>({});
