enum Direction {
  North,
  South,
  East,
  West,
}

pub fn conditionals() {
  let age = 24;
  let mut message = "";

  if age > 5 && age < 18 {
    println!("Você está muito jovem para aprender Rust!");
  } else if age > 18 && age <= 100 {
    println!("Você é ainda mais jovem para aprender Rust!");
  } else {
    println!("Infelizmente, eu ainda acho você jovem para iniciar em Rust!");
  }

  match age {
    1..=18 => {
      let _x = 10;
      message = "Você ainda é menor de idade."
    }
    19..=100 => message = "Você é maior de idade.",
    _ => message = "Não conheço sua idade.",
  }

  println!("Message é: {message}");

  let x = 3;

  match x {
    1 => println!("Um."),
    2 => println!("Dois."),
    3 => println!("Três."),
    _ => println!("Qualquer outro número."),
  }

  let mut message = match age {
    1..=18 => "Você é menor de idade.",
    19..=100 => "Você é maior de idade.",
    _ => "Não conheço a sua idade.",
  };

  println!("Message é: {message}");

  let direction = Direction::North;

  match direction {
    Direction::North => println!("Estamos indo para o Norte."),
    Direction::South => println!("Estamos indo para o Sul."),
    Direction::East => println!("Estamos indo para o Leste."),
    Direction::West => println!("Estamos indo para o Oeste."),
    _ => println!("Não sei para onde estamos indo."),
  }

  match age {
    1..=18 => message = "Menor.",
    19..=100 => message = "Maior.",
    _ => message = "Nao conheço.",
  }

  println!("Message é: {message}");
}
