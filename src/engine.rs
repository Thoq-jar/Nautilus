use std::fs;
use rlua::Lua;
use crate::version::show;

pub fn execute(path: &str, debug: bool) {
  let script: String = fs::read_to_string(path).expect(" [Nautilus/Panic] Failed to read the Lua script file");
  let engine: Lua = Lua::new();

  if debug == true {
    println!(" [Nautilus/Debug] Debug mode: true");
    println!(" [Nautilus/Debug] Final stage finished!\n");
    println!("-------- Program Output -------------------------------");
    engine.load(&script).exec().expect(" [Nautilus/Deubg.Panic] Panic occurred while executing!\n [Nautilus/Info] Shutting down...");
    println!("\n-------- Logs -------------------------------");
    println!(" [Nautilus/Info] Unloading modules...");
    println!(" [Nautilus/Info] Goodbye!");
  } else {
    engine.load(&script).exec().expect(" [Nautilus/Panic] Error running Script!");
  }
}

pub fn debug(path: &str) {
  println!("-------- Logs -------------------------------");
  println!(" [Nautilus/Loader] Starting...");
  show(false);
  println!("\n [Nautilus/Engine] Finished with state: ready");
  println!(" [Nautilus/Engine] Set to execute {}...", path);
  execute(path, true)
}