use clap::Parser;
use inquire::{Text, Select};
use regex::Regex;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;
use convert_case::{Casing, Case};
use serde::Deserialize;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(name = "rusty-svg")]
struct Args {
    /// Input directory
    #[arg(short, long, required = false)]
    input: Option<String>,

    /// Output directory
    #[arg(short, long, required = false)]
    output: Option<String>,

    /// Use TypeScript
    #[arg(long)]
    typescript: bool,
}

#[derive(Deserialize, Debug)]
struct Config {
    input: Option<String>,
    output: Option<String>,
    typescript: Option<bool>,
    case: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config: Option<Config> = fs::read_to_string("rusty-svg.config.toml")
        .ok()
        .and_then(|c| toml::from_str::<Config>(&c).ok());

    let input_given = args.input.is_some();
    let output_given = args.output.is_some();

    let input = args.input
        .or_else(|| config.as_ref().and_then(|c| c.input.clone()))
        .unwrap_or_else(|| {
            Text::new("Input folder?")
                .with_placeholder("icons")
                .prompt()
                .unwrap_or_else(|_| "icons".to_string())
        });

    let output = args.output
        .or_else(|| config.as_ref().and_then(|c| c.output.clone()))
        .unwrap_or_else(|| {
            Text::new("Output folder?")
                .with_placeholder("components")
                .prompt()
                .unwrap_or_else(|_| "components".to_string())
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

    let use_ts = if input_given && output_given {
        args.typescript
    } else {
        args.typescript
            || config.as_ref().and_then(|c| c.typescript).unwrap_or_else(|| {
            Select::new("Use TypeScript?", vec!["Yes", "No"])
                .with_starting_cursor(0)
                .prompt()
                .unwrap_or("No") == "Yes"
        })
    };

    let case_style = config
        .as_ref()
        .and_then(|c| c.case.clone())
        .unwrap_or_else(|| {
            Select::new("Component filename casing?", vec!["PascalCase", "kebab-case"])
                .with_starting_cursor(0)
                .prompt()
                .unwrap_or_else(|_| "PascalCase").to_owned()
        });

    let input_path = PathBuf::from(&input);
    for entry in WalkDir::new(&input_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e
            .path()
            .extension()
            .map_or(false, |ext| ext == "svg")
        )
    {
        let svg_path = entry.path();
        let svg_content = fs::read_to_string(svg_path)?;

        let file_stem = match svg_path.file_stem() {
            Some(stem) => stem.to_string_lossy(),
            None => {
                println!("❌ Invalid file name: {:?}", svg_path);
                "unknown".into()
            }
        };

        println!("🔍 Processing: {}", file_stem);

        let file_name = match case_style.as_str() {
            "PascalCase" => file_stem.to_case(Case::Pascal),
            "kebab-case" => file_stem.to_case(Case::Kebab),
            _ => file_stem.to_case(Case::Pascal),
        };

        let component_name = file_stem.to_case(Case::Pascal);
        let ext = if use_ts { "tsx" } else { "jsx" };

        let svg_tag_regex = Regex::new(r#"<svg([^>]*)>"#)?;

        let mut width = "24";
        let mut height = "24";
        let mut view_box = "0 0 24 24";

        if let Some(captures) = svg_tag_regex.captures(&svg_content) {
            if let Some(attrs) = captures.get(1) {
                let attrs_str = attrs.as_str();

                if let Some(w_cap) = Regex::new(r#"width="([^"]*)""#)?.captures(attrs_str) {
                    width = w_cap.get(1).unwrap().as_str();
                }

                if let Some(h_cap) = Regex::new(r#"height="([^"]*)""#)?.captures(attrs_str) {
                    height = h_cap.get(1).unwrap().as_str();
                }

                if let Some(vb_cap) = Regex::new(r#"viewBox="([^"]*)""#)?.captures(attrs_str) {
                    view_box = vb_cap.get(1).unwrap().as_str();
                }
            }
        }

        let new_svg_tag = format!(
            r#"<svg width="{}" height="{}" viewBox="{}" xmlns="http://www.w3.org/2000/svg" {{...props}}>"#,
            width, height, view_box
        );

        let mut cleaned_svg = svg_tag_regex
            .replace(&svg_content, &new_svg_tag)
            .to_string();

        let fill_regex = Regex::new(r#"fill="([^"]*)""#)?;
        cleaned_svg = fill_regex.replace_all(&cleaned_svg, r#"fill={props.fill || "$1" || "currentColor"}"#).to_string();

        cleaned_svg = cleaned_svg
            .replace("fill-rule", "fillRule")
            .replace("clip-rule", "clipRule")
            .replace("stroke-width", "strokeWidth")
            .replace("stroke-linecap", "strokeLinecap")
            .replace("stroke-linejoin", "strokeLinejoin")
            .replace("stroke-miterlimit", "strokeMiterlimit")
            .replace("stroke-dasharray", "strokeDasharray")
            .replace("stroke-dashoffset", "strokeDashoffset")
            .replace("stroke-opacity", "strokeOpacity")
            .replace("fill-opacity", "fillOpacity");

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
                svg = cleaned_svg
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
                svg = cleaned_svg
            )
        };

        let out_file = output_path.join(format!("{}.{}", file_name, ext));
        fs::write(out_file, component_code)?;
        println!("✔️ Generated: {} => {}.{}", component_name, file_name, ext);
    }

    Ok(())
}