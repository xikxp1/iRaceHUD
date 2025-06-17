CREATE TABLE telemetry_reference_brake_points (
    recording_id INTEGER NOT NULL,
    lap_dist INTEGER NOT NULL,
    PRIMARY KEY (recording_id, lap_dist)
);

CREATE INDEX idx_telemetry_reference_brake_points_recording_id ON telemetry_reference_brake_points (recording_id);
