import type { TrackInfos } from "$lib/types/track_info";
import { getContext, setContext } from "svelte";

interface TrackInfoState {
    data: TrackInfos;
    isLoading: boolean;
}

class TrackInfoStateImpl implements TrackInfoState {
    data: TrackInfos = $state<TrackInfos>({});
    isLoading: boolean = $state<boolean>(true);

    constructor() {
        this.initializeData();
    }

    private async initializeData() {
        try {
            const response = await fetch("/track_info_data/track_info.json");
            const data = await response.json() as TrackInfos;
            console.log(data);
            this.data = data;
        } catch (error) {
            console.error("Error loading track info:", error);
        } finally {
            this.isLoading = false;
        }
    }
}

const DEFAULT_KEY = '$_track_info_state';

export const getTrackInfoState = (key = DEFAULT_KEY) => {
    return getContext<TrackInfoState>(key);
}

export const setTrackInfoState = (key = DEFAULT_KEY) => {
    const trackInfoState = new TrackInfoStateImpl();
    setContext(key, trackInfoState);
}
