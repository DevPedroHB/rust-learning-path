#[derive(Debug)]
struct User {
  name: String,
  email: String,
  active: bool,
  created_at: u32,
}

impl User {
  fn set_email(&mut self, email: String) {
    self.email = email;
  }

  fn is_online(&self) -> bool {
    return self.active;
  }

  fn get_now() -> u32 {
    return 1234567890;
  }

  fn set_created_at(&mut self) -> bool {
    self.created_at = User::get_now();

    return true;
  }

  fn sign_in(&mut self) {
    self.active = true;
  }

  fn sign_out(mut self) -> bool {
    self.active = false;

    return self.active;
  }

  fn new(name: String, email: String) -> Self {
    return Self {
      name,
      email,
      active: false,
      created_at: 1234567890,
    };
  }
}

fn main() {
  // let mut user = User {
  //   name: String::from("Pedro Henrique Bergamo"),
  //   email: String::from("email@pedrohb.dev"),
  //   active: true,
  //   created_at: 1234567890,
  // };

  let mut user = User::new(
    String::from("Pedro Henrique Bergamo"),
    String::from("email@pedrohb.dev"),
  );

  user.sign_in();
  user.set_created_at();

  println!("User: {user:?}");

  user.name = String::from("Peter Henrique Bergamo");
  user.set_email(String::from("email@peterhb.dev"));

  println!("User: {user:?}");

  if user.is_online() {
    println!("O usuário {} está on-line.", user.name);
  } else {
    println!("O usuário {} está off-line.", user.name);
  }

  user.sign_out();

  // println!("User: {user:?}");
}
