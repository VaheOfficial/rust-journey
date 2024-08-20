// use std::fs::{self, File};
// use std::io::{Write, Error};
// use std::path::Path;

// pub fn write_output(path: &str, is_file: bool, content: Option<&[u8]>) -> Result<(), Error> {
//     if is_file {
//         // Write to a file, if content is provided
//         if let Some(content) = content {
//             let mut file = File::create(path)?;
//             file.write_all(content)?;
//             println!("File written to: {}", path);
//         } else {
//             println!("Error: No content provided for the file.");
//             return Err(Error::new(std::io::ErrorKind::InvalidInput, "No content for file"));
//         }
//     } else {
//         // Create a directory
//         if !Path::new(path).exists() {
//             fs::create_dir_all(path)?;
//             println!("Directory created at: {}", path);
//         } else {
//             println!("Directory already exists at: {}", path);
//         }
//     }
//     Ok(())
// }
