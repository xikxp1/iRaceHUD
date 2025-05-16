export type TrackInfo = {
    trackId: number;
    trackName: string;
    configName: string;
    activePath: string;
}

export type TrackInfos = {
    [key: number]: TrackInfo;
}
