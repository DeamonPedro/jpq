# jpq

[JSONPath](<https://github.com/json-path>) query command line tool

`jq` utility based project using JSONPath and developed in Rust :crab:.

## :rocket: Installation

You can obtain the executable file by downloading a release if you use a Linux system (x86_64 and i686) or by compiling the source code.

### :arrow_down: Download

- [Download](https://github.com/DeamonPedro/jpq/releases/tag/stable) the latest stable version
- Extract the zip file

### :construction_worker: Build

> To build for your system it is necessary to install the [Rust development environment](https://www.rust-lang.org/tools/install)

Clone the repository and run the build command:

```sh
git clone https://github.com/DeamonPedro/jpq.git
cd jpq
cargo build --release --target <TARGET>
```

Most used targets:

|           target          |                  notes                  |
|:-------------------------:|:---------------------------------------:|
| x86_64-unknown-linux-gnu  | 64-bit Linux (kernel 3.2+, glibc 2.17+) |
| i686-unknown-linux-gnu    | 32-bit Linux (kernel 3.2+, glibc 2.17+) |
| aarch64-unknown-linux-gnu | ARM64 Linux (kernel 4.1, glibc 2.17+) 1 |
| x86_64-pc-windows-msvc    | 64-bit MSVC (Windows 7+) 2              |
| x86_64-pc-windows-gnu     | 64-bit MinGW (Windows 7+) 2             |
| i686-pc-windows-msvc      | 32-bit MSVC (Windows 7+) 2              |
| i686-pc-windows-gnu       | 32-bit MinGW (Windows 7+) 2             |
| x86_64-apple-darwin       | 64-bit macOS (10.7+, Lion+)             |
</code>

With the executable file you can make it accessible to the shell by copying it to some folder of the environment variable $PATH, like `/sbin`, `/usr/sbin` or `/usr/local/bin`, as in the example:

```sh
cp jpq /usr/local/bin/
jpq -V
```

## :hammer_and_wrench: Usage

```sh
jpq [OPTIONS] <JSONPATH_QUERY>
```

### Options

- `-f, --file <FILE>` Input JSON file
- `-h, --help` Print help information
- `-p, --pain-text` Force plain text output
- `-V, --version` Print version information

### Input

- File input with `-f`

```sh
jpq -f file.json '$.key1.key2'
```

- Pipe input

```sh
cat file.json | jpq '$.key1.key2'
```

### Output

You can force output plain text with the flag `-p` (pretty output enabled by default)

```sh
jpq -f file.json '$' -p
```

Plain text output example:

```json
{"key1":{"key2":"value"}}
```

Pretty output example:

```json
{
  "key1": {
    "key2": "value"
  }
}
```
