use std::process::Command;

fn main() {
  let mut command =Command::new("shutdown.exe");
  command.arg("-r")
  .arg("-t")
  .arg("0");

  let _output = command
  .output()
  .expect("failed to ecec");
}
