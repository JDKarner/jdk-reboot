use std::process::Command;

fn main() {
  let mut command =Command::new("sudo");
   command.arg("shutdown.exe")
          .arg("-r")
          .arg("-fw")
          .arg("-t")
          .arg("0");

  let output = command
  .output()
  .expect("failed to ecec");

  if output.status.success() {
    println!("the command worked");
  }
  else {
    println!("shit broke")
  }

  
}
