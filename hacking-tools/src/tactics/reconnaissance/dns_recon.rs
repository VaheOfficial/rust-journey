use std::process::Command;

pub fn run_dns_recon() {
    loop {
        println!("Choose a DNS reconnaissance technique:");
        println!("1. Whois Lookup");
        println!("2. Dig Query (A, TXT, NS, MX)");
        println!("3. Dig with specific DNS Server");
        println!("4. Host Lookup");
        println!("5. DNSRecon AXFR");
        println!("6. Dnsenum");
        println!("7. Nslookup (interactive)");
        println!("8. Subdomain Enumeration");
        println!("9. BBOT Subdomain Enumeration");
        println!("10. Back to previous menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => whois_lookup(),
            "2" => dig_query(),
            "3" => dig_with_ns(),
            "4" => host_lookup(),
            "5" => dnsrecon_axfr(),
            "6" => dnsenum_lookup(),
            "7" => nslookup_query(),
            "8" => subdomain_enum(),
            "9" => bbot_subdomain_enum(),
            "10" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}

fn bbot_subdomain_enum() {
    let domain = crate::global::input_helper::prompt_user("Enter the domain for BBOT subdomain enumeration:");

    let output = Command::new("bbot")
        .arg("-t")
        .arg(domain)
        .arg("-f")
        .arg("subdomain-enum")
        .output()
        .expect("Failed to execute BBOT");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn whois_lookup() {
    let domain = crate::global::input_helper::prompt_user("Enter the domain for WHOIS lookup:");
    let output = Command::new("whois")
        .arg(domain)
        .output()
        .expect("Failed to execute whois");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn dig_query() {
    let domain = crate::global::input_helper::prompt_user("Enter the domain for dig query:");
    let query_type = crate::global::input_helper::prompt_user("Enter the query type (A, TXT, NS, MX):");
    let output = Command::new("dig")
        .arg(format!("-t{}", query_type))
        .arg(domain)
        .output()
        .expect("Failed to execute dig");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn dig_with_ns() {
    let domain = crate::global::input_helper::prompt_user("Enter the domain for dig query:");
    let query_type = crate::global::input_helper::prompt_user("Enter the query type (A, TXT, NS, MX):");
    let nameserver = crate::global::input_helper::prompt_user("Enter the nameserver (e.g., ns1.domain.com):");
    let output = Command::new("dig")
        .arg(format!("-t{}", query_type))
        .arg(domain)
        .arg(format!("@{}", nameserver))
        .output()
        .expect("Failed to execute dig with NS");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn host_lookup() {
    let domain = crate::global::input_helper::prompt_user("Enter the domain for host lookup:");
    let query_type = crate::global::input_helper::prompt_user("Enter the query type (A, TXT, NS, MX) or 'all' for -a:");
    
    let output = if query_type == "all" {
        Command::new("host")
            .arg("-a")
            .arg(domain)
            .output()
            .expect("Failed to execute host")
    } else {
        Command::new("host")
            .arg(format!("-t {}", query_type))
            .arg(domain)
            .output()
            .expect("Failed to execute host")
    };

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn dnsrecon_axfr() {
    let domain = crate::global::input_helper::prompt_user("Enter the domain for DNSRecon:");
    let nameserver = crate::global::input_helper::prompt_user("Enter the nameserver for AXFR (e.g., ns2.domain.com):");

    let output = Command::new("dnsrecon")
        .arg("-d")
        .arg(domain)
        .arg("-t")
        .arg("axfr")
        .arg(format!("@{}", nameserver))
        .output()
        .expect("Failed to execute DNSRecon");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn dnsenum_lookup() {
    let domain = crate::global::input_helper::prompt_user("Enter the domain for Dnsenum:");
    let output = Command::new("dnsenum")
        .arg(domain)
        .output()
        .expect("Failed to execute dnsenum");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn nslookup_query() {
    let domain = crate::global::input_helper::prompt_user("Enter the domain for nslookup:");
    
    let output = Command::new("nslookup")
        .arg(domain)
        .output()
        .expect("Failed to execute nslookup");

    println!("Interactive nslookup session (you will need to enter further commands manually):");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn subdomain_enum() {
    let subdomains_file = crate::global::input_helper::prompt_user("Enter the path to subdomains.txt:");
    let domain = crate::global::input_helper::prompt_user("Enter the domain to append to subdomains:");

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "for sub in $(cat {}); do host $sub.{} | grep 'has address'; done",
            subdomains_file, domain
        ))
        .output()
        .expect("Failed to execute subdomain enumeration");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
