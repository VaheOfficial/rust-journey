use std::process::Command;

pub fn msfconsole_creator(
    os: String,
    ip: String,
    port: String,
    shell: String,
    direction: String,
    stage: String,
    method: String,
    format: String,
    verbose: String    
    ){

    let verbosity;

    if verbose.to_lowercase() == "y"{
        verbosity = "verbose".to_string();
    } else {
        verbosity = "".to_string();
    }

    let vars = format!("{} {} {} {} {} {} {} {} {}", os, ip, port, shell, direction, stage, method, format, verbosity);
    println!("Command Output: {}", vars );
    let output = Command::new("msfpc")
        .arg(os)
        .arg(ip)
        .arg(port)
        .arg(shell)
        .arg(direction)
        .arg(stage)
        .arg(method)
        .arg(format)
        .arg(verbosity)
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
