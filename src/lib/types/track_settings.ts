export type TrackSetting = {
    direction: number;
    offset: number;
}

export type TrackSettings = {
    [key: number]: TrackSetting;
}
