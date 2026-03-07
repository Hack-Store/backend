pub mod logger {
    use colored::Colorize;

    pub struct Logger {}

    impl Logger {
        fn new() -> Logger {
            Logger {}
        }
        pub(crate) fn log(&self, message: &str, level: LogLevel) {
            let message = match level {
                LogLevel::INFO => format!("[INFO] {}", message).white(),
                LogLevel::ERROR => format!("[ERROR] {}", message).red(),
                LogLevel::WARN => format!("[WARN] {}", message).yellow(),
                LogLevel::DEBUG => format!("[DEBUG] {}", message).blue(),
                LogLevel::CRITICAL => format!("[CRITICAL] {}", message).red().bold(),
            };
            println!("{}", message);
        }
    }
    pub enum LogLevel {
        INFO,
        ERROR,
        WARN,
        DEBUG,
        CRITICAL,
    }
}
