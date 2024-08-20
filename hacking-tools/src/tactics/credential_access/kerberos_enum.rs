use std::process::Command;

pub fn kerberos_enum() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP (e.g., 10.0.0.1):");
    let realm = crate::global::input_helper::prompt_user("Enter the realm (e.g., test):");

    let output = Command::new("nmap")
        .arg(target)
        .arg("-p")
        .arg("88")
        .arg("--script")
        .arg("krb5-enum-users")
        .arg("--script-args")
        .arg(format!("krb5-enum-users.realm='{}'", realm))
        .output()
        .expect("Failed to execute Kerberos Enumeration");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
