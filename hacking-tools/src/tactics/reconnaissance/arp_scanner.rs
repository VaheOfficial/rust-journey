use crate::global::input_helper::{prompt_user};
use std::process::Command;

pub fn scan(){
    let interface = prompt_user("What interface you would like to use?");
    let output = Command::new("sudo")
        .arg("arp-scan")
        .arg(format!("--interface={}", interface))
        .arg("--localnet")
        .output()
        .expect("Failed to execute command");
   
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
        // Check if the command was successful
    if output.status.success() {
        // Print stdout (successful output)
        println!("Command Output:\n{}", stdout);
    } else {
        // Print stderr (error output)
        eprintln!("Command Failed:\n{}", stderr);
    }
}
