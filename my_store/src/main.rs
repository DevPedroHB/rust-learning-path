use my_store::{Product, apply_discount, calculate_total, check_stock};

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

  let product = &products[1];

  if check_stock(product) {
    let final_price = apply_discount(product.price, product.discount);

    println!("Preço final do {}: R$ {final_price:.2}", product.name);
  }

  let total = calculate_total(&products);

  println!("Total: R$ {total:.2}");
  println!("Total com desconto: R$ {:.2}", apply_discount(total, 10.0));
}
