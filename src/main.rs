#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, system::SystemControl};
use sunrise::{sunrise_sunset, DawnType, SolarDay, SolarEvent};

mod chicken;
use crate::chicken::Chicken;

#[embassy_executor::task]
async fn one_second_task() {
    let mut count = 0;

    loop {
        esp_println::println!("Spawn Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    esp_hal_embassy::init(
        &clocks,
        esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    spawner.spawn(one_second_task()).unwrap();

    let mut count = 0;
    loop {
        esp_println::println!("Main Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

#[embassy_executor::task]
async fn print_circadia_stats() {
    let dawn = SolarDay::new(43.6532, -79.3832, 2016, 1, 1)
        .with_altitude(54.)
        .event_time(SolarEvent::Dawn(DawnType::Civil));
}
