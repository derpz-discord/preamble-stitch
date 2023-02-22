use clap::Parser;
use preamble_stitch::cli::CliArgs;
use std::path::Path;
use tracing::{debug, error, info};
use preamble_stitch::{get_config_from_fs, read_files, stitch_and_render, write_file};

fn main() {
    // setup tracing with env filter
    let cli_opts = CliArgs::parse();
    tracing_subscriber::fmt()
        .with_max_level(match cli_opts.verbosity {
            0 => tracing::Level::INFO,
            1 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        })
        .init();


    info!("Building for environment {:?}", cli_opts.environment);
    debug!("CLI Options: {:?}", cli_opts);
    let config = get_config_from_fs(Path::new(&cli_opts.config_path));
    if let Err(e) = config {
        error!("Error reading config file: {}", e);
        std::process::exit(1);
    }
    let config = config.unwrap();

    let files = read_files(&config.order);
    if let Err(e) = files {
        error!("Error reading files: {}", e);
        std::process::exit(1);
    }
    let files = files.unwrap();
    let _ = files.iter().map(|file| info!("+ {}", file.path.display()));
    let stitched = stitch_and_render(&files, cli_opts.environment);
    if let Err(e) = stitched {
        error!("Error stitching files: {}", e);
        std::process::exit(1);
    }
    let stitched = stitched.unwrap();
    let output_file = match &config.output_file {
        Some(path) => path,
        None => Path::new("output.sty"),
    };
    info!("Done, now writing to `{}`", output_file.display());
    let write_status = write_file(stitched, output_file);
    if let Err(e) = write_status {
        error!("Error writing file: {}", e);
        std::process::exit(1);
    }
    info!("Done!");

}
