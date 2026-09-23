use pptx_to_md::{ParserConfig, PresentationContainer};
use std::{fs, path::Path};

fn main() -> std::io::Result<()> {
    let mut converted = 0u32;

    for entry in fs::read_dir("./")? {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Failed to read directory entry: {err}");
                continue;
            }
        };

        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("pptx") {
            continue;
        }

        let name = path.file_name().unwrap().to_string_lossy();
        println!("Converting: {name}");

        if let Err(e) = convert_to_pdf(&path) {
            eprintln!("PDF conversion failed for {name}: {e}");
        } else {
            converted += 1;
        }

        if let Err(e) = convert_to_md(&path) {
            eprintln!("Markdown conversion failed for {name}: {e}");
        } else {
            converted += 1;
        }
    }

    println!("Done. {converted} file(s) generated.");
    Ok(())
}

fn convert_to_pdf(input: &Path) -> Result<(), String> {
    let output = input.with_extension("pdf");
    minipdf::convert_to_pdf(input, &output).map_err(|e| format!("{e}"))
}

fn convert_to_md(input: &Path) -> Result<(), String> {
    let output = input.with_extension("md");
    let mut presentation =
        PresentationContainer::open(input, ParserConfig::default()).map_err(|e| format!("{e}"))?;
    let markdown = presentation
        .convert_to_md_multi_threaded()
        .map_err(|e| format!("{e}"))?;
    fs::write(&output, &markdown).map_err(|e| format!("{e}"))
}
