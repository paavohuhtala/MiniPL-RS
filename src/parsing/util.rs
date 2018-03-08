pub fn is_whitespace(ch: char) -> bool {
  match ch {
    ' ' | '\r' | '\n' | '\t' => true,
    _ => false,
  }
}

pub fn is_number(ch: char) -> bool {
  match ch {
    '0'...'9' => true,
    _ => false,
  }
}

// No Unicode support for now.
pub fn is_letter(ch: char) -> bool {
  match ch {
    'A'...'z' => true,
    _ => false,
  }
}

pub fn is_valid_in_identifier(c: char) -> bool {
  is_letter(c) || is_number(c) || c == '_'
}

// An identifier starts with a letter, followed by a mix of letters, numbers and underscores.
pub fn is_valid_identifier(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }

  for (i, c) in s.char_indices() {
    if i == 0 && !is_letter(c) {
      return false;
    }

    if !is_valid_in_identifier(c) {
      return false;
    }
  }

  true
}
