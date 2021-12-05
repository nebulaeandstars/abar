use std::process::Command;

/// A very quick and dirty wrapper function to make running shell scripts a bit
/// easier.
/// TODO: Make this better at handling errors
pub fn run(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        // r"foo" is a raw string (no escape sequences)
        .arg(command)
        .output()
        .expect("Error in shell command!")
        .stdout;

    let output = String::from_utf8(output).unwrap().trim().to_string();
    output
}
