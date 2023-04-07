use std::{sync::{Mutex, MutexGuard, atomic::AtomicBool}, fmt::Debug};
use std::time::SystemTime;

#[cfg(feature = "colors")]
pub use owo_colors::CssColors;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "colors")] {
            use owo_colors::OwoColorize;
            match self {
                LogLevel::Error => write!(f, "{}", "ERRR".red()),
                LogLevel::Warn  => write!(f, "{}", "WARN".yellow()),
                LogLevel::Info  => write!(f, "{}", "INFO".green()),
                LogLevel::Debug => write!(f, "{}", "DBUG".cyan()),
                LogLevel::Trace => write!(f, "{}", "TRCE".blue()),
            }
        }
        #[cfg(not(feature = "colors"))] {
            match self {
                LogLevel::Error => write!(f, "{}", "ERRR"),
                LogLevel::Warn  => write!(f, "{}", "WARN"),
                LogLevel::Info  => write!(f, "{}", "INFO"),
                LogLevel::Debug => write!(f, "{}", "DBUG"),
                LogLevel::Trace => write!(f, "{}", "TRCE"),
            }
        }
    }
}

pub struct LogMessage<'a> {
    pub namespace: &'a str,
    pub level: LogLevel,
    pub message: &'a str,
}

static LOGGING_LOGGING: AtomicBool = AtomicBool::new(false);

struct Logger {
    stdout: bool,
    custom_sinks: Vec<Box<dyn for<'a, 'b> Fn(&'a SystemTime, &'b LogMessage<'b>) + Send + Sync>>,
    filter_level: LogLevel,
    time_format: Option<&'static str>,
    exclude_namespaces: Vec<Vec<&'static str>>,

    #[cfg(feature = "colors")]
    crate_colors: std::collections::HashMap<&'static str, CssColors>,
}

impl std::default::Default for Logger {
    fn default() -> Self {
        Logger {
            stdout: true,
            custom_sinks: vec![],
            filter_level: LogLevel::Trace,
            time_format: None, // %y-%m-%d %H:%M:%S
            exclude_namespaces: vec![],

            #[cfg(feature = "colors")]
            crate_colors: std::collections::HashMap::from_iter([("justlogfox", CssColors::Orange)]),
        }
    }
}

impl Logger {

    fn log(&self, message: &LogMessage) {
        if message.level > self.filter_level {
            return;
        }
        let namespace_segments = message.namespace.split("::").collect::<Vec<_>>();
        if self.exclude_namespaces.iter().any(|ns| namespace_segments.starts_with(ns)) {
            return;
        }
    
        let now = SystemTime::now();
    
        if self.stdout {
            let time_segment = if let Some(time_format) = self.time_format {
                format!("{} ", chrono::DateTime::<chrono::Local>::from(now).format(time_format))
            } else {
                "".to_string()
            };
            let crate_ = format!("[{}]", message.namespace);
            #[cfg(feature = "colors")] {
                use owo_colors::OwoColorize;
                let color = self.crate_colors.get(namespace_segments[0]).unwrap_or(&CssColors::White);
                // let link_location = format!("file:{}#{}:{}", file!(), line!(), column!()); //, line!(), column!()
                // let link_esc = format!("\x1b]8;id=hi;{}\x1b\\", link_location);
                // let link_esc_end = "\x1b]8;;\x1b\\";
                // println!("{}{}{}{} {} {}", time_segment.bright_black(), link_esc, crate_.color(*color), link_esc_end, message.level, message.message);
                println!("{}{} {} {}", time_segment.bright_black(), crate_.color(*color), message.level, message.message);
            } #[cfg(not(feature = "colors"))] {
                println!("{}{} {} {}", time_segment, crate_, message.level, message.message);
            }
        }
        for sink in self.custom_sinks.iter() {
            sink(&now, message);
        }
    }


    fn log_log(&self, message: &str) {
        if LOGGING_LOGGING.load(std::sync::atomic::Ordering::Relaxed) {
            self.log(&LogMessage {
                namespace: "justlogfox",
                level: LogLevel::Trace,
                message,
            })
        }
    }
}

static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

fn ensure_init_logger<'a>() -> MutexGuard<'a, Option<Logger>> {
    let mut logger_guard = LOGGER.lock().unwrap();
    if logger_guard.is_none() {
        *logger_guard = Some(Logger::default());
        logger_guard.as_ref().unwrap().log_log("Initialized static logger");
    }
    logger_guard
}


pub fn log(namespace: &'static str, level: LogLevel, message: &str) {
    let guard = ensure_init_logger();
    let logger = guard.as_ref().unwrap();

    let log_msg = &LogMessage { namespace, level, message };
    logger.log(log_msg);
}

pub fn set_log_level(level: LogLevel) {
    let mut guard = ensure_init_logger();
    let logger = guard.as_mut().unwrap();
    logger.filter_level = level;
    logger.log_log(&format!("Set filter level: {:?}", level));
}

pub fn add_logger<F>(sink: F)
where
    F: for<'a, 'b> Fn(&'a SystemTime, &'b LogMessage<'b>) + Send + Sync + 'static
{
    let mut guard = ensure_init_logger();
    let logger = guard.as_mut().unwrap();
    logger.log_log("New sink added");
    logger.custom_sinks.push(Box::new(sink));
}

pub fn verbose_verbose_verbose() {
    LOGGING_LOGGING.store(true, std::sync::atomic::Ordering::Relaxed);
    set_log_level(LogLevel::Trace);
}

pub fn set_log_time_format(format: Option<&'static str>) {
    let mut guard = ensure_init_logger();
    let logger = guard.as_mut().unwrap();
    logger.time_format = format;
    logger.log_log(&format!("Set time format: {:?}", format));
}

#[cfg(feature = "colors")]
pub fn set_crate_color(crate_: &'static str, color: CssColors) {
    let mut guard = ensure_init_logger();
    let logger = guard.as_mut().unwrap();
    logger.crate_colors.insert(&crate_, color);
    logger.log_log(&format!("Set crate '{}' to color {:?}", crate_, color));
}

#[cfg(feature = "colors")]
#[macro_export]
macro_rules! set_crate_color {
    ($color:expr) => {
        let crate_ = module_path!().split("::").next().unwrap();
        $crate::set_crate_color(crate_, $color);
    };
}

/// Log a message at the specified level.
/// Accepts a format string and any number of arguments.
#[macro_export]
macro_rules! log {
    ([$namespace:path] $level:expr, $fmt:literal, $($fmt_args:tt),+) => {
        {
        let message = format!($fmt, $($fmt_args),+);
        let namespace = stringify!($namespace);
        $crate::log(namespace, $level, &message);
        }
    };
    ([$namespace:path] $level:expr, $one_arg:expr) => {
        $crate::log!([$namespace] $level, "{}", $one_arg);
    };

    ($level:expr, $fmt:literal, $($fmt_args:tt),+) => {
        {
        let message = format!($fmt, $($fmt_args),+);
        let namespace = module_path!();
        $crate::log(namespace, $level, &message);
        }
    };
    ($level:expr, $one_arg:expr) => {
        {
            let arg = $one_arg;
            $crate::log!($level, "{}", arg);
            arg
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($($args:tt),+) => {
        $crate::log!($crate::LogLevel::Error, $($args),+);
    };
    ([$namespace:path] $($args:tt),+) => {
        $crate::log!([$namespace] $crate::LogLevel::Error, $($args),+);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($args:tt),+) => {
        $crate::log!($crate::LogLevel::Warn, $($args),+);
    };
    ([$namespace:path] $($args:tt),+) => {
        $crate::log!([$namespace] $crate::LogLevel::Warn, $($args),+);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($args:tt),+) => {
        $crate::log!($crate::LogLevel::Info, $($args),+);
    };
    ([$namespace:path] $($args:tt),+) => {
        $crate::log!([$namespace] $crate::LogLevel::Info, $($args),+);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($args:tt),+) => {
        $crate::log!($crate::LogLevel::Debug, $($args),+);
    };
    ([$namespace:path] $($args:tt),+) => {
        $crate::log!([$namespace] $crate::LogLevel::Debug, $($args),+);
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($args:tt),+) => {
        $crate::log!($crate::LogLevel::Trace, $($args),+);
    };
    ([$namespace:path] $($args:tt),+) => {
        $crate::log!([$namespace] $crate::LogLevel::Trace, $($args),+);
    };
}

#[cfg(test)]
mod tests {

    use std::sync::atomic::Ordering::{Relaxed, Release, Acquire};

    use super::*;

    fn clear_loggers()
    {
        let mut guard = ensure_init_logger();
        let logger = guard.as_mut().unwrap();
        logger.log_log("All sinks removed");
        logger.custom_sinks.clear();
    }

    #[test]
    fn custom_sink_gets_info() {
        add_logger(|_, &LogMessage {namespace, level, message}| {
            assert_eq!(message, "Hello, world! 1 2");
            assert_eq!(namespace, "justlogfox::tests");
            assert_eq!(level, LogLevel::Error);
        });
        log_error!("Hello, world! {} {}", 1, 2);

        clear_loggers();
    }

    #[test]
    fn custom_sink_is_called() {
        static CALLED: AtomicBool = AtomicBool::new(false);

        add_logger(|_, _| {
            CALLED.store(true, Release);
        });
        log_info!("Hello, world! {} {}", 1, 2);

        assert!( CALLED.load(Acquire) );

        clear_loggers();
    }

    #[test]
    fn filters_level() {
        static CALLED: AtomicBool = AtomicBool::new(false);

        set_log_level(LogLevel::Warn);

        add_logger(|_, _| {
            CALLED.store(true, Relaxed);
        });
        log_info!("info");

        assert!( !CALLED.load(Relaxed) );

        log_warn!("warn");

        assert!( CALLED.load(Relaxed) );

        CALLED.store(false, Relaxed);

        set_log_level(LogLevel::Trace);

        log_trace!("trace");

        assert!( CALLED.load(Relaxed) );

        clear_loggers();
    }
}
