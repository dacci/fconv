# Data format converter

## Usage

```console
$ fconv --help
Data format converter

Usage: fconv [OPTIONS] [FILE]

Arguments:
  [FILE]  Specifies the path to the input file

Options:
      --formats                      Prints the supported formats
      --generate-completion <SHELL>  Generate shell completions [possible values: bash, elvish, fish, powershell, zsh]
  -f, --from-format <FORMAT>         Specifies the format of the input file [possible values: json, pickle, plist, plist-b, toml, yaml]
  -t, --to-format <FORMAT>           Specifies the format of the output file [possible values: json, pickle, plist, plist-b, toml, yaml]
  -o, --output <FILE>                Specifies the path to the output file (default: standard output)
  -h, --help                         Print help
  -V, --version                      Print version
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
  version: 0.7.1
  edition: '2021'
  description: Data format converter
  license: MIT
dependencies:
  clap:
    version: 4.3.11
    features:
    - derive
  indexmap: 2.0.0
  plist: 1.4.3
  serde: 1.0.167
  serde-pickle: 1.1.1
  serde_json: 1.0.100
  serde_yaml: 0.9.22
  toml: 0.7.6
dev-dependencies:
  serde_test: 1.0.167
profile:
  release:
    codegen-units: 1
    lto: true
    strip: true
```
