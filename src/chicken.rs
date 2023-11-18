use std::time::SystemTime;

pub struct Chicken {
    wake_up_time: SystemTime,
    bed_time: SystemTime,
}
impl Chicken {
    pub fn new() -> Self {
        Self {
            wake_up_time: Chicken::get_bed_time(),
            bed_time: Chicken::get_bed_time(),
        }
    }

    pub fn is_awake(&self) -> bool {
        let now = SystemTime::now();
        now > self.wake_up_time && now < self.bed_time
    }

    pub fn get_bed_time() -> SystemTime {
        SystemTime::now()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_up() {
        assert!(false);
    }
}
