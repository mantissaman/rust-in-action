use std::{fs::File, io::{BufReader, BufRead}};
use std::io;

// GREP-LITE
use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("cool light weight grep")
        .arg(
            Arg::with_name("pattern")
            .help("Pattern to search for")
            .takes_value(true)
            .required(true)
        )
        .arg(
            Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(false)
        )
        .get_matches();
    
    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap_or("-");

    let re=Regex::new(pattern).unwrap();
    
    if input =="-" {
        let test_input = io::stdin();
        let reader = test_input.lock();
        process_lines(reader, re);

    } else{
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
fn process_lines<T: BufRead + Sized>(reader: T, re:Regex) {
    for (i,liner) in reader.lines().enumerate() {
        let line = liner.unwrap();
        //let does_contain = re.find(&line);
        match re.find(&line)  {
            Some(_) => println!("{}: {}", i+1, line),
            None => ()
        }
    }
}

