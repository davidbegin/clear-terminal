use std::process::Command;

fn main() {
  println!("I can clear the screen!");

  let output = Command::new("clear").output().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
  });

  println!("{}", String::from_utf8_lossy(&output.stdout));

  println!("...see");
}
