extern crate clap;
use self::clap::{Arg, App};
use std::io;

pub struct GenAssetsApp {
    app_name: String,
    on_input_output_parsed: fn(String, String) -> io::Result<()>,
}

impl GenAssetsApp {
    pub fn new() -> GenAssetsApp {
        GenAssetsApp {
             app_name: String::from("genimg"),
             on_input_output_parsed: |_, _| { Ok(()) },
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let app = App::new(self.app_name.clone())
            .version("0.2.0")
            .author("Ion Ostafi <ostafi_ion@yahoo.com>")
            .about("Tool for generating code to access local static assets")
            .arg(
                Arg::with_name("input")
                .help("Path to folder containing assets")
                .short("-i")
                .long("--input")
                .takes_value(true)
                .required(true)
            )
            .arg(
                Arg::with_name("output")
                .help("File output to write generated code")
                .short("-o")
                .long("--output")
                .takes_value(true)
                .required(true)
            );
        let matches = app.get_matches();
        let input_path = matches.value_of("input")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid argument for input"))?;
        let output_path = matches.value_of("output")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid argument for output"))?;
        (self.on_input_output_parsed)(String::from(input_path), String::from(output_path))
    }

    pub fn set_on_input_output_parsed(&mut self, callback: fn(String, String) -> io::Result<()>) {
        self.on_input_output_parsed = callback;
    }
}
