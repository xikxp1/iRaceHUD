use std::sync::{Mutex, OnceLock};

use log::info;
use rand::Rng;
use serde::{Serialize, Serializer};
use specta::Type;
use tauri::Manager;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;
use crate::{APP_HANDLE, db};

static RECORDING_META_DATA: OnceLock<RecordingMetaData> = OnceLock::new();
static RECORDING_DATA: OnceLock<Mutex<RecordingData>> = OnceLock::new();

#[derive(Debug)]
struct RecordingMetaData {
    track_id: u32,
    car_class_id: u32,
    car_id: u32,
    recording_id: u32,
}

#[derive(Debug)]
struct RecordingData {
    telemetry: Vec<TelemetryReference>,
}

#[derive(Default, Type, PartialEq, Debug, sqlx::FromRow, Clone)]
pub struct TelemetryReference {
    lap_dist: u32, // in cm
    throttle: u32,
    brake: u32,
    steering_angle: i32, // in radian * 100
    gear: i32,
}

// Custom serialization to ensure we get a MessagePack map/object
impl Serialize for TelemetryReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("lap_dist", &self.lap_dist)?;
        map.serialize_entry("throttle", &self.throttle)?;
        map.serialize_entry("brake", &self.brake)?;
        map.serialize_entry("steering_angle", &self.steering_angle)?;
        map.serialize_entry("gear", &self.gear)?;
        map.end()
    }
}

impl EmittableEvent for TelemetryReference {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let telemetry = TelemetryReference {
            lap_dist: session.lap_dist,
            throttle: session.throttle,
            brake: session.brake,
            steering_angle: session.steering_angle,
            gear: session.gear,
        };
        Box::new(telemetry)
    }

    fn start_recording(&self, session: &SessionData) {
        let meta_data = RecordingMetaData {
            track_id: session.track_id,
            car_class_id: session.player_car_class,
            car_id: session.player_car_id.unwrap_or(0),
            recording_id: rand::rng().random::<u32>(),
        };
        RECORDING_META_DATA.set(meta_data).unwrap();
    }

    fn record(&self, session: &SessionData) {
        let recording_data = RECORDING_DATA.get_or_init(|| {
            Mutex::new(RecordingData {
                telemetry: Vec::new(),
            })
        });
        let telemetry = TelemetryReference {
            lap_dist: session.lap_dist,
            throttle: session.throttle,
            brake: session.brake,
            steering_angle: session.steering_angle,
            gear: session.gear,
        };
        recording_data.lock().unwrap().telemetry.push(telemetry);
    }

    async fn stop_recording(&self, _session: &SessionData) {
        let meta_data = RECORDING_META_DATA.get().unwrap();
        let telemetry_data = {
            let recording_data = RECORDING_DATA.get().unwrap();
            let mut recording_data = recording_data.lock().unwrap();
            let data = recording_data.telemetry.clone();
            recording_data.telemetry.clear();
            data
        };

        let db: tauri::State<'_, db::DatabaseState> = APP_HANDLE
            .get()
            .unwrap()
            .try_state::<db::DatabaseState>()
            .unwrap();

        let mut tx = db.0.begin().await.unwrap();

        let stmt = r#"
            INSERT OR REPLACE INTO telemetry_reference_data
            (recording_id, lap_dist, throttle, brake, steering_angle, gear)
            VALUES ($1, $2, $3, $4, $5, $6);
        "#;

        for telemetry in telemetry_data.iter() {
            let _ = sqlx::query(stmt)
                .bind(meta_data.recording_id)
                .bind(telemetry.lap_dist)
                .bind(telemetry.throttle)
                .bind(telemetry.brake)
                .bind(telemetry.steering_angle)
                .bind(telemetry.gear)
                .execute(&mut *tx)
                .await
                .unwrap();
        }

        let stmt = r#"
            INSERT OR REPLACE INTO telemetry_reference_meta
            (track_id, car_class_id, car_id, recording_id)
            VALUES ($1, $2, $3, $4);
        "#;

        let _ = sqlx::query(stmt)
            .bind(meta_data.track_id)
            .bind(meta_data.car_class_id)
            .bind(meta_data.car_id)
            .bind(meta_data.recording_id)
            .execute(&mut *tx)
            .await
            .unwrap();

        tx.commit().await.unwrap();
        info!("Telemetry reference recording stopped");
    }
}
