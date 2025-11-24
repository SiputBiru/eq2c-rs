use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::time::Instant;

use skybox_converter::codecs::ToneMapType;
use skybox_converter::{codecs, layouts};

#[derive(Parser)]
#[command(
    author, 
    version, 
    about, 
    long_about = None, 
    after_help = "EXAMPLES:\n  \
        # Convert HDR to PNG (Cross Layout)\n  \
        skybox_converter -i input.exr -o output.png\n\n  \
        # Convert to Horizontal Strip with ACES Tone Mapping\n  \
        skybox_converter -i input.hdr -o strip.png --layout strip-h --tonemap aces")]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long, value_enum, default_value_t = FormatArg::Png)]
    format: FormatArg,

    #[arg(short, long, value_enum, default_value_t = LayoutArg::Cross)]
    layout: LayoutArg,

    #[arg(short, long, value_enum, default_value_t = ToneMapType::Aces)]
    tonemap: ToneMapType,

    #[arg(short, long, default_value_t = 1024)]
    size: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FormatArg {
    Png,
    Exr,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LayoutArg {
    Cross,
    StripH,
    StripV,
}

fn main() {
    let args = Cli::parse();
    let start_time = Instant::now();

    println!("Loading {}...", args.input.display());

    // Load and convert to F32
    let img_result = image::open(&args.input);
    let img = match img_result {
        Ok(i) => i.into_rgb32f(),
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            std::process::exit(1);
        }
    };

    println!("Generating layout...");

    let layout_type = match args.layout {
        LayoutArg::Cross => layouts::LayoutType::Cross,
        LayoutArg::StripH => layouts::LayoutType::StripHorizontal,
        LayoutArg::StripV => layouts::LayoutType::StripVertical,
    };

    let result_buffer = layouts::generate_layout(layout_type, &img, args.size);

    println!("Encoding to output (Tone Map: {:?})...", args.tonemap);

    let format_type = match args.format {
        FormatArg::Png => codecs::OutputFormat::Png,
        FormatArg::Exr => codecs::OutputFormat::Exr,
    };

    // Pass the tonemap argument directly (it's already the correct Enum)
    let encoder = codecs::get_encoder(format_type, args.tonemap);

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
