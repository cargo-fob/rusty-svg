use clap::Parser;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;
use convert_case::{Casing, Case};

#[derive(Parser)]
struct Args {
    /// Input directory containing SVG files
    #[arg(short, long)]
    input: String,

    /// Output directory to save React components
    #[arg(short, long)]
    output: String,

    /// Use TypeScript (.tsx) instead of .jsx
    #[arg(long)]
    typescript: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let input_path = PathBuf::from(&args.input);
    let output_path = PathBuf::from(&args.output);
    fs::create_dir_all(&output_path)?;

    for entry in WalkDir::new(&input_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "svg"))
    {
        
        let svg_path = entry.path();
        println!("svg 경로: {:?}", &svg_path);

        let svg_content = fs::read_to_string(svg_path)?;
        println!("svg_content: {:?}", &svg_content);

        let file_stem = svg_path.file_stem().unwrap().to_string_lossy();
        let component_name = format!("Icon{}", file_stem.to_case(Case::Pascal));
        let ext = if args.typescript { "tsx" } else { "jsx" };

        let component_code = format!(
            r#"import React from 'react';

const {name} = () => (
    {svg}
);

export default {name};
"#,
            name = component_name,
            svg = svg_content.replace("<svg", "<svg {...props}")
        );

        let out_file = output_path.join(format!("{}.{}", component_name, ext));
        fs::write(out_file, component_code)?;
        println!("✔️ Generated: {}", component_name);
    }

    Ok(())
}
