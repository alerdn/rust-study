fn main() {
    let x = 5;
    let y = x as f32;
    println!("y: {y}");

    // Não é permitido fazer cast de immutable para mutable
    // let ref_y = &y;
    // let mut mut_ref_y = ref_y as &mut f32;

    // No entanto, é possível fazer o contrário
    let mut y: f32 = 10.0;
    let mut_ref_y = &mut y;
    let ref_y = mut_ref_y as &f32;
    // Porém, não é possível atribuir valores no mutable, porque ele está Frozen enquanto immutable existir -> reborrowing, ref_y é uma referencia borrowed de mut_ref_y
    // *mut_ref_y = 2.;
    println!("{:?} {:?}", mut_ref_y, ref_y);
}
