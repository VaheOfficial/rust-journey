pub mod arp_scanner;
pub mod nmap_scanner;
pub mod evasion_scan;
pub mod regular_scan;
pub mod dns_recon;
pub mod smb_discovery;
pub mod snmp_discovery;
pub mod smtp_discovery;

pub fn run_recon() {
    loop {
        println!("Choose a reconnaissance technique:");
        println!("1. ARP Scan");
        println!("2. Nmap Scan");
        println!("3. DNS Reconnaissance");  // New DNS Recon option
        println!("4. SMB Discovery"); 
        println!("5. SNMP Discovery");
        println!("6. SMTP Discovery");
        println!("7. Back to main menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => arp_scanner::scan(),
            "2" => nmap_scanner::scan(),
            "3" => dns_recon::run_dns_recon(),
            "4" => smb_discovery::run_smb_discovery(),
            "5" => snmp_discovery::run_snmp_discovery(),
            "6" => smtp_discovery::run_smtp_discovery(),
            "7" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}
