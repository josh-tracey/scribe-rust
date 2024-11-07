use std::sync::Arc;
use tracing::{Event, Level as TracingLevel, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

const FG_RED: &str = "\x1b[31m";
const FG_YELLOW: &str = "\x1b[33m";
const FG_BLUE: &str = "\x1b[34m";
const FG_GREEN: &str = "\x1b[32m";
const FG_GRAY: &str = "\x1b[90m";
const FG_RESET: &str = "\x1b[0m";

pub enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Gray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Trace => write!(f, "TRACE"),
            Level::Debug => write!(f, "DEBUG"),
            Level::Info => write!(f, "INFO"),
            Level::Warn => write!(f, "WARN"),
            Level::Error => write!(f, "ERROR"),
        }
    }
}

impl std::str::FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" => Ok(Level::Trace),
            "DEBUG" => Ok(Level::Debug),
            "INFO" => Ok(Level::Info),
            "WARN" => Ok(Level::Warn),
            "ERROR" => Ok(Level::Error),
            _ => Err(format!("Unknown log level: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logger {
    pub level: Level,
}

impl Logger {
    pub fn new(level: Level) -> Logger {
        Logger { level }
    }

    pub fn default() -> Arc<Logger> {
        let level = std::env::var("LOG_LEVEL")
            .map(|level| level.parse::<Level>().unwrap_or(Level::Info))
            .unwrap_or(Level::Info);

        Arc::new(Logger { level })
    }

    pub fn log(&self, level: Level, message: &str) {
        if level >= self.level {
            println!(
                "[{}{}{}] {}",
                match level {
                    Level::Trace => FG_GRAY,
                    Level::Debug => FG_BLUE,
                    Level::Info => FG_GREEN,
                    Level::Warn => FG_YELLOW,
                    Level::Error => FG_RED,
                },
                level,
                FG_RESET,
                message
            );
        }
    }

    pub fn trace(&self, message: &str) {
        self.log(Level::Trace, message);
    }

    pub fn debug(&self, message: &str) {
        self.log(Level::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.log(Level::Info, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(Level::Warn, message);
    }

    pub fn error(&self, message: &str) {
        self.log(Level::Error, message);
    }
}

use serde_json::{json, Value};
use std::collections::BTreeMap;
use tracing::field::{Field, Visit};

struct FieldVisitor {
    fields: BTreeMap<String, Value>,
}

impl FieldVisitor {
    fn new() -> Self {
        FieldVisitor {
            fields: BTreeMap::new(),
        }
    }
}

impl Visit for FieldVisitor {
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields
            .insert(field.name().to_string(), Value::Bool(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields
            .insert(field.name().to_string(), Value::Number(value.into()));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields
            .insert(field.name().to_string(), Value::Number(value.into()));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .insert(field.name().to_string(), Value::String(value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.fields.insert(
            field.name().to_string(),
            Value::String(format!("{:?}", value)),
        );
    }
}

pub struct CustomLoggerLayer {
    pub logger: Arc<Logger>,
}

impl<S> Layer<S> for CustomLoggerLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let level = match *event.metadata().level() {
            TracingLevel::TRACE => Level::Trace,
            TracingLevel::DEBUG => Level::Debug,
            TracingLevel::INFO => Level::Info,
            TracingLevel::WARN => Level::Warn,
            TracingLevel::ERROR => Level::Error,
        };

        if level >= self.logger.level {
            let mut visitor = FieldVisitor::new();
            event.record(&mut visitor);
            let message = visitor
                .fields
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or(event.metadata().name());

            let json_fields = json!(visitor.fields);
            self.logger
                .log(level, &format!("{} {}", message, json_fields));
        }
    }
}
