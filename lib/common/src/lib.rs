pub mod fixed_length_priority_queue;
pub mod validation;

use chrono::Local;
use log::{Level, Metadata, Record};

pub struct CustomFormatter;

impl log::Log for CustomFormatter {
  fn enabled(&self, metadata: &Metadata) -> bool {
    // Enable logging for all levels
    metadata.level() <= Level::max()
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
      let level = record.level();

      let current_thread = std::thread::current();
      let thread_name = current_thread.name().unwrap_or("main");

      let module_path = record.module_path().unwrap_or("<unknown>");

      let line = record.line().unwrap_or(0);

      // Format the log message
      let log_msg = format!(
        "[{}][{}][{}][{}:{}] > {}",
        timestamp,
        level,
        thread_name,
        module_path,
        line,
        record.args()
      );

      // Write the log message to stdout
      // let mut stdout = io::stdout();
      // writeln!(&mut stdout, "{}", log_msg).expect("Failed to write log");
      println!("{}", &log_msg);
    }
  }

  fn flush(&self) {}
}