use anyhow::{Result, Context};
use log::{debug, info};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader};
use std::fs::File;

use crate::cli::Args;
use crate::io::{file_discovery, reader::FileReader};
use crate::regex::RegexMatcher;

/// Search result statistics
#[derive(Debug, Default)]
pub struct SearchResult {
    pub total_matches: usize,
    pub files_searched: usize,
    pub files_with_matches: usize,
    pub bytes_processed: usize,
}

/// Main search execution function
pub fn execute_search(args: &Args) -> Result<SearchResult> {
    info!("Initializing search engine");
    
    // Create regex matcher
    let matcher = RegexMatcher::new(&args.pattern, args.case_sensitive)?;
    
    // Configure thread pool size
    let num_threads = if args.threads == 0 {
        // Auto-detect number of threads
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    } else {
        args.threads
    };
    info!("Using {} threads for search", num_threads);
    
    // Set global thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap_or_else(|e| {
            debug!("Failed to set global thread pool: {}", e);
        });
    
    // Discover files to search
    let files = file_discovery::find_files(&args.path, args.recursive)?;
    info!("Found {} files to search", files.len());
    
    // Setup progress display if enabled
    let progress = if args.progress {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({per_sec}) {msg}")
                .context("Failed to set progress bar style")?
        );
        Some(pb)
    } else {
        None
    };
    
    // Store search results
    let result = Arc::new(Mutex::new(SearchResult::default()));
    
    // Process files in parallel
    files.par_iter()
        .for_each(|file| {
            let file_str = file.to_string_lossy();
            debug!("Searching file: {}", file_str);
            
            // Process individual file
            match process_file(file, &matcher, args) {
                Ok(file_result) => {
                    // Update global results
                    let mut result_guard = result.lock().unwrap();
                    result_guard.total_matches += file_result.matches;
                    result_guard.files_searched += 1;
                    result_guard.bytes_processed += file_result.bytes_processed;
                    
                    if file_result.matches > 0 {
                        result_guard.files_with_matches += 1;
                        
                        // Output results based on format options
                        if !args.count && !args.files_with_matches {
                            // For real-time display, we display matches immediately
                            if args.line_numbers {
                                // Display with line numbers requires opening the file again
                                // This is not optimal but necessary for exact line numbers
                                if let Err(e) = display_matches_with_line_numbers(file, &matcher, args) {
                                    eprintln!("Error displaying matches for {}: {}", file_str, e);
                                }
                            } else {
                                println!("Found {} matches in {}", file_result.matches, file_str);
                            }
                        } else if args.files_with_matches && file_result.matches > 0 {
                            println!("{}", file_str);
                        } else if args.count {
                            println!("{}: {}", file_str, file_result.matches);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error processing file {}: {}", file_str, e);
                }
            }
            
            // Update progress
            if let Some(pb) = &progress {
                pb.inc(1);
            }
        });
    
    // Finish progress display
    if let Some(pb) = progress {
        pb.finish_with_message("Search complete");
    }
    
    let final_result = Arc::try_unwrap(result)
        .expect("Failed to unwrap Arc")
        .into_inner()
        .expect("Failed to unwrap Mutex");
    
    info!("Search completed. Found {} matches in {} files ({} had matches)",
          final_result.total_matches,
          final_result.files_searched,
          final_result.files_with_matches);
    
    Ok(final_result)
}

// Result for processing a single file
struct FileResult {
    matches: usize,
    bytes_processed: usize,
}

// Process individual file
fn process_file(file: &std::path::Path, matcher: &RegexMatcher, args: &Args) -> Result<FileResult> {
    let reader = FileReader::new(file)?;
    let file_size = reader.size() as usize;
    
    // For small files, just read the whole file at once
    if file_size < args.chunk_size * 1024 {
        let content = reader.read_all()?;
        let match_count = matcher.match_count(&content);
        
        return Ok(FileResult {
            matches: match_count,
            bytes_processed: content.len(),
        });
    }
    
    // For large files, process in chunks
    let chunk_size = args.chunk_size * 1024;
    let mut offset = 0;
    let mut total_matches = 0;
    
    while offset < file_size {
        let chunk = reader.read_chunk(offset, chunk_size)?;
        if chunk.is_empty() {
            break;
        }
        
        let matches = matcher.match_count(&chunk);
        total_matches += matches;
        
        offset += chunk.len();
    }
    
    Ok(FileResult {
        matches: total_matches,
        bytes_processed: file_size,
    })
}

// Display matches with line numbers
fn display_matches_with_line_numbers(file: &std::path::Path, matcher: &RegexMatcher, _args: &Args) -> Result<()> {
    let file_str = file.to_string_lossy();
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    
    println!("File: {}", file_str);
    
    for (line_num, line_result) in reader.lines().enumerate() {
        if let Ok(line) = line_result {
            if matcher.is_match(line.as_bytes()) {
                println!("{}: {}", line_num + 1, line);
            }
        }
    }
    
    println!("");
    Ok(())
} 