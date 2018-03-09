extern crate miniplrs;

use std::path::Path;
use std::io::Read;

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
  // Read the file into a character vector.
  let input_chars = read_file("./minipl/hello.pl").expect("File should be readable.");

  // Use ConsoleIo as the IO handler, which uses stdin and stdout for print and read.
  let mut io = ConsoleIo;

  match run_script(&input_chars, &mut io) {
    Ok(_) => return,
    Err(ExecutionError::ParserError(err)) => {
      println!("Parse error: {:?}", err);
    }
    Err(ExecutionError::TypeError(err)) => {
      println!("Type error: {:?}", err);
    }
  }
}
