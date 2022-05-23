mod variant;

use clap::Parser;
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io;
use std::process::{ExitCode, Termination};
use variant::Variant;

#[derive(Debug)]
enum Error {
    Usage(String),
    Io(io::Error),
    SerDe(Box<dyn StdError + Send + Sync>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usage(msg) => f.write_str(msg),
            Error::Io(cause) => write!(f, "{cause}"),
            Error::SerDe(cause) => write!(f, "{cause}"),
        }
    }
}

impl StdError for Error {}

impl Termination for Error {
    fn report(self) -> ExitCode {
        match self {
            Error::Usage(_) => ExitCode::FAILURE,
            Error::Io(_) => 2.into(),
            Error::SerDe(_) => 3.into(),
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Usage(msg)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

macro_rules! impl_error_serde {
    ($E:ty) => {
        impl From<$E> for Error {
            fn from(e: $E) -> Self {
                Error::SerDe(Box::new(e))
            }
        }
    };
}

impl_error_serde!(plist::Error);
impl_error_serde!(serde_json::Error);
impl_error_serde!(serde_pickle::Error);
impl_error_serde!(serde_yaml::Error);
impl_error_serde!(toml::de::Error);
impl_error_serde!(toml::ser::Error);

enum Format {
    Json,
    Pickle,
    Plist,
    PlistB,
    Toml,
    Yaml,
}

impl std::str::FromStr for Format {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "json" => Ok(Format::Json),
            "pickle" => Ok(Format::Pickle),
            "plist" => Ok(Format::Plist),
            "plistb" => Ok(Format::PlistB),
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => Err(format!("Illegal format: {name}").into()),
        }
    }
}

#[derive(clap::Parser)]
#[clap(about, version)]
struct Args {
    /// Prints the supported formats.
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

    /// Specifies the path to the output file (default: standard output).
    #[clap(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Specifies the path to the input file.
    #[clap(value_name = "FILE")]
    input: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if let Err(e) = main_impl(args) {
        eprintln!("Error: {e}");
        e.report()
    } else {
        ExitCode::SUCCESS
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
        Some("-") | None => from_reader(from_format, io::stdin())?,
        Some(path) => from_reader(from_format, File::open(path)?)?,
    };

    let to_format = args.to_format.unwrap();
    match args.output.as_deref() {
        Some("-") | None => to_writer(to_format, io::stdout(), value)?,
        Some(path) => to_writer(to_format, File::create(path)?, value)?,
    };

    Ok(())
}

fn from_reader(format: Format, mut reader: impl io::Read) -> Result<Variant> {
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

fn to_writer(format: Format, mut writer: impl io::Write, value: Variant) -> Result<()> {
    match format {
        Format::Json => serde_json::to_writer_pretty(writer, &value)?,
        Format::Pickle => {
            let opts = serde_pickle::SerOptions::new();
            serde_pickle::to_writer(&mut writer, &value, opts)?
        }
        Format::Plist => plist::to_writer_xml(writer, &value)?,
        Format::PlistB => plist::to_writer_binary(writer, &value)?,
        Format::Toml => {
            let s = toml::ser::to_string_pretty(&value)?;
            writer.write_all(s.as_bytes())?
        }
        Format::Yaml => serde_yaml::to_writer(writer, &value)?,
    };

    Ok(())
}
