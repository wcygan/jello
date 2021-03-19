/*


 */

mod parsing;
mod counting;

extern crate clap;
use clap::{Arg, App, SubCommand};
use crate::parsing::WordCountModifiers;
use std::path::Path;
use std::process::exit;

fn main() {

    /* Fetch matches */
    let matches = App::new("jello")
        .version("1.0")
        .about("Counts words in files!")
        .author("Will C.")
        .arg(Arg::with_name("files")
            .short("f")
            .long("files")
            .required(true)
            .min_values(1)
            .help("Specify the files you would like to analyze"))
        .arg(Arg::with_name("lines")
            .short("l")
            .long("lines")
            .required(false)
            .help("Prints out the number of lines in files"))
        .arg(Arg::with_name("words")
            .short("w")
            .long("words")
            .required(false)
            .help("Prints out the number of words in files"))
        .arg(Arg::with_name("chars")
            .short("m")
            .long("chars")
            .required(false)
            .help("Prints out the number of chars in files"))
        .arg(Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .required(false)
            .help("Prints out the number of bytes in files"))
        .get_matches();


    /* Fetch modifiers and filepaths */
    let modifiers = WordCountModifiers::new(&matches);
    let filepaths: Vec<String> = matches.values_of("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    /* Verify file paths */
    for path in filepaths {
        if !Path::new(path.as_str()).exists() {
            eprintln!("File '{}' doesn't exist!", path);
            exit(1);
        }
    }
}