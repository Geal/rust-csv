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
  EOL
}

pub struct Parser<'a, R> {
  priv count: uint,
  priv readlen: uint,
  priv delim: char,
  priv buffer: ~[char],
  priv acc: ~[char],
  priv row: Row,
  priv reader: &'a mut R,
  priv state: State
}


pub fn init<'a, R: io::Reader>(reader: &'a mut R) -> ~Parser<'a, R> {

  ~Parser {
    count: 0,
    readlen: 1024,
    delim: ',',
    buffer: ~[],
    acc: ~[],
    row: ~[],
    reader: reader,
    state: Continue
  }
}

impl<'a, R: Reader> Parser<'a, R> {
  fn parse_next_char(&mut self) -> State {
    if self.buffer.len() == 0 {

      let mut bytes = [0, .. 1024];
      let optnbread = self.reader.read(bytes);
      if bytes.len() == 0 {
        println!("0 bytes read");
        return Wait
      }

      match optnbread {
        Err(e)     => { println!("opntbread error: {}", e); return Wait},
        Ok(nb)     => {
          println!("optnbread {} bytes", nb);
          let s  = str::from_utf8(bytes);
          if s.is_some() {
            for el in s.unwrap().slice(0, nb).chars() {
              self.buffer.push(el);
            }
          }
        }
      }
    }

    let optc = self.buffer.shift();
    match optc {
      None    => {println!("optc is none");return Wait},
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
        if self.buffer.len() == 0 {
          self.row.push(str::from_chars(self.acc));
          self.acc.clear();
        }
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

impl<'a, R: Reader> Iterator<Row> for Parser<'a, R> {
  fn next(&mut self) -> Option<Row> {
    match self.state {
      _   => {
        while true {
          match self.parse_next_char() {
            EOL => {
              let row = self.extract_row();
              if row.len() > 0 {
                self.state = Continue;
                return Some(row);
              } else {
                self.state = EOL;
                return None;
              }
            }
            Continue => (),
            Wait => {
              self.state = Wait;
              let row = self.extract_row();
              if row.len() > 0 {
                return Some(row);
              }  else {
                return None
              }
            }
          }
        }
      }
    }
    None
  }
}

