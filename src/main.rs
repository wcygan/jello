/*


*/

mod counting;
mod parsing;

extern crate clap;
use crate::counting::ParsedResult;
use crate::parsing::WordCountModifiers;
use clap::{App, Arg};
use rayon::prelude::*;
use std::io::{Read};
use std::path::Path;
use std::process::exit;

static FILES: &str = "files";

fn main() {
    /* Fetch matches */
    let matches = App::new("jello")
        .version("1.0")
        .about("Counts words in files!")
        .author("Will C.")
        .arg(
            Arg::with_name("files")
                .short("f")
                .long("files")
                .required(false)
                .min_values(0)
                .help("Specify the files you would like to analyze"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .required(false)
                .help("Prints out the number of lines in files"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .required(false)
                .help("Prints out the number of words in files"),
        )
        .arg(
            Arg::with_name("chars")
                .short("c")
                .long("chars")
                .required(false)
                .help("Prints out the number of chars in files"),
        )
        .get_matches();

    /* Fetch modifiers and filepaths */
    let modifiers = WordCountModifiers::new(&matches);

    /* Parse the contents of the files or stdin */
    let results: Vec<ParsedResult> = match matches.occurrences_of(FILES) > 0 {
        /* Read from files */
        true => {
            let mut filepaths: Vec<&str> = matches.values_of(FILES).unwrap().collect();

            /* Verify file paths */
            for path in &filepaths {
                if !Path::new(path).exists() {
                    eprintln!("File '{}' doesn't exist!", path);
                    exit(1);
                }
            }

            /* Launch threads to parallelize result parsing */
            filepaths
                .par_iter_mut()
                .map(|path| ParsedResult::parse_file(path))
                .collect()
        }
        /* Attempt to read from stdin */
        false => {
            let mut text = String::new();
            std::io::stdin()
                .read_to_string(&mut text)
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                });

            let result = ParsedResult::parse_lines(text, None);

            vec![result]
        }
    };

    print_results(results, modifiers)
}

fn print_results(results: Vec<ParsedResult>, modifiers: WordCountModifiers) {
    match results.len() > 1 {
        true => {
            let totals = ParsedResult::totals(&results);
            results.iter().for_each(|r| r.print(&modifiers));
            totals.print(&modifiers)
        }
        false => results.iter().for_each(|r| r.print(&modifiers)),
    }
}
