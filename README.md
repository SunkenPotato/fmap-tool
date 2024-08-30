# fmap-tool
A little frequency map tool I made over the weekend in Rust

## Usage
```
Usage: fmap-tool [OPTIONS] < --infile [<file>]| --stdin| --raw [<string>] >

Options:
  -i, --infile [<file>]   Input file (optional)
  -o, --outfile [<file>]  Out file (optional) stdout is used otherwise
  -j, --json              Print output as JSON
      --stdin             Use stdin as input
  -r, --raw [<string>]    Use raw input
  -h, --help              Print help
```

One of the input arguments `--infile`, `--raw`, or `--stdin` must be specified.
`--infile` takes any file. Example:
```
$~ fmap-tool --infile hello.txt
```
`--raw` takes a string passed after it. Example:
```
$~ fmap-tool --raw "Hello, world"
```
`--stdin` reads from the standard input stream until EOF. Example:
```
$~ cat hello.txt | fmap-tool --stdin
```

`--outfile` is optional.
The `--json` flag returns the input as computer-readable JSON.

## Installation || macOS / Unix
```sh
git clone https://github.com/SunkenPotato/fmap-tool.git
cd fmap-tool
cargo build --release
cp target/release/fmap-tool /usr/local/bin
# Reload shell
```
### Windows
```sh
git clone https://github.com/SunkenPotato/fmap-tool.git
cd fmap-tool
cargo build --release
# Retrieve the executable from target/release
```