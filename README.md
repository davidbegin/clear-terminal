Clearing Terminal in Rust
---

I could not find info on this anywhere.
But finally I looked up how to run a bash command in rust,
found this http://stackoverflow.com/questions/21011330/in-rust-how-do-i-invoke-a-system-command-and-capture-its-output
and I just replace 'ls' with 'clear'.

but now I'm hoping to figure a little more about what is happening in this code,
and try some experiments.


```rust
use std::process::Command;

fn main() {
  println!("I can clear the screen!");

  let output = Command::new("clear").output().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
  });

  println!("{}", String::from_utf8_lossy(&output.stdout));

  println!("...see");
}
```
