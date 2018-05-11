extern crate miniplrs;

use std::rc::Rc;

use miniplrs::common::errors::*;
use miniplrs::common::logger::NullLogger;
use miniplrs::run_script;
use miniplrs::runtime::Io;

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
  fn write(&mut self, line: &str) {
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
        panic!(
          "assertion failed: expected pattern {}, was {:?}",
          stringify!($b),
          $a
        );
      }
    }
  };
}

macro_rules! integration_tests {
  {$( $test: ident($src: expr) {
    result $pattern: pat,
    input $input: tt,
    output $output: tt
  } )* } => {
    $(
      #[test]
      pub fn $test() {
        println!("Running test {}", stringify!($test));
        let input: &[&'static str] = &$input;
        let output: &[&'static str] = &$output;

        let source: &'static str = $src;
        let mut io = TestIo::new(input);
        let result = run_script(source, &mut io, Rc::new(NullLogger), None);

        let result_with_sliced_errors: Result<(), &[ExecutionError]> = if let Err(ref errors) = result {
          let sliced_errors = errors.as_slice();
          Err(sliced_errors)
        } else {
          Ok(())
        };

        assert_match!(result_with_sliced_errors => $pattern);
        assert_eq!(io.output, output);
      }
    )*
  };
}

use miniplrs::common::errors::LexerError::*;
use miniplrs::common::errors::ParserError::*;
use miniplrs::parsing::token::TokenKind::*;
use miniplrs::semantic::type_checker::TypeError::*;
use miniplrs::ExecutionError;

integration_tests! {
  empty_program("") {
    result Ok(_),
    input [],
    output []
  }

  hello_world(r#"
    print "Hello, world!";
  "#) {
    result Ok(_),
    input [],
    output ["Hello, world!"]
  }

  hello_world_without_semicolon(r#"
    print "Hello, world!"
  "#) {
    result Err(&[
      ExecutionError::ParserError(
        ErrWithCtx(UnexpectedToken { expected: SemicolonK, was: EndOfFileK }, _)
      )
    ]),
    input [],
    output []
  }

  bool_operators(r#"
    var a : int := 10;
    var b : int := 20;
    var c : bool := !(a = b) = !(b = a);
    assert c;
  "#) {
    result Ok(_),
    input [],
    output []
  }

  assignment_valid(r#"
    var a : int := 10;
    print a;
    a := 20;
    print a;
  "#) {
    result Ok(_),
    input [],
    output ["10", "20"]
  }

  assignment_non_existant_identifier(r#"
    a := 10;
    print a;
  "#) {
    result Err(&[ExecutionError::TypeError(UndeclaredIdentifier(_))]),
    input [],
    output []
  }

  read_roundtrip_string(r#"
    var a : string;
    read a;
    print a;
  "#) {
    result Ok(_),
    input ["Neat"],
    output ["Neat"]
  }

  read_int_add(r#"
    var a : int;
    read a;
    a := a + 101;
    print a;
  "#) {
    result Ok(_),
    input ["22"],
    output ["123"]
  }

  for_basic(r#"
    var i : int;
    for i in 0 .. 3 do
      print i;
    end for;
  "#) {
    result Ok(_),
    input [],
    output ["0", "1", "2", "3"]
  }

  for_nested(r#"
    var x : int;
    for x in 1 .. 3 do
      var y: int;
      for y in 1 .. 3 do
        print x * y;
      end for;
    end for;
  "#) {
    result Ok(_),
    input [],
    output ["1", "2", "3", "2", "4", "6", "3", "6", "9"]
  }

  arithmetic_sub_order(r#"
    var x : int := 3;
    var y : int := 5;
    print x - y;
    print y - x;
    print x - x;
    print y - y;
  "#) {
    result Ok(_),
    input [],
    output ["-2", "2", "0", "0"]
  }

  arithmetic_div_uses_truncation(r#"
    print 10 / 2;
    print 10 / 3;
    print 100 / 101;
  "#) {
    result Ok(_),
    input [],
    output ["5", "3", "0"]
  }

  string_comparison(r#"
    assert "aaa" < "bbb";
    assert !("bbb" < "aaa");
    assert "bbb" < "ccc";
  "#) {
    result Ok(_),
    input [],
    output []
  }

  logic_operators_mix(r#"
    assert (1 = 1) &
           (1 < 2) &
           (!(1 = 2) & !(2 = 1));
  "#) {
    result Ok(_),
    input [],
    output []
  }

  loop_variable_mutability(r#"
    var i : int;
    for i in 0 .. 10 do
      i := 100;
    end for;
  "#) {
    result Err(&[ExecutionError::TypeError(AssignToImmutable(_))]),
    input [],
    output []
  }

  comments(r#"
    // Line comment!
    /*
      Block comment!
    */
    assert 1 = 1;
  "#) {
    result Ok(_),
    input [],
    output []
  }

  just_comments(r#"
    // Line comment!
    /*Block comment!*/
  "#) {
    result Ok(_),
    input [],
    output []
  }

  invalid_block_comment_1(r#"/*/"#) {
    result Err(&[
      ExecutionError::ParserError(ErrWithCtx(LexerError(UnterminatedComment), _))
    ]),
    input [],
    output []
  }

  invalid_block_comment_2(r#"/* * * * * * / * / * / **"#) {
    result Err(&[
      ExecutionError::ParserError(ErrWithCtx(LexerError(UnterminatedComment), _))
    ]),
    input [],
    output []
  }

  example_program_1(r#"
    var X : int := 4 + (6 * 2);
    print X;
  "#) {
    result Ok(_),
    input [],
    output ["16"]
  }

  example_program_2(
r#"var nTimes : int := 0;
print "How many times?";
read nTimes;
var x : int;
for x in 0..nTimes-1 do
    print x;
    print " : Hello, World!\n";
end for;
assert (x = nTimes);
"#) {
    result Ok(_),
    input ["3"],
    output [
      "How many times?",
      "0", " : Hello, World!\n",
      "1", " : Hello, World!\n",
      "2", " : Hello, World!\n",
      "ASSERTION FAILED:\n[   9]  assert (x = nTimes);\n"
    ]
  }

example_program_3(
r#"print "Give a number";
var n : int;
read n;
var v : int := 1;
var i : int;
for i in 1..n do
    v := v * i;
end for;
print "The result is: ";
print v;
"#) {
  result Ok(_),
  input ["5"],
  output ["Give a number", "The result is: ", "120"]
}

  print_uninitialised_variable(r#"
    var a : int;
    print a;

    var b : string;
    print b;
  "#) {
    result Ok(_),
    input [],
    output ["0", ""]
  }
}
