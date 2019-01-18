extern crate clap;
use self::clap::{Arg, App, ArgMatches};
use std::io;

pub struct RootView<'a> {
    arguments: ArgMatches<'a>,
    on_input_output_parsed: fn(String, String) -> io::Result<()>,
}

impl<'a> RootView<'a> {
    pub fn new() -> RootView<'a> {
        RootView {
             arguments: App::new("genimg")
                 .version("0.2.0")
                 .author("Ion Ostafi <ostafi_ion@yahoo.com>")
                 .about("Generate enum for assets")
                 .arg(
                     Arg::with_name("input")
                         .help("Assets folder")
                         .short("-i")
                         .long("--input")
                         .takes_value(true)
                         .required(true)
                 )
                 .arg(
                     Arg::with_name("output")
                         .help("File (enum) output")
                         .long_help("If file does not exist, it will be created.\nIf file already exists, it will be overridden")
                         .short("-o")
                         .long("--output")
                         .takes_value(true)
                         .required(true)
                 ).get_matches(),
             on_input_output_parsed: |_, _| { Ok(()) },
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let args = &self.arguments;
        let input_path = args.value_of("input")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid argument for input"))?;
        let output_path = args.value_of("output")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid argument for output"))?;
        (self.on_input_output_parsed)(String::from(input_path), String::from(output_path))
    }

    pub fn set_on_input_output_parsed(&mut self, callback: fn(String, String) -> io::Result<()>) {
        self.on_input_output_parsed = callback;
    }
}
