use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::time::Instant;

use eq2c::{self, Eq2cError, codecs::ToneMapType};

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

/// Entry point for the CLI application that parses arguments, builds an eq2c configuration, runs the conversion, and exits with a status code on error.
///
/// On success prints the total execution time. On failure prints the error in red and exits with a specific status code depending on the error kind:
/// - `Eq2cError::Io(_)` -> exit code 74
/// - `Eq2cError::Image(_)` -> exit code 65
/// - `Eq2cError::InvalidDimensions { .. }` -> exit code 64
/// - other errors -> exit code 1
///
/// # Examples
///
/// ```no_run
/// // Build and run the CLI binary, e.g.:
/// // cargo run -- --input input.hdr --output out.png --format Png --layout Cross
/// ```
fn main() {
    let args = Cli::parse();
    let start = Instant::now();

    let config = eq2c::Config {
        input: args.input,
        output: args.output,
        format: match args.format {
            FormatArg::Png => eq2c::OutputFormat::Png,
            FormatArg::Exr => eq2c::OutputFormat::Exr,
        },
        layout: match args.layout {
            LayoutArg::Cross => eq2c::LayoutType::Cross,
            LayoutArg::StripH => eq2c::LayoutType::StripHorizontal,
            LayoutArg::StripV => eq2c::LayoutType::StripVertical,
            LayoutArg::Separate => eq2c::LayoutType::Separate,
        },
        tonemap: args.tonemap,
        exposure: args.exposure,
        size: args.size,
    };

    if let Err(e) = eq2c::run(config) {
        eprintln!("\x1b[31mError:\x1b[0m {}", e);

        match e {
            Eq2cError::Io(_) => std::process::exit(74),
            Eq2cError::Image(_) => std::process::exit(65),
            Eq2cError::InvalidDimensions { .. } => std::process::exit(64),
            _ => std::process::exit(1),
        }
    }

    println!("Total Time: {:?}", start.elapsed());
}