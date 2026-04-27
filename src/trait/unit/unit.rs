use crate::health::Health;

pub struct Unit {
    pub current_health: i32,
}

impl Health for Unit {
    fn get_current_health(&self) -> i32 {
        self.current_health
    }

    fn set_current_health(&mut self, new_health: i32) {
        self.current_health = new_health;
        println!("unit health is {}", self.current_health)
    }
}

impl Unit {
    pub fn new(max_health: i32) -> Self {
        Self {
            current_health: max_health,
        }
    }
}
