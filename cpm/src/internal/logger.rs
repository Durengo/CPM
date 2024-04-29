use std::sync::Arc;
use once_cell::sync::Lazy;
use spdlog::{prelude::*, sink::Sink, Logger, LoggerBuilder};

// Struct to hold all loggers
pub struct Loggers {
    def: Logger,
}

// Global instance of loggers
static LOGGERS: Lazy<Loggers> = Lazy::new(|| {
    // Get sinks from the default logger
    let sinks: Vec<Arc<dyn Sink>> = spdlog::default_logger().sinks().to_owned();
    let mut builder: LoggerBuilder = Logger::builder();
    let builder: &mut LoggerBuilder = builder.sinks(sinks).level_filter(LevelFilter::All);

    let def = builder.name("def").build().expect("Failed to build DEFAULT logger");

    default_logger();

    Loggers { def }
});

fn default_logger() {
    let default_logger: Arc<Logger> = spdlog::default_logger();
    default_logger.set_level_filter(LevelFilter::All);
}

impl Loggers {
    pub fn def() -> &'static Logger {
        &LOGGERS.def
    }
}
