use csv = lib;

use std::io;
use io::Reader;
use io::mem::MemReader;

mod lib;

#[test]
fn init() {
  let s = ~"a,b,c,d";
  let reader = ~MemReader::new(s.into_bytes());
  let mut p = csv::init(reader);

  let optrow = p.next();
  assert_eq!(optrow.unwrap(), ~[~"a", ~"b", ~"c", ~"d"]);
  assert!(p.next().is_none());
}


#[test]
fn multiline() {
  let s = ~"a,b,c,d\r\ne,f,g,h";
  let reader = ~MemReader::new(s.into_bytes());
  let mut p = csv::init(reader);

  let optrow = p.next();
  assert_eq!(optrow.unwrap(), ~[~"a", ~"b", ~"c", ~"d"]);
  let secoptrow = p.next();
  assert_eq!(secoptrow.unwrap(), ~[~"e", ~"f", ~"g", ~"h"]);
  assert!(p.next().is_none());
}

