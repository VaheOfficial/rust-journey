mod global;
mod tactics;

fn main() {
    loop {
        println!("Choose a tactic:");
        println!("1. Reconnaissance");
        println!("2. Discovery");
        println!("3. Credential Access");
        println!("4. Privilege Escalation");
        println!("5. Execution");
        println!("6. Exit");

        let choice = global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => tactics::reconnaissance::run_recon(),
            "2" => tactics::discovery::run_discovery(),
            "3" => tactics::credential_access::run_credential_access(),
            "4" => tactics::privilege_escalation::run_privilege_escalation(),
            "5" => tactics::execution::run_execution(),
            "6" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please select again."),
        }
    }
}
