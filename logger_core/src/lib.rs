use once_cell::sync::OnceCell;
use std::sync::Once;
use std::sync::RwLock;
use tracing::{self, event};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    filter::Filtered,
    fmt::{
        format::{DefaultFields, Format},
        Layer,
    },
    layer::Layered,
    Registry,
};

use tracing_subscriber::{
    self,
    filter::{self, LevelFilter},
    prelude::*,
    reload::{self, Handle},
};

// INIT_ONCE is responsible for making sure that we initiating the logger just once, and every other call to the
// init function will just reload the existing logger
static INIT_ONCE: Once = Once::new();

// reloadable handles for resetting the logger
pub static STDOUT_RELOAD: OnceCell<
    RwLock<reload::Handle<Filtered<Layer<Registry>, LevelFilter, Registry>, Registry>>,
> = OnceCell::new();

pub static FILE_RELOAD: OnceCell<
    RwLock<
        Handle<
            Filtered<
                Layer<
                    Layered<
                        reload::Layer<Filtered<Layer<Registry>, LevelFilter, Registry>, Registry>,
                        Registry,
                    >,
                    DefaultFields,
                    Format,
                    RollingFileAppender,
                >,
                LevelFilter,
                Layered<
                    reload::Layer<Filtered<Layer<Registry>, LevelFilter, Registry>, Registry>,
                    Registry,
                >,
            >,
            Layered<
                reload::Layer<Filtered<Layer<Registry>, LevelFilter, Registry>, Registry>,
                Registry,
            >,
        >,
    >,
> = OnceCell::new();

#[derive(Debug)]
pub enum Level {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}
impl Level {
    fn to_filter(&self) -> filter::LevelFilter {
        match self {
            Level::Trace => LevelFilter::TRACE,
            Level::Debug => LevelFilter::DEBUG,
            Level::Info => LevelFilter::INFO,
            Level::Warn => LevelFilter::WARN,
            Level::Error => LevelFilter::ERROR,
        }
    }
}

// Initialize the global logger to error level on the first call only
// In any of the calls to the function, including the first - resetting the existence loggers to the new setting
// provided by using the global reloadable handle
// The logger will save only logs of the given level or above.
pub fn init(minimal_level: Option<Level>, file_name: Option<&str>) -> Level {
    let level = minimal_level.unwrap_or(Level::Warn);
    let level_filter = level.to_filter();
    INIT_ONCE.call_once(|| {
        let stdout_fmt = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_filter(LevelFilter::ERROR); // Console logging ERROR by default

        let (stdout_layer, stdout_reload) = reload::Layer::new(stdout_fmt);

        let file_appender = RollingFileAppender::new(
            Rotation::HOURLY,
            "babushka-logs",
            file_name.unwrap_or("output.log"),
        );
        let file_fmt = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_writer(file_appender)
            .with_filter(LevelFilter::ERROR); // File logging ERROR by default
        let (file_layer, file_reload) = reload::Layer::new(file_fmt);
        tracing_subscriber::registry()
            .with(stdout_layer)
            .with(file_layer)
            .init();
        let _ = STDOUT_RELOAD.set(RwLock::new(stdout_reload));
        let _ = FILE_RELOAD.set(RwLock::new(file_reload));
    });

    match file_name {
        None => {
            let _ = STDOUT_RELOAD
                .get()
                .expect("error reloading stdout")
                .write()
                .expect("error reloading stdout")
                .modify(|layer| (*layer.filter_mut() = level_filter));
        }
        Some(file) => {
            let file_appender = RollingFileAppender::new(Rotation::HOURLY, "babushka-logs", file);
            let _ = FILE_RELOAD
                .get()
                .expect("error reloading stdout")
                .write()
                .expect("error reloading stdout")
                .modify(|layer| {
                    *layer.filter_mut() = level_filter;
                    *layer.inner_mut().writer_mut() = file_appender;
                });
        }
    };
    level
}

macro_rules! create_log {
    ($name:ident, $uppercase_level:tt) => {
        pub fn $name<Message: AsRef<str>, Identifier: AsRef<str>>(
            log_identifier: Identifier,
            message: Message,
        ) {
            if STDOUT_RELOAD.get().is_none() {
                init(Some(Level::Warn), None);
            };
            let message_ref = message.as_ref();
            let identifier_ref = log_identifier.as_ref();
            event!(
                tracing::Level::$uppercase_level,
                "{identifier_ref} - {message_ref}"
            )
        }
    };
}

create_log!(log_trace, TRACE);
create_log!(log_debug, DEBUG);
create_log!(log_info, INFO);
create_log!(log_warn, WARN);
create_log!(log_error, ERROR);

// Logs the given log, with log_identifier and log level prefixed. If the given log level is below the threshold of given when the logger was initialized, the log will be ignored.
// log_identifier should be used to add context to a log, and make it easier to connect it to other relevant logs. For example, it can be used to pass a task identifier.
// If this is called before a logger was initialized the log will not be registered.
// If logger doesn't exist, create the default
pub fn log<Message: AsRef<str>, Identifier: AsRef<str>>(
    log_level: Level,
    log_identifier: Identifier,
    message: Message,
) {
    match log_level {
        Level::Debug => log_debug(log_identifier, message),
        Level::Trace => log_trace(log_identifier, message),
        Level::Info => log_info(log_identifier, message),
        Level::Warn => log_warn(log_identifier, message),
        Level::Error => log_error(log_identifier, message),
    }
}
