use std::sync::Arc;

const FG_RED: &str = "\x1b[31m";
const FG_YELLOW: &str = "\x1b[33m";
const FG_BLUE: &str = "\x1b[34m";
const FG_GREEN: &str = "\x1b[32m";
//const FG_LIGHT_GREEN: &str = "\x1b[92m";
const FG_GRAY: &str = "\x1b[90m";
//const FG_WHITE: &str = "\x1b[37m";
const FG_RESET: &str = "\x1b[0m";

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
        match s {
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
