use anyhow::Result;
use log::{debug, warn};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Find files to search based on provided paths and recursion options
pub fn find_files(paths: &[String], recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for path in paths {
        let path = Path::new(path);
        
        if !path.exists() {
            warn!("Path does not exist: {}", path.display());
            continue;
        }
        
        if path.is_file() {
            debug!("Adding file: {}", path.display());
            files.push(path.to_path_buf());
        } else if path.is_dir() {
            if recursive {
                // Recursively traverse directory
                debug!("Recursively traversing directory: {}", path.display());
                add_files_recursive(path, &mut files)?;
            } else {
                // Add only top-level files
                debug!("Adding top-level files from directory: {}", path.display());
                add_files_nonrecursive(path, &mut files)?;
            }
        }
    }
    
    debug!("Found {} files to search", files.len());
    Ok(files)
}

/// Add files recursively from a directory
fn add_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
        
        let path = entry.path();
        
        if path.is_file() {
            debug!("Adding file: {}", path.display());
            files.push(path.to_path_buf());
        }
    }
    
    Ok(())
}

/// Add only top-level files from a directory (non-recursive)
fn add_files_nonrecursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            debug!("Adding file: {}", path.display());
            files.push(path);
        }
    }
    
    Ok(())
} 