enum CardType {
  Debit,
  Credit,
}

enum PaymentMethod {
  Pix(String),
  Card(CardType, u32),
  Ticket,
}

fn process_payment(method: PaymentMethod, value: f64) {
  match method {
    PaymentMethod::Pix(key) => {
      println!("Gerando o QR Code com a chave {key} no valor de R$ {value:.2}.");
    }
    PaymentMethod::Card(card_type, number) => match card_type {
      CardType::Debit => {
        println!("Passando o cartão de debito com o numero {number} no valor de R$ {value:.2}.");
      }
      CardType::Credit => {
        println!("Passando o cartão de credito com o numero {number} no valor de R$ {value:.2}.");
      }
    },
    PaymentMethod::Ticket => {
      println!("Gerando o boleto no valor de R$ {value:.2}.");
    }
  }
}

fn main() {
  process_payment(PaymentMethod::Pix(String::from("pix@pedrohb.dev")), 49.90);
  process_payment(PaymentMethod::Card(CardType::Debit, 1234), 49.90);
  process_payment(PaymentMethod::Card(CardType::Credit, 1234), 49.90);
  process_payment(PaymentMethod::Ticket, 49.90);
}
