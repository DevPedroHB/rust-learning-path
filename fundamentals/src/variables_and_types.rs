/*
* Variáveis
*
* Exercícios:
* - Exe1 : https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d0dde26ba6aba374f932059ed58f9259
* - Exe2: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=941789d869d16dee173a6ebf4c49f12a
* - Exe3: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=0abcbb7f268026f5fa057ed2b730e899
* */

pub fn variables_and_types() {
  let mut x: i32 = 10;
  let y: u32 = 8;
  let z: f64 = 3.14;

  println!("X é: {x}, Y é: {y}, Z é: {z}");

  x = 11;

  println!("X é: {x}");

  let name: &str = "Pedro";
  let _name2: String = String::from("Pedro");

  println!("Nome é: {name}");

  {
    let name = "Henrique";
    let calculation_base = 2;

    println!("Nome é: {name}");

    x = 12 * calculation_base;
  }

  // println!("Base de cálculo é: {calculation_base}");

  println!("X é: {x}");
  println!("Nome é: {name}");

  let age = 24;

  println!("Idade é: {age}");

  let age = 14;

  println!("Idade é: {age}");

  const ATTEMPTS: u8 = 10;

  println!("Tentativas é: {ATTEMPTS}");

  let first_letter: char = 'P';

  println!("Primeira letra é: {first_letter}");
}
