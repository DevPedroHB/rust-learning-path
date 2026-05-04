fn main() {
  let name = String::from("Pedro");

  print_name(&name);

  println!("Olá, meu nome é {name}!");

  let mut mut_name = String::from("Pedro");
  let z = &mut_name;

  println!("Olá, meu nome é {z}!");

  let changed_name = alter_name(&mut mut_name);

  println!("Olá, meu nome é {changed_name}!");
}

fn print_name(name: &String) {
  println!("Olá, meu nome é {name}!");
}

fn alter_name(name: &mut String) -> &String {
  name.push_str(" Henrique");

  return name;
}

// fn make_string() -> &String {
//   let s = String::from("Estou aprendendo Rust!");

//   return &s;
// }
