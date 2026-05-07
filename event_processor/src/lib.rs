use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProcessingResult<T> {
  Success(T),
  Discarded,
  Erro(String),
}

#[derive(Debug)]
pub struct LogPayload {
  message: String,
  level: String,
}

#[derive(Debug)]
pub struct MetricPayload {
  name: String,
  value: f64,
}

#[derive(Debug)]
pub struct AlertPayload {
  message: String,
  severity: u8,
}

pub trait Processable {
  fn process(&self);
  fn type_name(&self) -> &str;
  fn resume(&self) -> String {
    return format!("Processando {}.", self.type_name());
  }
}

impl Processable for LogPayload {
  fn process(&self) {
    println!("[LOG] {self:?}");
  }

  fn type_name(&self) -> &str {
    return "log";
  }
}

impl Processable for MetricPayload {
  fn process(&self) {
    println!("[METRIC] {self:?}");
  }

  fn type_name(&self) -> &str {
    return "metric";
  }
}

impl Processable for AlertPayload {
  fn process(&self) {
    println!("[ALERT] {self:?}");
  }

  fn type_name(&self) -> &str {
    return "alert";
  }
}

#[derive(Debug)]
pub struct Event<T> {
  timestamp: u64,
  payload: T,
}

impl<T: Processable> Event<T> {
  pub fn process_event(&self) {
    println!("Evento em {}", self.timestamp);

    self.payload.process();

    println!("Resumo: {}", self.payload.resume())
  }
}

impl<T> Event<T> {
  pub fn timestamp(&self) -> u64 {
    return self.timestamp;
  }

  pub fn payload(&self) -> &T {
    return &self.payload;
  }

  pub fn new(timestamp: u64, payload: T) -> Self {
    return Self { timestamp, payload };
  }
}

impl Display for LogPayload {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return write!(f, "[{}] {}", self.level, self.message);
  }
}

pub fn show_event<T: Display>(event: &Event<T>) {
  println!("{}: {}", event.timestamp(), event.payload());
}

pub fn process_event<T>(event: &Event<T>) -> ProcessingResult<u64> {
  if event.timestamp() == 0 {
    return ProcessingResult::Erro(String::from("Carimbo de data/hora inválido."));
  }

  return ProcessingResult::Success(event.timestamp());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_show_event() {
    let event = Event::new(0, String::from("Hello world!"));

    show_event(&event);
  }

  #[test]
  fn test_log_payload_process() {
    let log = LogPayload {
      message: String::from("Test message"),
      level: String::from("INFO"),
    };

    log.process();

    assert_eq!(log.type_name(), "log");
    assert_eq!(log.resume(), "Processando log.");
  }

  #[test]
  fn test_metric_payload_process() {
    let metric = MetricPayload {
      name: String::from("cpu_usage"),
      value: 85.5,
    };

    metric.process();

    assert_eq!(metric.type_name(), "metric");
    assert_eq!(metric.resume(), "Processando metric.");
  }

  #[test]
  fn test_alert_payload_process() {
    let alert = AlertPayload {
      message: String::from("High CPU usage"),
      severity: 3,
    };

    alert.process();

    assert_eq!(alert.type_name(), "alert");
    assert_eq!(alert.resume(), "Processando alert.");
  }

  #[test]
  fn test_event_new_and_accessors() {
    let log = LogPayload {
      message: String::from("Test"),
      level: String::from("DEBUG"),
    };
    let event = Event::new(1699999999, log);

    assert_eq!(event.timestamp(), 1699999999);
    assert_eq!(event.payload().type_name(), "log");
  }

  #[test]
  fn test_process_event_success() {
    let event = Event::new(100, String::from("test"));
    let result = process_event(&event);

    assert!(matches!(result, ProcessingResult::Success(100)));
  }

  #[test]
  fn test_process_event_invalid_timestamp() {
    let event = Event::new(0, String::from("test"));
    let result = process_event(&event);

    assert!(matches!(result, ProcessingResult::Erro(_)));
  }

  #[test]
  fn test_log_payload_display() {
    let log = LogPayload {
      message: String::from("Error occurred"),
      level: String::from("ERROR"),
    };
    let display = format!("{}", log);

    assert_eq!(display, "[ERROR] Error occurred");
  }

  #[test]
  fn test_processing_result_variants() {
    let success: ProcessingResult<i32> = ProcessingResult::Success(42);
    let discarded: ProcessingResult<i32> = ProcessingResult::Discarded;
    let error: ProcessingResult<i32> = ProcessingResult::Erro(String::from("Fail"));

    assert!(matches!(success, ProcessingResult::Success(42)));
    assert!(matches!(discarded, ProcessingResult::Discarded));
    assert!(matches!(error, ProcessingResult::Erro(_)));
  }

  #[test]
  fn test_event_process_event() {
    let log = LogPayload {
      message: String::from("Test"),
      level: String::from("INFO"),
    };
    let event = Event::new(1000, log);

    event.process_event();
  }
}
