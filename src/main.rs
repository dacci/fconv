mod variant;

use clap::{load_yaml, App};
use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;
use std::str::FromStr;
use variant::Variant;

#[derive(Debug)]
struct CliError(String);

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        f.write_str(&self.0)
    }
}

impl Error for CliError {}

macro_rules! cli_error {
    ($($arg:tt)*) => {
        Box::new(CliError(format!($($arg)*)))
    };
}

enum Format {
    Json,
    Yaml,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            _ => Err(format!("Illegal format: {}", name)),
        }
    }
}

fn main() {
    let r = main_impl();
    if r.is_err() {
        eprintln!("Error: {}", r.unwrap_err());
        exit(1)
    }
}

fn main_impl() -> Result<(), Box<dyn Error>> {
    let cli_def = load_yaml!("cli.yaml");
    let matches = App::from_yaml(cli_def)
        .name(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    if matches.is_present("formats") {
        print!(
            "Supported formats:
  json    JavaScript Object Notation
  yaml    YAML Ain't Markup Language
"
        );
        return Ok(());
    }

    let input_format = match matches.value_of("from") {
        Some(format) => format.parse()?,
        None => return Err(cli_error!("Input format is not specified")),
    };

    let output_format = match matches.value_of("to") {
        Some(format) => format.parse()?,
        None => return Err(cli_error!("Output format is not specified")),
    };

    let value = match matches.value_of("input") {
        Some("-") | None => from_reader(input_format, stdin())?,
        Some(path) => from_reader(input_format, File::open(path)?)?,
    };

    match matches.value_of("output") {
        Some("-") | None => to_writer(output_format, stdout(), &value)?,
        Some(path) => to_writer(output_format, File::create(path)?, &value)?,
    };

    Ok(())
}

fn from_reader<R>(format: Format, reader: R) -> Result<Variant, Box<dyn Error>>
where
    R: Read,
{
    let value = match format {
        Format::Json => serde_json::from_reader(reader)?,
        Format::Yaml => serde_yaml::from_reader(reader)?,
    };

    Ok(value)
}

fn to_writer<W>(format: Format, writer: W, value: &Variant) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    match format {
        Format::Json => serde_json::to_writer_pretty(writer, value)?,
        Format::Yaml => serde_yaml::to_writer(writer, value)?,
    };

    Ok(())
}
