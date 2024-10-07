use std::fs;
use rlua::Lua;

pub fn execute(path: &str) {
  let script: String = fs::read_to_string(path).expect(" [Nautilus/Panic] Failed to read the Lua script file");
  let engine: Lua = Lua::new();

  engine.load(&script).exec().expect(" [Nautilus/Panic] Error running Script!");
}