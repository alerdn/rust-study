fn main() {
    // Array é fixo
    let array_1 = [1, 2, 3];
    let item_1 = array_1[0];

    // {:?} usa o trait debug do tipo
    println!("array_1: {:?}", array_1);
    println!("item_1: {}", item_1);

    // Vec é dinâmico
    let mut vec_1 = vec![1, 2, 3];
    vec_1.push(4);
    let item_vec_4 = vec_1[3];

    println!("vec_1: {:?}", vec_1);
    println!("item_vec_1: {}", item_vec_4);

    // Tuple podem ser desestruturados
    let tuple_1 = ("Alexandre", 10000);
    let (name, salary) = tuple_1;

    println!("name: {}", name);
    println!("salary: {}", salary);
}
