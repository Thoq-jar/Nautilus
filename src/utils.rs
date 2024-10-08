use std::process::exit;

pub fn args_error() {
    eprintln!("[Nautilus/Usage] nautilus [debug] <script.lua> or nautilus version / help");
    exit(1);
}