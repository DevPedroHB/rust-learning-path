use std::fmt;

#[derive(Debug)]
struct Product {
  name: String,
  price: f64,
}

#[derive(Debug)]
enum CardType {
  Debit,
  Credit,
}

#[derive(Debug)]
struct Card {
  kind: CardType,
  number: u32,
}

#[derive(Debug)]
enum PaymentMethod {
  Pix(String),
  Card(Card),
  Ticket,
}

#[derive(Debug)]
enum PaymentError {
  InvalidValue,
  UserNotFound,
  EmptyCart,
}

impl fmt::Display for PaymentError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PaymentError::InvalidValue => write!(f, "Valor inválido"),
      PaymentError::UserNotFound => write!(f, "Usuário não encontrado"),
      PaymentError::EmptyCart => write!(f, "Carrinho vazio"),
    }
  }
}

const EMAIL: &str = "pix@pedrohb.dev";

fn get_user_name(email: &str) -> Result<&'static str, PaymentError> {
  if email == EMAIL {
    return Ok("Pedro Henrique Bergamo");
  }

  return Err(PaymentError::UserNotFound);
}

fn validate_value(value: f64) -> Result<f64, PaymentError> {
  if value <= 0.0 {
    return Err(PaymentError::InvalidValue);
  }

  return Ok(value);
}

fn calculate_total(cart: &[Product]) -> f64 {
  return cart.iter().map(|p| p.price).sum();
}

fn display_cart(cart: &[Product]) {
  println!("Carrinho de compras:");

  for (index, product) in cart.iter().enumerate() {
    println!("{index}: {product:?}");
  }

  let total = calculate_total(cart);

  println!("Total: R$ {total:.2}");
}

fn process_payment(name: &str, method: &PaymentMethod, value: f64) -> Result<String, PaymentError> {
  let value = validate_value(value)?;

  let message = match method {
    PaymentMethod::Pix(key) => {
      format!("QR Code gerado com chave {key} no valor de R$ {value:.2} para {name}.")
    }
    PaymentMethod::Card(card) => {
      let label = match card.kind {
        CardType::Debit => "débito",
        CardType::Credit => "crédito",
      };

      format!("Cartão {label} {} no valor de R$ {value:.2}.", card.number)
    }
    PaymentMethod::Ticket => {
      format!("Boleto gerado no valor de R$ {value:.2}.")
    }
  };

  Ok(message)
}

fn finalize_purchase(
  email: &str,
  cart: &[Product],
  method: &PaymentMethod,
) -> Result<String, PaymentError> {
  let name = get_user_name(email)?;

  if cart.is_empty() {
    return Err(PaymentError::EmptyCart);
  }

  let total = calculate_total(cart);

  let payment_message = process_payment(name, method, total)?;

  Ok(format!(
    "{payment_message}\nPagamento realizado com sucesso para {name} no valor de R$ {total:.2}."
  ))
}

fn main() {
  let prices = vec![9.90, 19.90, 29.90, 39.90, 49.90];
  let cart: Vec<Product> = prices
    .iter()
    .enumerate()
    .map(|(index, price)| Product {
      name: format!("Livro {}", index),
      price: *price,
    })
    .collect();

  display_cart(&cart);

  let method = PaymentMethod::Pix(String::from(EMAIL));

  match finalize_purchase(EMAIL, &cart, &method) {
    Ok(message) => println!("{message}"),
    Err(e) => println!("Erro: {e}"),
  }
}
