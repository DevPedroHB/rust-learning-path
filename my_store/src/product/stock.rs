use super::Product;

pub fn check_stock(product: &Product) -> bool {
  println!("Verificando estoque do produto: {}", product.name);

  return product.stock > 0;
}
