mod cli;
mod parser;

use std::{
  fs::File,
  io::{Read, Write},
};

use anyhow::Result;
use clap::Parser;

use cli::Cli;
use parser::parse;

fn main() {
  let args = Cli::parse();

  if let Err(error) = run(args) {
    eprintln!("Compile Error!\n{error:?}");
  }
}

fn run(args: Cli) -> Result<()> {
  let input = {
    let mut file = File::open(args.input)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    buf
  };

  let ast = parse(&input)?;

  {
    let mut output_file = File::create(args.output)?;
    write!(output_file, "{ast:#?}")?;
  }

  Ok(())
}
