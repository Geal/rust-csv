# Parsing CSV in Rust

This parser is a rewrite, different from the original [rust-csv](https://github.com/brson/rust-csv), because the previous project used an old syntax. For a failed attempt to update it to current Rust, see [rust-csv-legacy](https://github.com/Geal/rust-csv-legacy)
```Rust
    use csv;
    use std::io::mem::Memreader;

    let s = ~"a;b;c;d\r\ne;f;g;h";
    let mut parser = csv.init(~MemReader::new(s.into_bytes()));

    //default delimitor is ','
    parser.delim(';');

    for row in parser {
      println!("first element is: {}", row[0]);
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
