fn main() {
    // ------------------------
    // Variables in Rust
    // ------------------------

    let mut x = 5;
    x = 6 + x;
    println!("The value of the variable x = {}", x);

    // ------------------------
    // Data Types
    //      - Scalar Data Types
    //          - Integer
    //              - Signed    i8, i16, i32, i64
    //                      -2^(i-1)-1 to 2^(i-1)-1
    //              - Unsigned  u8, u16, u32, u64
    //          - Floating Point
    //              - f32, f64
    //          - Boolean
    //              - bool
    //          - Character
    //              - char
    // ------------------------
    println!("The maximum number in i8 is equal to {}", std::i8::MAX);
    println!("The minimum number in i8 is equal to {}", std::i8::MIN);
    println!("The maximum number in u8 is equal to {}", std::u8::MAX);
    println!("The minimum number in u8 is equal to {}", std::u8::MIN);
    println!("The maximum number in f32 is equal to {}", std::f32::MAX);
    println!("The minimum number in f32 is equal to {}", std::f32::MIN);
}
