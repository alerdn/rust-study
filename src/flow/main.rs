fn main() {
    // 'outer é a label do loop, usado para para um loop específico
    let loop_result = 'outer: loop {
        loop {
            println!("Simple loop");
            break 'outer 1;
        }
    };

    println!(
        "Loops são expression, então podem retornar valor: {{'outer: {}}}",
        loop_result
    );

    // For loops
    let vec = vec![1, 2, 3, 4, 5];
    for i in vec {
        println!("{i}");
    }

    println!("For with range [0, 5)");
    for i in 0..5 {
        println!("{i}");
    }

    println!("For with range [0, 5]");
    for i in 0..=5 {
        println!("{i}");
    }

    println!("Range rev");
    for i in (0..5).rev() {
        println!("{i}");
    }

    println!("Range step by 2");
    for i in (0..=10).step_by(2) {
        println!("{i}")
    }

    // While loops
    let mut num = 0;
    while num < 10 {
        num += 1;
    }
    println!("num depois do while {num}");
}
