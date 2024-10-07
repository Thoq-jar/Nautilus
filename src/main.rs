mod parser;
mod version;
mod engine;

use rlua::Result;
use std::env::args;
use std::process::exit;
use crate::parser::parse;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("[Nautilus/Usage] nautilus <script.lua> or nautilus version");
        exit(1);
    }

    let command: &String = &args[1];
    parse(command);
    Ok(())
}