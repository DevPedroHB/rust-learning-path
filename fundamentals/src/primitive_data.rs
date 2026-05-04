/*
*  Tipos de dados básicos:
*  - Integers
*  - Floats
*  - Chars
*  - Boolean
*/

pub fn primitive_data() {
  let unsigned_integer: u8 = 5; // u8, u16, u32, u64, u128
  let signed_integer: i8 = -5; // i8, i16, i32, i64, i128
  let float: f32 = 3.14; // f16, f32, f64, f128
  let architecture_unsigned_integer: usize = 5;
  let architecture_signed_integer: isize = 5;
  let char: char = 'a';
  let boolean: bool = true;

  println!("unsigned_integer: {unsigned_integer}");
  println!("signed_integer: {signed_integer}");
  println!("float: {float}");
  println!("architecture_unsigned_integer: {architecture_unsigned_integer}");
  println!("architecture_signed_integer: {architecture_signed_integer}");
  println!("char: {char}");
  println!("boolean: {boolean}");

  type Age = u8;

  let age: Age = 24;

  println!("Idade: {age}");

  let a: i32 = 10;
  let b = a as f64;

  println!("b: {b}");
}
