extern crate tokio_trace;

struct Span {
    label: String,
    ref_count: usize,
}

use std::fmt;
use std::sync::Mutex;

use tokio_trace::field::{Field, Visit};
use tokio_trace::metadata::Level;
use tokio_trace::{span, Event, Metadata, Subscriber};
pub struct Visitor;

impl Visit for Visitor {
    fn record_debug(&mut self, field: &Field, value: &fmt::Debug) {
        if field.name() == "message" {
            print!("{:?}: ", value);
        } else {
            print!("{} = {:?} ", field.name(), value);
        }
    }
}

pub struct DemoSubscriber {
    spans: Mutex<Vec<Span>>,
    stack: Mutex<Vec<u64>>,
}

impl DemoSubscriber {
    pub fn new() -> DemoSubscriber {
        DemoSubscriber {
            spans: Mutex::default(),
            stack: Mutex::default(),
        }
    }
}

impl Subscriber for DemoSubscriber {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn new_span(&self, attributes: &span::Attributes) -> span::Id {
        // TODO: correlate spans based on their metadata
        let mut spans = self.spans.lock().unwrap();
        let id = spans.len() + 1;
        spans.push(Span {
            label: attributes.metadata().name().to_string(),
            ref_count: 1,
        });
        span::Id::from_u64(id as u64)
    }

    fn record(&self, span: &span::Id, _values: &span::Record) {
        // Not entirely sure when this is called
        unimplemented!("Span({}): ", span.into_u64());
        //values.record(&mut Visitor);
    }

    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
        unimplemented!(
            "This is new for me, don't judge: {} - {}",
            span.into_u64(),
            follows.into_u64()
        );
    }

    fn event(&self, event: &Event) {
        let level = match *event.metadata().level() {
            Level::DEBUG => "DEBUG",
            Level::ERROR => "ERROR",
            Level::INFO => " INFO", // Whitespace for easy alignment
            Level::TRACE => "TRACE",
            Level::WARN => " WARN", // Whitespace for easy alignment
        };
        let stack = self.stack.lock().unwrap();
        let spans = self.spans.lock().unwrap();
        if let Some(label) = stack.last().map(|id| &spans[(id - 1) as usize].label) {
            print!("{} - {}: ", level, label);
        } else {
            print!("{}: ", level);
        }
        event.record(&mut Visitor);
        println!("(Stack: {:?})", *stack);
    }

    fn enter(&self, span: &span::Id) {
        let mut stack = self.stack.lock().unwrap();
        stack.push(span.into_u64())
    }

    fn exit(&self, span: &span::Id) {
        let mut stack = self.stack.lock().unwrap();
        let top = stack.pop().unwrap();
        assert_eq!(top, span.into_u64());
    }

    fn clone_span(&self, id: &span::Id) -> span::Id {
        let mut spans = self.spans.lock().unwrap();
        spans[id.into_u64() as usize - 1].ref_count += 1;
        id.clone()
    }

    fn drop_span(&self, id: span::Id) {
        let mut spans = self.spans.lock().unwrap();
        // This cannot underflow because it's guaranteed to be called correctly
        spans[id.into_u64() as usize - 1].ref_count -= 1;
    }
}
