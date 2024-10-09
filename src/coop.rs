mod chicken;
use crate::coop::chicken::Chicken;

pub struct Coop {
    chicken: Chicken,
}

impl Coop {
    pub fn new() -> Coop {
        Coop {
            // chicken: { Chicken { awake: false } },
            chicken: Chicken::new(),
        }
    }

    pub async fn run(&self) {
        loop {
            self.is_time_to_wake_up().await;
            self.turn_on_light();
            self.open_door();
            // join(self.open_door(), GATE_POSITION.send(self.door_position())).await;
            self.is_light_enough_to_see().await;
            self.turn_off_light();
            self.is_time_to_get_ready_for_bed().await;
            self.turn_on_light();
            self.close_door();
            self.is_time_for_sleep().await;
            self.turn_off_light();
        }
    }

    pub async fn is_time_to_wake_up(&self) {}
    pub async fn is_light_enough_to_see(&self) {}
    pub async fn is_time_to_get_ready_for_bed(&self) {}
    pub async fn is_time_for_sleep(&self) {}
    pub fn turn_on_light(&self) {}
    pub fn turn_off_light(&self) {}
    pub fn open_door(&self) {}
    pub fn close_door(&self) {}
    pub fn door_position(&self) -> u32 {
        1
    }
}
