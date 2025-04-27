use anyhow::{Result, Context};
use regex::{Regex, RegexBuilder};
use std::borrow::Cow;

/// A match found in the text
#[derive(Debug, Clone)]
pub struct Match<'a> {
    pub text: Cow<'a, str>,
    pub line_number: Option<usize>,
    pub byte_offset: usize,
    pub byte_length: usize,
}

/// Regex matcher for searching text content
pub struct RegexMatcher {
    regex: Regex,
    pattern: String,
}

impl RegexMatcher {
    /// Create a new RegexMatcher with the given pattern
    pub fn new(pattern: &str, case_sensitive: bool) -> Result<Self> {
        // Build regex with appropriate options
        let regex = RegexBuilder::new(pattern)
            .case_insensitive(!case_sensitive)
            .multi_line(true)
            .dot_matches_new_line(false)
            .build()
            .with_context(|| format!("Failed to compile regex pattern: {}", pattern))?;
        
        Ok(Self {
            regex,
            pattern: pattern.to_string(),
        })
    }
    
    /// Get the regex pattern
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
    
    /// Find all matches in the given text
    pub fn find_matches<'a>(&self, text: &'a [u8], with_line_numbers: bool) -> Vec<Match<'a>> {
        let text_str = match std::str::from_utf8(text) {
            Ok(s) => s,
            Err(_) => return Vec::new(), // Skip non-UTF8 content
        };
        
        // Line number mapping (only if needed)
        let line_map = if with_line_numbers {
            self.create_line_map(text_str)
        } else {
            Vec::new()
        };
        
        // Find all regex matches
        self.regex.find_iter(text_str)
            .map(|m| {
                let line_number = if with_line_numbers {
                    self.find_line_number(&line_map, m.start())
                } else {
                    None
                };
                
                Match {
                    text: Cow::Borrowed(&text_str[m.start()..m.end()]),
                    line_number,
                    byte_offset: m.start(),
                    byte_length: m.end() - m.start(),
                }
            })
            .collect()
    }
    
    /// Create a map of newline positions for line number calculations
    fn create_line_map(&self, text: &str) -> Vec<usize> {
        let mut positions = Vec::new();
        positions.push(0); // First line starts at position 0
        
        for (i, c) in text.char_indices() {
            if c == '\n' {
                positions.push(i + 1);
            }
        }
        
        positions
    }
    
    /// Find the line number for a given byte offset
    fn find_line_number(&self, line_map: &[usize], offset: usize) -> Option<usize> {
        match line_map.binary_search(&offset) {
            Ok(exact) => Some(exact + 1),
            Err(insert_pos) => {
                if insert_pos > 0 {
                    Some(insert_pos)
                } else {
                    None
                }
            }
        }
    }
    
    /// Check if the text contains any match
    pub fn is_match(&self, text: &[u8]) -> bool {
        if let Ok(text_str) = std::str::from_utf8(text) {
            self.regex.is_match(text_str)
        } else {
            false
        }
    }
    
    /// Count the number of matches in the text
    pub fn match_count(&self, text: &[u8]) -> usize {
        if let Ok(text_str) = std::str::from_utf8(text) {
            self.regex.find_iter(text_str).count()
        } else {
            0
        }
    }
} 