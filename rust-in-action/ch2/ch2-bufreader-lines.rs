use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("readme.md").unwrap();
  let mut reader = BufReader::new(f);

  for line_ in reader.lines() {
    let line = line_.unwrap();
    println!("{} ({} bytes long)", line, line.len());
  }
}