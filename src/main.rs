#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use clap::Parser;

mod notebook;
mod file_io;
mod onenote_converter;

use notebook::{Notebook, Page};
use file_io::NotebookFileHandler;
use onenote_converter::OneNoteConverter;

// Standard US Letter page dimensions for text content
// US Letter: 8.5" x 11" at 96 DPI with 1" margins = 6.5" x 9" text area
// At 14px font (typical), ~46 lines of text, ~80 characters per line
const PAGE_MAX_LINES: usize = 46;
const PAGE_MAX_CHARS_PER_LINE: usize = 80;
const PAGE_MAX_CHARS: usize = PAGE_MAX_LINES * PAGE_MAX_CHARS_PER_LINE; // ~3680 chars
const HINT_TEXT: &str = "Start writing your notes here...";

#[derive(Parser)]
#[command(name = "notebookx")]
#[command(about = "A cross-platform notebook application similar to OneNote")]
struct Args {
    #[arg(long, help = "Enable debug mode (shows console on Windows)")]
    debug: bool,
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();
    
    // On Windows in debug mode, allocate console for debug output
    #[cfg(windows)]
    if args.debug {
        unsafe {
            use std::ffi::CString;
            extern "system" {
                fn AllocConsole() -> i32;
                fn SetConsoleTitleA(title: *const i8) -> i32;
            }
            AllocConsole();
            let title = CString::new("NotebookX Debug Console").unwrap();
            SetConsoleTitleA(title.as_ptr());
        }
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "NotebookX",
        options,
        Box::new(|_cc| Ok(Box::new(NotebookXApp::default()))),
    )
}

#[derive(Default)]
struct NotebookXApp {
    notebook: Option<Notebook>,
    current_page_id: Option<String>,
    page_title_buffer: String,
    page_content_buffer: String,
    scroll_offset: f32,
    file_handler: NotebookFileHandler,
    onenote_converter: OneNoteConverter,
    show_open_dialog: bool,
    show_save_dialog: bool,
    show_convert_dialog: bool,
    autosave_enabled: bool,
    current_file_path: Option<std::path::PathBuf>,
}

impl NotebookXApp {
    fn clean_content(&self, content: &str) -> String {
        // Remove hint text if it's the only content or at the beginning
        let cleaned = if content == HINT_TEXT {
            String::new()
        } else if content.starts_with(HINT_TEXT) {
            content.strip_prefix(HINT_TEXT).unwrap_or(content).to_string()
        } else {
            content.to_string()
        };
        
        // Remove any trailing hint text
        if cleaned.ends_with(HINT_TEXT) {
            cleaned.strip_suffix(HINT_TEXT).unwrap_or(&cleaned).to_string()
        } else {
            cleaned
        }
    }
    
    fn get_clean_content_length(&self) -> usize {
        self.clean_content(&self.page_content_buffer).len()
    }
    
    fn ensure_notebook(&mut self) {
        if self.notebook.is_none() {
            let mut notebook = Notebook::new("Default Notebook".to_string());
            let welcome_page = Page::new(
                "Welcome to NotebookX".to_string(),
                "Welcome to NotebookX!\n\nThis is your first page. You can:\n- Create new pages\n- Edit page titles and content\n- Save your notebook\n- Open existing notebooks\n\nStart writing your notes here!".to_string(),
                Some(1),
            );
            notebook.add_page(welcome_page);
            self.notebook = Some(notebook);
            
            if let Some(notebook) = &self.notebook {
                if let Some(first_page) = notebook.pages.first() {
                    let page_id = first_page.id.clone();
                    self.select_page(&page_id);
                }
            }
        }
    }
    
    fn select_page(&mut self, page_id: &str) {
        if let Some(notebook) = &self.notebook {
            if let Some(page) = notebook.get_page(page_id) {
                self.current_page_id = Some(page_id.to_string());
                self.page_title_buffer = page.title.clone();
                self.page_content_buffer = page.content.clone();
            }
        }
    }
    
    fn save_current_page(&mut self) {
        let clean_content = self.clean_content(&self.page_content_buffer);
        
        if let (Some(notebook), Some(page_id)) = (&mut self.notebook, &self.current_page_id) {
            notebook.update_page(
                page_id,
                self.page_title_buffer.clone(),
                clean_content,
            );
            
            // Auto-save to file if enabled and file path exists
            if self.autosave_enabled {
                if let Some(file_path) = &self.current_file_path {
                    let _ = self.file_handler.save_notebook(notebook, file_path.clone());
                }
            }
        }
    }
    
    fn handle_page_overflow(&mut self) -> bool {
        let clean_content = self.clean_content(&self.page_content_buffer);
        if clean_content.len() <= PAGE_MAX_CHARS {
            return false;
        }
        
        if let Some(notebook) = &mut self.notebook {
            // Save current page first to preserve content
            if let Some(page_id) = &self.current_page_id {
                notebook.update_page(
                    page_id,
                    self.page_title_buffer.clone(),
                    clean_content.clone(),
                );
            }
            
            // Find a good break point (prefer line breaks)
            let mut split_point = PAGE_MAX_CHARS;
            let chars: Vec<char> = clean_content.chars().collect();
            
            // Look backwards for a good break point (newline or space)
            for i in (PAGE_MAX_CHARS.saturating_sub(200)..PAGE_MAX_CHARS.min(chars.len())).rev() {
                if chars[i] == '\n' {
                    split_point = i + 1;
                    break;
                } else if chars[i] == ' ' {
                    split_point = i + 1;
                }
            }
            
            // Split the clean content
            let current_content: String = chars.iter().take(split_point).collect();
            let overflow_content: String = chars.iter().skip(split_point).collect();
            
            // Update current page with truncated content
            self.page_content_buffer = current_content.clone();
            
            // Save the updated current page immediately
            if let Some(page_id) = &self.current_page_id {
                notebook.update_page(
                    page_id,
                    self.page_title_buffer.clone(),
                    current_content,
                );
            }
            
            // Create new page with overflow content
            let current_page_title = self.page_title_buffer.clone();
            let new_page_title = if current_page_title.contains("(cont.)") {
                current_page_title.clone()
            } else {
                format!("{} (cont.)", current_page_title)
            };
            
            let new_page = Page::new(
                new_page_title.clone(),
                overflow_content.clone(),
                None,
            );
            
            let new_page_id = new_page.id.clone();
            notebook.add_page(new_page);
            
            // Switch to the new page immediately
            self.current_page_id = Some(new_page_id);
            self.page_title_buffer = new_page_title;
            self.page_content_buffer = overflow_content;
            
            return true;
        }
        false
    }
    
    fn create_new_page(&mut self) {
        if let Some(notebook) = &mut self.notebook {
            let new_page = Page::new(
                "New Page".to_string(),
                "".to_string(),
                None,
            );
            let page_id = new_page.id.clone();
            notebook.add_page(new_page);
            self.select_page(&page_id);
        }
    }
    
    fn open_notebook(&mut self) {
        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter("NotebookX Files", &["txt"])
            .pick_file()
        {
            match self.file_handler.load_notebook(file_path.clone()) {
                Ok(notebook) => {
                    self.notebook = Some(notebook);
                    self.current_file_path = Some(file_path);
                    if let Some(first_page) = self.notebook.as_ref().unwrap().pages.first() {
                        let page_id = first_page.id.clone();
                        self.select_page(&page_id);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load notebook: {}", e);
                }
            }
        }
    }
    
    fn save_notebook(&mut self) {
        self.save_current_page(); // Save current changes first
        
        if let Some(notebook) = &self.notebook {
            if let Some(file_path) = rfd::FileDialog::new()
                .add_filter("NotebookX Files", &["txt"])
                .save_file()
            {
                match self.file_handler.save_notebook(notebook, file_path.clone()) {
                    Ok(_) => {
                        self.current_file_path = Some(file_path);
                        println!("Notebook saved successfully");
                    }
                    Err(e) => {
                        eprintln!("Failed to save notebook: {}", e);
                    }
                }
            }
        }
    }
    
    fn convert_onenote_file(&mut self) {
        if let Some(file_path) = rfd::FileDialog::new()
            .add_filter("OneNote Files", &["one"])
            .pick_file()
        {
            match self.onenote_converter.convert_to_notebookx(file_path.clone()) {
                Ok(mut converted_notebook) => {
                    // Add a conversion report page
                    match self.onenote_converter.create_conversion_report(file_path) {
                        Ok(report_page) => {
                            converted_notebook.add_page(report_page);
                        }
                        Err(e) => {
                            eprintln!("Failed to create conversion report: {}", e);
                        }
                    }
                    
                    self.notebook = Some(converted_notebook);
                    if let Some(first_page) = self.notebook.as_ref().unwrap().pages.first() {
                        let page_id = first_page.id.clone();
                        self.select_page(&page_id);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to convert OneNote file: {}", e);
                }
            }
        }
    }
}

impl eframe::App for NotebookXApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ensure_notebook();
        
        egui::SidePanel::left("pages_panel")
            .min_width(300.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("NotebookX");
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("New Page").clicked() {
                            self.create_new_page();
                        }
                        if ui.button("Open").clicked() {
                            self.open_notebook();
                        }
                        if ui.button("Save").clicked() {
                            self.save_notebook();
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Convert OneNote").clicked() {
                            self.convert_onenote_file();
                        }
                    });
                    
                    ui.separator();
                    
                    // Autosave toggle
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.autosave_enabled, "Auto-save");
                        if self.autosave_enabled && self.current_file_path.is_some() {
                            ui.colored_label(egui::Color32::from_rgb(0, 128, 0), "✓");
                        } else if self.autosave_enabled {
                            ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "⚠ No file");
                        }
                    });
                    
                    ui.separator();
                    
                    let mut selected_page_id: Option<String> = None;
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if let Some(notebook) = &self.notebook {
                            for page in &notebook.pages {
                                let is_selected = self.current_page_id.as_ref() == Some(&page.id);
                                let response = ui.selectable_label(
                                    is_selected,
                                    format!("{}\n#{} • {}", 
                                        if page.title.is_empty() { "Untitled" } else { &page.title },
                                        page.number.unwrap_or(0),
                                        page.created.format("%m/%d/%Y")
                                    )
                                );
                                
                                if response.clicked() {
                                    selected_page_id = Some(page.id.clone());
                                }
                            }
                        }
                    });
                    
                    if let Some(page_id) = selected_page_id {
                        self.save_current_page();
                        self.select_page(&page_id);
                    }
                });
            });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    let title_response = ui.text_edit_singleline(&mut self.page_title_buffer);
                    if title_response.changed() {
                        // Auto-save on title change with a delay would be implemented here
                    }
                });
                
                // Metadata display
                if let (Some(notebook), Some(page_id)) = (&self.notebook, &self.current_page_id) {
                    if let Some(page) = notebook.get_page(page_id) {
                        ui.label(format!(
                            "Page {} • Created: {} • Modified: {}",
                            page.number.unwrap_or(0),
                            page.created.format("%m/%d/%Y %H:%M"),
                            page.modified.format("%m/%d/%Y %H:%M")
                        ));
                    }
                }
                
                ui.separator();
                
                // Page size indicator
                let chars_used = self.get_clean_content_length();
                let chars_remaining = PAGE_MAX_CHARS.saturating_sub(chars_used);
                let usage_percent = (chars_used as f32 / PAGE_MAX_CHARS as f32 * 100.0).min(100.0);
                
                ui.horizontal(|ui| {
                    ui.label(format!("Page usage: {:.1}% ({}/{})", usage_percent, chars_used, PAGE_MAX_CHARS));
                    if chars_remaining < 500 {
                        ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "⚠ Near page limit");
                    }
                    if chars_used > PAGE_MAX_CHARS {
                        ui.colored_label(egui::Color32::from_rgb(255, 0, 0), "⚠ Page overflow - will auto-split");
                    }
                });
                
                ui.separator();
                
                // Content editor with scrolling
                egui::ScrollArea::vertical()
                    .stick_to_bottom(false)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let text_edit = egui::TextEdit::multiline(&mut self.page_content_buffer)
                            .desired_width(f32::INFINITY)
                            .desired_rows(30)
                            .hint_text(HINT_TEXT)
                            .font(egui::TextStyle::Monospace)
                            .code_editor();
                        
                        let content_response = ui.add_sized([ui.available_width(), ui.available_height()], text_edit);
                        
                        // Handle keyboard shortcuts
                        if content_response.has_focus() {
                            if ui.input(|i| i.key_pressed(egui::Key::PageDown)) {
                                // Scroll down
                                ui.scroll_with_delta(egui::Vec2::new(0.0, -300.0));
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::PageUp)) {
                                // Scroll up  
                                ui.scroll_with_delta(egui::Vec2::new(0.0, 300.0));
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::Home) && i.modifiers.ctrl) {
                                // Go to beginning of document
                                ui.scroll_to_rect(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(1.0, 1.0)), None);
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::End) && i.modifiers.ctrl) {
                                // Go to end of document
                                ui.scroll_to_rect(egui::Rect::from_min_size(egui::Pos2::new(0.0, f32::MAX), egui::Vec2::new(1.0, 1.0)), None);
                            }
                        }
                        
                        if content_response.changed() {
                            // Check for immediate page overflow using clean content
                            if self.get_clean_content_length() > PAGE_MAX_CHARS {
                                self.handle_page_overflow();
                            } else if self.autosave_enabled {
                                // Auto-save if enabled and content changed
                                self.save_current_page();
                            }
                        }
                    });
            });
        });
        
    }
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_current_page();
    }
}