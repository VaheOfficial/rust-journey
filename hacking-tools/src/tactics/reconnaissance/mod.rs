pub mod arp_scanner;
pub mod nmap_scanner;
pub mod regular_scan;
pub mod evasion_scan;

pub fn run_recon() {
    loop {
        println!("Choose a reconnaissance technique:");
        println!("1. ARP Scan");
        println!("2. Nmap Scan");
        println!("3. Back to main menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => {
                arp_scanner::scan();
            }
            "2" => {
                nmap_scanner::scan();
            }
            "3" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}
