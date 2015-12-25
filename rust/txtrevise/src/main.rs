// Txtrevise
// Command line text editing tool
// Version 1.0.0 (equivalent to Python impl. 1.1)
// Copyright (c) 2015 Sam Saint-Pettersen
// Released under the MIT License.

extern crate regex;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use regex::Regex;

// Display error and then usage information.
fn display_error(err: &str) {
    println!("Error: {}.", err);
    display_usage();
}

/// Display usage information.
fn display_usage() {
    println!("\nTxtrevise v 1.0.0 (rust)");
    println!("Command line text editing tool");
    println!("Copyright (c) 2015 Sam Saint-Pettersen");
    println!("\nReleased under the MIT License");
    println!("Usage: txtrevise.py [-h] (-q) -f <file> -l <line #> -m <word(s)>");
    println!("-r <word(s)>");
    println!("\n-f: File to edit");
    println!("-l: Line number to edit text on (starts at 1)");
    println!("-m: Word(s) to match");
    println!("-r: Replacement word(s) for matched word(s)");
    println!("-q: Quiet mode. Only output to console for errors");
    println!("-h: This help information");
}

/// Process file.
fn process_file(filename: &str, line_no: usize, matches: &str, repl: &str, verbose: bool) {
    let mut line_num: usize = 0;
    let mut index: usize = 0;
    let mut all_lines: Vec<String> = Vec::new();

    // Read each line in file sequentially, store selected line no.
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        all_lines.push(line.unwrap());
        if line_num == line_no - 1 { // - 1, because lines start at 0.
            index = line_num;
        }
        line_num += 1;
    }    
    all_lines[index] = match_replace(&all_lines[index], line_no, matches, repl, verbose);
    all_lines.push(String::new());

    let mut w = File::create(filename).unwrap();
    let _ = w.write_all(all_lines.join("\n").as_bytes());
}

/// Match and replace word(s).
fn match_replace(line: &str, line_no: usize, matches: &str, repl: &str, verbose: bool) -> String {
    let new_line: String;
    let re = Regex::new(matches).unwrap();

    // If word(s) are matched, return edited line with replacment word(s).
    if re.is_match(line) {
        if verbose {
            println!("\nMatched at Line {}: {}", line_no, line);
        }
        new_line = re.replace_all(line, repl);
        if verbose {
            println!("Replaced with: {}", new_line);
        }
    }
    // Otherwise, return same line as before
    else {
        if verbose {
            println!("\nNo matches at Line {}", line_no);
        }
        new_line = line.to_string();
    }
    new_line
}

/// Entry method.
fn main() {
    let args: Vec<_> = env::args().collect();
    let mut i: usize = 0;
    let mut filename = String::new();
    let mut matches = String::new();
    let mut repl = String::new();
    let mut verbose: bool = true;
    let mut line_no: usize = 1;
    if args.len() > 1 {
        for a in args {
            if a == "-h" {
                display_usage();
            }
            if a == "-f" {
                filename = format!("{:?}", env::args().nth(i + 1));
                filename = filename[6..filename.len() - 2].to_string();
            }
            if a == "-l" {
                let mut l = format!("{:?}", env::args().nth(i + 1));
                l = l[6..l.len() - 2].to_string();
                let ol = l.parse::<usize>().ok();
                line_no = match ol {
                    Some(line_no) => line_no,
                    None => {
                        display_error("Line number must be an integer");
                        return;
                    }
                };
                if line_no == 0 {
                    display_error("Line number must be greater than 0");
                    return;
                }
            }
            if a == "-m" {
                matches = format!("{:?}", env::args().nth(i + 1));
                matches = matches[6..matches.len() - 2].to_string();
            }
            if a == "-r" {
                repl = format!("{:?}", env::args().nth(i + 1));
                repl = repl[6..repl.len() - 2].to_string();
            }
            if a == "-q" {
                verbose = false;
            }
            i = i + 1;
        }
    }
    else {
        display_error("No options specified");
    } 

    // With necessary arguments, process file.
    if env::args().len() > 2 && filename.len() > 0 {
        process_file(&filename, line_no, &matches, &repl, verbose);
    }
}
