# Data format converter

## Usage

```console
$ fconv --help
Data format converter

Usage: fconv [OPTIONS] [FILE]

Arguments:
  [FILE]  Specifies the path to the input file

Options:
      --formats               Prints the supported formats
  -f, --from-format <FORMAT>  Specifies the format of the input file
  -t, --to-format <FORMAT>    Specifies the format of the output file
  -o, --output <FILE>         Specifies the path to the output file (default: standard output)
  -h, --help                  Print help information
  -V, --version               Print version information
```

## Supported formats

```
$ fconv --formats
Supported formats:
  json    JavaScript Object Notation
  pickle  Python's serialization format
  plist   Property list (XML)
  plistb  Property list (binary)
  toml    Tom's Obvious, Minimal Language
  yaml    YAML Ain't Markup Language
```

## Example

```console
$ fconv -f toml -t yaml Cargo.toml
package:
  name: fconv
  version: 0.6.0
  edition: '2021'
  description: Data format converter
  license: MIT
dependencies:
  clap:
    version: 4.0.18
    features:
    - derive
  indexmap: 1.9.1
  plist: 1.3.1
  serde: 1.0.142
  serde-pickle: 1.1.1
  serde_json: 1.0.83
  serde_yaml: 0.9.4
  toml: 0.5.9
dev-dependencies:
  serde_test: 1.0.142
profile:
  release:
    codegen-units: 1
    lto: true
    strip: true
```
