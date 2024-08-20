pub mod nfs_escalation;

pub fn run_privilege_escalation() {
    loop {
        println!("Choose a privilege escalation technique:");
        println!("1. NFS Share Enumeration");
        println!("2. NFS Privilege Escalation (sid-shell)");
        println!("3. Back to main menu");

        let choice = crate::global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => nfs_escalation::show_nfs_shares(),
            "2" => nfs_escalation::sid_shell_escalation(),
            "3" => break,
            _ => println!("Invalid choice, please select again."),
        }
    }
}
