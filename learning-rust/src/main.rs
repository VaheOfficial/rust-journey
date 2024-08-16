mod calculator;  // Import the calculator module
mod global;      // Import the global module
mod youtube_downloader; //Import youtube downloader module

fn main() {
    loop {
        println!("Choose an option:");
        println!("1. Calculator");
        println!("2. Youtube Video Downloader");
        println!("3. Exit");

        let choice = global::input_acceptor::read_input().trim().to_string();

        match choice.as_str() {
            "1" => {
                calculator::start_calculator();  // Call the calculator function
            }
            "2" => {
                youtube_downloader::yt_downloader();
                break;
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please select again."),
        }
    }
}
