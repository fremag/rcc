pub mod lexer;
pub mod parser;
pub mod ast_model;
pub mod asm_constructs;

use std::env;
use std::fs;
use std::process;
use std::process::Command;
use std::path::{Path, PathBuf};
use crate::lexer::Lexer;
use crate::parser::Parser;

fn change_extension(input_file: &str, extension : &str) -> PathBuf {
    let mut output_file = PathBuf::from(input_file);
    output_file.set_extension(extension);
    output_file
}

fn run_preprocessor(gcc_path: &str, input_file: &str, output_file: &Path) -> std::io::Result<process::Output> {
    Command::new(gcc_path)
        .args(["-E", "-P", input_file, "-o", output_file.to_str().unwrap()])
        .output()
}

fn run_codegen(gcc_path: &str, asm_file: &str, output_file: &Path) -> std::io::Result<process::Output> {
    Command::new(gcc_path)
        .args([asm_file, "-o", output_file.to_str().unwrap()])
        .output()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let mut action = &String::from("all");
    let mut input_file = &args[1];
    if &args.len() > &2 {
        action = &args[1];
        input_file = &args[2];
    }
    let output_file = change_extension(input_file, "i");
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

    if action == "--lex" {

        println!("Tokens:");
        for token in &tokens {
            println!("{token}");
        }

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

    let program = program_result.unwrap();
    if action == "--parse" {
        print!("{program:?}");

        // we only want to parse, so let's exit here
        process::exit(0);
    }

    let program_asm = program.to_asm();

    if action == "--codegen" {
        print!("{program_asm:?}");

        // we only want to generate code, so let's exit here
        process::exit(0);
    }

    let asm_code = program_asm.to_code();
    println!("{asm_code}");
    
    let asm_file = change_extension(input_file, "s");
    fs::write(asm_file, asm_code).expect("Should have been able to write the file");
    let exe_file = change_extension(input_file, "");
    let command = run_codegen("/usr/bin/gcc", input_file, &exe_file)
        .expect("failed to execute process");
    let output = String::from_utf8_lossy(&command.stdout);
    println!("{output}");

    println!("Done.");
    process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_output_file_replaces_c_extension() {
        let out = change_extension("foo.c", "i");
        assert_eq!(out, PathBuf::from("foo.i"));
    }

    #[test]
    fn compute_output_file_replaces_other_extension() {
        let out = change_extension("path/to/source.cpp","i");
        assert_eq!(out, PathBuf::from("path/to/source.i"));
    }

    #[test]
    fn compute_output_file_adds_extension_when_missing() {
        let out = change_extension("Makefile", "i");
        assert_eq!(out, PathBuf::from("Makefile.i"));
    }

    #[test]
    fn compute_output_file_preserves_directory() {
        let out = change_extension("/tmp/dir/program.c", "i");
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
