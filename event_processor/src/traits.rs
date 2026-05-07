pub trait Processable {
  fn process(&self);
  fn type_name(&self) -> &str;
  fn summary(&self) -> String {
    return format!("[{}] processado", self.type_name());
  }
}

pub trait Formattable {
  fn for_json(&self) -> String;
  fn to_text(&self) -> String;
}

pub trait Alertable: Processable + Formattable {
  fn should_alert(&self) -> bool;
  fn alert_level(&self) -> &str;
  fn trigger_alert(&self) {
    if self.should_alert() {
      println!("ALERTA!!! [{}]: {}", self.alert_level(), self.to_text());
      println!(" JSON: {}", self.for_json());
      println!(" Tipo: {}", self.type_name());
    }
  }
}
