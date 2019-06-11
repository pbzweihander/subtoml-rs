# subtoml-rs

[![crates.io badge]][crates.io]

**subtoml-rs** is a CLI utility that substitutes parts of TOML file, inspired by [dahlia/subtoml], but written in Rust.

## Usage

```
subtoml 0.1.0
pbzweihander <pbzweihander@gmail.com>
Substitute parts of TOML file

USAGE:
    subtoml [OPTIONS] [--] [KEY=VALUE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delete <KEY>...       Keys to delete
    -i, --input-file <FILE>     Toml file to read. default: stdin
    -o, --output-file <FILE>    Toml file to write. default: stdout

ARGS:
    <KEY=VALUE>...    Entries to add or modify. Must be form of KEY[.SUBKEY...]=VALUE
```

### Example

```bash
$ subtoml -i Cargo.toml \
    -d package \
    -d dependencies.structopt \
    dependencies.toml_edit=foo

[dependencies]
toml_edit = "foo"

```

------

_subtoml-rs_ is distributed under the terms of both [MIT License] and
[Apache License 2.0].
See [COPYRIGHT] for detail.

[crates.io]: https://crates.io/crates/subtoml
[crates.io badge]: https://badgen.net/crates/v/subtoml

[dahlia/subtoml]: https://bitbucket.org/dahlia/subtoml/

[MIT License]: LICENSE-MIT
[Apache License 2.0]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
