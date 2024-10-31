import type { Position, CurrentTime, LapTimes, Standings, StrengthOfField, Laps, Proximity, Relative, LapTime, DeltaTime, Telemetry, SessionState, Gap, TrackID, TrackMap, Gear, Speed, RPM, Active, GearRPM, Incidents } from "$lib/types/telemetry";
import { Channel, invoke } from "@tauri-apps/api/core";

import { readable } from 'svelte/store';

export const active = readable<Active>(false, (set) => {
    let active_channel = new Channel<Active>();
    active_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "active",
        onEvent: active_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "active",
        });
    };
});

export const currentTime = readable<CurrentTime>("--:--", (set) => {
    let current_time_channel = new Channel<CurrentTime>();
    current_time_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "current_time",
        onEvent: current_time_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "current_time",
        });
    };
});

export const lapTimes = readable<LapTimes>([], (set) => {
    let lap_times_channel = new Channel<LapTimes>();
    lap_times_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "player_lap_times",
        onEvent: lap_times_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "player_lap_times",
        });
    };
});

export const standings = readable<Standings>([], (set) => {
    let standings_channel = new Channel<Standings>();
    standings_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "standings",
        onEvent: standings_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "standings",
        });
    };
});

export const strengthOfField = readable<StrengthOfField>(0, (set) => {
    let strength_of_field_channel = new Channel<StrengthOfField>();
    strength_of_field_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "strength_of_field",
        onEvent: strength_of_field_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "strength_of_field",
        });
    };
});

export const positionsTotal = readable<Position>(0, (set) => {
    let positions_total_channel = new Channel<Position>();
    positions_total_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "positions_total",
        onEvent: positions_total_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "positions_total",
        });
    };
});

export const raceLaps = readable<Laps>(0, (set) => {
    let race_laps_channel = new Channel<Laps>();
    race_laps_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "race_laps",
        onEvent: race_laps_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "race_laps",
        });
    };
});

export const proximity = readable<Proximity>({ is_left: false, is_right: false }, (set) => {
    let proximity_channel = new Channel<Proximity>();
    proximity_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "proximity",
        onEvent: proximity_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "proximity",
        });
    };
});

export const relative = readable<Relative>([], (set) => {
    let relative_channel = new Channel<Relative>();
    relative_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "relative",
        onEvent: relative_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "relative",
        });
    };
});

export const lapTime = readable<LapTime>("–:––.–––", (set) => {
    let lap_time_channel = new Channel<LapTime>();
    lap_time_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "lap_time",
        onEvent: lap_time_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "lap_time",
        });
    };
});

export const deltaOptimalTime = readable<DeltaTime>("–", (set) => {
    let delta_optimal_time_channel = new Channel<DeltaTime>();
    delta_optimal_time_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "delta_optimal_time",
        onEvent: delta_optimal_time_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "delta_optimal_time",
        });
    };
});

export const deltaLastTime = readable<DeltaTime>("–", (set) => {
    let delta_last_time_channel = new Channel<DeltaTime>();
    delta_last_time_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "delta_last_time",
        onEvent: delta_last_time_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "delta_last_time",
        });
    };
});

export const telemetry = readable<Telemetry>({ ts: 0, throttle: 0, brake: 0, abs_active: false }, (set) => {
    let telemetry_channel = new Channel<Telemetry>();
    telemetry_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "telemetry",
        onEvent: telemetry_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "telemetry",
        });
    };
});

export const sessionState = readable<SessionState>("", (set) => {
    let session_state_channel = new Channel<SessionState>();
    session_state_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "session_state",
        onEvent: session_state_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "session_state",
        });
    };
});

export const gapNext = readable<Gap>("-", (set) => {
    let gap_next_channel = new Channel<Gap>();
    gap_next_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "gap_next",
        onEvent: gap_next_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "gap_next",
        });
    };
});

export const gapPrev = readable<Gap>("-", (set) => {
    let gap_prev_channel = new Channel<Gap>();
    gap_prev_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "gap_prev",
        onEvent: gap_prev_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "gap_prev",
        });
    };
});

export const trackID = readable<TrackID>(0, (set) => {
    let track_id_channel = new Channel<TrackID>();
    track_id_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "track_id",
        onEvent: track_id_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "track_id",
        });
    };
});

export const trackMap = readable<TrackMap>([], (set) => {
    let track_map_channel = new Channel<TrackMap>();
    track_map_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "track_map",
        onEvent: track_map_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "track_map",
        });
    };
});

export const gear = readable<Gear>("N", (set) => {
    let gear_channel = new Channel<Gear>();
    gear_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "gear",
        onEvent: gear_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "gear",
        });
    };
});

export const speed = readable<Speed>(0, (set) => {
    let speed_channel = new Channel<Speed>();
    speed_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "speed",
        onEvent: speed_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "speed",
        });
    };
});

export const rpm = readable<RPM>(0, (set) => {
    let rpm_channel = new Channel<RPM>();
    rpm_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "rpm",
        onEvent: rpm_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "rpm",
        });
    };
});

export const gearShiftRPM = readable<GearRPM>(0, (set) => {
    let gear_shift_rpm_channel = new Channel<GearRPM>();
    gear_shift_rpm_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "gear_shift_rpm",
        onEvent: gear_shift_rpm_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "gear_shift_rpm",
        });
    };
});

export const gearBlinkRPM = readable<GearRPM>(0, (set) => {
    let gear_blink_rpm_channel = new Channel<GearRPM>();
    gear_blink_rpm_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "gear_blink_rpm",
        onEvent: gear_blink_rpm_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "gear_blink_rpm",
        });
    };
});

export const lap = readable<Laps>(0, (set) => {
    let lap_channel = new Channel<Laps>();
    lap_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "lap",
        onEvent: lap_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "lap",
        });
    };
});

export const lapsTotal = readable<Laps>(0, (set) => {
    let laps_total_channel = new Channel<Laps>();
    laps_total_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "laps_total",
        onEvent: laps_total_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "laps_total",
        });
    };
});

export const position = readable<Position>(0, (set) => {
    let position_channel = new Channel<Position>();
    position_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "position",
        onEvent: position_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "position",
        });
    };
});

export const incidents = readable<Incidents>(0, (set) => {
    let incidents_channel = new Channel<Incidents>();
    incidents_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "incidents",
        onEvent: incidents_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "incidents",
        });
    };
});

export const incidentLimit = readable<Incidents>(0, (set) => {
    let incidents_limit_channel = new Channel<Incidents>();
    incidents_limit_channel.onmessage = (message) => {
        set(message);
    };

    invoke("register_event_emitter", {
        event: "incident_limit",
        onEvent: incidents_limit_channel,
    });

    return () => {
        invoke("unregister_event_emitter", {
            event: "incident_limit",
        });
    };
});
