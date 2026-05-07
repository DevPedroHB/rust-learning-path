// Traits mais comuns
// #[derive(Debug)]
// #[derive(Default)]
// #[derive(Clone)]
// #[derive(PartialEq)]
// #[derive(Eq)]
// #[derive(Hash)]

use crate::{Alertable, Formattable, Processable};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AlertPayload {
  pub message: String,
  pub severity: u8,
}

impl Processable for AlertPayload {
  fn process(&self) {
    let icon = if self.severity >= 5 { "🔴" } else { "🟡" };

    println!("[ALERTA][{icon}] {}", self.message);
  }

  fn type_name(&self) -> &str {
    return "alert";
  }
}

impl Formattable for AlertPayload {
  fn for_json(&self) -> String {
    return format!(
      r#"{{"tipo": "alerta", "mensagem": "{}", "severidade": "{}"}}"#,
      self.message, self.severity
    );
  }

  fn to_text(&self) -> String {
    let icon = if self.severity >= 5 { "🔴" } else { "🟡" };

    return format!("[ALERTA][{icon}] {}", self.message);
  }
}

impl Alertable for AlertPayload {
  fn should_alert(&self) -> bool {
    return self.severity >= 5;
  }

  fn alert_level(&self) -> &str {
    return match self.severity {
      0..=3 => "BAIXO",
      4..=6 => "MÉDIO",
      7..=9 => "ALTO",
      _ => "CRITICO",
    };
  }
}
