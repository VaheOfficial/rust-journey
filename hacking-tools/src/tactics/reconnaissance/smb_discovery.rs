use std::process::Command;

pub fn run_smb_discovery() {
    loop {
        println!("Choose an SMB discovery technique:");
        println!("1. RPC Info");
        println!("2. NBTSCan");
        println!("3. SMBClient (List Shares)");
        println!("4. RPC Client (Null Session)");
        println!("5. Enum4Linux");
        println!("6. Back to main menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => rpcinfo(),
            "2" => nbtscan(),
            "3" => smbclient_list_shares(),
            "4" => rpcclient_null_session(),
            "5" => enum4linux(),
            "6" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}

fn rpcinfo() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");

    let output = Command::new("rpcinfo")
        .arg("-p")
        .arg(target)
        .output()
        .expect("Failed to execute rpcinfo");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn nbtscan() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");

    let output = Command::new("nbtscan")
        .arg(target)
        .output()
        .expect("Failed to execute nbtscan");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn smbclient_list_shares() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");

    let output = Command::new("smbclient")
        .arg(format!("-L //{} -U \"\"", target))
        .output()
        .expect("Failed to execute smbclient");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn rpcclient_null_session() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");

    let output = Command::new("rpcclient")
        .arg("-U")
        .arg("\"\"")
        .arg(target)
        .output()
        .expect("Failed to execute rpcclient");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn enum4linux() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");

    let output = Command::new("enum4linux")
        .arg(target)
        .output()
        .expect("Failed to execute enum4linux");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
