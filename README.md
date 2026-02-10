# Pretty JSON
![Pretty json](/pretty_json_demo.png)

A simple CLI tool that accepts a JSON filepath and formats its contents.

Importantly this tool doesn't validate JSON files and will fail when parsing invalid input.

Features:
- Override file contents by default
- Supports shortcut `--ouput|-o` that outputs changes to new file
- Select pretty or compact output via shortcut `--format|-f`; defaults to `pretty`

## Getting started

Test the tool with cargo
```bash
cargo run data/test.json
```

Which will prettify the specified json file.

```bash
cargo run data/test.json -o data/foo.json
```

Prettify's the json file and creates a new file at the specified output path.

```bash
cargo run data/test.json -f compact
```

Formats the json file using the compact setting so the contents are on a single line.


Create a production version
```bash
cargo build --release
```

Then you can run the tool as a binary like
```bash
./target/release/pretty_json data/test.json --output data/foo_bar.json
```

Installing the tool via Cargo makes it easy to run the binary like any other CLI tool.
```bash
cargo install --path .
pj data/test.json
```

Note: pj == pretty_json. i prefer a shorthand when typing. This can be changed to anything else via bin in [Cargo.toml](/Cargo.toml)

```toml
[[bin]]
name = "pj" # pretty_json | pjson | etc.
path = "src/main.rs"
```

