extern crate clap;

use clap::{Arg, App, SubCommand};
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::{Error, ErrorKind as IOErrorKind};
use std::io::{BufRead, BufReader};
use std::io;

fn open_file(file: &str) -> io::Result<File> {
    if Path::new(&file).exists() {
        let f = File::open(file).expect("[iris Error] File not found.");
        Ok(f)
    } else {
        Result::Err(Error::new(IOErrorKind::NotFound, "[iris Error] File not exist."))
    }
}

fn import_scan_data(file: &str) -> Vec<Vec<f64>> {
    let mut f = BufReader::new(open_file(file).unwrap());
    let mut s = String::new();

    // Skip first line.
    f.read_line(&mut s).unwrap();

    let array: Vec<Vec<f64>> = f.lines()
        .map(|l| l.unwrap().split(char::is_whitespace)
            .map(|number| number.parse().unwrap())
            .collect())
        .collect();

    println!("{:?}", array);

    array
}

fn main() {
    let app = App::new("iris")
        .version("0.1.0")
        .author("Derek W. birdinforest@gmail.com")
        .about("CLI.")
        .subcommand(
            // Convert a file to target format.
            SubCommand::with_name("convert")
                .about("Convert file format.")
                .arg(Arg::from_usage("-f, --format=[FORMAT] 'Format convert to.'"))
                .arg(Arg::from_usage("-i, --input=[FILE] 'Provides an input file to convert.'"))
                .arg(Arg::from_usage("-o, --output=[FILE] 'Provides an output file path.'"))
        )
        .subcommand(
            // Import a file
            // So far it just import a txt file contains points cloud scan data and
            // store data into a collection of Vec<Vec<f64>>.
            SubCommand::with_name("import")
                .about("Import file.")
                .arg(Arg::from_usage("-i, --input=[FILE] 'Provides an input file to import.'"))
        )
        .get_matches();

    match app.subcommand() {
        ("convert", Some(sub)) => {
            println!("Subcommand convert.");
            if let Some(format) = sub.value_of("format") {
                println!("Convert to format: {}", format);
            }
            if let Some(input) = sub.value_of("input") {
                println!("Input path: {}", input);
                match open_file(input) {
                    Ok(file) => {
                        // Read and print lines
                        let bufreader: BufReader<File> = BufReader::new(file);
                        for line in bufreader.lines().map(|x| x.unwrap()) {
                            println!("{}", line);
                        }
                    },
                    Err(e) => {
                        println!("{}{}", e, " Exist!");
                        process::exit(1);
                    },
                };
            }
            if let Some(output) = sub.value_of("output") {
                println!("Output path: {}", output);
            }
        }

        ("import", Some(sub)) => {
            println!("Subcommand import.");
            if let Some(input) = sub.value_of("input") {
                println!("Input path: {}", input);
                import_scan_data(input);
            }
        }
        _ => {}
    }
}
