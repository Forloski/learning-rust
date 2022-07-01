fn main() {
    // ------------------------------------------------
    //                  Tuples
    //                         tuples are fixed length
    // ------------------------------------------------

    let my_information = ("Salary", 40_000);
    println!("The {} is {}", my_information.0, my_information.1);

    let (salary, salary_value) = my_information;
    println!("The {} is {}", salary, salary_value);

    let nested_tuple = ("NestedTuple", ("Salary", 40_000));
    println!("The {} is {}", nested_tuple.1 .0, nested_tuple.1 .1);

    //let empty_tuple = ();

    // ------------------------------------------------
    //                  Arrays
    //                         -arrays are fixed length
    //                         -updating elements
    //                         -slices
    // ------------------------------------------------

    let mut my_array = [1, 2, 3, 4, 5];
    println!("The first element of the array is {}", my_array[0]);
    println!("{:?}", my_array);

    my_array[4] = 6;
    println!("{:?}", my_array);

    let array_with_same_elements = [0; 10];

    let mut string_array_1 = ["apple", "tomato", "grapes"];
    println!("{:?}", string_array_1);

    let mut number_array_1 = [1, 2, 3, 4, 5];
    let subset_array = &number_array_1[0..=3];
    println!("{:?}", subset_array);
    println!("Size of array {}", number_array_1.len());
    println!(
        "The array is occupying {} bytes",
        std::mem::size_of_val(&number_array_1)
    );
    let check_index = number_array_1.get(4);
    println!("{:?}", check_index);
}
