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

    // While loops
    let mut num = 0;
    while num < 10 {
        num += 1;
    }
    println!("num depois do while {num}");
}
