import type { TrackSettings } from "$lib/types/track_settings";
import { getContext, setContext } from "svelte";

interface TrackSettingsState {
    data: TrackSettings;
    isLoading: boolean;
}

class TrackSettingsStateImpl implements TrackSettingsState {
    data: TrackSettings = $state<TrackSettings>({});
    isLoading: boolean = $state<boolean>(true);

    constructor() {
        this.initializeData();
    }

    private async initializeData() {
        try {
            const response = await fetch("/track_info_data/track_settings.json");
            const data = await response.json() as TrackSettings;
            this.data = data;
        } catch (error) {
            console.error("Error loading track settings:", error);
        } finally {
            this.isLoading = false;
        }
    }
}

const DEFAULT_KEY = '$_track_settings_state';

export const getTrackSettingsState = (key = DEFAULT_KEY) => {
    return getContext<TrackSettingsState>(key);
}

export const setTrackSettingsState = (key = DEFAULT_KEY) => {
    const trackSettingsState = new TrackSettingsStateImpl();
    setContext(key, trackSettingsState);
}
