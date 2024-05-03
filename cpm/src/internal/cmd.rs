use std::process::Command;

pub fn execute() {
    let output = Command::new("cmd")
                        .args(&["/C", "echo Hello, world!"])
                        .output()
                        .expect("Failed to execute command");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", result);
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed: {}", err);
    }
}
