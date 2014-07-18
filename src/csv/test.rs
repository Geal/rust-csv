use csv = lib;

use std::io;
use std::io::BufReader;

mod lib;

#[test]
fn init() {
  println!("{}", "pouet".to_string());
  let s = "a,b,c,d".to_string();
  let mut reader = BufReader::new(s.as_bytes());
  let mut p = csv::init(&mut reader);
  let optrow = p.next();
  assert_eq!(optrow.unwrap(), vec!("a".to_string(), "b".to_string(), "c".to_string(),
    "d".to_string()));
  assert!(p.next().is_none());
}

#[test]
fn multiline() {
  let s = "a,b,c,d\r\ne,f,g,h".to_string();
  let mut reader = BufReader::new(s.as_bytes());
  let mut p = csv::init(&mut reader);

  let optrow = p.next();
  assert_eq!(optrow.unwrap(), vec!("a".to_string(), "b".to_string(), "c".to_string(),
    "d".to_string()));
  let secoptrow = p.next();
  assert_eq!(secoptrow.unwrap(), vec!("e".to_string(), "f".to_string(), "g".to_string(),
    "h".to_string()));
  assert!(p.next().is_none());
}

#[test]
fn delim() {
  let s = "aA ;b;c;d; e\r\nf;g;h; I;j\r\n".to_string();
  let mut reader = BufReader::new(s.as_bytes());
  let mut p = csv::init(&mut reader);
  p.delim(';');

  let optrow = p.next();
  assert_eq!(optrow.unwrap(), vec!("aA ".to_string(), "b".to_string(), "c".to_string(),
    "d".to_string(), " e".to_string()));
  let secoptrow = p.next();
  assert_eq!(secoptrow.unwrap(), vec!("f".to_string(), "g".to_string(), "h".to_string(),
    " I".to_string(), "j".to_string()));
  assert!(p.next().is_none());
}
