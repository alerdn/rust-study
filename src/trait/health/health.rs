pub trait Health {
    fn get_current_health(&self) -> i32;
    fn set_current_health(&mut self, new_health: i32);
}
