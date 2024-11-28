mod cli;
mod config;
mod logger;

use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_arguments()?;
    logger::init_logger(&args.log_level)?;

    log::info!("Leyendo archivo de configuración: {}", args.config_path);

    // Leer y deserializar el archivo YAML
    let config = config::read_config(&args.config_path)?;
    log::info!("Archivo YAML leído con éxito: {:?}", config);

    Ok(())
}
