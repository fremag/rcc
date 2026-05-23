use std::env;
use std::process;
use std::process::{Command};
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let mut output_file = PathBuf::from(input_file);
    output_file.set_extension("i");
    let x = output_file.to_str().unwrap();

    println!("{input_file} => {x}");

    let c = Command::new("/usr/bin/gcc")
        .args(["-E", "-P", input_file, "-o", x])
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&c.stdout);
    println!("{output}");
    println!("Done.");
    process::exit(0);
}
