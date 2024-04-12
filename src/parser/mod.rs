mod ast;

use anyhow::{bail, Result};
use pest::Parser;
use pest_derive::Parser;

use ast::MidlAst;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct MidlParser;

pub fn parse(src: &str) -> Result<MidlAst> {
  match MidlParser::parse(Rule::specification, src)?.next() {
    Some(spec) => Ok(MidlAst::try_from(spec)?),
    None => {
      bail!("Failed to parse the source file");
    }
  }
}
