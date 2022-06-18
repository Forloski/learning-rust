fn main() {
  println!("Hello from Rust!");
  println!("The value of the constant is {}", 10);
  println!("My first name is {}, my last name is {}", "John", "Doe");
  print!("This is a print command ");
  print!("This is another print command");
  println!("");
  println!("This is going to be
  printed on multiple
  lines");
  println!("\\n\n this is going to be printed after 1 line, \t this will have a tab");
  println!("THis will print single quotes \' as this will print double quote \"");
  println!("This will print a backslash \\");
  println!("This will be overwritten \r This text will only appear on screen");
  println!("\n i doing {2}, from {1} years and i {0}", "like", 20, "programming");
  println!("{language} is a system programming language which is cool", language = "Rust");
  println!("The sum of 25 + 10 = {}", 25 + 10);
}
