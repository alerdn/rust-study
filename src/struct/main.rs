use crate::entity::{Car, Vector2};

mod entity;

fn main() {
    //* Bacis structs
    let my_car = Car {
        owner: String::from("Alexandre"),
        year: 2026,
    };

    my_car.display_info();

    let other_car = Car { ..my_car };
    println!(
        "Other car created using my_car attributes - onwer: {}, year: {}",
        other_car.owner, other_car.year
    );

    // println!("Cannot access my_car.owner because it was moved to other_car: {}", my_car.owner);
    // That is because String is a pointer to a text in the heap memory, so when assigned to another variable, the original becomes invalid

    //* Tuple structs
    let simple_tuple = (1, 3);
    println!(
        "Printing tuple value, but unclear what they represent: ({}, {})",
        simple_tuple.0, simple_tuple.1
    );

    let tuple_struct = Vector2(1920, 1080);
    println!("Resolution: ({}, {})", tuple_struct.0, tuple_struct.1);
}
