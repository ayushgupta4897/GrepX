use anyhow::{Result, Context};
use memmap2::{Mmap, MmapOptions};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

const MIN_SIZE_FOR_MMAP: u64 = 1024 * 1024; // 1MB

/// File reader that uses memory mapping for large files and buffered I/O for small files
pub struct FileReader {
    path: std::path::PathBuf,
    size: u64,
}

impl FileReader {
    /// Create a new file reader for the given path
    pub fn new(path: &Path) -> Result<Self> {
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Failed to get metadata for file: {}", path.display()))?;
        
        Ok(Self {
            path: path.to_path_buf(),
            size: metadata.len(),
        })
    }
    
    /// Get the file size
    pub fn size(&self) -> u64 {
        self.size
    }
    
    /// Read the entire file content as bytes
    pub fn read_all(&self) -> Result<Vec<u8>> {
        let content = if self.should_use_mmap() {
            self.read_mmap()?
        } else {
            self.read_buffered()?
        };
        
        Ok(content)
    }
    
    /// Read a chunk of the file from the given offset with specified size
    pub fn read_chunk(&self, offset: usize, size: usize) -> Result<Vec<u8>> {
        if offset as u64 > self.size {
            return Ok(Vec::new());
        }
        
        let actual_size = std::cmp::min(size, (self.size as usize).saturating_sub(offset));
        
        if actual_size == 0 {
            return Ok(Vec::new());
        }
        
        let content = if self.should_use_mmap() {
            let mmap = self.create_mmap()?;
            let end = std::cmp::min(offset + actual_size, mmap.len());
            mmap[offset..end].to_vec()
        } else {
            let mut file = File::open(&self.path)?;
            file.seek(SeekFrom::Start(offset as u64))?;
            
            let mut buffer = vec![0; actual_size];
            let bytes_read = file.read(&mut buffer)?;
            buffer.truncate(bytes_read);
            buffer
        };
        
        Ok(content)
    }
    
    /// Read file lines using a buffered reader
    pub fn read_lines(&self) -> Result<impl Iterator<Item = Result<String, io::Error>>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        Ok(reader.lines())
    }
    
    /// Determine if memory mapping should be used based on file size
    fn should_use_mmap(&self) -> bool {
        self.size >= MIN_SIZE_FOR_MMAP
    }
    
    /// Read file using memory mapping
    fn read_mmap(&self) -> Result<Vec<u8>> {
        let mmap = self.create_mmap()?;
        Ok(mmap.to_vec())
    }
    
    /// Create memory map for file
    fn create_mmap(&self) -> Result<Mmap> {
        let file = File::open(&self.path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        Ok(mmap)
    }
    
    /// Read file using buffered I/O
    fn read_buffered(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(self.size as usize);
        let mut file = File::open(&self.path)?;
        io::Read::read_to_end(&mut file, &mut buffer)?;
        Ok(buffer)
    }
} 