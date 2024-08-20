pub mod malware_creator;
pub mod msfconsole;

pub fn run_execution() {
    loop {
        println!("Choose an execution technique:");
        println!("1. Malware Creator (MSFVenom)");
        println!("2. Back to main menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => {
                malware_creator::create_malware();
            }
            "2" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}
