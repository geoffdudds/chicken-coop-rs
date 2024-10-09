#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
// use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, system::SystemControl};
use esp_println::print;
// use sunrise::{sunrise_sunset, DawnType, SolarDay, SolarEvent};
use embassy_futures::join::join;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

mod coop;

// Declare a channel of 2 u32s
static GATE_POSITION: Channel<CriticalSectionRawMutex, u32, 2> = Channel::new();

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    esp_hal_embassy::init(
        &clocks,
        esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    // let mut count = 0;
    // loop {
    //     esp_println::println!("Main Task Count: {}", count);
    //     count += 1;
    //     Timer::after(Duration::from_millis(5_000)).await;
    // }

    spawner.spawn(run_app()).unwrap();
    let coop: coop::Coop = coop::Coop::new();

    join(coop.run(), GATE_POSITION.send(coop.door_position())).await;
    // loop {
    //     coop.is_time_to_wake_up().await;
    //     coop.turn_on_light();
    //     join(coop.open_door(), GATE_POSITION.send(coop.door_position())).await;
    //     coop.is_light_enough_to_see().await;
    //     coop.turn_off_light();
    //     coop.is_time_to_get_ready_for_bed().await;
    //     coop.turn_on_light();
    //     coop.close_door();
    //     coop.is_time_for_sleep().await;
    //     coop.turn_off_light();
    // }
}

#[embassy_executor::task]
async fn run_app() {
    loop {
        let posn = GATE_POSITION.receive().await;
        print!("position: {posn}");
    }
}

// #[embassy_executor::task]
// async fn print_circadia_stats() {
//     let dawn = SolarDay::new(43.6532, -79.3832, 2016, 1, 1)
//         .with_altitude(54.)
//         .event_time(SolarEvent::Dawn(DawnType::Civil));
// }

// #[embassy_executor::task]
// async fn one_second_task() {
//     let mut count = 0;

//     loop {
//         esp_println::println!("Spawn Task Count: {}", count);
//         count += 1;
//         Timer::after(Duration::from_millis(1_000)).await;
//     }
// }
