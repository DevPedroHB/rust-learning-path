use crate::{Event, Processable};

pub struct Pipeline<T: Processable> {
  events: Vec<Event<T>>,
}

impl<T: Processable> Pipeline<T> {
  pub fn add(&mut self, event: Event<T>) {
    self.events.push(event);
  }

  pub fn process_all(&self) {
    for event in &self.events {
      event.process_event();
    }
  }

  pub fn new() -> Self {
    return Self { events: Vec::new() };
  }
}
