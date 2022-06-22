fn main() {
    // ------------------------------------------------
    //                  Strings
    //                         String slices (&str) - fixed length strings
    // ------------------------------------------------

    let some_string = "Fixed length string";
    println!("The string is \"{}\"", some_string);

    // ------------------------------------------------
    //                  Strings
    //                         variable length strings
    //                         adding removing characters
    //                         operations on strings
    //                         concatenation
    // ------------------------------------------------

    let mut growable_string = String::from("Variable length string");
    println!("The string is \"{}\"", growable_string);

    growable_string.push('s');
    println!("The string is \"{}\"", growable_string);

    growable_string.pop();
    println!("The string is \"{}\"", growable_string);

    growable_string.push_str(" and a new string");
    println!("The string is \"{}\"", growable_string);

    println!(
        "I am going to tell you some basic things about the string,
  Is the string empty {},
  The length of the string is {},
  The string has {} bytes,
  Does the string contains the word 'length' {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("length")
    );

    growable_string.push_str("    ");
    println!(
        "The string length is \"{}\" and the length of the string after the trim is \"{}\"",
        growable_string.len(),
        growable_string.trim().len()
    );

    let number = 6;

    println!("the value of number in string is {}", number.to_string());
    println!(
        "is the number really a string {}",
        number.to_string() == "6"
    );

    let some_char = 'c';
    println!(
        "the value of some_char in string is {}",
        some_char.to_string()
    );
    println!(
        "is the some_char really a string {}",
        some_char.to_string() == "c"
    );

    let my_name = "Fernando Orloski".to_string();
    println!("my name is {}", my_name);

    let empty_string = String::new();
    println!("length is {}", empty_string.len());

    let s_1 = "Fernando".to_string();
    let s_2 = "Orloski".to_string();

    let s_3 = format!("My First name is {} and {}", s_1, s_2);
    println!("{}", s_3);

    let string_1 = String::from("Fernando");
    let string_2 = String::from("Orloski");

    let string_3 = format!("My First name is {} and {}", string_1, string_2);
    println!("{}", string_3);

    let string_4 = string_1 + " " + &string_2;
    println!("{}", string_4);
}
