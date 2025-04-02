use clap::Parser;
use inquire::{Text, Select};
use std::{fs, path::PathBuf};
use walkdir::WalkDir;
use convert_case::{Casing, Case};

#[derive(Parser, Debug)]
#[command(name = "rusty-svg")]
struct Args {
    /// Input directory
    #[arg(short, long)]
    input: Option<String>,

    /// Output directory
    #[arg(short, long)]
    output: Option<String>,

    /// Use TypeScript
    #[arg(long)]
    typescript: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let input_given = args.input.is_some();
    let output_given = args.output.is_some();

    // Input 디렉토리 처리
    let input = args.input.unwrap_or_else(|| {
        Text::new("Input folder?")
            .with_placeholder("icons")
            .prompt()
            .unwrap_or("icons".to_string())
    });

    // Output 디렉토리 처리
    let output = args.output.unwrap_or_else(|| {
        Text::new("Output folder?")
            .with_placeholder("components")
            .prompt()
            .unwrap_or("components".to_string())
    });

    let output_path = PathBuf::from(&output);
    if output_path.exists() {
        let overwrite = Select::new(
            "Output folder already exists. Overwrite?",
            vec!["Yes", "No"]
        )
        .with_starting_cursor(1)
        .prompt()
        .unwrap_or("No") == "Yes";

        if !overwrite {
            println!("❌ Operation canceled.");
            return Ok(());
        }
        fs::remove_dir_all(&output_path)?;
    }
    fs::create_dir_all(&output_path)?;

    // 타입스크립트 여부
    let use_ts = if input_given && output_given {
        args.typescript
    } else {
        Select::new("Use TypeScript?", vec!["Yes", "No"])
            .with_starting_cursor(0)
            .prompt()
            .unwrap_or("No") == "Yes"
    };

    // SVG 파일 처리
    let input_path = PathBuf::from(&input);
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
        let ext = if use_ts { "tsx" } else { "jsx" };

        let component_code = if use_ts {
            format!(
                r#"import React from 'react';

type Props = React.SVGProps<SVGSVGElement>;

const {name} = (props: Props) => (
    {svg}
);

export default {name};
"#,
                name = component_name,
                svg = svg_content.replace("<svg", "<svg {...props}")
            )
        } else {
            format!(
                r#"import React from 'react';

const {name} = (props) => (
    {svg}
);

export default {name};
"#,
                name = component_name,
                svg = svg_content.replace("<svg", "<svg {...props}")
            )
        };

        let out_file = output_path.join(format!("{}.{}", component_name, ext));
        fs::write(out_file, component_code)?;
        println!("✔️ Generated: {}", component_name);
    }

    Ok(())
}
