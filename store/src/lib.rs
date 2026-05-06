#[derive(Debug)]
pub struct Store {
  name: String,
  balance: f64,
}

impl Store {
  pub fn name(&self) -> &String {
    return &self.name;
  }

  pub fn balance(&self) -> f64 {
    return self.balance;
  }

  pub fn sell(&mut self, price: f64) -> Result<f64, String> {
    if price < 0.0 {
      return Err(format!("O valor da venda deve ser maior que zero."));
    }

    self.balance += price;

    return Ok(self.balance);
  }

  pub fn new(name: String) -> Self {
    return Self { name, balance: 0.0 };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_store() {
    let store = Store::new(String::from("Test Store"));

    assert_eq!(store.name(), &String::from("Test Store"));
    assert_eq!(store.balance(), 0.0);
  }

  #[test]
  fn test_sell_positive() {
    let mut store = Store::new(String::from("Test Store"));
    let result = store.sell(10.0);

    assert_eq!(result, Ok(10.0));
    assert_eq!(store.balance(), 10.0);
  }

  #[test]
  fn test_sell_zero() {
    let mut store = Store::new(String::from("Test Store"));
    let result = store.sell(0.0);

    assert_eq!(result, Ok(0.0));
    assert_eq!(store.balance(), 0.0);
  }

  #[test]
  fn test_sell_negative() {
    let mut store = Store::new(String::from("Test Store"));
    let result = store.sell(-5.0);

    assert_eq!(
      result,
      Err(String::from("O valor da venda deve ser maior que zero."))
    );
    assert_eq!(store.balance(), 0.0);
  }
}
