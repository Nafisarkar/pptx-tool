use std::{fs, path::Path};

use pptx_to_md::{ParserConfig, PresentationContainer};

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir("./")? {
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("pptx") {
                    println!("Found PowerPoint file: {:?}", path.file_name().unwrap());
                    convert_pptx_to_pdf(&path);
                    convert_pptx_to_md(&path);
                }
            }
            Err(err) => {
                eprintln!("Failed to read directory entry: {}", err);
            }
        }
    }
    Ok(())
}

fn convert_pptx_to_md(input_path: &Path) {
    let output_path = input_path.with_extension("md");

    let mut presentation =
        PresentationContainer::open(input_path, ParserConfig::default()).unwrap();
    match presentation.convert_to_md_multi_threaded() {
        Ok(markdown) => match std::fs::write(&output_path, &markdown) {
            Ok(()) => println!("Markdown created successfully at: {:?}", output_path),
            Err(e) => println!(
                "Error while creating Markdown for {:?}: {:?}",
                output_path, e
            ),
        },
        Err(e) => {
            println!(
                "Error while parsing presentation for {:?}: {:?}",
                input_path, e
            );
        }
    }
}

fn convert_pptx_to_pdf(input_path: &Path) {
    let output_path = input_path.with_extension("pdf");
    match minipdf::convert_to_pdf(input_path, &output_path) {
        Ok(()) => println!("PDF created successfully at: {:?}", output_path),
        Err(e) => println!("Error while creating PDF for {:?}: {:?}", input_path, e),
    }
}
