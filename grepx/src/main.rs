mod cli;
mod engine;
mod io;
mod regex;
mod utils;

use anyhow::Result;
use log::info;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Parse command line arguments
    let args = cli::parse_args()?;
    
    info!("Starting GrepX search with pattern: {}", args.pattern);
    
    // Execute search based on arguments
    let result = engine::execute_search(&args)?;
    
    // Display results summary
    println!("Found {} matches in {} files", 
             result.total_matches, 
             result.files_searched);
    
    Ok(())
}
