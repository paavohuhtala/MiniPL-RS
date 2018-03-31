extern crate miniplrs;

use std::env;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

use miniplrs::common::configuration::parse_command_line_args;
use miniplrs::common::errors::{ErrorWithContext, ErrorWithReason};
use miniplrs::common::logger::ConsoleLogger;
use miniplrs::diagnostics::file_context::FileContextSource;
use miniplrs::runtime::console_io::ConsoleIo;
use miniplrs::{run_script, ExecutionError};

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
  let mut input_file = std::fs::File::open(path)?;
  let mut file_content = String::new();
  input_file
    .read_to_string(&mut file_content)
    .expect("Can't read source file.");
  Ok(file_content)
}

fn main() {
  let options = parse_command_line_args(env::args());
  let source = read_file(&options.input_file).expect("File should be readable.");

  // Use ConsoleIo as the IO handler, which uses stdin and stdout for print and read.
  let mut io = ConsoleIo;

  let logger = ConsoleLogger::from_options(&options);

  let file_context = Rc::new(FileContextSource::from_str(
    &source,
    Some(options.input_file),
  ));

  match run_script(
    &source,
    &mut io,
    Rc::new(logger),
    Some(file_context.clone()),
  ) {
    Ok(_) => return,
    Err(errors) => {
      // println!("Errors: {:?}", errors);
      print_errors(&errors, &file_context);
    }
  }
}

fn print_errors(errors: &[ExecutionError], ctx: &FileContextSource) {
  let file_info_part = if let Some(ref file_name) = ctx.file_name {
    format!(" in {}", file_name)
  } else {
    "".to_string()
  };

  println!(
    "Encountered {} {}{}:\n",
    errors.len(),
    if errors.len() == 1 { "error" } else { "errors" },
    file_info_part
  );

  for error in errors {
    match error {
      ExecutionError::ParserError(err) => {
        let offset = err.get_offset();
        let position = ctx.decode_offset(offset).unwrap();
        let quoted_line_range = position.row - 1..position.row + 2;

        let quoted_lines = quoted_line_range
          .map(|i| (i, ctx.get_line(i)))
          .filter_map(|(i, line)| line.map(|x| FileContextSource::format_source_quote_line(i, &x)))
          .fold(String::new(), |acc, x| acc + &x);

        println!("On row {}, column {}:", position.row, position.column);
        println!("Parser error: {}", err.get_reason().unwrap());

        println!("{}", quoted_lines);
      }
      ExecutionError::TypeError(err) => {
        println!("Type error: {}", err.get_reason().unwrap());
      }
    }
  }
}
