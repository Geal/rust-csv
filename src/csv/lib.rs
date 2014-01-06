#[crate_type = "lib"];
#[desc = "CSV parser"];
#[license = "MIT"];

use std::io;
use std::str;
use std::iter::Iterator;

type Row = ~[~str];

enum State {
  Continue,
  Wait,
  EOL,
  EOF
}

pub struct Parser<R> {
  priv count: uint,
  priv readlen: uint,
  priv delim: char,
  priv buffer: ~[char],
  priv acc: ~[char],
  priv row: Row,
  priv reader: ~R,
  priv state: State
}


pub fn init<R: io::Reader>(reader: ~R) -> ~Parser<R> {

  ~Parser {
    count: 5,
    readlen: 1024,
    delim: ',',
    buffer: ~[],
    acc: ~[],
    row: ~[],
    reader: reader,
    state: Continue
  }
}

impl<R: Reader> Parser<R> {
  fn parse_next_char(&mut self) -> State {
    if self.buffer.len() == 0 {
      if self.reader.eof() {
        return EOF
      }

      let mut bytes = [0, .. 1024];
      let optnbread = self.reader.read(bytes);
      if bytes.len() == 0 && ! self.reader.eof() {
        return Wait
      }

      for el in str::from_utf8(bytes).slice(0, optnbread.unwrap()).chars() {
        self.buffer.push(el);
      }
    }

    let optc = self.buffer.shift_opt();
    match optc {
      None    => return Wait,
      Some(c) => return self.parse_char(c)
    }
  }

  fn parse_char(&mut self, c: char) -> State {
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

  fn extract_last_row(&mut self) -> Row {
    self.row.push(str::from_chars(self.acc));
    self.acc.clear();
    self.extract_row()
  }

  pub fn delim(&mut self, delim:char) {
    self.delim = delim;
  }

  fn state(&self) -> State {
    self.state
  }
}

impl<R: Reader> Iterator<Row> for Parser<R> {
  fn next(&mut self) -> Option<Row> {
    match self.state {
      EOF => return None,
      _   => {
        while(true) {
          match self.parse_next_char() {
            EOL => {
              let row = self.extract_row();
              if row.len() > 0 {
                self.state = Continue;
                return Some(row);
              } else {
                self.state = EOF;
                return None;
              }
            }
            EOF => {
              // the row may not be complete
              println!("EOF");
              let row = self.extract_last_row();
              self.state = EOF;
              if row.len() == 0 || (row.len() == 1 && row[0].len() == 0) {
                println!("none row");
                return None;
              } else {
                println!("row > 0");
                return Some(row);
              }
            }
            Continue => (),
            Wait => return None
          }
        }
      }
    }
    None
  }
}
