# Parsing CSV in Rust

    use csv;
    use std::io::mem::Memreader;

    let s = ~"a;b;c;d\r\ne;f;g;h";
    let mut parser = csv.init(~MemReader::new(s.into_bytes()));

    //default delimitor is ','
    parser.delim(';');

    for row in parser {
      println!("first element is: {}", row[0]);
    }

## Installation

    rustpkg install github.com/Geal/rust-csv

## Unit testing

Clone the project, and in its repository:

    $ rustpkg build csv && rustpkg test csv

