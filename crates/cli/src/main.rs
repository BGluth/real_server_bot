use clap::Parser;

use crate::prog_args::ProgArgs;

mod prog_args;

fn main() {
    let p_args = ProgArgs::parse();
}
