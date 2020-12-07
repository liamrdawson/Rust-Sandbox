/**
 *  Primitive str = Immutable fixed length string in memory.
 *  String        = Growable, heap-allocated data structure.
 *                  Use when you need to modiy or own string data.
 * 
 * Rust String Docs https://doc.rust-lang.org/std/string/index.html
 */

pub fn run() {
  // let hello = "Hello";                //  Primitive str
  let mut hello = String::from("Hello ");  //  String

  hello.push('W');
  hello.push_str("orld!");


  // String method examples
  println!("Length: {}", hello.len());
  println!("Capacity: {}", hello.capacity());
  println!("Is Empty: {}", hello.is_empty());
  println!("Contains 'World': {}", hello.contains("World"));
  println!("Replace: {}", hello.replace("World","There"));

  //  Loop through a string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  //  Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  //  Assertion testing - no output if successful.
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{}", s);
}