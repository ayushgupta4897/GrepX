use anyhow::Result;
use clap::{Parser, ValueEnum};

/// GrepX - A distributed, multi-threaded regex search engine
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Regex pattern to search for
    #[arg(index = 1, required = true)]
    pub pattern: String,

    /// File paths to search (supports glob patterns)
    #[arg(index = 2, default_value = ".")]
    pub path: Vec<String>,
    
    /// Number of threads to use (0 = auto)
    #[arg(short, long, default_value_t = 0)]
    pub threads: usize,
    
    /// Recursively search directories
    #[arg(short = 'r', long)]
    pub recursive: bool,
    
    /// Case-sensitive matching
    #[arg(short = 's', long)]
    pub case_sensitive: bool,
    
    /// Show line numbers
    #[arg(short = 'n', long)]
    pub line_numbers: bool,
    
    /// Only print filenames with matches
    #[arg(short = 'l', long)]
    pub files_with_matches: bool,
    
    /// Count matches per file
    #[arg(short = 'c', long)]
    pub count: bool,
    
    /// Display progress bar
    #[arg(short = 'p', long)]
    pub progress: bool,
    
    /// Chunk size in KB for parallel processing
    #[arg(long, default_value_t = 64)]
    pub chunk_size: usize,
    
    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
    
    /// Set logging level
    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Grep,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub fn parse_args() -> Result<Args> {
    let args = Args::parse();
    
    // Set log level based on argument
    match args.log_level {
        LogLevel::Off => std::env::set_var("RUST_LOG", "off"),
        LogLevel::Error => std::env::set_var("RUST_LOG", "error"),
        LogLevel::Warn => std::env::set_var("RUST_LOG", "warn"),
        LogLevel::Info => std::env::set_var("RUST_LOG", "info"),
        LogLevel::Debug => std::env::set_var("RUST_LOG", "debug"),
        LogLevel::Trace => std::env::set_var("RUST_LOG", "trace"),
    }
    
    Ok(args)
} 