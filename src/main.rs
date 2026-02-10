use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// A CLI tool to display folder structures in readable text format
#[derive(Parser, Debug)]
#[command(name = "file-struct-stringer")]
#[command(about = "Convert folder structures into readable text format", long_about = None)]
struct Cli {
    /// Target directory to display (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// List only folders, no files
    #[arg(short, long)]
    folders_only: bool,

    /// Filter by file extensions (comma-separated, e.g., "rs,toml")
    #[arg(short = 'e', long, value_delimiter = ',')]
    format: Option<Vec<String>>,

    /// Number of dashes in branch characters (default: 2)
    #[arg(short, long, default_value = "2")]
    dashes: usize,
}

struct DisplayOptions {
    folders_only: bool,
    extensions: Option<Vec<String>>,
    dash_count: usize,
}

fn main() {
    let cli = Cli::parse();

    // Validate path exists
    if !cli.path.exists() {
        eprintln!("Error: Path '{}' does not exist", cli.path.display());
        std::process::exit(1);
    }

    let options = DisplayOptions {
        folders_only: cli.folders_only,
        extensions: cli.format,
        dash_count: cli.dashes,
    };

    // Display the directory structure
    display_tree(&cli.path, &options);
}

fn display_tree(root_path: &Path, options: &DisplayOptions) {
    // Print the root directory name
    println!(
        "{}/",
        root_path
            .file_name()
            .unwrap_or(root_path.as_os_str())
            .to_string_lossy()
    );

    // Get all entries and sort them
    let mut entries: Vec<DirEntry> = WalkDir::new(root_path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| !is_ignored(e))
        .filter_map(|e| e.ok())
        .filter(|e| should_include(e, options))
        .collect();

    entries.sort_by(|a, b| a.path().cmp(b.path()));

    // Build a tree structure for proper formatting
    for (idx, entry) in entries.iter().enumerate() {
        let is_last = idx == entries.len() - 1;
        format_entry(entry, root_path, is_last, &entries, options.dash_count);
    }
}

fn is_ignored(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();

    // Skip common ignored directories
    let ignored_dirs = [".git", "node_modules", "target", ".idea", ".vscode"];

    if entry.file_type().is_dir() && ignored_dirs.contains(&file_name.as_ref()) {
        return true;
    }

    false
}

fn should_include(entry: &DirEntry, options: &DisplayOptions) -> bool {
    let is_dir = entry.file_type().is_dir();

    // Always include directories
    if is_dir {
        return true;
    }

    // If folders-only mode, skip files
    if options.folders_only {
        return false;
    }

    // If extension filter is specified, check if file matches
    if let Some(ref extensions) = options.extensions {
        if let Some(ext) = entry.path().extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            return extensions.iter().any(|e| e.to_lowercase() == ext_str);
        }
        // No extension, skip it
        return false;
    }

    // No filter, include everything
    true
}

fn format_entry(
    entry: &DirEntry,
    root: &Path,
    _is_last: bool,
    all_entries: &[DirEntry],
    dash_count: usize,
) {
    let path = entry.path();
    let relative_path = path.strip_prefix(root).unwrap();

    // Calculate depth and build prefix
    let depth = relative_path.components().count() - 1;
    let mut prefix = String::new();

    // Build the tree structure prefix
    if depth > 0 {
        // Get parent path components to determine if we need vertical bars
        let components: Vec<_> = relative_path.components().collect();

        for d in 0..depth {
            let ancestor_path: PathBuf = components[..=d].iter().collect();
            let ancestor_full = root.join(&ancestor_path);

            // Check if the ancestor at this level has more siblings after it
            // This determines if we need a vertical bar at this position
            let needs_vertical_bar = !is_last_sibling(&ancestor_full, all_entries, root);

            if needs_vertical_bar {
                prefix.push_str("│   ");
            } else {
                prefix.push_str("    ");
            }
        }
    }

    // Generate branch character dynamically based on dash_count
    let dashes = "─".repeat(dash_count);
    let branch = if is_last_sibling(path, all_entries, root) {
        format!("└{}", dashes)
    } else {
        format!("├{}", dashes)
    };

    // Format the output
    let name = path.file_name().unwrap().to_string_lossy();
    if entry.file_type().is_dir() {
        println!("{}{} {}/", prefix, branch, name);
    } else {
        println!("{}{} {}", prefix, branch, name);
    }
}

fn is_last_sibling(path: &Path, all_entries: &[DirEntry], _root: &Path) -> bool {
    let parent = path.parent().unwrap();

    // Find all siblings (entries with same parent)
    let siblings: Vec<_> = all_entries
        .iter()
        .filter(|e| {
            if let Some(p) = e.path().parent() {
                p == parent
            } else {
                false
            }
        })
        .collect();

    // Check if this is the last sibling
    if let Some(last) = siblings.last() {
        last.path() == path
    } else {
        false
    }
}

