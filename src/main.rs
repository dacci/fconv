mod variant;

use clap::{load_yaml, App, ArgMatches};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;
use std::str::FromStr;
use variant::Variant;

enum CliError {
    Usage(String),
    Io(String),
    SerDe(String),
}

type StdResult<T, E> = std::result::Result<T, E>;
type Result<T> = StdResult<T, CliError>;

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(format!("{}", e))
    }
}

impl From<serde_json::Error> for CliError {
    fn from(e: serde_json::Error) -> Self {
        CliError::SerDe(format!("{}", e))
    }
}

impl From<serde_pickle::Error> for CliError {
    fn from(e: serde_pickle::Error) -> Self {
        CliError::SerDe(format!("{}", e))
    }
}

impl From<plist::Error> for CliError {
    fn from(e: plist::Error) -> Self {
        CliError::SerDe(format!("{}", e))
    }
}

impl From<toml::de::Error> for CliError {
    fn from(e: toml::de::Error) -> Self {
        CliError::SerDe(format!("{}", e))
    }
}

impl From<toml::ser::Error> for CliError {
    fn from(e: toml::ser::Error) -> Self {
        CliError::SerDe(format!("{}", e))
    }
}

impl From<serde_yaml::Error> for CliError {
    fn from(e: serde_yaml::Error) -> Self {
        CliError::SerDe(format!("{}", e))
    }
}

enum Format {
    Json,
    Pickle,
    Plist,
    PlistB,
    Toml,
    Yaml,
}

impl FromStr for Format {
    type Err = CliError;

    fn from_str(name: &str) -> StdResult<Self, Self::Err> {
        match name {
            "json" => Ok(Format::Json),
            "pickle" => Ok(Format::Pickle),
            "plist" => Ok(Format::Plist),
            "plistb" => Ok(Format::PlistB),
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => Err(CliError::Usage(format!("Illegal format: {}", name))),
        }
    }
}

fn main() {
    let cli_def = load_yaml!("cli.yaml");
    let matches = App::from_yaml(cli_def)
        .name(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    if let Err(e) = main_impl(matches) {
        let (msg, code) = match e {
            CliError::Usage(msg) => (msg, 1),
            CliError::Io(msg) => (msg, 2),
            CliError::SerDe(msg) => (msg, 3),
        };
        eprintln!("Error: {}", msg);
        exit(code);
    }
}

fn main_impl(matches: ArgMatches) -> Result<()> {
    if matches.is_present("formats") {
        print!(
            "Supported formats:
  json    JavaScript Object Notation
  pickle  Python's serialization format
  plist   Property list (XML)
  plistb  Property list (binary)
  toml    Tom's Obvious, Minimal Language
  yaml    YAML Ain't Markup Language
"
        );
        return Ok(());
    }

    let input_format = match matches.value_of("from") {
        Some(format) => format.parse()?,
        None => return Err(CliError::Usage("Input format is not specified".to_owned())),
    };

    let output_format = match matches.value_of("to") {
        Some(format) => format.parse()?,
        None => return Err(CliError::Usage("Output format is not specified".to_owned())),
    };

    let value = match matches.value_of("input") {
        Some("-") | None => from_reader(input_format, &mut stdin())?,
        Some(path) => from_reader(input_format, &mut File::open(path)?)?,
    };

    match matches.value_of("output") {
        Some("-") | None => to_writer(output_format, &mut stdout(), &value)?,
        Some(path) => to_writer(output_format, &mut File::create(path)?, &value)?,
    };

    Ok(())
}

fn from_reader<R>(format: Format, reader: &mut R) -> Result<Variant>
where
    R: Read,
{
    let value = match format {
        Format::Json => serde_json::from_reader(reader)?,
        Format::Pickle => {
            let opts = serde_pickle::DeOptions::new();
            serde_pickle::from_reader(reader, opts)?
        }
        Format::Plist => plist::from_reader_xml(reader)?,
        Format::PlistB => {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes)?;
            plist::from_bytes(&bytes)?
        }
        Format::Toml => {
            let mut s = String::new();
            reader.read_to_string(&mut s)?;
            toml::de::from_str(&s)?
        }
        Format::Yaml => serde_yaml::from_reader(reader)?,
    };

    Ok(value)
}

fn to_writer<W>(format: Format, writer: &mut W, value: &Variant) -> Result<()>
where
    W: Write,
{
    match format {
        Format::Json => serde_json::to_writer_pretty(writer, value)?,
        Format::Pickle => {
            let opts = serde_pickle::SerOptions::new();
            serde_pickle::to_writer(writer, value, opts)?
        }
        Format::Plist => plist::to_writer_xml(writer, value)?,
        Format::PlistB => plist::to_writer_binary(writer, value)?,
        Format::Toml => {
            let s = toml::ser::to_string_pretty(value)?;
            writer.write_all(s.as_bytes())?;
        }
        Format::Yaml => serde_yaml::to_writer(writer, value)?,
    };

    Ok(())
}
