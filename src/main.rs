pub mod lexer;
pub mod parser;
pub mod ast_modele;

use std::env;
use std::fs;
use std::process;
use std::process::Command;
use std::path::{Path, PathBuf};
use crate::lexer::Lexer;
use crate::parser::Parser;

fn compute_output_file(input_file: &str) -> PathBuf {
    let mut output_file = PathBuf::from(input_file);
    output_file.set_extension("i");
    output_file
}

fn run_preprocessor(gcc_path: &str, input_file: &str, output_file: &Path) -> std::io::Result<process::Output> {
    Command::new(gcc_path)
        .args(["-E", "-P", input_file, "-o", output_file.to_str().unwrap()])
        .output()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let action = &args[1];
    let input_file = &args[2];

    let output_file = compute_output_file(input_file);
    let output_file_str = output_file.to_str().unwrap();

    println!("{input_file} => {output_file_str}");

    let command = run_preprocessor("/usr/bin/gcc", input_file, &output_file)
        .expect("failed to execute process");
    let output = String::from_utf8_lossy(&command.stdout);
    println!("{output}");

    let contents = fs::read_to_string(&output_file)
         .expect(&format!("Should have been able to read the file {}", output_file_str));
    let lexer  = Lexer::new(contents);
    let mut tokens = match lexer.tokenize()  {
        Err(err_msg) => {
            print!("{err_msg}");
            process::exit(1);
        }
        Ok(tokens) => tokens
    };

    println!("Tokens:");
    for token in &tokens {
        println!("{token}");
    }

    if action == "--lex" {
        // we only want to lex so let's exit here
        process::exit(0);
    }

    let parser = Parser::new();

    let program_result = parser.parse_program(&mut tokens);
    if let Err(_) = program_result  {
        process::exit(1);
    }

    if tokens.len() != 0 {
        process::exit(1);
    }

    if action == "--parse" {
        // we only want to lex so let's exit here
        process::exit(0);
    }

    let program = program_result.unwrap();
    print!("{program:?}");
    println!("Done.");
    process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_output_file_replaces_c_extension() {
        let out = compute_output_file("foo.c");
        assert_eq!(out, PathBuf::from("foo.i"));
    }

    #[test]
    fn compute_output_file_replaces_other_extension() {
        let out = compute_output_file("path/to/source.cpp");
        assert_eq!(out, PathBuf::from("path/to/source.i"));
    }

    #[test]
    fn compute_output_file_adds_extension_when_missing() {
        let out = compute_output_file("Makefile");
        assert_eq!(out, PathBuf::from("Makefile.i"));
    }

    #[test]
    fn compute_output_file_preserves_directory() {
        let out = compute_output_file("/tmp/dir/program.c");
        assert_eq!(out, PathBuf::from("/tmp/dir/program.i"));
    }

    #[test]
    fn run_preprocessor_returns_error_for_missing_binary() {
        let result = run_preprocessor(
            "/nonexistent/path/to/gcc",
            "input.c",
            Path::new("output.i"),
        );
        assert!(result.is_err());
    }
}
