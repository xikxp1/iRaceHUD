use std::time::Duration;

use iracing_telem::{Client, DataUpdateResult};

use anyhow::Result;
use log::debug;

pub fn connect() -> Result<(), iracing_telem::Error> {
    let mut c = Client::new();
    loop {
        debug!("start iRacing");
        unsafe {
            match c.wait_for_session(Duration::new(600, 0)) {
                None => {
                    println!("remember to start iRacing!");
                    return Ok(());
                }
                Some(mut s) => {
                    let gear = s.find_var("Gear").unwrap();
                    let speed = s.find_var("Speed").unwrap();
                    loop {
                        match s.wait_for_data(Duration::from_millis(20)) {
                            DataUpdateResult::Updated => {
                                // You can call value and it'll try and map the result to the relevent type
                                let gear_value: i32 = s.value(&gear)?;
                                let speed_value: f32 = s.value(&speed)?;
                                debug!("{:?} {:?}", gear_value, speed_value);
                            }
                            DataUpdateResult::NoUpdate => {
                                debug!("no update")
                            }
                            DataUpdateResult::FailedToCopyRow => {
                                debug!("too slow")
                            }
                            DataUpdateResult::SessionExpired => break,
                        }
                    }
                }
            }
        }
    }
}
