CREATE TABLE telemetry_reference_meta (
    track_id INTEGER NOT NULL,
    car_class_id INTEGER NOT NULL,
    car_id INTEGER NOT NULL,
    recording_id INTEGER NOT NULL,
    PRIMARY KEY (track_id, car_class_id)
);

CREATE INDEX idx_telemetry_reference_meta_track_id ON telemetry_reference_meta (track_id);

CREATE TABLE telemetry_reference_data (
    recording_id INTEGER NOT NULL,
    lap_dist INTEGER NOT NULL,
    throttle INTEGER NOT NULL,
    brake INTEGER NOT NULL,
    steering_angle INTEGER NOT NULL,
    gear INTEGER NOT NULL,
    PRIMARY KEY (recording_id, lap_dist)
);
