fn main() {
  let phrase = String::from(
    "Cillum ipsum in cillum proident consequat occaecat enim labore cillum excepteur occaecat ipsum.",
  );
  let first_part = first_part(&phrase);

  println!("Primeira parte (15 caracteres): {first_part}");
}

fn first_part(phrase: &str) -> &str {
  return &phrase[..15];
}
