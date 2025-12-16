use serde::Serialize;
use std::fs;
use std::path::Path;

// --- Data Models ---

#[derive(Serialize, Debug, Clone)]
struct TreeNode {
    name: String,
    // Nivo requires specific fields for value/children
    #[serde(rename = "loc")] // 'loc' or 'value' for size
    size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TreeNode>>,

    // Metadata for Visualization
    entropy: f32,   // 0.0 to 8.0
    risk_score: u8, // 0 to 100
}

// --- Logic ---

// Mocking the Phase 2 entropy logic for the GUI example
fn calculate_mock_entropy(path: &Path) -> f32 {
    // In production, import spectra_core::analysis::entropy
    // Here, we simulate entropy based on extension
    if let Some(ext) = path.extension() {
        match ext.to_string_lossy().as_ref() {
            "zip" | "enc" => 7.8,
            "png" | "jpg" => 6.5,
            "rs" | "txt" | "md" => 3.2,
            _ => 4.0,
        }
    } else {
        4.0
    }
}

fn scan_directory_recursive(path: &Path, depth: usize, max_depth: usize) -> Option<TreeNode> {
    if depth > max_depth {
        return None;
    }

    let metadata = fs::metadata(path).ok()?;

    // Handle root directories (e.g., C:\, /, etc.) which don't have a file_name
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    if metadata.is_file() {
        let entropy = calculate_mock_entropy(path);
        return Some(TreeNode {
            name,
            size: metadata.len(),
            children: None,
            entropy,
            risk_score: (entropy * 10.0) as u8,
        });
    } else if metadata.is_dir() {
        let mut children = Vec::new();
        let mut dir_size = 0;
        let mut total_entropy = 0.0;
        let mut file_count = 0;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(node) = scan_directory_recursive(&entry.path(), depth + 1, max_depth) {
                    dir_size += node.size;
                    total_entropy += node.entropy; // Simplified aggregation
                    file_count += 1;
                    children.push(node);
                }
            }
        }

        // Directory entropy is average of children (simplified)
        let avg_entropy = if file_count > 0 {
            total_entropy / file_count as f32
        } else {
            0.0
        };

        return Some(TreeNode {
            name,
            size: dir_size,
            children: Some(children),
            entropy: avg_entropy,
            risk_score: (avg_entropy * 10.0) as u8,
        });
    }
    None
}

// --- Commands ---

#[tauri::command]
fn get_scan_tree(path: String) -> Result<TreeNode, String> {
    let root = Path::new(&path);

    // Check if path exists
    if !root.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    // Check if we have permission to read
    if let Err(e) = fs::metadata(root) {
        return Err(format!("Cannot access path: {}", e));
    }

    scan_directory_recursive(root, 0, 3) // Limit depth for demo performance
        .ok_or_else(|| format!("Failed to scan path: {}. Try a subdirectory instead.", path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_scan_tree])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
