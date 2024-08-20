use crate::global::input_helper::{prompt_user, prompt_ip};
use std::process::Command;

static mut EVASION_MISTAKES: i32 = 0; // Keep track of evasion mistakes

pub fn evasion_scanner(message: String) {
    println!("{}", message);

    let mut command_args = vec![];
    let mut fragment_selected = false;

    // Fragment packets
    let fragment_packets = prompt_user("Would you like to fragment packets (-f)? (Y/n)");
    if fragment_packets.to_lowercase() == "y" {
        fragment_selected = true;
        command_args.push("-f".to_string());
        println!("Fragmenting packets like we're playing hide and seek... 😏");
    }

    // MTU
    let mtu_size = prompt_user("Want to adjust the MTU size (--mtu)? (Y/n)");
    if mtu_size.to_lowercase() == "y" {
        if fragment_selected {
            println!("Warning: You can't use both fragmenting (-f) and adjusting MTU (--mtu) at the same time. Ignoring MTU.");
        } else {
            let mtu_value = prompt_user("Enter the MTU size (suggested 8, 16, 32):");
            command_args.push(format!("--mtu{}", mtu_value)); // No space here
            println!("Tweaking MTU size. A bit of packet gymnastics. 🤸‍♂️");
        }
    }

    // Decoy scan
    let decoy_scan = prompt_user("Want to use decoys to confuse the target (-D RND:10)? (Y/n)");
    if decoy_scan.to_lowercase() == "y" {
        command_args.push("-D".to_string());
        command_args.push("RND:10".to_string()); // Separate -D and its value
        println!("Launching decoys like it's Mission Impossible... 🕵️‍♂️");
    }

    // Scan delay
    let scan_delay = prompt_user("Want to introduce a delay between packets (--scan-delay)? (Y/n)");
    if scan_delay.to_lowercase() == "y" {
        let delay_value = prompt_user("Enter delay (e.g., 100ms):");
        command_args.push(format!("--scan-delay {}", delay_value));
        println!("Slow and steady wins the race... Or at least avoids detection. 🐢");
    }

    // SYN Scan (Stealth scan)
    let stealth_scan = prompt_user("Would you like to stealth scan (-sS)? (Y/n)");
    if stealth_scan.to_lowercase() == "y" {
        command_args.push("-sS".to_string());
        println!("Stealth mode: ON. Time to sneak past those defenses. 🕶️");
    }

    // Timing template
    let timing_template = prompt_user("Choose a timing template (1-5). (1 is paranoid, 5 is fast)");
    let timing = match timing_template.parse::<i32>() {
        Ok(t) if t >= 1 && t <= 5 => t,
        _ => {
            unsafe {
                EVASION_MISTAKES += 1;
                handle_evasion_mistake(EVASION_MISTAKES);
            }
            evasion_scanner("Please enter a valid timing template between 1 and 5.".to_string());
            return;
        }
    };
    let timing_arg = format!("-T{}", timing);
    command_args.push(timing_arg);

    // Target IP
    let target_ip = prompt_ip("What’s the IP of the target? Let’s hope it’s not a honeypot...");
    command_args.push(target_ip.to_string());

    // Print the constructed command for debugging
    println!("Running command with evasion: {:?}", command_args);

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

unsafe fn handle_evasion_mistake(mistakes: i32) {
    match mistakes {
        1 => println!("Strike one! Even the IDS is laughing at your input. 😬"),
        2 => println!("You really want to get caught, don’t you? Keep it up. 😂"),
        3 => println!("Third time's a charm... or not. IDS systems have your number now. 🙄"),
        _ => println!("Attempt {}... Ever heard of OPSEC? Clearly not. 😅", mistakes),
    }
}
