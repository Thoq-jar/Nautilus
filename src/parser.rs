use crate::engine::execute;
use crate::engine::debug;
use crate::version::show;
use crate::utils::args_error;

pub fn parse(args: Vec<String>) {
    if args.len() == 2 {
        let command = &args[1];
        match command.as_str() {
            cmd if cmd.ends_with(".lua") => {
                execute(command, false);
            },
            "version" | "help" => show(true),
            _ => args_error(),
        }
    } else if args.len() == 3 && args[1] == "debug" {
        let script = &args[2];
        if script.ends_with(".lua") {
            debug(script);
        } else {
            args_error();
        }
    } else {
        args_error();
    }
}
