
#[macro_export]
macro_rules! assert_match {
  ($a:expr => $b:pat) => {
    match $a {
      $b => (),
      _ => {
        panic!("assertion failed: expected pattern {}, was {:?}", stringify!($b), $a);
      }
    }
  };
  ($a:expr => $b:pat, $($arg:tt)+) => {
    match $a {
      $b => (),
      _ => {
        panic!("assertion failed: expected pattern {}, was {:?}\n{}", stringify!($b), $a, format_args!($($arg)+));
      }
    }
  }
}
