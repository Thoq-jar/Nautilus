pub fn show(help: bool) {
  let banner: &str = r#"
 ███╗   ██╗ █████╗ ██╗   ██╗████████╗██╗██╗     ██╗   ██╗███████╗
 ████╗  ██║██╔══██╗██║   ██║╚══██╔══╝██║██║     ██║   ██║██╔════╝
 ██╔██╗ ██║███████║██║   ██║   ██║   ██║██║     ██║   ██║███████╗
 ██║╚██╗██║██╔══██║██║   ██║   ██║   ██║██║     ██║   ██║╚════██║
 ██║ ╚████║██║  ██║╚██████╔╝   ██║   ██║███████╗╚██████╔╝███████║
 ╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝    ╚═╝   ╚═╝╚══════╝ ╚═════╝ ╚══════╝
  "#;
  let version: &str = env!("CARGO_PKG_VERSION");
  let license: &str = env!("CARGO_PKG_LICENSE");

  println!("{}", banner);
  
  println!(" -------- Info -------------------------------");
  println!(" Nautilus v{version}");
  println!(" License: {license}");
 
  if help == true {
    println!("\n-------- Help -------------------------------");
    println!(" Commands: ");
    println!("  version - show this screen");
    println!("  main.lua - replace main.lua with the name of your script");
  }
}