mod parser;
mod version;
mod engine;
mod utils;

use rlua::Result;
use std::env::args;
use crate::parser::parse;
use crate::utils::args_error;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        args_error();
    }

    parse(args);
    Ok(())
}