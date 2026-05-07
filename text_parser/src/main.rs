pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
  return if a.len() > b.len() { a } else { b };
}

pub fn word_counter(texto: &str) -> usize {
  return texto.split_whitespace().count();
}

fn main() {
  let sentence1 = String::from("Frase mais longa");
  let sentence2 = String::from("Frase curta");
  let result = longer(&sentence1, &sentence2);

  println!("Frase maior: {result}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn returns_the_longest_word() {
    let a = "Pedro";
    let b = "Henrique";

    assert_eq!(longer(a, b), "Henrique");
  }
}
