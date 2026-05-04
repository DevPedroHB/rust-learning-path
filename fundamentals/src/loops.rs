/*
* Controle de repetição ou Laços de repetição:
* - Loops
* - For
* - While
*/

pub fn loops() {
  let s = 1;

  loop {
    println!("Este é um laço de repetição loop.");

    if s == 10 {
      break;
    }
  }

  'outside: loop {
    'inside: loop {
      println!("Este é mais um laço de repetição usando loop encadeado.");

      if s == 10 {
        break 'inside;
      }
    }

    if s == 100 {
      break 'outside;
    }
  }

  let x = loop {
    if s > 0 {
      break 5;
    }
  };

  println!("x é: {x}");

  let v = vec![1, 2, 3, 4, 5, 6];

  for item in v {
    println!("{item}");
  }

  let mut number = 0;

  while number < 10 {
    // number = number + 1;
    number += 1;
  }
}
