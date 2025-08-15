use crate::notebook::{Notebook, Page};
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct OneNoteConverter {
    // Future: Could include configuration options for conversion
}

impl OneNoteConverter {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Convert a OneNote .one file to NotebookX format
    /// Note: This is a basic implementation that attempts to extract text content
    /// Full OneNote parsing would require implementing the complete MS-ONESTORE specification
    pub fn convert_to_notebookx(&self, one_file_path: PathBuf) -> io::Result<Notebook> {
        // For now, we'll implement a basic converter that creates a placeholder
        // In a full implementation, this would parse the binary OneNote format
        
        let file_name = one_file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Converted Notebook");
            
        let mut notebook = Notebook::new(format!("Converted from {}", file_name));
        
        // Placeholder implementation - in reality this would parse the binary format
        let conversion_page = Page::new(
            "OneNote Conversion Note".to_string(),
            format!(
                "This notebook was converted from a OneNote file: {}\n\n\
                 IMPORTANT: This is a basic conversion placeholder.\n\n\
                 To implement full OneNote conversion, the following would be needed:\n\
                 \n\
                 1. Parse the OneNote Revision Store File Format (.one)\n\
                 2. Extract the header structure\n\
                 3. Parse object spaces and property sets\n\
                 4. Extract text content, formatting, and metadata\n\
                 5. Convert images and attachments\n\
                 6. Map OneNote sections and pages to NotebookX format\n\
                 \n\
                 For a complete implementation, refer to:\n\
                 - MS-ONESTORE specification\n\
                 - OneNote File Format documentation\n\
                 \n\
                 Original file: {}",
                one_file_path.display(),
                one_file_path.display()
            ),
            Some(1),
        );
        
        notebook.add_page(conversion_page);
        
        Ok(notebook)
    }
    
    /// Attempt to extract basic text content from OneNote file
    /// This is a very basic approach and won't work for all OneNote files
    pub fn extract_basic_text(&self, one_file_path: PathBuf) -> io::Result<Vec<String>> {
        let content = fs::read(&one_file_path)?;
        
        // Basic text extraction - look for UTF-16 strings
        let mut extracted_text = Vec::new();
        let mut current_text = String::new();
        
        // Simple approach: look for readable text sequences
        // This is very basic and won't capture all content
        for chunk in content.chunks(2) {
            if chunk.len() == 2 {
                let utf16_char = u16::from_le_bytes([chunk[0], chunk[1]]);
                
                if let Some(ch) = char::from_u32(utf16_char as u32) {
                    if ch.is_ascii_graphic() || ch.is_whitespace() {
                        current_text.push(ch);
                    } else if !current_text.trim().is_empty() {
                        extracted_text.push(current_text.trim().to_string());
                        current_text.clear();
                    }
                }
            }
        }
        
        if !current_text.trim().is_empty() {
            extracted_text.push(current_text.trim().to_string());
        }
        
        // Filter out very short strings that are likely noise
        let filtered: Vec<String> = extracted_text
            .into_iter()
            .filter(|s| s.len() > 5 && s.chars().any(|c| c.is_alphabetic()))
            .take(50) // Limit to prevent overwhelming output
            .collect();
            
        Ok(filtered)
    }
    
    /// Create a detailed conversion report
    pub fn create_conversion_report(&self, one_file_path: PathBuf) -> io::Result<Page> {
        let file_size = fs::metadata(&one_file_path)?.len();
        let extracted_text = self.extract_basic_text(one_file_path.clone())?;
        
        let report_content = format!(
            "OneNote File Conversion Report\n\
             ================================\n\
             \n\
             Source File: {}\n\
             File Size: {} bytes\n\
             Extracted Text Fragments: {}\n\
             \n\
             Extracted Content Preview:\n\
             --------------------------\n\
             {}\n\
             \n\
             Note: This is a basic extraction. For complete OneNote support,\n\
             implement the full MS-ONESTORE specification.",
            one_file_path.display(),
            file_size,
            extracted_text.len(),
            extracted_text.join("\n")
        );
        
        Ok(Page::new(
            "Conversion Report".to_string(),
            report_content,
            Some(1),
        ))
    }
}

impl Default for OneNoteConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_convert_placeholder() {
        let converter = OneNoteConverter::new();
        
        // Create a temporary "OneNote" file for testing
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Fake OneNote content").unwrap();
        
        let result = converter.convert_to_notebookx(temp_file.path().to_path_buf());
        assert!(result.is_ok());
        
        let notebook = result.unwrap();
        assert_eq!(notebook.pages.len(), 1);
        assert!(notebook.title.contains("Converted from"));
    }
}