use event_processor::{
  AlertPayload, Alertable, DynamicPipeline, Event, LogPayload, MetricPayload, Pipeline, Processable,
};

fn check_alerts<T: Alertable>(events: &[Event<T>]) {
  for event in events {
    let payload = event.payload();

    if payload.should_alert() {
      payload.trigger_alert();
    } else {
      println!("{} - ok: {}", payload.type_name(), payload.to_text());
    }
  }
}

fn main() {
  let log = LogPayload {
    message: String::from("Teste"),
    level: String::from("INFO"),
  };
  let log2 = LogPayload::default();

  println!("{log2:?}");
  println!("{log:?}");

  let metric = MetricPayload {
    name: String::from("cpu_usage"),
    value: 80.0,
  };

  println!("{metric:?}");

  let metric2 = MetricPayload::default();

  println!("{metric2:?}");

  let alerts = vec![
    Event::new(
      289371928,
      AlertPayload {
        message: String::from("Disco quase cheio!"),
        severity: 9,
      },
    ),
    Event::new(
      23894729387,
      AlertPayload {
        message: String::from("CPU levemente alta"),
        severity: 3,
      },
    ),
  ];
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
  let mut pipeline1 = DynamicPipeline::new();

  for event in events {
    pipeline1.add(event);
  }

  pipeline1.process_all();

  check_alerts(&alerts);

  let mut pipeline = Pipeline::new();

  pipeline.add(Event::new(
    170998530498,
    LogPayload {
      message: String::from("Rapina foi iniciado"),
      level: String::from("INFO"),
    },
  ));
  pipeline.add(Event::new(
    170998530498,
    LogPayload {
      message: String::from("Request recebida"),
      level: String::from("DEBUG"),
    },
  ));

  pipeline.process_all();

  let alert = Event::new(
    13284208,
    AlertPayload {
      message: String::from("Disco do DB está cheio"),
      severity: 5,
    },
  );

  alert.process_event();
}
