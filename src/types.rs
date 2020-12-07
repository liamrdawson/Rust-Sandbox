/**
 *  Primitive Types:
 *    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits taken in memory, i = integer / u = unsigned (no negative values))
 *    Floats: f32, f64
 *    Boolean (bool)
 *    Characters (char) -> 1 character specifically (not a string)
 *    Tuples
 *    Arrays
 * 
 *  Rust is a statically typed language, which means that it must know of all variables at compile time. However, the compiler can infer what type we want to use based on the value and how it is used.
 * 
*/

pub fn run() {
  //  Defaults to "i32" type
  let x = 1;

  //  Defaults to "f64"
  let y = 2.5;

  //  Add explicit type
  let z: i64 = 45454545454545;

  //  Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  //  Boolean
  let is_active: bool = true;

  //  Return boolean from an expression
  let is_greater = 10 > 5;

  let a1 = 'a';           //  char in '' only
  let face = '\u{1F600}'; //  can contain only one code point

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}