pub struct CharacterOwnership {
    level: i32,
}

impl CharacterOwnership {
    pub fn new() -> Self {
        CharacterOwnership { level: 1 }
    }

    pub fn get_level(self) -> (Self, i32) {
        let level = self.level;
        (self, level)
    }

    pub fn level_up(mut self) -> Self {
        self.level += 1;
        self
    }

    pub fn show() {
        let amon = CharacterOwnership::new();

        let (mut amon, level) = amon.get_level();
        println!("Amon level is {}", level);

        println!("Amon defeated the boss!");

        amon = amon.level_up();
        let (_, level) = amon.get_level();

        println!("Amon level is {}", level);
    }
}
