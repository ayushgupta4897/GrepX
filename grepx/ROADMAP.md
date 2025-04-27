# GrepX Roadmap

## Implemented Features

- [x] Basic CLI structure with robust command-line arguments
- [x] Regex pattern matching using Rust's regex engine
- [x] Parallel file processing with Rayon
- [x] Memory-mapped file I/O for efficient file access
- [x] Smart chunking for large files
- [x] Progress bar display
- [x] Different output formats:
  - [x] Regular output
  - [x] Line numbers
  - [x] Count only
  - [x] Files with matches
- [x] Recursive directory traversal

## Next Steps

- [ ] Implement colored output for matches
- [ ] Add context lines option (-A, -B, -C for after, before, and context)
- [ ] Add support for file inclusion/exclusion patterns
- [ ] Implement distributed mode for searching across multiple machines
- [ ] Add binary file handling (skip, search, etc.)
- [ ] Add benchmarking module against grep and ripgrep
- [ ] Implement multiple pattern matching
- [ ] Add SIMD acceleration for common patterns
- [ ] Implement real-time streaming results for large files
- [ ] Add custom output formatting templates
- [ ] Support for reading from stdin
- [ ] Implement Web UI with heatmap (long-term)

## Performance Optimizations

- [ ] Optimize regex compilation with pre-filtering
- [ ] Implement Boyer-Moore/Aho-Corasick for literal substrings
- [ ] Optimize chunk size based on file type and system
- [ ] Implement result buffering for smoother output
- [ ] Add adaptive work stealing for better core utilization
- [ ] Optimize memory usage for very large files

## Testing Plan

- [ ] Unit tests for each module
- [ ] Integration tests for full functionality
- [ ] Performance benchmarks against:
  - [ ] GNU grep
  - [ ] ripgrep
  - [ ] ag (The Silver Searcher)
  - [ ] ugrep
- [ ] Test on various file sizes:
  - [ ] Small files (<1MB)
  - [ ] Medium files (1MB-100MB)
  - [ ] Large files (100MB-10GB)
  - [ ] Huge files (>10GB)
- [ ] Test on various file types:
  - [ ] Text files
  - [ ] Code repositories
  - [ ] Log files
  - [ ] Binary files

## Release Plan

- [ ] v0.1.0: Initial release with basic functionality
- [ ] v0.2.0: Performance optimizations and colored output
- [ ] v0.3.0: Context lines and multiple pattern matching
- [ ] v0.4.0: Distributed mode
- [ ] v1.0.0: Full-featured release with performance matching or exceeding ripgrep 