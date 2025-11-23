use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::time::Instant;

use skybox_converter::{codecs, layouts};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long, value_enum, default_value_t = FormatArg::Png)]
    format: FormatArg,

    #[arg(short, long, value_enum, default_value_t = LayoutArg::Cross)]
    layout: LayoutArg,

    #[arg(short, long, default_value_t = 512)]
    size: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FormatArg {
    Png,
    Exr,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LayoutArg {
    Cross,  // Standard Cubemap Cross
    StripH, // Horizontal Strip (6x1)
    StripV, // Vertical Strip (1x6)
}

fn main() {
    let args = Cli::parse();
    let start_time = Instant::now();

    // 1. Load Image
    println!("Loading {}...", args.input.display());
    let img_result = image::open(&args.input);

    let img = match img_result {
        Ok(i) => i.into_rgb32f(),
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            // Exit with error code 1
            std::process::exit(1);
        }
    };

    // 2. Process Layout (The Heavy Math)
    println!("Generating layout...");

    // Map CLI Enum -> Library Enum
    let layout_type = match args.layout {
        LayoutArg::Cross => layouts::LayoutType::Cross,
        LayoutArg::StripH => layouts::LayoutType::StripHorizontal,
        LayoutArg::StripV => layouts::LayoutType::StripVertical,
    };

    let result_buffer = layouts::generate_layout(layout_type, &img, args.size);

    // 3. Encode & Save
    println!("Encoding to output...");

    let format_type = match args.format {
        FormatArg::Png => codecs::OutputFormat::Png,
        FormatArg::Exr => codecs::OutputFormat::Exr,
    };

    let encoder = codecs::get_encoder(format_type);

    match encoder.encode(&result_buffer, &args.output) {
        Ok(_) => {
            let duration = start_time.elapsed();
            println!(
                "Success! Saved to {} ({:?})",
                args.output.display(),
                duration
            );
        }
        Err(e) => {
            eprintln!("Error saving file: {}", e);
            std::process::exit(1);
        }
    }
}
