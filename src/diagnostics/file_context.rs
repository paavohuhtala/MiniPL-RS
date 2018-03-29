use std::ops::Range;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct FilePosition {
  pub offset: usize,
  pub column: usize,
  pub row: usize,
}

pub struct FileContextSource {
  file_name: Option<String>,
  // Values are tuples of (offset from start, content)
  // Line data is stored using reference counted pointers so that
  // we can safely give out references to them.
  pub lines: Vec<(usize, Rc<String>)>,
}

impl FileContextSource {
  pub fn from_str(source: &str, file_name: Option<String>) -> FileContextSource {
    let lines = source
      .lines()
      .scan(0, |offs, line| {
        let initial_offs = *offs;
        *offs = *offs + line.len() + 1;
        Some((initial_offs, line.to_string().into()))
      })
      .collect();

    FileContextSource { lines, file_name }
  }

  pub fn decode_offset(&self, offset: usize) -> Option<FilePosition> {
    for (row_index, &(first_index, ref row_content)) in self.lines.iter().enumerate() {
      let len = row_content.len();

      println!(
        "{} should be in inclusive range [{}, {}]",
        offset,
        first_index,
        first_index + len
      );

      if offset >= first_index && offset <= first_index + len {
        return Some(FilePosition {
          offset,
          row: row_index + 1,
          column: offset - first_index + 1,
        });
      }
    }

    None
  }

  // Terminology:
  // A row is a 1-based index into the file
  // A line is a string, containing the contents of a particular row.
  pub fn get_line(&self, row: usize) -> Option<Rc<String>> {
    self
      .lines
      .get(row - 1)
      .map(|&(_, ref content_pointer)| content_pointer.clone())
  }

  pub fn get_range_lines(&self, range: &Range<usize>) -> Vec<Rc<String>> {
    let start = self
      .decode_offset(range.start)
      .expect("This should be a valid offset.");

    let end = self
      .decode_offset(range.end)
      .expect("This should be a valid offset.");

    println!("Range: {:?}, start: {:?}, end: {:?}", range, start, end);

    (start.row..end.row + 1)
      .map(|x| self.get_line(x).expect("Should be a valid row."))
      .collect()
  }
}
