use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let output_path = "output.md";
    let src_dir = "../src"; // Change this to your target directory

    // 1. Create or truncate the output file
    let mut output_file = File::create(output_path)?;
    writeln!(output_file, "# Source Code Export\n")?;

    // 2. Walk through the directory
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // 3. Only process files (skip directories)
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                // Filter for source code extensions
                match ext {
                    "rs" | "ts" | "js" | "vue" | "css" | "html" | "py" => {
                        append_file_to_markdown(path, &mut output_file, ext)?;
                    }
                    _ => continue,
                }
            }
        }
    }

    println!("Done! All code compressed into {}", output_path);
    Ok(())
}

fn append_file_to_markdown(path: &Path, output: &mut File, ext: &str) -> io::Result<()> {
    let mut content = String::new();
    let mut input_file = File::open(path)?;

    // Read file content
    if input_file.read_to_string(&mut content).is_ok() {
        println!("Compressing: {:?}", path);

        // Write header and code block
        writeln!(output, "## File: {:?}\n", path)?;
        writeln!(output, "```{}", ext)?;
        writeln!(output, "{}", content)?;
        writeln!(output, "```\n---\n")?;
    }
    Ok(())
}
