#![feature(test)]

use clap::Parser;

// BEGIN_MOD_LIST
mod day00;
//END_MOD_LIST

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  day: usize,
  #[arg(short, long)]
  input: String,
}

impl read::HasFile for Args {
  fn file(&self) -> String { self.input.clone() }
}

fn main() {
    println!("Hello, world!");
}
