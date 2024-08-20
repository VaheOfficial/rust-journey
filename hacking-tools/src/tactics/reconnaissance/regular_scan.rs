use crate::global::input_helper::{prompt_user, prompt_ip};
use std::process::Command;

static mut SCAN_SPEED_MISTAKES: i32 = 0; // Keep track of scan speed mistakes
static mut PORT_CHOICE_MISTAKES: i32 = 0; // Keep track of port choice mistakes

pub fn regular_scanner(message: String) {
    println!("{}", message);

    let mut command_args = vec![];

    // A scan (this includes -O and -sV)
    let a_scan = prompt_user("Would you like to perform an aggressive scan (-A)? (Y/n)");
    if a_scan.to_lowercase() == "y" {
        command_args.push("-A".to_string());
    } else {
        // OS detection
        let os_detection = prompt_user("Would you like to scan the OS? (Y/n)");
        if os_detection.to_lowercase() == "y" {
            command_args.push("-O".to_string());
        }

        // Service detection
        let service_detection = prompt_user("Would you like to scan services? (Y/n)");
        if service_detection.to_lowercase() == "y" {
            command_args.push("-sV".to_string());
        }
    }

    // Stealth scan
    let stealth_scan = prompt_user("Would you like to stealth scan (-sS)? (Y/n)");
    if stealth_scan.to_lowercase() == "y" {
        command_args.push("-sS".to_string());
    }

    // UDP scan
    let udp_scan = prompt_user("Would you like to perform a UDP scan (-sU)? (Y/n)");
    if udp_scan.to_lowercase() == "y" {
        command_args.push("-sU".to_string());
    }

    // Scan speed
    let scan_speed_string = prompt_user("Enter scan speed from 1-5.");
    let scan_speed = match scan_speed_string.parse::<i32>() {
        Ok(speed) if speed >= 1 && speed <= 5 => speed,
        Ok(_) => {
            unsafe {
                SCAN_SPEED_MISTAKES += 1;
                handle_scan_speed_mistake(SCAN_SPEED_MISTAKES);
            }
            regular_scanner("Please enter a proper scan speed between 1 and 5.".to_string());
            return;
        }
        Err(_) => {
            unsafe {
                SCAN_SPEED_MISTAKES += 1;
                handle_scan_speed_mistake(SCAN_SPEED_MISTAKES);
            }
            regular_scanner("DONT PUT LETTERS IN NUMBER SLOTS... seriously 😠".to_string());
            return;
        }
    };
    let scan_speed_arg = format!("-T{}", scan_speed);
    command_args.push(scan_speed_arg);

    // Ports
    let port_choice = prompt_user("
Would you rather scan by port or scan the top ports?
1. Choose port
2. Top ports
    ");

    match port_choice.as_str() {
        "1" => {
            let ports = prompt_user("
Which ports would you like to scan? Enter a range like 1-65535 or 22,80-443.
            ");
            let port_arg = format!("-p {}", ports);
            command_args.push(port_arg);
        }
        "2" => {
            let top_ports = prompt_user("
How many top ports would you like to scan? Just a number, please. 👀
            ");
            command_args.push("--top-ports".to_string()); // Separate argument
            command_args.push(top_ports); // Top ports as a separate argument
        }
        _ => {
            unsafe {
                PORT_CHOICE_MISTAKES += 1;
                handle_port_choice_mistake(PORT_CHOICE_MISTAKES);
            }
            regular_scanner("It's 1 or 2. Not rocket science. Let's try this again. 🤦‍♂️".to_string());
            return;
        }
    }

    let target_ip = prompt_ip("And finally, what is the IP of the target?");
    println!("You’ve survived the setup. Let's see if you mess up now!");

    // Add IP as separate argument
    command_args.push(target_ip);

    // Print the constructed command for debugging
    println!("Command Args: {:?}", command_args);

    // Run the command
    let output = Command::new("sudo")
        .arg("nmap")
        .args(&command_args)
        .output()
        .expect("Failed to execute nmap command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check if the command was successful
    if output.status.success() {
        println!("Command Output:\n{}", stdout);
    } else {
        eprintln!("Command Failed:\n{}", stderr);
    }
}

unsafe fn handle_scan_speed_mistake(mistakes: i32) {
    match mistakes {
        1 => println!("Strike one! Did you put a letter in? We don't do that here. 🤔"),
        2 => println!("Again? Seriously? I’m starting to question your life choices. 👀"),
        3 => println!("Alright, this is the third time. Should I even bother anymore? 😒"),
        _ => println!("Attempt {}... You seriously still trying? 😩", mistakes),
    }
}

unsafe fn handle_port_choice_mistake(mistakes: i32) {
    match mistakes {
        1 => println!("Port choice fail #1... 1 or 2. That's all you gotta do. 😕"),
        2 => println!("Port choice fail #2... You really trying to test me? 🙄"),
        3 => println!("Three times? Do you even know what you're doing at this point? 🤷"),
        _ => println!("Attempt {}... Still trolling? You're good at it. 😑", mistakes),
    }
}
