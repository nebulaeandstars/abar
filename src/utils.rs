use std::process::Command;

/// A very quick and dirty wrapper function to make running shell scripts a bit
/// easier. Could (should) be improved upon.
pub fn run(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Error in shell command!")
        .stdout;

    String::from_utf8(output).unwrap().trim().to_string()
}
