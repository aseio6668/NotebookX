# NotebookX

A cross-platform GUI notebook application built in Rust, similar to Microsoft OneNote, with support for converting OneNote files to a custom text-based format.

## Features

- **Cross-platform GUI**: Built with egui/eframe for native performance on Windows, macOS, and Linux
- **OneNote-like Interface**: Left sidebar with page list, main content editing area
- **Custom File Format**: Simple text-based `.txt` format with embedded metadata
- **OneNote Conversion**: Basic converter for OneNote `.one` files (with limitations)
- **Page Management**: Create, edit, and navigate between pages
- **Automatic Numbering**: Pages are automatically numbered and timestamped
- **File Operations**: Open, save, and manage notebook files

## NotebookX File Format

NotebookX uses a simple text-based format that embeds metadata within the file:

```
--- NOTEBOOKX NOTEBOOK ---
NOTEBOOK_ID: unique-id
NOTEBOOK_TITLE: My Notebook
CREATED: 2025-08-14T10:30:00Z
MODIFIED: 2025-08-14T10:30:00Z
--- END NOTEBOOK HEADER ---

--- NOTEBOOKX METADATA ---
PAGE_ID: page-unique-id
TITLE: Page Title
NUMBER: 1
CREATED: 2025-08-14T10:30:00Z
MODIFIED: 2025-08-14T10:30:00Z
--- END METADATA ---

Page content goes here...
Multiple lines supported.

--- PAGE BREAK ---

--- NOTEBOOKX METADATA ---
TITLE: Another Page
NUMBER: 2
CREATED: 2025-08-14T11:00:00Z
MODIFIED: 2025-08-14T11:00:00Z
--- END METADATA ---

Second page content...
```

## OneNote Conversion

The OneNote converter provides basic conversion functionality:

- **Placeholder Implementation**: Creates a conversion report with extracted text
- **Text Extraction**: Attempts to extract readable text from OneNote files
- **Conversion Report**: Generates a detailed report of the conversion process

**Note**: Full OneNote support would require implementing the complete MS-ONESTORE specification. The current implementation is a basic demonstration.

## Getting Started

### Prerequisites

- Rust 1.70+ 
- Cargo

### Building

```bash
git clone <repository-url>
cd notebookx
cargo build --release
```

### Running

```bash
cargo run
```

## Usage

1. **Creating Pages**: Click "New Page" to create a new page
2. **Editing**: Click on a page in the sidebar to select it, then edit the title and content
3. **Saving**: Click "Save" to save your notebook to a `.txt` file
4. **Opening**: Click "Open" to load an existing NotebookX file
5. **Converting OneNote**: Click "Convert OneNote" to import a `.one` file

## Architecture

### Core Components

- **`notebook.rs`**: Core data structures (`Notebook`, `Page`) with metadata management
- **`file_io.rs`**: File I/O handler for the NotebookX format
- **`onenote_converter.rs`**: OneNote file conversion (basic implementation)
- **`main.rs`**: GUI implementation using egui

### Data Structures

```rust
struct Notebook {
    id: String,
    title: String,
    pages: Vec<Page>,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}

struct Page {
    id: String,
    title: String,
    content: String,
    number: Option<u32>,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}
```

## Future Enhancements

- **Rich Text Formatting**: Add support for bold, italic, links, etc.
- **Image Support**: Embed and display images
- **Search Functionality**: Full-text search across all pages
- **Export Options**: Export to PDF, HTML, Markdown
- **Full OneNote Support**: Complete implementation of MS-ONESTORE specification
- **Synchronization**: Cloud sync capabilities
- **Themes**: Dark/light mode support
- **Plugin System**: Extensible architecture for custom features

## Dependencies

- `egui`: Immediate mode GUI framework
- `eframe`: Application framework for egui
- `serde`: Serialization framework
- `chrono`: Date and time handling
- `uuid`: UUID generation
- `rfd`: Native file dialogs

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Acknowledgments

- Microsoft for the OneNote file format specifications
- The Rust community for excellent GUI libraries
- OneNote for inspiration on the user interface design