enum CardType {
  Debit,
  Credit,
}

enum PaymentMethod {
  Pix(String),
  Card(CardType, u32),
  Ticket,
}

fn get_user_name(email: &String) -> Option<String> {
  if email == "pix@pedrohb.dev" {
    return Some(String::from("Pedro Henrique Bergamo"));
  }

  return None;
}

fn process_payment(method: PaymentMethod, value: f64) {
  match method {
    PaymentMethod::Pix(key) => match get_user_name(&key) {
      Some(name) => {
        println!("Gerando o QR Code com a chave {key} no valor de R$ {value:.2} para {name}.")
      }
      None => println!("Gerando o QR Code com a chave {key} no valor de R$ {value:.2}."),
    },
    PaymentMethod::Card(card_type, number) => match card_type {
      CardType::Debit => {
        println!("Passando o cartão de debito com o numero {number} no valor de R$ {value:.2}.")
      }
      CardType::Credit => {
        println!("Passando o cartão de credito com o numero {number} no valor de R$ {value:.2}.")
      }
    },
    PaymentMethod::Ticket => println!("Gerando o boleto no valor de R$ {value:.2}."),
  }
}

fn main() {
  let email = String::from("pix@pedrohb.dev");

  process_payment(PaymentMethod::Pix(String::from("pix@pedrohb.de")), 49.90);
  process_payment(PaymentMethod::Card(CardType::Debit, 1234), 49.90);
  process_payment(PaymentMethod::Card(CardType::Credit, 1234), 49.90);
  process_payment(PaymentMethod::Ticket, 49.90);

  if let Some(name) = get_user_name(&email) {
    println!("Olá, {name}!");
  }
}
