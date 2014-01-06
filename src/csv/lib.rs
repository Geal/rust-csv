#[link(name = "csv", vers = "0.1", author = "geal")];
#[crate_type = "lib"];
#[desc = "CSV parser"];
#[license = "MIT"];

use std::io;
use std::str;
use std::vec;
use std::iter::Iterator;

type Row = ~[~str];

pub struct Parser<R> {
  priv count: uint,
  priv readlen: uint,
  priv delim: char,
  priv buffer: ~[char],
  priv acc: ~[char],
  priv row: Row,
  priv reader: ~R
}


pub fn init<R: io::Reader>(reader: ~R) -> ~Parser<R> {

  ~Parser {
    count: 5,
    readlen: 1024,
    delim: ',',
    buffer: ~[],
    acc: ~[],
    row: ~[],
    reader: reader
  }
}

enum State {
  Continue,
  Wait,
  EOL,
  EOF
}

impl<R: Reader> Parser<R> {
  fn parse_next_char(&mut self) -> State {
    if self.buffer.len() == 0 {
      if self.reader.eof() {
        return EOF
      }

      let bytes = self.reader.read_bytes(self.readlen);
      if bytes.len() == 0 && ! self.reader.eof() {
        return Wait
      }

      for el in str::from_utf8(bytes).chars() {
        self.buffer.push(el);
      }
      return Continue
    }

    let optc = self.buffer.shift_opt();
    match optc {
      None    => return Wait,
      Some(c) => return self.parse_char(c)
    }
    Continue
  }

  fn parse_char(&mut self, c: char) -> State {
    let delim = self.delim;
    if c == self.delim {
        self.row.push(str::from_chars(self.acc));
        self.acc.clear();
        return Continue
    }

    match c {
      '\r' => Continue,
      '\n' => {
        self.row.push(str::from_chars(self.acc));
        self.acc.clear();
        EOL
      },
      _    => {
        self.acc.push(c);
        Continue
      }
    }
  }

  fn extract_row(&mut self) -> Row {
    let row = self.row.clone();
    self.row.clear();
    row
  }
}

impl<R: Reader> Iterator<Row> for Parser<R> {
  fn next(&mut self) -> Option<Row> {
    /*while(1) {
      match self.parse_next_char() {
        EOL => got a row
        EOF => maybe got a row (test)
        Continue => Continue
        Wait => wait for data
      }
    }*/
    if self.count ==  0 {
      return None
    }

    self.count -= 1;
    return Some(~[~"a", ~"b", ~"c", ~"d"]);
  }
}
