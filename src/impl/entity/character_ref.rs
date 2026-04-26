pub struct CharacterRef {
    level: i32,
}

impl CharacterRef {
    pub fn new() -> Self {
        CharacterRef { level: 1 }
    }

    pub fn get_level(&self) -> i32 {
        self.level
    }

    pub fn level_up(&mut self) {
        self.level += 1;
    }

    pub fn show() {
        let mut amon = CharacterRef::new();

        println!("Amon level is {}", amon.get_level());

        println!("Amon defeated the boss!");

        amon.level_up();

        println!("Amon level is {}", amon.get_level());
    }
}
