# Parsing CSV in Rust

[![Build Status](https://travis-ci.org/Geal/rust-csv.png?branch=master)](https://travis-ci.org/Geal/rust-csv)

This parser is a rewrite, different from the original [rust-csv](https://github.com/brson/rust-csv), because the previous project used an old syntax. For a failed attempt to update it to current Rust, see [rust-csv-legacy](https://github.com/Geal/rust-csv-legacy)
```Rust
    use csv;
    use std::io::MemReader;

    fn main() {
      let s = "a;b;c;d\r\ne;f;g;h";

      let reader = MemReader::new(Vec::from_slice(s.as_bytes()));
      let mut parser = csv::init(reader);

      //default delimitor is ','
      parser.delim(';');

      for row in parser {
        println!("first element is: {}", row.get(0));
      }
    }
```

## Installation

```Shell
    $ rustpkg install github.com/Geal/rust-csv
```

## Unit testing

Clone the project, and in its repository:

```Shell
    $ rustpkg build csv && rustpkg test csv
```
