#![allow(unused_variables)]
#![allow(dead_code)]
#[allow(unused_imports)]

use std::env;
use colored::*;
use clap::Parser;

/// MiniGrep Command Line Tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// expression to search for
    #[arg(short, long)]
    expression: String,

    /// lines that match the expression
    #[arg(short='i', long)]
    case_insensitive: bool,

    /// show lines that do not match the expression
    #[arg(short='v', long)]
    invert_match: bool,

    /// search for recursively in directory
    #[arg(short='r', long)]
    recursive: bool,

    /// file/directory to search
    #[arg(short, long)]
    file: String,
}


fn get_file(filename: &str) -> String {
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

fn pluck(expression: &str, file: &str) {
    for (i, value) in file.lines().enumerate() {
        if value.contains(expression) {
            let val: Vec<&str> = value.split(expression).collect();
            println!("{}:{}", (i+1).to_string().green(),val.join(&expression.bright_red().to_string()))
        }
    }
}

fn pluck_inverse(expression: &str, file: &str) {
    for (i, value) in file.lines().enumerate() {
        if !value.contains(expression) {
            println!("{}:{}", (i+1).to_string().green(),value)
        }
    }
}

fn main() {
    let args = Args::parse();

    let expression = if args.case_insensitive {
        &args.expression.to_lowercase()
    } else {
        &args.expression
    };

    if args.recursive {
        println!("Recursive search is not implemented yet.");
    } else {
    
        let file = get_file(&args.file);

        if args.invert_match {
            pluck_inverse(expression, &file);
        } else {
            pluck(expression, &file);
        }
    }
}
