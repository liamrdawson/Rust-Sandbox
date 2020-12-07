/**
 *  Variables hold primitive data or references to data.
 *  Variables are immutable by default.
 *  Rust is a block-scoped language.
 */

pub fn run() {
  let name = "Liam";
  let mut age = 30;   // warning: value assigned to `age` is never read
  age = 31;
  println!("My name is {} and I am {}", name, age);

  //  Define constant
  const ID: i32 = 100;  // const - typcally uppercase and requires type
  println!("ID: {}", ID);

  //  Assign multiple vars
  let ( my_name, my_age ) = ("Liam", 31);
  println!("{} is {}", my_name, my_age);
}