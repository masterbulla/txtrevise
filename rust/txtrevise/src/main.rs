/* 
    Txtrevise (rust)
    Command line text editing tool
    Version 1.0.0 (equivalent to Python impl. 1.1)
    Copyright (c) 2015 Sam Saint-Pettersen
    Released under the MIT License.
*/

extern crate clioptions;
extern crate regex;
use clioptions::CliOptions;
use regex::Regex;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::process::exit;

// Display error and then usage information.
fn display_error(program: &str, err: &str) {
    println!("Error: {}.", err);
    display_usage(&program, -1);
}

/// Display usage information.
fn display_usage(program: &str, code: i32) {
    println!("\nTxtrevise v 1.0.0 (rust)");
    println!("Command line text editing tool");
    println!("Copyright (c) 2015 Sam Saint-Pettersen");
    println!("\nReleased under the MIT License");
    println!("\nUsage: {} [-h|--help] (-q|--quiet) -f|--file <file>", program);
    println!("-l|--line <line #> -m|--match <word(s)> -r|--repl <word(s)>");
    println!("\n-f|--file: File to edit");
    println!("-l|--line: Line number to edit text on (starts at 1)");
    println!("-m|--match: Word(s) to match");
    println!("-r|--repl: Replacement word(s) for matched word(s)");
    println!("-q|--quiet: Quiet mode. Only output to console for errors");
    println!("-h|--help: This help information");
    exit(code);
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

    // Revise the selected line and append newline.
    all_lines[index] = match_replace(&all_lines[index], line_no, matches, repl, verbose);
    all_lines.push(String::new());

    // Write out changed lines to file.
    let mut w = File::create(filename).unwrap();
    let _ = w.write_all(all_lines.join("\n").as_bytes());
}

/// Match and replace word(s).
fn match_replace(line: &str, line_no: usize, matches: &str, repl: &str, verbose: bool) -> String {
    let new_line: String;
    let re = Regex::new(matches).unwrap();

    // If word(s) are matched, return edited line with replacement word(s).
    if re.is_match(line) {
        if verbose {
            println!("\nMatched at Line {}: {}", line_no, line);
        }
        new_line = re.replace_all(line, repl);
        if verbose {
            println!("Replaced with: {}", new_line);
        }
    }
    // Otherwise, return same line as before.
    else {
        if verbose {
            println!("\nNo matches at Line {}", line_no);
        }
        new_line = line.to_string();
    }
    new_line
}

fn main() {
    let cli = CliOptions::new("txtrevise");
    let program = cli.get_program();
    let args = cli.get_args();
    let mut filename = String::new();
    let mut matches = String::new();
    let mut repl = String::new();
    let mut verbose: bool = true;
    let mut line_no: usize = 0;
    if cli.get_num() > 1 {
        for (i, a) in args.iter().enumerate() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-f" | "--file" => filename = cli.next_argument(i),
                "-l" | "--line" => {
                    let l = cli.next_argument(i);
                    let ol = l.parse::<usize>().ok();
                    line_no = match ol {
                        Some(line_no) => line_no,
                        None => {
                            display_error(&program, "Line number must be an integer");
                            return;
                        }
                    };
                    if line_no == 0 {
                        display_error(&program, "Line number must be greater than 0");
                        return;
                    }
                },
                "-m" | "--match" => matches = cli.next_argument(i),
                "-r" | "--repl" => repl = cli.next_argument(i),
                "-q" | "--quiet" => verbose = false,
                _ => continue,
            }
        }
    }
    else {
        display_error(&program, "No options specified");
    } 

    if !filename.is_empty() && !matches.is_empty() && !repl.is_empty() {
        process_file(&filename, line_no, &matches, &repl, verbose);
    }
    else {
        display_error(&program, "Incomplete options specified");
    }
}
