use std::process::Command;

fn main() {
    println!("Building client scripts");
    Command::new("sh")
        .args(["-c", "bunx tailwindcss -i ./src/client/common.css -o ./out/common.css && bun build ./src/client/common.js"])
        .output()
        .expect("failed to execute process");
}