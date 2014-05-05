use csv = lib;

use std::io::BufReader;

mod lib;

#[test]
fn init() {
  let s = "a,b,c,d";
  let mut reader = ~BufReader::new(s.as_bytes());
  let mut p = csv::init(reader);
  let optrow = p.next();
  assert_eq!(optrow.unwrap(), owned_string_vec(&["a", "b", "c", "d"]));
  assert!(p.next().is_none());
}

#[test]
fn multiline() {
  let s = "a,b,c,d\r\ne,f,g,h";
  let mut reader = ~BufReader::new(s.as_bytes());
  let mut p = csv::init(reader);

  let optrow = p.next();
  assert_eq!(optrow.unwrap(), owned_string_vec(&["a", "b", "c", "d"]));
  let secoptrow = p.next();
  assert_eq!(secoptrow.unwrap(), owned_string_vec(&["e", "f", "g", "h"]));
  assert!(p.next().is_none());
}

#[test]
fn delim() {
  let s = "aA ;b;c;d; e\r\nf;g;h; I;j\r\n";
  let mut reader = ~BufReader::new(s.as_bytes());
  let mut p = csv::init(reader);
  p.delim(';');

  let optrow = p.next();
  assert_eq!(optrow.unwrap(), owned_string_vec(&["aA ", "b", "c", "d", " e"]));
  let secoptrow = p.next();
  assert_eq!(secoptrow.unwrap(), owned_string_vec(&["f", "g", "h", " I", "j"]));
  assert!(p.next().is_none());
}

fn owned_string_vec(slice: &[&str]) -> Vec<~str> {
  slice.iter().map(|string| string.to_owned()).collect::<Vec<~str>>()
}
