export type TrackInfo = {
    trackId: number;
    trackName: string;
    configName: string;
    activePath: string;
    turns: { textContent: string, transform: string }[];
}

export type TrackInfos = {
    [key: number]: TrackInfo;
}
