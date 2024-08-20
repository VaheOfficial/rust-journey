use crate::global::input_acceptor::read_input;
use std::net::Ipv4Addr;

pub fn input_helper(mut input:String) -> String{
    while input.ends_with('\n') || input.ends_with('\r'){
        input.pop();
    }
    return input.to_string();
}

pub fn prompt_user(prompt: &str) -> String {
    println!("{}", prompt);
    let input = read_input();
    return input_helper(input)
}

pub fn prompt_ip(prompt: &str) -> String {
    loop {
        let ip = prompt_user(prompt);
        if ip.parse::<Ipv4Addr>().is_ok() {
            return ip;
        } else {
            println!("Invalid IP address. Please enter a valid IPv4 address.");
        }
    }
}

pub fn prompt_port(prompt: &str) -> String {
    loop {
        let port = prompt_user(prompt);
        if let Ok(p) = port.parse::<u32>(){            
            if p > 0 && p < 65536 {
                return port;
            }
        }
        println!("Invalid port number. Please enter a valid port (1-65535)");
    }
}
