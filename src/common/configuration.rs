use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
  Normal,
  Debug,
}

#[derive(Debug)]
pub struct Options {
  pub log_level: LogLevel,
  pub input_file: String,
}

impl Default for Options {
  fn default() -> Options {
    Options {
      log_level: LogLevel::Normal,
      input_file: "./minipl/hello.pl".to_string(),
    }
  }
}

/// Parse a type convertible to a string iterator into an Options object. Panics on errors.
pub fn parse_command_line_args<I>(args: I) -> Options
where
  I: IntoIterator<Item = String>,
{
  let mut args: VecDeque<String> = args.into_iter().skip(1).collect();
  let mut options = Options::default();

  while let Some(arg) = args.pop_front() {
    match arg.as_str() {
      "--verbose" | "--debug" | "-v" => options.log_level = LogLevel::Debug,
      "--file" | "-f" => {
        let file_name = args.pop_front().expect("Expected file name after --file.");
        options.input_file = file_name;
      }
      otherwise => println!("WARNING: Unknown command line argument: {}", otherwise),
    }
  }

  options
}
