#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::fs;
use std::io;
use colored::*;
use clap::Parser;

/// MiniGrep Command Line Tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// expression to search for
    #[arg(short='e', long)]
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
    #[arg(short='f', long)]
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

fn display_matches(file: &str, args: &Args, expression: &str) {
    let mut file = get_file(file);
    if args.case_insensitive {
        file = file.to_lowercase();
    }

    if args.invert_match {
        pluck_inverse(expression, &file);
    } else {
        pluck(expression, &file);
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
        let dir_path = &args.file;
        for entry in fs::read_dir(dir_path).unwrap() {
            let path = entry.unwrap().path();
            println!("in: {}", path.display());
            if path.is_file() {
                display_matches(&path.display().to_string(), &args, expression);
            }
        }   
    } else {
        display_matches(&args.file, &args, expression);
    }
}
