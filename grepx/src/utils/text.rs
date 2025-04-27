use std::borrow::Cow;

/// Extract a line containing the match with optional context
pub fn extract_context<'a>(text: &'a str, match_start: usize, match_end: usize, context_lines: usize) -> Cow<'a, str> {
    if context_lines == 0 {
        // Just return the match
        return Cow::Borrowed(&text[match_start..match_end]);
    }
    
    // Find line start
    let line_start = text[..match_start].rfind('\n').map_or(0, |pos| pos + 1);
    
    // Find line end
    let line_end = text[match_end..].find('\n')
        .map_or(text.len(), |pos| match_end + pos);
    
    // For context lines before
    let mut context_start = line_start;
    let mut lines_before = 0;
    while lines_before < context_lines && context_start > 0 {
        if let Some(pos) = text[..context_start-1].rfind('\n') {
            context_start = pos + 1;
            lines_before += 1;
        } else {
            context_start = 0;
            break;
        }
    }
    
    // For context lines after
    let mut context_end = line_end;
    let mut lines_after = 0;
    while lines_after < context_lines && context_end < text.len() {
        if let Some(pos) = text[context_end+1..].find('\n') {
            context_end = context_end + 1 + pos;
            lines_after += 1;
        } else {
            context_end = text.len();
            break;
        }
    }
    
    Cow::Borrowed(&text[context_start..context_end])
}

/// Format a byte size as a human-readable string
pub fn format_size(size: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;
    
    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

/// Format a duration in seconds as a human-readable string
pub fn format_duration(seconds: f64) -> String {
    if seconds < 0.001 {
        format!("{:.2} μs", seconds * 1_000_000.0)
    } else if seconds < 1.0 {
        format!("{:.2} ms", seconds * 1_000.0)
    } else if seconds < 60.0 {
        format!("{:.2} s", seconds)
    } else {
        let minutes = (seconds / 60.0) as u64;
        let remaining_seconds = seconds % 60.0;
        format!("{}m {:.2}s", minutes, remaining_seconds)
    }
}

/// Calculate search speed in bytes per second
pub fn calculate_speed(bytes: usize, duration_secs: f64) -> f64 {
    if duration_secs > 0.0 {
        bytes as f64 / duration_secs
    } else {
        0.0
    }
}

/// Format search speed as a human-readable string
pub fn format_speed(bytes_per_sec: f64) -> String {
    format!("{}/s", format_size(bytes_per_sec as usize))
} 