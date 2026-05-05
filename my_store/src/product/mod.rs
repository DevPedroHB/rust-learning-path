pub mod discount;
pub mod stock;

pub struct Product {
  pub name: String,
  pub price: f64,
  pub stock: u32,
  pub discount: f64,
}

pub fn calculate_total(products: &[Product]) -> f64 {
  return products.iter().fold(0.0, |acc, p| acc + p.price);
}
