mod variant;

use clap::Parser;
use std::fmt;
use std::fs::File;
use std::io;
use variant::Variant;

#[derive(Debug)]
enum CliError {
    Usage(String),
    Io(String),
    SerDe(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CliError::Usage(s) => s,
            CliError::Io(s) => s,
            CliError::SerDe(s) => s,
        };
        f.write_str(s)
    }
}

impl std::error::Error for CliError {}

type Result<T, E = CliError> = std::result::Result<T, E>;

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(format!("{e}"))
    }
}

impl From<serde_json::Error> for CliError {
    fn from(e: serde_json::Error) -> Self {
        CliError::SerDe(format!("{e}"))
    }
}

impl From<serde_pickle::Error> for CliError {
    fn from(e: serde_pickle::Error) -> Self {
        CliError::SerDe(format!("{e}"))
    }
}

impl From<plist::Error> for CliError {
    fn from(e: plist::Error) -> Self {
        CliError::SerDe(format!("{e}"))
    }
}

impl From<toml::de::Error> for CliError {
    fn from(e: toml::de::Error) -> Self {
        CliError::SerDe(format!("{e}"))
    }
}

impl From<toml::ser::Error> for CliError {
    fn from(e: toml::ser::Error) -> Self {
        CliError::SerDe(format!("{e}"))
    }
}

impl From<serde_yaml::Error> for CliError {
    fn from(e: serde_yaml::Error) -> Self {
        CliError::SerDe(format!("{e}"))
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

impl std::str::FromStr for Format {
    type Err = CliError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "json" => Ok(Format::Json),
            "pickle" => Ok(Format::Pickle),
            "plist" => Ok(Format::Plist),
            "plistb" => Ok(Format::PlistB),
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => Err(CliError::Usage(format!("illegal format: {name}"))),
        }
    }
}

#[derive(clap::Parser)]
#[clap(about, version)]
struct Args {
    /// Prints the supported formats
    #[clap(long, exclusive = true)]
    formats: bool,

    /// Specifies the format of the input file.
    #[clap(
        short,
        long,
        required_unless_present = "formats",
        value_name = "FORMAT"
    )]
    from_format: Option<Format>,

    /// Specifies the format of the output file.
    #[clap(
        short,
        long,
        required_unless_present = "formats",
        value_name = "FORMAT"
    )]
    to_format: Option<Format>,

    /// Specifies the path to the output file (default: standard input)
    #[clap(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Specifies the path to the input file.
    #[clap(value_name = "FILE")]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = main_impl(args) {
        let (msg, code) = match e {
            CliError::Usage(msg) => (msg, 1),
            CliError::Io(msg) => (msg, 2),
            CliError::SerDe(msg) => (msg, 3),
        };
        eprintln!("Error: {msg}");
        std::process::exit(code);
    }
}

fn main_impl(args: Args) -> Result<()> {
    if args.formats {
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

    let from_format = args.from_format.unwrap();
    let value = match args.input.as_deref() {
        Some("-") | None => from_reader(from_format, &mut io::stdin())?,
        Some(path) => from_reader(from_format, &mut File::open(path)?)?,
    };

    let to_format = args.to_format.unwrap();
    match args.output.as_deref() {
        Some("-") | None => to_writer(to_format, &mut io::stdout(), &value)?,
        Some(path) => to_writer(to_format, &mut File::create(path)?, &value)?,
    };

    Ok(())
}

fn from_reader<R>(format: Format, reader: &mut R) -> Result<Variant>
where
    R: io::Read,
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
    W: io::Write,
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
