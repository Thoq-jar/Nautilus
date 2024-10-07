use std::process::exit;
use crate::engine::execute;
use crate::version::show;

pub fn parse(command: &str) {
  match command {
    cmd if cmd.ends_with(".lua") => execute(command),
    "version" => show(),
    _ => args_error(),
  }
}

fn args_error() {
  eprintln!("[Nautilus/Usage] nautilus <script.lua> or nautilus version");
  exit(1);
}