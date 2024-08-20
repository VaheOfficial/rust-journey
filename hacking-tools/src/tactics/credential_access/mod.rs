pub mod kerberos_enum;

pub fn run_credential_access() {
    loop {
        println!("Choose a credential access technique:");
        println!("1. Kerberos User Enumeration");
        println!("2. Back to main menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => kerberos_enum::kerberos_enum(),
            "2" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}
