pub mod web_discovery;

pub fn run_discovery() {
    loop {
        println!("Choose a discovery technique:");
        println!("1. Gobuster");
        println!("2. Nikto");
        println!("3. WPScan");
        println!("4. Back to main menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => web_discovery::run_gobuster(),
            "2" => web_discovery::run_nikto(),
            "3" => web_discovery::run_wpscan(),
            "4" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}
