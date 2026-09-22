use std::{fs, path::Path};

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir("./")? {
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("pptx") {
                    println!("Found PowerPoint file: {:?}", path.file_name().unwrap());
                    convert_pptx_to_pdf(&path);
                }
            }
            Err(err) => {
                eprintln!("Failed to read directory entry: {}", err);
            }
        }
    }
    Ok(())
}

fn convert_pptx_to_pdf(input_path: &Path) {
    let output_path = input_path.with_extension("pdf");
    match minipdf::convert_to_pdf(input_path, &output_path) {
        Ok(()) => println!("PDF created successfully at: {:?}", output_path),
        Err(e) => println!("Error while creating PDF for {:?}: {:?}", input_path, e),
    }
}
