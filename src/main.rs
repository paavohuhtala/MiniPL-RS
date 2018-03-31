extern crate miniplrs;

use std::env;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

use miniplrs::common::configuration::parse_command_line_args;
use miniplrs::common::logger::ConsoleLogger;
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
  let input_chars = read_file(&options.input_file).expect("File should be readable.");

  // Use ConsoleIo as the IO handler, which uses stdin and stdout for print and read.
  let mut io = ConsoleIo;

  let logger = ConsoleLogger::from_options(&options);

  match run_script(&input_chars, &mut io, Rc::new(logger)) {
    Ok(_) => return,
    Err(errors) => {
      println!("Errors: {:?}", errors);
    }
  }
}
