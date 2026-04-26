fn main() {
    let sum = calculate(1, 2);
    println!("sum: {}", sum);

    let is_alive = check_health(100);
    println!("is_alive: {}", is_alive);

    let points = 99;
    let rank = validate_rank(points);
    println!("rank: {}", rank);

    let mut events = vec![
        String::from("Batalha iniciou"),
        String::from("Jogador atacou"),
    ];
    add_event(&mut events, String::from("Inimigo derrotado"));
    println!("events: {:?}", events);
}

fn calculate(a: i32, b: i32) -> i32 {
    a + b
}

fn check_health(health: i32) -> bool {
    if health > 0 { true } else { false }
}

fn validate_rank(points: i32) -> char {
    match points {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

fn add_event(events: &mut Vec<String>, event: String) {
    events.push(event);
}
