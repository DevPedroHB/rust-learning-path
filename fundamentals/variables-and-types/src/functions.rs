fn sum(n1: i32, n2: i32) -> i32 {
  let sum = n1 + n2;

  return sum;
}

fn sum_double(mut n1: i32, mut n2: i32) -> i32 {
  n1 = n1 * n2;
  n2 = n1 * n2;

  return n1 + n2;
}

fn good_morning(name: &str, _surname: &str) {
  println!("Olá {name}, bom dia!");
}

fn multiplication_table(n: u32) {
  for i in 1..=10 {
    println!("{n} x {i} = {}", n * i);
  }
}

pub fn functions() {
  let name = "Pedro Henrique";

  good_morning(name, "Bergamo");

  let n1 = 20;
  let n2 = 30;

  let sum_result = sum(n1, n2);

  println!("O resultado da soma é: {sum_result}");

  let sum_double_result = sum_double(n1, n2);

  println!("O resultado da soma dobrado é: {sum_double_result}");

  multiplication_table(8);
}
