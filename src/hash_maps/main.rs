use std::collections::{HashMap, hash_map::Entry};

fn main() {
    let fake_map = vec![("name", "Alexandre"), ("idade", "26")];
    println!("Vec<()>: {:?}", fake_map);

    let mut real_map: HashMap<&str, &str> = HashMap::new();
    real_map.insert("name", "Alexandre");
    real_map.insert("idade", "26");

    match real_map.entry("cargo") {
        Entry::Occupied(o) => {
            println!("Cargo já foi adicionado: {}", o.get());
        }
        Entry::Vacant(v) => {
            v.insert("Programador");
        }
    }

    match real_map.entry("idade") {
        Entry::Occupied(o) => {
            println!("Idade já foi adicionada: {}", o.get());
        }
        Entry::Vacant(v) => {
            v.insert("Programador");
        }
    }

    println!("HashMap: {:?}", real_map);
}
