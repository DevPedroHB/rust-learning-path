use crate::Processable;

pub struct DynamicPipeline {
  events: Vec<Box<dyn Processable>>,
}

impl DynamicPipeline {
  pub fn add(&mut self, event: Box<dyn Processable>) {
    self.events.push(event)
  }

  pub fn total(&self) -> usize {
    return self.events.len();
  }

  pub fn process_all(&self) {
    println!("====== Processando {} eventos ======", self.total());

    for event in &self.events {
      println!("[{}] {}", event.type_name(), event.summary());

      event.process();
    }

    println!("==== FIM ====");
  }

  pub fn new() -> Self {
    return Self { events: Vec::new() };
  }
}
