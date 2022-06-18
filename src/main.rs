fn main() {
    // ------------------------------------------------
    //         Initializing Multiple Variables
    // ------------------------------------------------

    let (first_number, second_number) = (250, 480.2);
    println!(
        "The first number is {} and the second number is {}",
        first_number, second_number
    );

    // ------------------------------------------------
    //         Readability of large number
    // ------------------------------------------------

    let large_number = 1_000_000_000;
    println!("The large number is {}", large_number);

    // ------------------------------------------------
    //         Integer overflow
    // ------------------------------------------------

    // let overflow_number: u8 = 256;
    // println!("The overflow number is {}", overflow_number);

    // ------------------------------------------------
    //         Decimal numbers in other formats
    // ------------------------------------------------

    let x = 255;
    println!(
        "The value of variable x in hexadecimal is {:o} and in octal is {:x} and binary is {:b}",
        x, x, x
    );

    // ------------------------------------------------
    //         Snake case convention
    // ------------------------------------------------

    let number = 45;

    // ------------------------------------------------------
    //         Operations on number in different formats
    // ------------------------------------------------------

    let n1 = 14;
    let n2 = 15.6;
    let n3 = n1 as f64 + n2;
    println!("The sum of n1 and n2 is {}", n3);

    // ------------------------------------------------------
    //         Shadowing of variables
    // ------------------------------------------------------

    // let s = 5;
    // let s = 5 * 5;
    // println!("The value of s is {}", s);

    // let s = 32;
    // println!("The value of s is {} is currently and integer", s);

    // let s = 'A';
    // println!("The value of s is {} is currently and char", s);

    // let s = 64.5;
    // println!("The value of s is {} is currently and float", s);

    let mut s = 65;

    {
        s = 60;
        println!("The value of s inside the inner scope is {}", s);
    }

    println!("The value of s outside the inner scope is {}", s);

    // ------------------------------------------------------
    //         Constants
    // ------------------------------------------------------

    const MAX_SALARY: u32 = 100_000;
    println!("The maximum salary is {}", MAX_SALARY);
}
