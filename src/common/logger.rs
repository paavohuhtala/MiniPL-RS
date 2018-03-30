use common::configuration::{LogLevel, Options};

pub trait Logger {
  fn log_level(&self) -> LogLevel;
  fn write_line(&self, s: &str);
}

#[derive(Debug)]
pub struct ConsoleLogger {
  pub log_level: LogLevel,
}

impl Logger for ConsoleLogger {
  fn log_level(&self) -> LogLevel {
    self.log_level
  }

  fn write_line(&self, s: &str) {
    println!("{}", s);
  }
}

impl ConsoleLogger {
  pub fn from_options(options: &Options) -> ConsoleLogger {
    ConsoleLogger {
      log_level: options.log_level,
    }
  }
}

pub struct NullLogger;

impl Logger for NullLogger {
  fn log_level(&self) -> LogLevel {
    LogLevel::Normal
  }

  fn write_line(&self, s: &str) {}
}

#[macro_export]
macro_rules! debug_log {
  ($logger: expr, $fmt: tt, $($arg: expr),*) => {
    use common::configuration::{LogLevel};

    if $logger.log_level() >= LogLevel::Debug {
      println!($fmt, $($arg),*);
    }
  };
}
