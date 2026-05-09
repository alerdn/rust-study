pub struct Car {
    pub owner: String,
    pub year: u32,
}

impl Car {
    pub fn display_info(&self) {
        println!("{}, {}", self.owner, self.year)
    }
}
