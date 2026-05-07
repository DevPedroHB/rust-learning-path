use crate::{Formattable, Processable};
use core::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MetricPayload {
  pub name: String,
  pub value: f64,
}

impl Processable for MetricPayload {
  fn process(&self) {
    println!("[MÉTRICA][{}] {}", self.name, self.value);
  }

  fn type_name(&self) -> &str {
    return "metric";
  }
}
impl Formattable for MetricPayload {
  fn for_json(&self) -> String {
    return format!(
      r#"{{"tipo": "métrica", "nome": "{}", "valor": "{}"}}"#,
      self.name, self.value
    );
  }

  fn to_text(&self) -> String {
    return format!("{} = {}", self.name, self.value);
  }
}

pub struct Metrics(pub Vec<String>);

impl fmt::Display for Metrics {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for item in &self.0 {
      return write!(f, "{item}\n");
    }

    return Ok(());
  }
}
