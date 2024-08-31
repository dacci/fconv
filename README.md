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
  version: 0.8.0
  edition: '2021'
  description: Data format converter
  license: MIT
dependencies:
  clap:
    version: 4.5.16
    features:
    - derive
  clap_complete: 4.5.24
  indexmap: 2.5.0
  plist: 1.7.0
  serde: 1.0.209
  serde-pickle: 1.1.1
  serde_json: 1.0.127
  serde_yaml_ng: 0.10.0
  toml: 0.8.19
dev-dependencies:
  serde_test: 1.0.177
profile:
  release:
    codegen-units: 1
    lto: true
    strip: true
```
