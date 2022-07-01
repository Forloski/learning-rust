fn main() {
    // ---------------------------------------------------------------------
    //                  Vector
    //                         Vector does not have a fixed size
    // ---------------------------------------------------------------------

    let my_vector = vec![1, 2, 3, 4, 5];
    println!("The first element of the vector is {}", my_vector[0]);
    println!("{:?}", my_vector);

    let array_with_same_elements = [0; 10];
    println!("{:?}", array_with_same_elements);

    let mut string_vector = vec!["apple", "tomato", "grapes", "banana"];

    let subset_vector = &string_vector[0..=2];
    println!("{:?}", subset_vector);

    string_vector.push("orange");
    println!("{:?}", string_vector);

    string_vector.remove(2);
    println!("{:?}", string_vector);
}
