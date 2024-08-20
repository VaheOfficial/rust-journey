mod global;
mod tactics;

fn main() {
    loop {
        println!("Choose a tactic:");
        println!("1. Reconnaissance");
        println!("2. Execution");
        println!("3. Exit");

        let choice = global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => {
                tactics::reconnaissance::run_recon();
            }
            "2" => {
                tactics::execution::run_execution();
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please select again."),
        }
    }
}
