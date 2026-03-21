//! gen-aarch32-cpu - a tool for generating (parts of) the aarch32-cpu crate
//!
//! The `aarch32-cpu` crate contains a lot of system register definitions. Copy-pasting these
//! from PDFs and converting them into Rust code is tedious. This tool automates that process
//! using the official Arm JSON files [1][arm-json].
//!
//! [arm-json]: https://developer.arm.com/Architectures/A-Profile%20Architecture#Downloads

use clap::Parser;

#[derive(clap::Parser)]
/// Turn Arm register specifications into Rust code
struct Args {
    /// The config file to use
    #[arg(short, long)]
    filename: std::path::PathBuf,
}

/// Entry point to the program
fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let args = Args::parse();
    log::info!("Loading config from file {}", args.filename.display());
    let config_str = std::fs::read_to_string(args.filename)?;
    gen_aarch32_cpu::process(&config_str)?;
    Ok(())
}
