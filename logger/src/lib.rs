struct Logger {}
impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        use colored::Colorize;
        use log::Level::*;
        let level = match record.level() {
            Error => format!("{:<5}", record.level()).red(),
            Warn => format!("{:<5}", record.level()).yellow(),
            Info => format!("{:<5}", record.level()).cyan(),
            Debug => format!("{:<5}", record.level()).purple(),
            Trace => format!("{:<5}", record.level()).normal(),
        };
        println!(
            "{} {}:{} {}",
            level,
            record.file().unwrap_or("?"),
            record.line().unwrap_or(0),
            record.args()
        );
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger {};

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
}
