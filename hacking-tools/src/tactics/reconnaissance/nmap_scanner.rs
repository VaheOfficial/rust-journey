use crate::global::input_helper::prompt_user;
use crate::tactics::reconnaissance::regular_scan::regular_scanner;
use crate::tactics::reconnaissance::evasion_scan::evasion_scanner;

pub fn scan(){
    loop{
        println!("What are you looking for?"); 
        let choice = prompt_user("
1. Regular Scan (You will be asked to provide additional values)
2. IDS/IPS Evasion
            ");
        match choice.as_str() {
            "1" => {
                regular_scanner("Welcome to regular NMAP scanner".to_string());
                break;
            }
            "2" => {
                evasion_scanner("Psst this will try to evade defenses".to_string());
                break;
            }
            _ => println!("Invalid choice, please select again."),
            
        } 
    } 
}
