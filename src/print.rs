pub fn run() {
  //  Print to console
  println!("Hello from the print.rs file");

  //  Basic text formatting
  println!("{} is from {}", "Liam", "Wales");

  //  Positional Arguments
  println!("{0} is from {1} and {0} likes to {2}", "Liam", "Wales", "code");

  //  Named Arguments
  println!(
      "{name} likes to play {activity}", 
      name = "Liam",
      activity = "Guitar"
  );

  //  Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  //  Placeholder for debug trait
  println!("{:?}", (12, true, "Hello"));

  //  Basic maths
  println!("10 + 10 = {}", 10 + 10);

}
