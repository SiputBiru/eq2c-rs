use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

use eq2c::codecs::ToneMapType;
use eq2c::math::CubeFace;
use eq2c::{codecs, layouts};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    after_help = "EXAMPLES:\n  \
        # Convert HDR to 6 Separate PNGs\n  \
        eq2c -i input.exr -o skybox.png --layout separate\n\n  \
        # Convert to Horizontal Strip\n  \
        eq2c -i input.hdr -o strip.png --layout strip-h"
)]
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

    #[arg(short, long, default_value_t = 1.0)]
    exposure: f32,

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
    Cross,
    StripH,
    StripV,
    Separate,
}

fn main() {
    let args = Cli::parse();
    let start_time = Instant::now();

    println!("Loading {}...", args.input.display());
    let img_result = image::open(&args.input);
    let img = match img_result {
        Ok(i) => i.into_rgb32f(),
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            std::process::exit(1);
        }
    };

    // --- Stats Check ---
    let raw_pixels = img.as_raw();
    let max_brightness = raw_pixels
        .par_iter()
        .cloned()
        .reduce(|| 0.0f32, |a, b| a.max(b));
    if max_brightness > 10.0 {
        let suggested = 1.0 / (max_brightness * 0.1);
        println!(
            "Note: Max Brightness = {:.2}. Recommended exposure: ~{:.4}",
            max_brightness, suggested
        );
    }

    println!("Generating layout...");

    let layout_type = match args.layout {
        LayoutArg::Cross => layouts::LayoutType::Cross,
        LayoutArg::StripH => layouts::LayoutType::StripHorizontal,
        LayoutArg::StripV => layouts::LayoutType::StripVertical,
        LayoutArg::Separate => layouts::LayoutType::Separate,
    };

    let layout_output = layouts::generate_layout(layout_type, &img, args.size);

    println!(
        "Encoding to output (Tone Map: {:?}, Exposure: {})...",
        args.tonemap, args.exposure
    );

    let format_type = match args.format {
        FormatArg::Png => codecs::OutputFormat::Png,
        FormatArg::Exr => codecs::OutputFormat::Exr,
    };
    let encoder = codecs::get_encoder(format_type, args.tonemap, args.exposure);

    // --- Save Logic ---
    match layout_output {
        layouts::LayoutOutput::Single(buffer) => match encoder.encode(&buffer, &args.output) {
            Ok(_) => println!("Success! Saved to {}", args.output.display()),
            Err(e) => eprintln!("Error: {}", e),
        },

        layouts::LayoutOutput::Frames(faces) => {
            for (face, buffer) in faces {
                // Determine Suffix (px, nx, py, ny, pz, nz)
                let suffix = match face {
                    CubeFace::Right => "px",
                    CubeFace::Left => "nx",
                    CubeFace::Top => "py",
                    CubeFace::Bottom => "ny",
                    CubeFace::Front => "pz",
                    CubeFace::Back => "nz",
                };

                // Inject suffix into filename: "output.png" -> "output_px.png"
                let new_path = append_suffix(&args.output, suffix);

                match encoder.encode(&buffer, &new_path) {
                    Ok(_) => println!("Saved {}", new_path.display()),
                    Err(e) => eprintln!("Error saving {}: {}", suffix, e),
                }
            }
        }
    }

    println!("Total Time: {:?}", start_time.elapsed());
}

// Helper to modify paths
fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("png");
    path.with_file_name(format!("{}_{}.{}", stem, suffix, ext))
}
