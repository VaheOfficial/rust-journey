use youtube_dl::YoutubeDl;
use crate::global::input_acceptor;
use crate::global::io_helper;

pub fn yt_downloader() {
    println!("Please provide a video link:");
    let link = input_acceptor::read_input().trim().to_string();
    
    println!("Where would you like the video saved to?");
    let read_input = &input_acceptor::read_input();
    let path = read_input.trim();
    
    // Ensure the directory exists
    io_helper::write_output(path, false, None).expect("Failed to create directory");

    println!("What is the file's name (including extension, e.g., video.mp4)?");
    let read_input = &input_acceptor::read_input();
    let name = read_input.trim();
    
    let file_path = format!("{}/{}", path, name);
    
    // Ensure correct path formatting
    println!("Saving video to: {}", file_path);

    // Use the `--output` argument explicitly
    match YoutubeDl::new(link.clone())
        .extra_arg("--output")
        .extra_arg(&file_path)
        .run() {
            Ok(_) => println!("Video successfully saved to {}", file_path),
            Err(e) => eprintln!("Failed to download video from {}: {}", link, e),
    };
}
