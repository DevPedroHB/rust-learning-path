use event_processor::{
  AlertPayload, DynamicPipeline, Event, LogPayload, MetricPayload, Processable,
};

#[test]
fn event_string_created_correctly() {
  let event = Event::new(100, String::from("teste"));

  assert_eq!(event.timestamp(), 100);
  assert_eq!(event.payload(), "teste");
}

#[test]
fn event_f64_created_correctly() {
  let event = Event::new(200, 42.5);

  assert_eq!(event.timestamp(), 200);
  assert_eq!(*event.payload(), 42.5);
}

#[test]
fn event_bool_created_correctly() {
  let event = Event::new(300, true);

  assert_eq!(event.timestamp(), 300);
  assert_eq!(*event.payload(), true);
}

#[test]
fn payload_uppercase_only_works_with_string() {
  let event = Event::new(100, String::from("hello"));

  assert_eq!(event.payload_in_uppercase(), "HELLO");
}

#[test]
fn rounded_payload_only_works_with_f64() {
  let event = Event::new(100, 42.567);

  assert_eq!(event.rounded_payload(), 42.57);
}

#[test]
fn event_with_processable_payload() {
  let event = Event::new(
    178923828349,
    LogPayload {
      message: String::from("Rapina foi iniciado"),
      level: String::from("INFO"),
    },
  );

  event.process_event();
}

#[test]
fn dynamic_pipeline_accepts_mixed_types() {
  let mut pipeline = DynamicPipeline::new();
  let events: Vec<Box<dyn Processable>> = vec![
    Box::new(LogPayload {
      message: String::from("Rapina foi iniciado"),
      level: String::from("INFO"),
    }),
    Box::new(MetricPayload {
      name: String::from("cpu"),
      value: 73.3,
    }),
    Box::new(AlertPayload {
      message: String::from("Disco cheio"),
      severity: 5,
    }),
  ];

  for event in events {
    pipeline.add(event);
  }

  assert_eq!(pipeline.total(), 3);
}

#[test]
fn debug_works() {
  let log = LogPayload {
    message: String::from("test"),
    level: String::from("INFO"),
  };
  let debug = format!("{log:?}");

  assert!(debug.contains("test"));
  assert!(debug.contains("INFO"));
}
