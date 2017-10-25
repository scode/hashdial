#[macro_use]
extern crate clap;
extern crate hashdial;

use std::error::Error;
use std::fmt;

use clap::App;
use clap::Arg;
use clap::SubCommand;

#[derive(Debug)]
struct UserError {
    msg: String,
}

impl<'a> From<&'a str> for UserError {
    fn from(s: &'a str) -> Self {
        UserError { msg: s.to_owned() }
    }
}

impl Error for UserError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let matches = App::new("hashdial")
        .long_about("Implements some useful hashdial mechanisms.")
        .arg(
            Arg::with_name("partition")
                .short("p")
                .long("partition")
                .value_name("N")
                .required(true)
                .help("The partition for which to accept lines."),
        )
        .arg(
            Arg::with_name("num-partitions")
                .short("n")
                .long("num-partitions")
                .value_name("N")
                .required(true)
                .help("Total number of partitions"),
        )
        .subcommand(SubCommand::with_name("accept").about(
            "filter lines accepting only those in the given partition",
        ))
        .get_matches();

    let cfg = hashdial::common::Config {
        num_partitions: value_t!(matches, "partition", usize).unwrap(),
        partition: value_t!(matches, "num-partitions", usize).unwrap(),
    };

    if let Some(_) = matches.subcommand_matches("accept") {
        hashdial::filter::filter(cfg)
    } else {
        Err(Box::new(
            UserError::from("sub-command required; please see --help"),
        ))
    }
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}
