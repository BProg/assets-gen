extern crate clap;
extern crate genimg;
use self::clap::{App, Arg};
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use genimg::make_typescript_enum;

fn main() -> io::Result<()> {
    let lapp = define_app();
    let matches = lapp.get_matches();
    let input_path = matches.value_of("input")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid argument for input"))?;
    let output_path = matches.value_of("output")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid argument for output"))?;

    let ts_enum = make_typescript_enum(&input_path).map_err(|e| {
        println!("input argument is wrong");
        e
    })?;
    let mut ts_enum_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_path)
        .map_err(|e| {
            println!("output argument is wrong");
            e
        })?;
    ts_enum_file.write_all(ts_enum.as_bytes()).map_err(|e| {
        println!("cannot write to file {:?}", ts_enum_file);
        e
    })
}

fn define_app<'a>() -> App<'a, 'a> {
    let app = App::new(String::from("genimg"))
        .version("0.2.0")
        .author("Ion Ostafi <ostafi_ion@yahoo.com>")
        .about("Tool for generating code to access local static assets")
        .arg(
            Arg::with_name("input")
                .help("Path to folder containing assets")
                .short("-i")
                .long("--input")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("File output to write generated code")
                .short("-o")
                .long("--output")
                .takes_value(true)
                .required(true),
        );
    app
}
