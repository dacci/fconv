# Data format converter

## Usage

```console
$ fconv --help
fconv 0.3.0
Data format converter

USAGE:
    fconv [OPTIONS] [FILE]

ARGS:
    <FILE>    Specifies the path to the input file

OPTIONS:
    -f, --from-format <FORMAT>    Specifies the format of the input file
        --formats                 Prints the supported formats
    -h, --help                    Print help information
    -o, --output <FILE>           Specifies the path to the output file (default: standard input)
    -t, --to-format <FORMAT>      Specifies the format of the output file
    -V, --version                 Print version information
```

## Supported formats

```
$ fconv --formats
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
---
package:
  name: fconv
  version: 0.3.0
  edition: "2021"
  description: Data format converter
dependencies:
  clap:
    version: 3.1.12
    features:
      - derive
  linked-hash-map: 0.5.4
  plist: 1.3.1
  serde: 1.0.136
  serde-pickle: 1.1.0
  serde_json: 1.0.79
  serde_test: 1.0.136
  serde_yaml: 0.8.23
  toml: 0.5.9
```
