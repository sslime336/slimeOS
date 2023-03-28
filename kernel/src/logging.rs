use log::{self, Level, LevelFilter, Log, Metadata, Record};

struct KernelLogger;

static LOGGER: KernelLogger = KernelLogger;

impl Log for KernelLogger {
    /// Returns true if the log level is enabled for the specified metadata.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // 红色
            Level::Warn => 93,  // 黄色
            Level::Info => 34,  // 蓝色
            Level::Debug => 32, // 绿色
            Level::Trace => 36, // 青色
        };
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }

    fn flush(&self) {
        todo!()
    }
}

pub fn init() {
    log::set_logger(&LOGGER).unwrap();

    log::set_max_level(match option_env!("LOG_LV") {
        // 通过环境变量 LOG_LV 来控制日志的输出级别
        Some(log_level) => match log_level {
            "ERROR" => LevelFilter::Error,
            "WARN" => LevelFilter::Warn,
            "INFO" => LevelFilter::Info,
            "DEBUG" => LevelFilter::Debug,
            "TRACE" => LevelFilter::Trace,
            _ => LevelFilter::Off,
        },
        None => LevelFilter::Info,
    });
}
