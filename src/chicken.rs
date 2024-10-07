// use chrono::prelude::*;
// use chrono::LocalResult;
// use esp_idf_svc::sys::__time_t;
// use std::time::SystemTime;
use sunrise::{DawnType, SolarDay, SolarEvent};

pub struct Chicken {
    // wake_time: Option<u32>,
    // bed_time: Option<u32>,
    awake: bool,
}

// todo: need a timer that is a future to wait on, then have an async process
// to update chicken info on events

impl Chicken {
    pub fn new() -> Self {
        Self {
            // wake_time: Option::None,
            // bed_time: Option::None,
            awake: false,
        }
    }

    async fn wake_and_sleep() {
        // day.dawn.await;
        // awake = true;
        // day.dusk.await;
        // awake = false;
    }
}

// impl Chicken {
// pub fn new() -> Self {
//     Self {
//         wake_time: Option::None,
//         bed_time: Option::None,
//     }
// }

//     pub fn is_awake(&self) -> bool {
//         let secs_from_midnight = Local::now().num_seconds_from_midnight();
//         secs_from_midnight > self.wake_time.unwrap() && secs_from_midnight < self.bed_time.unwrap()
//     }

//     pub fn get_time() -> u32 {
//         Local::now().num_seconds_from_midnight()
//     }

//     pub fn get_bed_time() -> Option<u32> {
//         // need to offset so its relative to the start of a day
//         let date_time: DateTime<Local> = Local::now();
//         let dusk = SolarDay::new(
//             49.1659,
//             -123.9401,
//             date_time.year(),
//             date_time.month(),
//             date_time.day(),
//         )
//         .event_time(SolarEvent::Dusk(DawnType::Civil));

//         // convert unix timestamp into secs since midnight
//         match Local.timestamp_opt(dusk, 0) {
//             LocalResult::Single(dusk_time) => Some(dusk_time.num_seconds_from_midnight()),
//             _ => None,
//         }
//     }

//     pub fn get_wake_time() -> Option<u32> {
//         // need to offset so its relative to the start of a day
//         let date_time: DateTime<Local> = Local::now();
//         let dawn = SolarDay::new(
//             49.1659,
//             -123.9401,
//             date_time.year(),
//             date_time.month(),
//             date_time.day(),
//         )
//         .event_time(SolarEvent::Dawn(DawnType::Civil));

//         // convert unix timestamp into secs since midnight
//         match Local.timestamp_opt(dawn, 0) {
//             LocalResult::Single(dawn_time) => Some(dawn_time.num_seconds_from_midnight()),
//             _ => None,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_is_up() {
//         assert!(false);
//     }
// }
