use eframe::egui;

mod notebook;
mod file_io;
mod onenote_converter;

use notebook::{Notebook, Page};
use file_io::NotebookFileHandler;
use onenote_converter::OneNoteConverter;

fn main() -> Result<(), eframe::Error> {
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
    file_handler: NotebookFileHandler,
    onenote_converter: OneNoteConverter,
    show_open_dialog: bool,
    show_save_dialog: bool,
    show_convert_dialog: bool,
}

impl NotebookXApp {
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
        if let (Some(notebook), Some(page_id)) = (&mut self.notebook, &self.current_page_id) {
            notebook.update_page(
                page_id,
                self.page_title_buffer.clone(),
                self.page_content_buffer.clone(),
            );
        }
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
            match self.file_handler.load_notebook(file_path) {
                Ok(notebook) => {
                    self.notebook = Some(notebook);
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
                match self.file_handler.save_notebook(notebook, file_path) {
                    Ok(_) => {
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
                
                // Content editor
                let content_response = ui.add(
                    egui::TextEdit::multiline(&mut self.page_content_buffer)
                        .desired_width(f32::INFINITY)
                        .desired_rows(25)
                );
                
                if content_response.changed() {
                    // Auto-save on content change with a delay would be implemented here
                }
            });
        });
        
    }
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_current_page();
    }
}