use crate::notebook::{Notebook, Page};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotebookFileHandler {
    // Configuration and state for file operations
}

impl NotebookFileHandler {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn save_notebook(&self, notebook: &Notebook, file_path: PathBuf) -> io::Result<()> {
        let content = self.serialize_notebook(notebook)?;
        fs::write(file_path, content)?;
        Ok(())
    }
    
    pub fn load_notebook(&self, file_path: PathBuf) -> io::Result<Notebook> {
        let content = fs::read_to_string(file_path)?;
        self.deserialize_notebook(&content)
    }
    
    fn serialize_notebook(&self, notebook: &Notebook) -> io::Result<String> {
        let mut content = String::new();
        
        // Write notebook header
        content.push_str(&format!("--- NOTEBOOKX NOTEBOOK ---\n"));
        content.push_str(&format!("NOTEBOOK_ID: {}\n", notebook.id));
        content.push_str(&format!("NOTEBOOK_TITLE: {}\n", notebook.title));
        content.push_str(&format!("CREATED: {}\n", notebook.created.to_rfc3339()));
        content.push_str(&format!("MODIFIED: {}\n", notebook.modified.to_rfc3339()));
        content.push_str(&format!("--- END NOTEBOOK HEADER ---\n\n"));
        
        // Write each page
        for (index, page) in notebook.pages.iter().enumerate() {
            if index > 0 {
                content.push_str("--- PAGE BREAK ---\n\n");
            }
            
            content.push_str("--- NOTEBOOKX METADATA ---\n");
            content.push_str(&format!("PAGE_ID: {}\n", page.id));
            content.push_str(&format!("TITLE: {}\n", page.title));
            if let Some(number) = page.number {
                content.push_str(&format!("NUMBER: {}\n", number));
            }
            content.push_str(&format!("CREATED: {}\n", page.created.to_rfc3339()));
            content.push_str(&format!("MODIFIED: {}\n", page.modified.to_rfc3339()));
            content.push_str("--- END METADATA ---\n\n");
            
            content.push_str(&page.content);
            content.push_str("\n\n");
        }
        
        Ok(content)
    }
    
    fn deserialize_notebook(&self, content: &str) -> io::Result<Notebook> {
        let sections: Vec<&str> = content.split("--- PAGE BREAK ---").collect();
        
        if sections.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid NotebookX file format"
            ));
        }
        
        let first_section = sections[0];
        let (notebook_header, first_page_content) = self.extract_notebook_header(first_section)?;
        
        let mut notebook = self.parse_notebook_header(&notebook_header)?;
        
        // Parse first page if it exists
        if !first_page_content.trim().is_empty() {
            if let Ok(page) = self.parse_page_section(&first_page_content) {
                notebook.pages.push(page);
            }
        }
        
        // Parse remaining pages
        for section in sections.iter().skip(1) {
            if let Ok(page) = self.parse_page_section(section) {
                notebook.pages.push(page);
            }
        }
        
        Ok(notebook)
    }
    
    fn extract_notebook_header(&self, content: &str) -> io::Result<(String, String)> {
        if let Some(header_start) = content.find("--- NOTEBOOKX NOTEBOOK ---") {
            if let Some(header_end) = content.find("--- END NOTEBOOK HEADER ---") {
                let header = content[header_start..header_end + "--- END NOTEBOOK HEADER ---".len()].to_string();
                let remaining = content[header_end + "--- END NOTEBOOK HEADER ---".len()..].to_string();
                return Ok((header, remaining));
            }
        }
        
        // Fallback: treat entire content as page content with default notebook
        Ok((String::new(), content.to_string()))
    }
    
    fn parse_notebook_header(&self, header: &str) -> io::Result<Notebook> {
        let mut notebook = Notebook::new("Untitled Notebook".to_string());
        
        for line in header.lines() {
            if line.starts_with("NOTEBOOK_ID: ") {
                notebook.id = line[13..].to_string();
            } else if line.starts_with("NOTEBOOK_TITLE: ") {
                notebook.title = line[16..].to_string();
            } else if line.starts_with("CREATED: ") {
                if let Ok(created) = DateTime::parse_from_rfc3339(&line[9..]) {
                    notebook.created = created.with_timezone(&Utc);
                }
            } else if line.starts_with("MODIFIED: ") {
                if let Ok(modified) = DateTime::parse_from_rfc3339(&line[10..]) {
                    notebook.modified = modified.with_timezone(&Utc);
                }
            }
        }
        
        Ok(notebook)
    }
    
    fn parse_page_section(&self, section: &str) -> io::Result<Page> {
        let mut page = Page::new("Untitled".to_string(), String::new(), None);
        
        if let Some(metadata_start) = section.find("--- NOTEBOOKX METADATA ---") {
            if let Some(metadata_end) = section.find("--- END METADATA ---") {
                let metadata = &section[metadata_start..metadata_end];
                let content_start = metadata_end + "--- END METADATA ---".len();
                let content = section[content_start..].trim().to_string();
                
                // Parse metadata
                for line in metadata.lines() {
                    if line.starts_with("PAGE_ID: ") {
                        page.id = line[9..].to_string();
                    } else if line.starts_with("TITLE: ") {
                        page.title = line[7..].to_string();
                    } else if line.starts_with("NUMBER: ") {
                        if let Ok(number) = line[8..].parse::<u32>() {
                            page.number = Some(number);
                        }
                    } else if line.starts_with("CREATED: ") {
                        if let Ok(created) = DateTime::parse_from_rfc3339(&line[9..]) {
                            page.created = created.with_timezone(&Utc);
                        }
                    } else if line.starts_with("MODIFIED: ") {
                        if let Ok(modified) = DateTime::parse_from_rfc3339(&line[10..]) {
                            page.modified = modified.with_timezone(&Utc);
                        }
                    }
                }
                
                page.content = content;
                return Ok(page);
            }
        }
        
        // Fallback: treat entire section as content
        page.content = section.trim().to_string();
        if page.content.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Empty page content"
            ));
        }
        
        Ok(page)
    }
}

impl Default for NotebookFileHandler {
    fn default() -> Self {
        Self::new()
    }
}