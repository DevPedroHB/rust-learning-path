fn main() {
  let s = String::from("Pedro está ensinando Rust!");
  let s2 = s.clone();
  let s3 = &s;

  println!("Valor de 's' é: {s}");
  println!("Valor de 's2' é: {s2}");
  println!("Valor de 's3' é: {s3}");

  let a = 10;
  let b = a;

  println!("Valor de 'a' é: {a}");
  println!("Valor de 'b' é: {b}");

  let name = String::from("Pedro");

  print_name(name.clone());

  println!("Olá, meu nome é {name}!");

  let name = alter_name(name);

  println!("Olá, meu nome é {name}!");
}

fn print_name(name: String) {
  println!("Olá, meu nome é {name}!");
}

fn alter_name(mut name: String) -> String {
  name.push_str(" Henrique");

  return name;
}
