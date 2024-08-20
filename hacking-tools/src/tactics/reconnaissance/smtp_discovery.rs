use std::process::Command;

pub fn run_smtp_discovery() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");
    let wordlist = "/usr/share/wordlists/names.txt";  // You can make this configurable if needed

    let output = Command::new("smtp-user-enum")
        .arg("-U")
        .arg(wordlist)
        .arg("-t")
        .arg(target)
        .arg("-m")
        .arg("150")
        .output()
        .expect("Failed to execute smtp-user-enum");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
