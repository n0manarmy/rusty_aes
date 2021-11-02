
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::Handle;


pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct LogBuilder {}

impl LogBuilder {

    /// Builds our logger based on a programmatic implementation. This reduces the required
    /// config files needed to run the application.
    pub fn build_logger(log_level: LogLevel) -> Handle {
        let stdout = ConsoleAppender::builder().build();
        let appender = Appender::builder().build("stdout", Box::new(stdout));
    
        let mut logger = Logger::builder().build("app::backend::db", LevelFilter::Info);
        let mut builder = Root::builder().appender("stdout").build(LevelFilter::Info);
    
        match log_level {
            LogLevel::Debug => {
                logger = Logger::builder().build("app::backend::db", LevelFilter::Debug);
                builder = Root::builder().appender("stdout").build(LevelFilter::Debug);
            }
            LogLevel::Info => (),
            LogLevel::Warn => {
                logger = Logger::builder().build("app::backend::db", LevelFilter::Warn);
                builder = Root::builder().appender("stdout").build(LevelFilter::Warn);
                
            }
            LogLevel::Error => {
                logger = Logger::builder().build("app::backend::db", LevelFilter::Error);
                builder = Root::builder().appender("stdout").build(LevelFilter::Error);
            }
        }
        let config = Config::builder().appender(appender).logger(logger).build(builder).unwrap();
    
        log4rs::init_config(config).unwrap()
    }
}

