use csv = lib;

use std::io;
use io::Reader;
use io::mem::MemReader;

mod lib;

#[test]
fn init() {
  let s = ~"a;b;c;d";
  let reader = ~MemReader::new(s.into_bytes());
  let mut p = csv::init(reader);

  for row in p {
    assert_eq!(row, ~[~"a", ~"b", ~"c", ~"d"]);
  }
}
