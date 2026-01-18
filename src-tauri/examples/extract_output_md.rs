use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

fn main() -> std::io::Result<()> {
    let markdown_file = "output.md";
    let target_dir = "../src";

    // 1. Define the extensions that count as "code files"
    let code_extensions = vec!["ts", "vue", "css", "js", "json"];

    // 2. Clean only the code files in the target directory
    if Path::new(target_dir).exists() {
        println!("Cleaning code files in: {}", target_dir);
        clean_code_files(Path::new(target_dir), &code_extensions)?;
    }

    // 3. Read and Parse the Markdown
    let content = fs::read_to_string(markdown_file)
        .expect("Error: 'output.md' not found.");

    // Regex to capture: ## File: "path" and the code block
    let re = Regex::new(r##"(?m)^## File: "(.+?)"\s+^```\w*\r?\n([\s\S]+?)\r?\n^```"##).unwrap();

    let mut count = 0;
    for cap in re.captures_iter(&content) {
        let raw_path = &cap[1];
        let file_content = &cap[2];

        // Normalize path for cross-platform (Windows \ to /)
        let normalized_path = raw_path.replace("\\", "/");
        let path = PathBuf::from(&normalized_path);

        // Create parent directories if they were deleted
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the fresh file
        fs::write(&path, file_content)?;
        println!("Restored: {:?}", path);
        count += 1;
    }

    println!("\nProcess complete. Restored {} code files.", count);
    Ok(())
}

/// Recursively deletes files with specific extensions
fn clean_code_files(dir: &Path, extensions: &[&str]) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                clean_code_files(&path, extensions)?;
            } else {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if extensions.contains(&ext) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }
    }
    Ok(())
}
