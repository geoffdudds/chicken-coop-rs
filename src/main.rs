mod chicken;
use crate::chicken::Chicken;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    // create a chicken object (updates at midnight)
    // future - add app overrides
    // create a gate controller (init based on schedule, eg isAwake(offset))
    // create gate driver

    let chicken = Chicken::new();
    if chicken.is_awake() {
        log::info!("the chickens are awake!");
    }
    log::info!("wake time: {}", Chicken::get_wake_time().unwrap());
    log::info!("bed time: {}", Chicken::get_bed_time().unwrap());
    log::info!("now: {}", Chicken::get_time());
}
