use crate::{LogPayload, Processable};
use std::fmt::Display;

pub struct Event<T> {
  pub timestamp: u64,
  pub payload: T,
}
pub enum ProcessingResult<T> {
  Success(T),
  Discarded,
  Erro(String),
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

impl Event<String> {
  pub fn payload_in_uppercase(&self) -> String {
    return self.payload.to_uppercase();
  }
}

impl Event<f64> {
  pub fn rounded_payload(&self) -> f64 {
    return (self.payload * 100.0).round() / 100.0;
  }
}

impl<T: Processable> Event<T> {
  pub fn process_event(&self) {
    println!("---- Event em {} ----", self.timestamp);

    self.payload.process();

    println!("Resumo: {}", self.payload.summary());
    println!();
  }
}
fn create_default_event() -> Event<impl Processable> {
  return Event::new(
    0,
    LogPayload {
      message: String::from("Event padrão"),
      level: String::from("INFO"),
    },
  );
}
fn process_and_display<T>(payload: &T)
where
  T: Processable + Display + Clone + PartialEq,
{
  println!("process_and_display: {payload}");

  payload.process();
  payload.summary();
}

pub fn display<T: Display>(event: &Event<T>) {
  println!("[{}] {}", event.timestamp, event.payload());
}

pub fn process_event<T>(event: &Event<T>) -> ProcessingResult<u64> {
  if event.timestamp() == 0 {
    return ProcessingResult::Erro(String::from("Timestamp inválido."));
  }

  return ProcessingResult::Success(event.timestamp());
}
