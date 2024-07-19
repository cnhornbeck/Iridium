// use std::env;
// use std::fs;
// use std::path::PathBuf;

// fn main() {
//     // Get the path to the output directory where the binary will be placed
//     let out_dir = env::var("OUT_DIR").unwrap();
//     let out_dir = PathBuf::from(out_dir);
//     let target_dir = out_dir.ancestors().nth(3).unwrap(); // Adjust this as necessary

//     // Copy the config file to the target directory
//     fs::copy("data.json", target_dir.join("data.json")).unwrap();
// }
