#[derive(Debug)]
struct User {
  name: String,
  email: String,
  active: bool,
  created_at: u32,
}

fn main() {
  let mut user = User {
    name: String::from("Pedro Henrique Bergamo"),
    email: String::from("email@pedrohb.dev"),
    active: true,
    created_at: 1234567890,
  };

  println!("User: {user:?}");

  user.name = String::from("Peter Henrique Bergamo");

  println!("Name: {}", user.name);

  if is_online(&user) {
    println!("O usuário {} está on-line.", user.name);
  } else {
    println!("O usuário {} está off-line.", user.name);
  }
}

fn is_online(user: &User) -> bool {
  return user.active;
}
