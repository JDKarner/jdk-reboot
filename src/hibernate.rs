use std::process::Command;

fn main() {
  let mut command =Command::new("sudo");
   command.arg("shutdown.exe")
          .arg("-h");

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
