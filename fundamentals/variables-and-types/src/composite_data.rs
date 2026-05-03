/*
* Tipos de dados compostos:
* - &str e String
* - Arrays
* - Vetores
* - Tuplas
* - Tupla vazia
*
* Exercícios:
* - Ex1: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6db26333288f93021ec01b3358d4075c
* - Ex2: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e789fad69c6f9768fb3fe4309cbf4296
* - EX3: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b8b4c79ad0a352f294fb96c5662545e2
*/

pub fn composite_data() {
  let fixed_size_string: &str = "Pedro";
  let mut flexible_size_string: String = String::from("Pedro");

  flexible_size_string.push('s');

  println!("fixed_size_string: {fixed_size_string}, flexible_size_string: {flexible_size_string}");

  let array_1: [i32; 5] = [1, 2, 3, 4, 5];
  let array_2 = [0; 10];
  let number = array_1[1] as u8;

  println!("array_1: {array_1:?}, array_2: {array_2:?}, number: {number}");

  let vetor_1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
  let number = vetor_1[3];

  println!("vetor_1: {vetor_1:?}, number: {number}");

  let my_information: (&'static str, &'static str, &'static str, i8) =
    ("Nome", "Pedro", "Idade", 24);
  let age = my_information.3;

  println!("my_information: {my_information:?}, age: {age}");

  let (name, name_value, age, age_value) = my_information;
  let unit: () = ();

  println!(
    "name: {name}, name_value: {name_value}, age: {age}, age_value: {age_value}, unit: {unit:?}"
  );
}
