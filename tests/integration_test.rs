use std::borrow::Cow;
use std::process::{Command, Output};
use std::fs::{write, remove_file};

#[test]
fn test_run_lua_script() {
  let msg: &str = ".... .. .... Test Passed .... .. ....";
  let test: String = format!(r#"print("{msg}")"#);
  let path: &str = "test_script.lua";

  write(path, &test).expect("Failed to write test script");

  let output: Output = Command::new("cargo")
    .arg("run")
    .arg(path)
    .output()
    .expect("[Nautilus/Test] Failed to execute (test) command");

  let stdout: Cow<str> = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains(msg));

  remove_file(path).expect("[Nautilus/Test] Failed to remove test script");
}