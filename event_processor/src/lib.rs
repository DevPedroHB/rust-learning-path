pub enum ProcessingResult<T> {
  Success(T),
  Discarded,
  Erro(String),
}

#[derive(Debug)]
pub struct Event<T> {
  timestamp: u64,
  payload: T,
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

pub fn show_event<T>(event: &Event<T>) {
  println!("{}", event.timestamp);
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
}
