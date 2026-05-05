use store_core::{Product, calculate_total};

fn main() {
  let products = vec![
    Product {
      name: String::from("Livro 01"),
      price: 9.90,
      stock: 10,
      discount: 10.0,
    },
    Product {
      name: String::from("Livro 02"),
      price: 19.90,
      stock: 10,
      discount: 10.0,
    },
    Product {
      name: String::from("Livro 03"),
      price: 29.90,
      stock: 10,
      discount: 10.0,
    },
  ];

  let total = calculate_total(&products);

  println!("Total: R$ {total:.2}");
}
