# GrepX

The world's fastest distributed, multi-threaded regex search engine, outpacing ripgrep for blazing-fast, scalable log and codebase analysis.

## Features

- **Unparalleled Speed**: Outperforms traditional grep tools and even ripgrep with advanced parallelization
- **Drop-in Replacement**: Familiar syntax makes switching from other grep tools seamless
- **Memory Efficient**: Memory-mapped I/O for handling petabyte-scale data
- **Powerful Regex**: Full regex pattern support with SIMD acceleration
- **Real-time Progress**: Live search progress tracking through massive datasets
- **Distributed Search**: Scale searches across multiple machines for incredible throughput
- **User-friendly Output**: Colorized, contextual match display

## Installation

### From Source

```bash
git clone git@github.com:ayushgupta4897/GrepX.git
cd grepx
cargo build --release
```

The binary will be available at `./target/release/grepx`.

## Usage

```bash
grepx [OPTIONS] <PATTERN> [PATH]...
```

### Examples

Search for a pattern in current directory:
```bash
grepx "function\s+\w+\(" .
```

Search with case sensitivity:
```bash
grepx -s "Error" /var/log/
```

Show line numbers in matches:
```bash
grepx -n "TODO" src/
```

Search recursively with progress bar:
```bash
grepx -r -p "impl.*for" .
```

### Options

```
Options:
  -t, --threads <THREADS>            Number of threads to use (0 = auto) [default: 0]
  -r, --recursive                    Recursively search directories
  -s, --case-sensitive               Case-sensitive matching
  -n, --line-numbers                 Show line numbers
  -l, --files-with-matches           Only print filenames with matches
  -c, --count                        Count matches per file
  -p, --progress                     Display progress bar
      --chunk-size <CHUNK_SIZE>      Chunk size in KB for parallel processing [default: 64]
  -f, --format <FORMAT>              Output format [default: text] [possible values: text, json, grep]
      --log-level <LOG_LEVEL>        Set logging level [default: info]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Performance

GrepX is engineered to be the world's fastest grep tool:

- **Multi-threading**: Utilizes every available CPU core efficiently
- **Zero-copy I/O**: Memory mapping for minimal overhead
- **SIMD Acceleration**: Hardware-level pattern matching
- **Adaptive Work Stealing**: Optimal task distribution across cores
- **I/O Overlapping**: Asynchronous prefetching while processing
- **Distributed Search**: Scale horizontally across multiple machines

## License

MIT 