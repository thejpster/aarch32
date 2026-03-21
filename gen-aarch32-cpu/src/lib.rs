//! gen-aarch32-cpu library code

use std::fmt::Write;

pub use arm_sysregs_json::RegisterEntry;

mod config;
mod register;

/// Process the given configuration
///
/// This is the function you should call to convert JSON to Rust, using a config
/// file.
///
/// Pass the *contents* of the TOML config file (load it however you like). The
/// input, generate and output parameters are stored within the configuration.
pub fn process(config_str: &str) -> Result<(), anyhow::Error> {
    let config: config::TopLevel = toml::from_str(config_str)?;
    let registers = config.parse_registers()?;
    std::fs::create_dir_all(&config.export.folder)?;
    export_top_level(&config.export.folder, &registers)?;
    export_modules(&config.export.folder, &registers)?;
    Ok(())
}

/// Export the top-level mod.rs containing the list of modules
fn export_top_level(folder: &std::path::Path, registers: &[register::Info]) -> Result<(), anyhow::Error> {
    let file = folder.join("mod.rs");
    let mut buffer = String::new();
    writeln!(buffer, "//! Auto-generated register definitions")?;
    writeln!(buffer)?;
    for register in registers {
        log::debug!("Found register {:?}", register);
        writeln!(buffer, "{}", register.module_definition()?)?;
    }
    writeln!(buffer)?;
    std::fs::write(file, buffer)?;
    Ok(())
}

/// Export the modules containing the register definitions
///
/// You get one module per register
fn export_modules(folder: &std::path::Path, registers: &[register::Info]) -> Result<(), anyhow::Error> {
    for register in registers {
        let name = register.name.to_ascii_lowercase();
        log::debug!("Creating folder for {}", name);
        let mut file = folder.join(&name);
        file.set_extension("rs");
        std::fs::write(&file, register.module_contents()?)?;
    }
    Ok(())
}
