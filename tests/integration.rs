extern crate miniplrs;

use miniplrs::runtime::Io;
use miniplrs::run_script;

struct TestIo {
  input: Vec<String>,
  output: Vec<String>,
}

impl TestIo {
  pub fn new(input: &[&str]) -> TestIo {
    TestIo {
      input: input.iter().map(|s| s.to_string()).collect(),
      output: Vec::new(),
    }
  }
}

impl Io for TestIo {
  fn write_line(&mut self, line: &str) {
    self.output.push(line.to_string());
  }

  fn read_line(&mut self) -> String {
    self.input.remove(0)
  }
}

macro_rules! assert_match {
  ($a:expr => $b:pat) => {
    match $a {
      $b => (),
      _ => {
        panic!("assertion failed: expected pattern {}, was {:?}", stringify!($b), $a);
      }
    }
  }
}

use miniplrs::ExecutionError;
use miniplrs::common::errors::*;
use miniplrs::common::types::*;
use miniplrs::semantic::type_checker::TypeError;

#[test]
pub fn run_hello_world() {
  let source = r#"
    print "Hello, world!";
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["Hello, world!"]);
}

#[test]
pub fn run_hello_world_without_semicolon() {
  let source = r#"
    print "Hello, world!"
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Err(
    ExecutionError::ParserError(
      ParserError::UnexpectedToken {
        expected: TokenKind::SemicolonK,
        was: TokenKind::EndOfFileK
      }
    )
  ));
  assert_eq!(io.output.len(), 0);
}

#[test]
pub fn bool_operators() {
  let source = r#"
    var a : int := 10;
    var b : int := 20;
    var c : bool := !(a = b) = !(b = a);
    assert c;
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output.len(), 0);
}

#[test]
pub fn assignment_valid() {
  let source = r#"
    var a : int := 10;
    print a;
    a := 20;
    print a;
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["10", "20"]);
}

#[test]
pub fn assignment_non_existant_identifier() {
  let source = r#"
    a := 10;
    print a;
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Err(ExecutionError::TypeError(TypeError::UndeclaredIdentifier(_a))));
}

#[test]
pub fn read_roundtrip_string() {
  let source = r#"
    var a : string;
    read a;
    print a;
  "#;

  let mut io = TestIo::new(&["Well this is neat."]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["Well this is neat."]);
  assert_eq!(io.input.len(), 0, "Input was consumed.");
}

#[test]
pub fn read_int_add() {
  let source = r#"
    var a : int;
    read a;
    a := a + 101;
    print a;
  "#;

  let mut io = TestIo::new(&["22"]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["123"]);
  assert_eq!(io.input.len(), 0, "Input was consumed.");
}

#[test]
pub fn for_basic() {
  let source = r#"
    var i : int;
    for i in 0 .. 3 do
      print i;
    end for;
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["0", "1", "2", "3"]);
}

#[test]
pub fn for_nested() {
  let source = r#"
    var x : int;
    for x in 1 .. 3 do
      var y: int;
      for y in 1 .. 3 do
        print x * y;
      end for;
    end for;
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["1", "2", "3", "2", "4", "6", "3", "6", "9"]);
}

#[test]
pub fn arithmetic_sub_order() {
  let source = r#"
    var x : int := 3;
    var y : int := 5;
    print x - y;
    print y - x;
    print x - x;
    print y - y;
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);
  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["-2", "2", "0", "0"]);
}

#[test]
pub fn arithmetic_div_uses_truncation() {
  let source = r#"
    print 10 / 2;
    print 10 / 3;
    print 100 / 101;
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);
  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["5", "3", "0"]);
}

#[test]
pub fn string_comparison() {
  let source = r#"
    assert "aaa" < "bbb";
    assert !("bbb" < "aaa");
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);
  assert_match!(result => Ok(()));
}
