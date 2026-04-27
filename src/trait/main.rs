use crate::{health::Health, unit::Unit};

mod health;
mod unit;

fn main() {
    let hero = Unit::new(10);
    let enemy = Unit::new(10);

    let units: Vec<Box<dyn Health>> = vec![Box::new(hero), Box::new(enemy)];
    for unit in units {
        apply_damage(unit, 5);
    }
}

fn apply_damage(mut damageable: Box<dyn Health>, damage: i32) {
    damageable.set_current_health(damageable.get_current_health() - damage);
}
