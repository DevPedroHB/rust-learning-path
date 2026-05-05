pub fn apply_discount(price: f64, discount: f64) -> f64 {
  return price - (price * (discount / 100.0));
}
