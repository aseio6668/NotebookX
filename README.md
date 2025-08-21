# NotebookX

A cross-platform GUI notebook application built in Rust, similar to Microsoft OneNote, with support for converting OneNote files to a custom text-based format.

## Features

### Core Features
- **Cross-platform GUI**: Built with egui/eframe for native performance on Windows, macOS, and Linux
- **OneNote-like Interface**: Left sidebar with page list, main content editing area
- **Custom File Format**: Simple text-based `.txt` format with embedded metadata
- **OneNote Conversion**: Basic converter for OneNote `.one` files (with limitations)
- **Page Management**: Create, edit, and navigate between pages
- **Automatic Numbering**: Pages are automatically numbered and timestamped
- **File Operations**: Open, save, and manage notebook files

### Advanced Features
- **Page Size Limits**: Automatic page overflow based on standard US Letter paper dimensions (~3680 characters per page)
- **Auto-Split Pages**: When content exceeds page limits, automatically creates new continuation pages
- **Smart Page Breaks**: Finds natural break points (line breaks, spaces) when splitting pages
- **Scrollable Content**: Full scrollbar support with smooth scrolling in content area
- **Keyboard Navigation**: 
  - Page Up/Down for scrolling
  - Ctrl+Home/End for document start/end navigation
- **Page Usage Indicator**: Real-time display of page usage percentage and warnings
- **Windows Console Control**: No terminal window on Windows by default, use `--debug` flag to enable
- **Auto-save Toggle**: Optional auto-save feature with visual indicators in the top menu
- **Immediate Page Overflow**: Content automatically splits to new pages as you type when limit is exceeded
- **Content Preservation**: No data loss during page overflow - all content is preserved and properly transferred

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
# Normal mode (no console on Windows)
cargo run

# Debug mode (shows console on Windows)
cargo run -- --debug

# Release build
cargo run --release

# Or use the built executable
./target/release/notebookx --debug
```

### Command Line Options

- `--debug`: Enable debug mode (shows console window on Windows for debug output)
- `--help`: Show help information

## Usage

### Basic Operations
1. **Creating Pages**: Click "New Page" to create a new page
2. **Editing**: Click on a page in the sidebar to select it, then edit the title and content
3. **Saving**: Click "Save" to save your notebook to a `.txt` file
4. **Opening**: Click "Open" to load an existing NotebookX file
5. **Converting OneNote**: Click "Convert OneNote" to import a `.one` file
6. **Auto-save**: Toggle the "Auto-save" checkbox in the sidebar to enable automatic saving

### Advanced Usage
- **Page Overflow**: When typing exceeds the page limit (~3680 characters), a new continuation page is automatically created and you continue typing in the new page
- **Auto-save Feature**: 
  - Check "Auto-save" in the sidebar to enable automatic saving
  - Green checkmark (✓) indicates auto-save is active with a file
  - Orange warning (⚠ No file) indicates auto-save is enabled but no file is selected
  - Content is automatically saved on every change when enabled
- **Keyboard Navigation**: Use Page Up/Down to scroll, Ctrl+Home/End to jump to document boundaries
- **Page Monitoring**: Watch the page usage indicator to see how much space is remaining
- **Word Wrapping**: Content automatically wraps within the editor area
- **Smart Page Breaks**: Page overflow finds natural break points (line breaks, spaces) when splitting content
- **Clean Content Handling**: Hint text ("Start writing your notes here...") is automatically filtered out and never saved to files or included in page overflow calculations

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

## Troubleshooting

### Common Issues

- **Hint text appearing in saved content**: This has been fixed in the latest version. The placeholder text "Start writing your notes here..." is automatically filtered out and never saved to files or included in page overflow.
- **Page overflow not working**: Ensure you're typing actual content, not just the hint text. The page counter only includes real content.
- **Auto-save not working**: Make sure you have:
  1. Checked the "Auto-save" checkbox in the sidebar
  2. Saved the file at least once to establish a file path
  3. Look for the green checkmark (✓) indicating auto-save is active

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Acknowledgments

- Microsoft for the OneNote file format specifications
- The Rust community for excellent GUI libraries
- OneNote for inspiration on the user interface design