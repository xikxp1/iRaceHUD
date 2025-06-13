export type TrackSetting = {
    direction: number;
    offset: number;
    customTrackPath?: string;
}

export type TrackSettings = {
    [key: number]: TrackSetting;
}
