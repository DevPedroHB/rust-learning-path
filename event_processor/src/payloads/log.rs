use crate::{Formattable, Processable};
use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct LogPayload {
  pub message: String,
  pub level: String,
}

impl Processable for LogPayload {
  fn process(&self) {
    println!("[LOG][{}] {}", self.level, self.message);
  }

  fn type_name(&self) -> &str {
    return "log";
  }

  fn summary(&self) -> String {
    return format!("[{}] {}", self.level, self.message);
  }
}

impl Formattable for LogPayload {
  fn for_json(&self) -> String {
    return format!(
      r#"{{"tipo": "log", "nível": "{}", "mensagem": "{}"}}"#,
      self.level, self.message
    );
  }

  fn to_text(&self) -> String {
    return format!("[{}] {}", self.level, self.message);
  }
}

impl fmt::Display for LogPayload {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    return write!(f, "[{}] {}", self.level, self.message);
  }
}

// Orphan Rule
// NewType Pattern
pub struct Logs(pub Vec<String>);

impl fmt::Display for Logs {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for item in &self.0 {
      return write!(f, "{item}\n");
    }

    return Ok(());
  }
}
