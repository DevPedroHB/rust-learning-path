#[derive(Debug)]
enum CardType {
  Debit,
  Credit,
}

#[derive(Debug)]
enum PaymentMethod {
  Pix(String),
  Card(CardType, u32),
  Ticket,
}

#[derive(Debug)]
enum PaymentError {
  InvalidValue,
  UserNotFound,
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

fn process_payment(email: &str, method: &PaymentMethod, value: f64) -> Result<(), PaymentError> {
  let name = get_user_name(email)?;
  let value = validate_value(value)?;

  match method {
    PaymentMethod::Pix(key) => {
      println!("QR Code gerado com chave {key} no valor de R$ {value:.2} para {name}.");
    }
    PaymentMethod::Card(card_type, number) => {
      let card_label = match card_type {
        CardType::Debit => "débito",
        CardType::Credit => "crédito",
      };

      println!("Cartão {card_label} {number} no valor de R$ {value:.2}.");
    }
    PaymentMethod::Ticket => println!("Boleto gerado no valor de R$ {value:.2}."),
  }

  return Ok(());
}

fn finalize_purchase(email: &str, method: &PaymentMethod, value: f64) -> Result<(), PaymentError> {
  let name = get_user_name(email)?;
  let value = validate_value(value)?;

  println!("Compra finalizada para {name} no valor de R$ {value:.2}. Método: {method:?}");

  return Ok(());
}

fn main() {
  let method = PaymentMethod::Pix(String::from(EMAIL));

  match process_payment(EMAIL, &method, 49.90) {
    Ok(_) => println!("Pagamento realizado com sucesso!"),
    Err(e) => println!("Erro: {e:?}"),
  }

  match finalize_purchase(EMAIL, &method, 49.90) {
    Ok(_) => println!("Compra finalizada com sucesso!"),
    Err(e) => println!("Erro: {e:?}"),
  }
}
