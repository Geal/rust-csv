#[link(name = "csv", vers = "0.1", author = "geal")];
#[crate_type = "lib"];
#[desc = "CSV parser"];
#[license = "MIT"];

use std::io;
use std::iter::Iterator;

pub struct Parser {
  priv count: uint
}

type Row = ~[~str];

pub fn init<R: io::Reader>(reader: ~R) -> ~Parser {

  ~Parser {
    count: 5
  }
}

impl Iterator<Row> for Parser {
  fn next(&mut self) -> Option<Row> {
    if self.count ==  0 {
      return None
    }

    self.count -= 1;
    return Some(~[~"a", ~"b", ~"c", ~"d"]);
  }
}
