use std::process::Command;

fn main() {

    let mut process = Command::new("scp");

    process.current_dir("[directory]");

    process.arg("[file]");

    process.arg("[user]@[address]:[path]");

    let result = process.output().expect("Could not excute process");

    println!("Output:\n{}",String::from_utf8_lossy(&result.stdout));
}
