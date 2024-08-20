use std::process::{Command, Stdio};
use std::fs::File;

pub fn run_gobuster() {
    let target = crate::global::input_helper::prompt_user("Enter the target (IP or domain):");

    // Prepare the command to write to a file instead of using a pipe (| tee)
    let output_file = format!("{}-gobuster", target);
    let file = File::create(&output_file).expect("Unable to create output file");

    let mut command = Command::new("gobuster")
        .arg("dir")
        .arg("-u")
        .arg(format!("https://{}", target))  // Correct formatting for the target
        .arg("-r")
        .arg("-w")
        .arg("/usr/share/wordlists/dirbuster/directory-list-2.3-medium.txt")
        .arg("-x")
        .arg("php,txt")
        .arg("-t")
        .arg("150")
        .stdout(Stdio::from(file))  // Redirect output to the file
        .spawn()
        .expect("Failed to execute Gobuster");

    // Wait for the command to complete
    let status = command.wait().expect("Failed to wait for gobuster process");

    if status.success() {
        println!("Gobuster completed successfully. Output saved to {}", output_file);
    } else {
        println!("Gobuster failed with status: {:?}", status);
    }
}

pub fn run_nikto() {
    let target = crate::global::input_helper::prompt_user("Enter the target (IP or domain):");

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("nikto -h https://{0} | tee {0}-nikto", target))
        .output()
        .expect("Failed to execute Nikto");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn run_wpscan() {
    let target = crate::global::input_helper::prompt_user("Enter the target (IP or domain):");

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("wpscan --url https://{0} --enumerate u,t,p | tee {0}-wpscan-enum", target))
        .output()
        .expect("Failed to execute WPScan");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
