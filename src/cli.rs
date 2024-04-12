use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  /// The MIDL source file
  pub input: PathBuf,

  /// Place the output into <OUTPUT>
  #[arg(short, default_value = "SyntaxOut.txt")]
  pub output: PathBuf,
}
