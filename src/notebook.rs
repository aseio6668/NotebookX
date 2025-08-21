use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub content: String,
    pub number: Option<u32>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

impl Page {
    pub fn new(title: String, content: String, number: Option<u32>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            number,
            created: now,
            modified: now,
        }
    }
    
    pub fn update_content(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.modified = Utc::now();
    }
    
    pub fn set_number(&mut self, number: Option<u32>) {
        self.number = number;
        self.modified = Utc::now();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub title: String,
    pub pages: Vec<Page>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

impl Notebook {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            pages: Vec::new(),
            created: now,
            modified: now,
        }
    }
    
    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
        self.modified = Utc::now();
        self.update_page_numbers();
    }
    
    pub fn remove_page(&mut self, page_id: &str) -> Option<Page> {
        if let Some(pos) = self.pages.iter().position(|p| p.id == page_id) {
            let page = self.pages.remove(pos);
            self.modified = Utc::now();
            self.update_page_numbers();
            Some(page)
        } else {
            None
        }
    }
    
    pub fn get_page_mut(&mut self, page_id: &str) -> Option<&mut Page> {
        self.pages.iter_mut().find(|p| p.id == page_id)
    }
    
    pub fn get_page(&self, page_id: &str) -> Option<&Page> {
        self.pages.iter().find(|p| p.id == page_id)
    }
    
    pub fn update_page(&mut self, page_id: &str, title: String, content: String) -> bool {
        if let Some(page) = self.get_page_mut(page_id) {
            page.update_content(title, content);
            self.modified = Utc::now();
            true
        } else {
            false
        }
    }
    
    fn update_page_numbers(&mut self) {
        for (index, page) in self.pages.iter_mut().enumerate() {
            page.set_number(Some((index + 1) as u32));
        }
    }
    
    pub fn reorder_pages(&mut self, from_index: usize, to_index: usize) {
        if from_index < self.pages.len() && to_index < self.pages.len() {
            let page = self.pages.remove(from_index);
            self.pages.insert(to_index, page);
            self.update_page_numbers();
            self.modified = Utc::now();
        }
    }
}