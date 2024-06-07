mod analyze;
mod cli;
mod codegen;
mod parser;

use std::{
  fs::File,
  io::{Read, Write},
};

use anyhow::{bail, Result};
use clap::Parser;

use analyze::analyze;
use cli::Cli;
use codegen::codegen;
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

  if let Err(error) = analyze(&ast) {
    bail!(error);
  }

  let result = codegen(&ast);

  {
    let mut output_file = File::create(args.output)?;
    writeln!(output_file, "{result}")?;
  }

  Ok(())
}
