use std::process::exit;

pub fn parser(args: &String) {
  match args.as_str() {
    "version" => exit(0),
    _ => { args_error(args) }
  }
}

fn args_error(args: &String) {
  if args.len() < 2 {
    eprintln!("Usage: nautilus <script.lua> or nautilus version");
    exit(1);
  }
}