use std::process::Command;

pub fn run_snmp_discovery() {
    loop {
        println!("Choose an SNMP discovery technique:");
        println!("1. SNMP User Accounts");
        println!("2. SNMP Running Programs");
        println!("3. SNMP Hostname");
        println!("4. SNMP Share Information");
        println!("5. SNMP TCP Ports");
        println!("6. SNMP Software Name");
        println!("7. Brute-force Community Strings");
        println!("8. SNMP Check");
        println!("9. Back to main menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => snmp_user_accounts(),
            "2" => snmp_running_programs(),
            "3" => snmp_hostname(),
            "4" => snmp_share_info(),
            "5" => snmp_tcp_ports(),
            "6" => snmp_software_name(),
            "7" => onesixtyone_bruteforce(),
            "8" => snmp_check(),
            "9" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}

fn snmp_user_accounts() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");
    let output = Command::new("snmpwalk")
        .arg("-c")
        .arg("public")
        .arg("-v1")
        .arg(target)
        .arg("1.3.6.1.4.1.77.1.2.25")
        .output()
        .expect("Failed to execute snmpwalk");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn snmp_running_programs() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");
    let output = Command::new("snmpwalk")
        .arg("-c")
        .arg("public")
        .arg("-v1")
        .arg(target)
        .arg("1.3.6.1.2.1.25.4.2.1.2")
        .output()
        .expect("Failed to execute snmpwalk");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn snmp_hostname() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");
    let output = Command::new("snmpwalk")
        .arg("-c")
        .arg("public")
        .arg("-v1")
        .arg(target)
        .arg(".1.3.6.1.2.1.1.5")
        .output()
        .expect("Failed to execute snmpwalk");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn snmp_share_info() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");
    let output = Command::new("snmpwalk")
        .arg("-c")
        .arg("public")
        .arg("-v1")
        .arg(target)
        .arg("1.3.6.1.4.1.77.1.2.3.1.1")
        .output()
        .expect("Failed to execute snmpwalk");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn snmp_tcp_ports() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");
    let output = Command::new("snmpwalk")
        .arg("-c")
        .arg("public")
        .arg("-v1")
        .arg(target)
        .arg("1.3.6.1.2.1.6.13.1.3")
        .output()
        .expect("Failed to execute snmpwalk");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn snmp_software_name() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");
    let output = Command::new("snmpwalk")
        .arg("-c")
        .arg("public")
        .arg("-v1")
        .arg(target)
        .arg("1.3.6.1.2.1.25.6.3.1.2")
        .output()
        .expect("Failed to execute snmpwalk");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn onesixtyone_bruteforce() {
    let ips_file = crate::global::input_helper::prompt_user("Enter the path to snmp-ips.txt:");
    let community_file = crate::global::input_helper::prompt_user("Enter the path to community.txt:");

    let output = Command::new("onesixtyone")
        .arg("-i")
        .arg(ips_file)
        .arg("-c")
        .arg(community_file)
        .output()
        .expect("Failed to execute onesixtyone");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn snmp_check() {
    let target = crate::global::input_helper::prompt_user("Enter the target IP:");
    let output = Command::new("snmp-check")
        .arg(target)
        .output()
        .expect("Failed to execute snmp-check");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
