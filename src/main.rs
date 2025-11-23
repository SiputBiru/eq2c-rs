use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::time::Instant;

use skybox_converter::{codecs, process};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long, value_enum, default_value_t = FormatArg::Png)]
    format: FormatArg,

    #[arg(short, long, default_value_t = 512)]
    size: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FormatArg {
    Png,
    Exr,
}

fn main() {
    let args = Cli::parse();
    let start_time = Instant::now();

    println!("Loading {}...", args.input.display());
    let img_result = image::open(&args.input);

    let img = match img_result {
        Ok(i) => i.into_rgb32f(), // Force internal logic to use Float32
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            return;
        }
    };

    println!("Projecting to Cross Layout (Face Size: {})...", args.size);
    let options = process::ConvertOptions {
        face_size: args.size,
    };

    let result_buffer = process::generate_cross_layout(&img, &options);

    println!("Encoding to output...");

    let format = match args.format {
        FormatArg::Png => codecs::OutputFormat::Png,
        FormatArg::Exr => codecs::OutputFormat::Exr,
    };

    let encoder = codecs::get_encoder(format);

    if let Err(e) = encoder.encode(&result_buffer, &args.output) {
        eprintln!("Error saving file: {}", e);
    } else {
        let duration = start_time.elapsed();
        println!(
            "Success! Saved to {} in {:.2?}",
            args.output.display(),
            duration
        );
    }
}
